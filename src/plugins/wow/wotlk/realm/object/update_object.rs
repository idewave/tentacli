use std::collections::{BTreeMap, HashMap};
use std::io::Read;
use std::sync::Arc;
use async_trait::async_trait;
use binrw::{BinRead, BinWrite};
use flate2::read::DeflateDecoder;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::Serialize;
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::realm::object::types::movement::Movement;
use crate::plugins::wow::wotlk::realm::object::types::packed_guid::PackedGuid;
use crate::plugins::wow::wotlk::realm::object::types::update_data::{ObjectTypeMask, UpdateData};
use crate::plugins::wow::wotlk::realm::object::types::update_fields::{
    ContainerField, CorpseField, DynamicObjectField, FieldValue, GameObjectField,
    ItemField, ObjectField, PlayerField, UnitField
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
        context: Arc<RwLock<CtxMap>>
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
            let option = guard.get::<HashMap<PackedGuid, Object>>();
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
        packet.set_offset_info(
            ExtractMetadata::extract_metadata(&incoming)
        );
        packet.set_json(
            serialize_packet_json(&incoming)?
        );


        let mut creates: Vec<(PackedGuid, Object)> = Vec::new();
        let mut updates: Vec<(PackedGuid, UpdateData)> = Vec::new();

        for block in incoming.blocks {
            match block.block_type {
                BlockType::CreateObject | BlockType::CreateObject2 => {
                    match Object::try_from(block) {
                        Ok(object) => {
                            creates.push((object.guid, object));
                        },
                        Err(err) => {
                            output.push(HandlerOutput::Messages(vec![
                                Message {
                                    msg_type: MsgType::Error,
                                    text: format!("Failed to build Object from block: {:?}", err),
                                }
                            ]));
                        }
                    }
                },
                BlockType::Values => {
                    let guid = block.guid
                        .ok_or_else(|| anyhow::anyhow!("Values block without guid"))?;
                    let update_data = block.update_data
                        .ok_or_else(|| anyhow::anyhow!("Values block without update_data"))?;

                    updates.push((guid, update_data));
                },
                _ => {}
            }
        }

        if !creates.is_empty() || !updates.is_empty() {
            output.push(HandlerOutput::Requests(vec![
                Request::SetContext(Some(Box::new(move |ctx: &mut CtxMap| {
                    let Some(objects) = ctx.get_mut::<HashMap<PackedGuid, Object>>() else {
                        return;
                    };

                    // Reserve upfront to avoid rehash storms
                    if !creates.is_empty() {
                        objects.reserve(creates.len());
                    }

                    // Apply creates
                    for (guid, object) in creates {
                        objects.insert(guid, object);
                    }

                    // Apply updates
                    for (guid, update_data) in updates {
                        if let Some(object) = objects.get_mut(&guid) {
                            object.update(update_data);
                        }
                    }
                })))
            ]));
        }

        Ok(output)
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
    Values            = 0,
    Movement          = 1,
    CreateObject      = 2,
    CreateObject2     = 3,
    OutOfRangeObjects = 4,
    NearObjects       = 5,
}

impl CalculateMetadata for BlockType {
    fn calculate<'a>(&self, ctx: &'a mut MetadataContext) -> &'a mut MetadataContext {
        let size = size_of::<u8>();
        ctx.metadata.insert(
            ctx.current_key.clone(),
            MetadataValue { size, offset: ctx.offset },
        );
        ctx.offset += size;
        ctx
    }
}

#[derive(BinRead, BinWrite, IntoPrimitive, TryFromPrimitive, Serialize, Copy, Clone, Default)]
#[br(repr = i8)]
#[bw(repr = i8)]
#[repr(i8)]
pub enum ObjectTypeId {
    #[default]
    None          = -1,
    Object        = 0,
    Item          = 1,
    Container     = 2,
    Unit          = 3,
    Player        = 4,
    GameObject    = 5,
    DynamicObject = 6,
    Corpse        = 7,
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
            MetadataValue { size, offset: ctx.offset },
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
        self.object_fields.extend(update.object_fields);

        match self.object_type_id {
            ObjectTypeId::Player => {
                self.unit_fields.extend(update.unit_fields);
                self.player_fields.extend(update.player_fields);
            }
            ObjectTypeId::Unit => {
                self.unit_fields.extend(update.unit_fields);
            }
            ObjectTypeId::Item => {
                self.item_fields.extend(update.item_fields);
            }
            ObjectTypeId::Container => {
                self.item_fields.extend(update.item_fields);
                self.container_fields.extend(update.container_fields);
            }
            ObjectTypeId::GameObject => {
                self.game_object_fields.extend(update.game_object_fields);
            }
            ObjectTypeId::DynamicObject => {
                self.dynamic_object_fields.extend(update.dynamic_object_fields);
            }
            ObjectTypeId::Corpse => {
                self.corpse_fields.extend(update.corpse_fields);
            }
            _ => {}
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
