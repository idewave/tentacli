use async_trait::async_trait;
use binrw::{BinRead, BinWrite};
use flate2::read::DeflateDecoder;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::Serialize;
use std::collections::BTreeMap;
use std::io::Read;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::realm::object::lifecycle::{
    ObjectLifecycle, ObjectLifecycleRegistry, ObjectRemovalReason, now_millis,
};
use crate::plugins::wow::wotlk::realm::object::types::movement::Movement;
use crate::plugins::wow::wotlk::realm::object::types::packed_guid::PackedGuid;
use crate::plugins::wow::wotlk::realm::object::types::update_data::{ObjectTypeMask, UpdateData};
use crate::plugins::wow::wotlk::realm::object::ObjectMap;
use crate::plugins::wow::wotlk::realm::object::types::update_fields::{
    ContainerField, CorpseField, DynamicObjectField, FieldValue, GameObjectField, ItemField,
    ObjectField, PlayerField, UnitField,
};

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
struct Incoming {
    blocks_amount: u32,
    #[br(count = blocks_amount)]
    blocks: Vec<Block>,
}

pub struct Handler {
    pub is_compressed: bool,
}

#[async_trait]
impl PacketHandler for Handler {
    async fn handle(
        &mut self,
        packet: &mut Packet,
        context: Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<Vec<HandlerOutput>> {
        let mut output = vec![];

        if self.is_compressed {
            let mut buffer = Vec::new();
            let mut decoder = DeflateDecoder::new(&packet.content.body[6..]);
            decoder.read_to_end(&mut buffer)?;

            packet.content.body = buffer;
        }

        let mut incoming = Incoming::unpack(packet)?;

        {
            let guard = context.read().await;
            let option = guard.get::<ObjectMap>();
            if let Some(objects) = option {
                for block in incoming.blocks.iter_mut() {
                    if let BlockType::Values = block.block_type
                        && let Some(guid) = block.guid.as_ref()
                        && let Some(object) = objects.get(guid)
                        && let Some(update_data) = block.update_data.as_mut()
                    {
                        sanitize_update_data(update_data, object.object_type_mask);
                    }
                }
            }
        }

        packet.set_packet_size(packet.content.body.len());
        packet.set_offset_info(ExtractMetadata::extract_metadata(&incoming));
        packet.set_json(serialize_packet_json(&incoming)?);

        let mut mutations = Vec::with_capacity(incoming.blocks.len());

        for block in incoming.blocks {
            match block.block_type {
                BlockType::CreateObject | BlockType::CreateObject2 => {
                    match Object::try_from(block) {
                        Ok(object) => mutations.push(ObjectMutation::Create(object)),
                        Err(err) => {
                            output.push(HandlerOutput::Messages(vec![Message {
                                msg_type: MsgType::Error,
                                text: format!("Failed to build Object from block: {:?}", err),
                            }]));
                        }
                    }
                }
                BlockType::Values => {
                    let guid = block
                        .guid
                        .ok_or_else(|| anyhow::anyhow!("Values block without guid"))?;
                    let update_data = block
                        .update_data
                        .ok_or_else(|| anyhow::anyhow!("Values block without update_data"))?;

                    mutations.push(ObjectMutation::Values(guid, update_data));
                }
                BlockType::Movement => {
                    let guid = block
                        .guid
                        .ok_or_else(|| anyhow::anyhow!("Movement block without guid"))?;
                    let movement = block
                        .movement
                        .ok_or_else(|| anyhow::anyhow!("Movement block without movement"))?;

                    mutations.push(ObjectMutation::Movement(guid, movement));
                }
                BlockType::OutOfRangeObjects => {
                    mutations.extend(block.guids.into_iter().map(ObjectMutation::OutOfRange));
                }
                BlockType::NearObjects => {}
            }
        }

        if !mutations.is_empty() {
            let processed_at = now_millis();
            let create_count = mutations
                .iter()
                .filter(|mutation| matches!(mutation, ObjectMutation::Create(_)))
                .count();

            output.push(HandlerOutput::Requests(vec![Request::SetContext(Some(
                Box::new(move |ctx: &mut CtxMap| {
                    apply_mutations(ctx, mutations, create_count, processed_at);
                }),
            ))]));
        }

        Ok(output)
    }
}

enum ObjectMutation {
    Create(Object),
    Values(PackedGuid, UpdateData),
    Movement(PackedGuid, Movement),
    OutOfRange(PackedGuid),
}

enum LifecycleMutation {
    Created(PackedGuid),
    Updated(PackedGuid),
    Removed(PackedGuid, ObjectRemovalReason),
}

fn apply_mutations(
    ctx: &mut CtxMap,
    mutations: Vec<ObjectMutation>,
    create_count: usize,
    processed_at: u64,
) {
    let mut lifecycle_mutations = Vec::with_capacity(mutations.len());

    {
        let Some(objects) = ctx.get_mut::<ObjectMap>() else {
            return;
        };

        if create_count > 0 {
            objects.reserve(create_count);
        }

        for mutation in mutations {
            match mutation {
                ObjectMutation::Create(object) => {
                    let guid = object.guid;
                    objects.insert(guid, object);
                    lifecycle_mutations.push(LifecycleMutation::Created(guid));
                }
                ObjectMutation::Values(guid, update_data) => {
                    if let Some(object) = objects.get_mut(&guid) {
                        object.update(update_data);
                        lifecycle_mutations.push(LifecycleMutation::Updated(guid));
                    }
                }
                ObjectMutation::Movement(guid, movement) => {
                    if let Some(object) = objects.get_mut(&guid) {
                        object.movement = Some(movement);
                        lifecycle_mutations.push(LifecycleMutation::Updated(guid));
                    }
                }
                ObjectMutation::OutOfRange(guid) => {
                    objects.remove(&guid);
                    lifecycle_mutations.push(LifecycleMutation::Removed(
                        guid,
                        ObjectRemovalReason::OutOfRange,
                    ));
                }
            }
        }
    }

    let Some(registry) = ctx.get_mut::<ObjectLifecycleRegistry>() else {
        return;
    };

    for mutation in lifecycle_mutations {
        match mutation {
            LifecycleMutation::Created(guid) => {
                registry.insert(guid, ObjectLifecycle::created(processed_at));
            }
            LifecycleMutation::Updated(guid) => {
                if let Some(lifecycle) = registry.get_mut(&guid) {
                    lifecycle.mark_updated(processed_at);
                }
            }
            LifecycleMutation::Removed(guid, reason) => {
                if let Some(lifecycle) = registry.get_mut(&guid) {
                    lifecycle.mark_removed(processed_at, reason);
                }
            }
        }
    }
}

fn sanitize_update_data(update_data: &mut UpdateData, object_mask: ObjectTypeMask) {
    if !object_mask.contains(ObjectTypeMask::OBJECT) {
        update_data.object_fields.clear();
    }
    if !object_mask.contains(ObjectTypeMask::UNIT) {
        update_data.unit_fields.clear();
    }
    if !object_mask.contains(ObjectTypeMask::PLAYER) {
        update_data.player_fields.clear();
    }
    if !object_mask.contains(ObjectTypeMask::ITEM) {
        update_data.item_fields.clear();
    }
    if !object_mask.contains(ObjectTypeMask::CONTAINER) {
        update_data.container_fields.clear();
    }
    if !object_mask.contains(ObjectTypeMask::GAMEOBJECT) {
        update_data.game_object_fields.clear();
    }
    if !object_mask.contains(ObjectTypeMask::DYNAMICOBJECT) {
        update_data.dynamic_object_fields.clear();
    }
    if !object_mask.contains(ObjectTypeMask::CORPSE) {
        update_data.corpse_fields.clear();
    }
}

#[derive(BinRead, BinWrite, FieldsMetadata, Serialize)]
pub struct Block {
    pub block_type: BlockType,
    #[br(if(matches!(block_type, BlockType::Movement
                               | BlockType::CreateObject
                               | BlockType::CreateObject2
                               | BlockType::Values)))]
    #[bw(if(matches!(block_type, BlockType::Movement
                               | BlockType::CreateObject
                               | BlockType::CreateObject2
                               | BlockType::Values)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guid: Option<PackedGuid>,
    #[br(if(matches!(block_type, BlockType::CreateObject | BlockType::CreateObject2)))]
    #[bw(if(matches!(block_type, BlockType::CreateObject | BlockType::CreateObject2)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type_id: Option<ObjectTypeId>,
    #[br(if(matches!(block_type, BlockType::Movement
                               | BlockType::CreateObject
                               | BlockType::CreateObject2)))]
    #[bw(if(matches!(block_type, BlockType::Movement
                               | BlockType::CreateObject
                               | BlockType::CreateObject2)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement: Option<Movement>,
    #[br(if(matches!(block_type, BlockType::Values
                               | BlockType::CreateObject
                               | BlockType::CreateObject2)))]
    #[bw(if(matches!(block_type, BlockType::Values
                               | BlockType::CreateObject
                               | BlockType::CreateObject2)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_data: Option<UpdateData>,
    #[br(if(matches!(block_type, BlockType::OutOfRangeObjects | BlockType::NearObjects)))]
    #[bw(if(matches!(block_type, BlockType::OutOfRangeObjects | BlockType::NearObjects)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guid_count: Option<u32>,
    #[br(
        if(matches!(block_type, BlockType::OutOfRangeObjects | BlockType::NearObjects)),
        count = guid_count.unwrap_or(0) as usize
    )]
    #[bw(if(matches!(block_type, BlockType::OutOfRangeObjects | BlockType::NearObjects)))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub guids: Vec<PackedGuid>,
}

#[derive(BinRead, BinWrite, IntoPrimitive, TryFromPrimitive, Serialize)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[repr(u8)]
pub enum BlockType {
    Values = 0,
    Movement = 1,
    CreateObject = 2,
    CreateObject2 = 3,
    OutOfRangeObjects = 4,
    NearObjects = 5,
}

impl CalculateMetadata for BlockType {
    fn calculate<'a>(&self, ctx: &'a mut MetadataContext) -> &'a mut MetadataContext {
        let size = size_of::<u8>();
        ctx.metadata.insert(
            ctx.current_key.clone(),
            MetadataValue {
                size,
                offset: ctx.offset,
            },
        );
        ctx.offset += size;
        ctx
    }
}

#[derive(
    BinRead, BinWrite, IntoPrimitive, TryFromPrimitive, Serialize, Copy, Clone, Default, Debug,
    PartialEq, Eq,
)]
#[br(repr = i8)]
#[bw(repr = i8)]
#[repr(i8)]
pub enum ObjectTypeId {
    #[default]
    None = -1,
    Object = 0,
    Item = 1,
    Container = 2,
    Unit = 3,
    Player = 4,
    GameObject = 5,
    DynamicObject = 6,
    Corpse = 7,
}

impl ObjectTypeId {
    #[inline]
    pub fn is_none(v: &Self) -> bool {
        matches!(v, ObjectTypeId::None)
    }
}

impl CalculateMetadata for ObjectTypeId {
    fn calculate<'a>(&self, ctx: &'a mut MetadataContext) -> &'a mut MetadataContext {
        let size = size_of::<i8>();
        ctx.metadata.insert(
            ctx.current_key.clone(),
            MetadataValue {
                size,
                offset: ctx.offset,
            },
        );
        ctx.offset += size;
        ctx
    }
}

pub struct Object {
    pub guid: PackedGuid,
    pub object_type_id: ObjectTypeId,
    pub object_type_mask: ObjectTypeMask,
    pub movement: Option<Movement>,
    pub object_fields: BTreeMap<ObjectField, FieldValue>,
    pub unit_fields: BTreeMap<UnitField, FieldValue>,
    pub player_fields: BTreeMap<PlayerField, FieldValue>,
    pub item_fields: BTreeMap<ItemField, FieldValue>,
    pub container_fields: BTreeMap<ContainerField, FieldValue>,
    pub game_object_fields: BTreeMap<GameObjectField, FieldValue>,
    pub dynamic_object_fields: BTreeMap<DynamicObjectField, FieldValue>,
    pub corpse_fields: BTreeMap<CorpseField, FieldValue>,
}

impl Object {
    pub fn update(&mut self, update: UpdateData) {
        merge_field_map(&mut self.object_fields, update.object_fields);

        match self.object_type_id {
            ObjectTypeId::Player => {
                merge_field_map(&mut self.unit_fields, update.unit_fields);
                merge_field_map(&mut self.player_fields, update.player_fields);
            }
            ObjectTypeId::Unit => {
                merge_field_map(&mut self.unit_fields, update.unit_fields);
            }
            ObjectTypeId::Item => {
                merge_field_map(&mut self.item_fields, update.item_fields);
            }
            ObjectTypeId::Container => {
                merge_field_map(&mut self.item_fields, update.item_fields);
                merge_field_map(&mut self.container_fields, update.container_fields);
            }
            ObjectTypeId::GameObject => {
                merge_field_map(&mut self.game_object_fields, update.game_object_fields);
            }
            ObjectTypeId::DynamicObject => {
                merge_field_map(&mut self.dynamic_object_fields, update.dynamic_object_fields);
            }
            ObjectTypeId::Corpse => {
                merge_field_map(&mut self.corpse_fields, update.corpse_fields);
            }
            _ => {}
        }
    }
}

fn merge_field_map<K: Ord>(
    current: &mut BTreeMap<K, FieldValue>,
    incoming: BTreeMap<K, FieldValue>,
) {
    use std::collections::btree_map::Entry;

    for (field, value) in incoming {
        match current.entry(field) {
            Entry::Occupied(mut entry) => entry.get_mut().merge_from(value),
            Entry::Vacant(entry) => {
                entry.insert(value);
            }
        }
    }
}

impl TryFrom<Block> for Object {
    type Error = ObjectBuildError;

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        let Block {
            guid,
            object_type_id,
            movement,
            update_data,
            ..
        } = block;

        let guid = guid.ok_or(ObjectBuildError::MissingGuid)?;
        let object_type_id = object_type_id.ok_or(ObjectBuildError::MissingObjectTypeId)?;
        let update_data = update_data.ok_or(ObjectBuildError::MissingUpdateData)?;

        Ok(Object {
            guid,
            object_type_id,
            object_type_mask: update_data.object_type_mask,
            movement,

            object_fields: update_data.object_fields,
            unit_fields: update_data.unit_fields,
            player_fields: update_data.player_fields,
            item_fields: update_data.item_fields,
            container_fields: update_data.container_fields,
            game_object_fields: update_data.game_object_fields,
            dynamic_object_fields: update_data.dynamic_object_fields,
            corpse_fields: update_data.corpse_fields,
        })
    }
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum ObjectBuildError {
    MissingGuid,
    MissingObjectTypeId,
    MissingUpdateData,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unit_object(guid: PackedGuid) -> Object {
        Object {
            guid,
            object_type_id: ObjectTypeId::Unit,
            object_type_mask: ObjectTypeMask::OBJECT | ObjectTypeMask::UNIT,
            movement: None,
            object_fields: BTreeMap::new(),
            unit_fields: BTreeMap::new(),
            player_fields: BTreeMap::new(),
            item_fields: BTreeMap::new(),
            container_fields: BTreeMap::new(),
            game_object_fields: BTreeMap::new(),
            dynamic_object_fields: BTreeMap::new(),
            corpse_fields: BTreeMap::new(),
        }
    }

    fn player_object(guid: PackedGuid) -> Object {
        Object {
            guid,
            object_type_id: ObjectTypeId::Player,
            object_type_mask: ObjectTypeMask::OBJECT
                | ObjectTypeMask::UNIT
                | ObjectTypeMask::PLAYER,
            movement: None,
            object_fields: BTreeMap::new(),
            unit_fields: BTreeMap::new(),
            player_fields: BTreeMap::new(),
            item_fields: BTreeMap::new(),
            container_fields: BTreeMap::new(),
            game_object_fields: BTreeMap::new(),
            dynamic_object_fields: BTreeMap::new(),
            corpse_fields: BTreeMap::new(),
        }
    }

    #[test]
    fn partial_array_updates_preserve_previously_known_elements() {
        let guid = PackedGuid(1);
        let mut object = unit_object(guid);
        object.unit_fields.insert(
            UnitField::Powers,
            FieldValue::IntegerArray(vec![Some(100), Some(200), None, None, None, None, None]),
        );

        let mut update = UpdateData::default();
        update.unit_fields.insert(
            UnitField::Powers,
            FieldValue::IntegerArray(vec![None, Some(250), None, None, None, None, None]),
        );

        object.update(update);

        assert_eq!(
            object.unit_fields.get(&UnitField::Powers),
            Some(&FieldValue::IntegerArray(vec![
                Some(100),
                Some(250),
                None,
                None,
                None,
                None,
                None,
            ]))
        );
    }

    #[test]
    fn partial_custom_array_updates_preserve_previously_known_elements() {
        let guid = PackedGuid(2);
        let mut object = player_object(guid);
        object.player_fields.insert(
            PlayerField::VisibleItems,
            FieldValue::CustomArray(vec![
                vec![
                    Some(FieldValue::Integer(1000)),
                    Some(FieldValue::TwoShorts((1, 2))),
                ],
                vec![
                    Some(FieldValue::Integer(2000)),
                    Some(FieldValue::TwoShorts((3, 4))),
                ],
            ]),
        );

        let mut update = UpdateData::default();
        update.player_fields.insert(
            PlayerField::VisibleItems,
            FieldValue::CustomArray(vec![
                vec![None, Some(FieldValue::TwoShorts((9, 10)))],
                vec![Some(FieldValue::Integer(2500)), None],
            ]),
        );

        object.update(update);

        assert_eq!(
            object.player_fields.get(&PlayerField::VisibleItems),
            Some(&FieldValue::CustomArray(vec![
                vec![
                    Some(FieldValue::Integer(1000)),
                    Some(FieldValue::TwoShorts((9, 10))),
                ],
                vec![
                    Some(FieldValue::Integer(2500)),
                    Some(FieldValue::TwoShorts((3, 4))),
                ],
            ]))
        );
    }

    #[test]
    fn movement_update_advances_lifecycle_updated_at() {
        let guid = PackedGuid(3);
        let mut ctx = CtxMap::default();

        let mut objects = ObjectMap::default();
        objects.insert(guid, unit_object(guid));
        ctx.insert(objects);

        let mut registry = ObjectLifecycleRegistry::default();
        registry.insert(guid, ObjectLifecycle::created(100));
        ctx.insert(registry);

        let movement = Movement {
            world_object_position: Some(
                crate::plugins::wow::wotlk::realm::object::types::movement::OrientedPoint3D {
                    point: crate::plugins::wow::wotlk::realm::object::types::movement::Point3D {
                        x: 10.0,
                        y: 20.0,
                        z: 30.0,
                    },
                    direction: 0.75,
                },
            ),
            ..Default::default()
        };

        apply_mutations(
            &mut ctx,
            vec![ObjectMutation::Movement(guid, movement)],
            0,
            225,
        );

        let object = &ctx.get::<ObjectMap>().unwrap()[&guid];
        assert_eq!(
            object
                .movement
                .as_ref()
                .and_then(|movement| movement.world_object_position.as_ref())
                .map(|position| position.point.x),
            Some(10.0)
        );
        let lifecycle = &ctx.get::<ObjectLifecycleRegistry>().unwrap()[&guid];
        assert_eq!(lifecycle.updated_at(), 225);
    }

    #[test]
    fn values_update_advances_lifecycle_updated_at() {
        let guid = PackedGuid(6);
        let mut ctx = CtxMap::default();

        let mut objects = ObjectMap::default();
        objects.insert(guid, unit_object(guid));
        ctx.insert(objects);

        let mut registry = ObjectLifecycleRegistry::default();
        registry.insert(guid, ObjectLifecycle::created(100));
        ctx.insert(registry);

        let mut update = UpdateData::default();
        update
            .unit_fields
            .insert(UnitField::Health, FieldValue::Integer(500));
        apply_mutations(
            &mut ctx,
            vec![ObjectMutation::Values(guid, update)],
            0,
            200,
        );

        let object = &ctx.get::<ObjectMap>().unwrap()[&guid];
        assert_eq!(
            object.unit_fields.get(&UnitField::Health),
            Some(&FieldValue::Integer(500))
        );
        let lifecycle = &ctx.get::<ObjectLifecycleRegistry>().unwrap()[&guid];
        assert_eq!(lifecycle.created_at(), 100);
        assert_eq!(lifecycle.updated_at(), 200);
        assert_eq!(lifecycle.removed_at(), None);
    }

    #[test]
    fn out_of_range_removes_object_and_retains_lifecycle_tombstone() {
        let guid = PackedGuid(7);
        let mut ctx = CtxMap::default();

        let mut objects = ObjectMap::default();
        objects.insert(guid, unit_object(guid));
        ctx.insert(objects);

        let mut registry = ObjectLifecycleRegistry::default();
        registry.insert(guid, ObjectLifecycle::created(100));
        ctx.insert(registry);

        apply_mutations(
            &mut ctx,
            vec![ObjectMutation::OutOfRange(guid)],
            0,
            200,
        );

        assert!(!ctx.get::<ObjectMap>().unwrap().contains_key(&guid));
        let lifecycle = &ctx.get::<ObjectLifecycleRegistry>().unwrap()[&guid];
        assert_eq!(lifecycle.created_at(), 100);
        assert_eq!(lifecycle.updated_at(), 100);
        assert_eq!(lifecycle.removed_at(), Some(200));
        assert_eq!(
            lifecycle.removal_reason(),
            Some(ObjectRemovalReason::OutOfRange)
        );
    }

    #[test]
    fn out_of_range_block_roundtrips_multiple_guids() {
        let guids = vec![PackedGuid(60), PackedGuid(61), PackedGuid(62)];
        let block = Block {
            block_type: BlockType::OutOfRangeObjects,
            guid: None,
            object_type_id: None,
            movement: None,
            update_data: None,
            guid_count: Some(guids.len() as u32),
            guids: guids.clone(),
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        block.write_le(&mut cursor).unwrap();
        cursor.set_position(0);
        let decoded = Block::read_le(&mut cursor).unwrap();

        assert!(matches!(decoded.block_type, BlockType::OutOfRangeObjects));
        assert_eq!(decoded.guid_count, Some(3));
        assert_eq!(decoded.guids, guids);
    }

    #[test]
    fn out_of_range_removes_multiple_objects_and_marks_each_lifecycle() {
        let guids = [PackedGuid(70), PackedGuid(71), PackedGuid(72)];
        let mut ctx = CtxMap::default();

        let mut objects = ObjectMap::default();
        let mut registry = ObjectLifecycleRegistry::default();
        for guid in guids {
            objects.insert(guid, unit_object(guid));
            registry.insert(guid, ObjectLifecycle::created(100));
        }
        ctx.insert(objects);
        ctx.insert(registry);

        apply_mutations(
            &mut ctx,
            guids
                .into_iter()
                .map(ObjectMutation::OutOfRange)
                .collect(),
            0,
            240,
        );

        let objects = ctx.get::<ObjectMap>().unwrap();
        let registry = ctx.get::<ObjectLifecycleRegistry>().unwrap();
        for guid in guids {
            assert!(!objects.contains_key(&guid));
            let lifecycle = &registry[&guid];
            assert_eq!(lifecycle.removed_at(), Some(240));
            assert_eq!(
                lifecycle.removal_reason(),
                Some(ObjectRemovalReason::OutOfRange)
            );
        }
    }

    #[test]
    fn create_after_removal_starts_a_new_lifecycle() {
        let guid = PackedGuid(9);
        let mut ctx = CtxMap::default();
        ctx.insert(ObjectMap::default());

        let mut registry = ObjectLifecycleRegistry::default();
        let mut old = ObjectLifecycle::created(100);
        old.mark_removed(150, ObjectRemovalReason::OutOfRange);
        registry.insert(guid, old);
        ctx.insert(registry);

        apply_mutations(
            &mut ctx,
            vec![ObjectMutation::Create(unit_object(guid))],
            1,
            300,
        );

        assert!(ctx.get::<ObjectMap>().unwrap().contains_key(&guid));
        let lifecycle = &ctx.get::<ObjectLifecycleRegistry>().unwrap()[&guid];
        assert_eq!(lifecycle.created_at(), 300);
        assert_eq!(lifecycle.updated_at(), 300);
        assert_eq!(lifecycle.removed_at(), None);
        assert_eq!(lifecycle.removal_reason(), None);
    }

    #[test]
    fn initial_create_inserts_object_and_initializes_lifecycle() {
        let guid = PackedGuid(10);
        let mut ctx = CtxMap::default();
        ctx.insert(ObjectMap::default());
        ctx.insert(ObjectLifecycleRegistry::default());

        apply_mutations(
            &mut ctx,
            vec![ObjectMutation::Create(unit_object(guid))],
            1,
            400,
        );

        assert!(ctx.get::<ObjectMap>().unwrap().contains_key(&guid));
        let lifecycle = &ctx.get::<ObjectLifecycleRegistry>().unwrap()[&guid];
        assert_eq!(lifecycle.created_at(), 400);
        assert_eq!(lifecycle.updated_at(), 400);
        assert_eq!(lifecycle.removed_at(), None);
        assert_eq!(lifecycle.removal_reason(), None);
        assert!(!lifecycle.is_removed());
    }

    #[test]
    fn late_values_after_removal_do_not_resurrect_or_advance_lifecycle() {
        let guid = PackedGuid(11);
        let mut ctx = CtxMap::default();

        let mut objects = ObjectMap::default();
        objects.insert(guid, unit_object(guid));
        ctx.insert(objects);

        let mut registry = ObjectLifecycleRegistry::default();
        registry.insert(guid, ObjectLifecycle::created(100));
        ctx.insert(registry);

        apply_mutations(
            &mut ctx,
            vec![ObjectMutation::OutOfRange(guid)],
            0,
            200,
        );

        let mut update = UpdateData::default();
        update
            .unit_fields
            .insert(UnitField::Health, FieldValue::Integer(999));
        apply_mutations(
            &mut ctx,
            vec![ObjectMutation::Values(guid, update)],
            0,
            300,
        );

        assert!(!ctx.get::<ObjectMap>().unwrap().contains_key(&guid));
        let lifecycle = &ctx.get::<ObjectLifecycleRegistry>().unwrap()[&guid];
        assert_eq!(lifecycle.created_at(), 100);
        assert_eq!(lifecycle.updated_at(), 100);
        assert_eq!(lifecycle.removed_at(), Some(200));
        assert_eq!(
            lifecycle.removal_reason(),
            Some(ObjectRemovalReason::OutOfRange)
        );
    }

    #[test]
    fn unknown_out_of_range_guid_does_not_invent_lifecycle_history() {
        let guid = PackedGuid(12);
        let mut ctx = CtxMap::default();
        ctx.insert(ObjectMap::default());
        ctx.insert(ObjectLifecycleRegistry::default());

        apply_mutations(
            &mut ctx,
            vec![ObjectMutation::OutOfRange(guid)],
            0,
            500,
        );

        assert!(!ctx.get::<ObjectMap>().unwrap().contains_key(&guid));
        assert!(!ctx
            .get::<ObjectLifecycleRegistry>()
            .unwrap()
            .contains_key(&guid));
    }
}
