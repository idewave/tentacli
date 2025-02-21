use std::collections::HashMap;

use async_trait::async_trait;
use tentacli_traits::PacketHandler;
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::custom_fields::PackedGuid;
use tentacli_traits::types::movement::Movement;
use tentacli_traits::types::opcodes::Opcode;
use tentacli_traits::types::shared::Object;
use tentacli_traits::types::update_data::{BlockType, ObjectTypeID, ObjectTypeMask, UpdateData};
use tentacli_traits::types::update_fields::{FieldValue, ItemField, ObjectField};

#[derive(WorldPacket, Serialize, Debug)]
pub struct Incoming {
    pub blocks_amount: u32,
    #[depends_on(blocks_amount)]
    pub blocks: Vec<Block>,
}

fn is_zero(&x: &u32) -> bool {
    x == 0
}

#[derive(Serialize, Segment, Debug, Clone, Default)]
pub struct Block {
    pub block_type: BlockType,
    #[conditional]
    #[serde(skip_serializing_if = "PackedGuid::is_default")]
    pub guid: PackedGuid,
    #[conditional]
    #[serde(skip_serializing_if = "ObjectTypeID::is_none")]
    pub object_type_id: ObjectTypeID,
    #[conditional]
    #[serde(skip_serializing_if = "Movement::is_default")]
    pub movement: Movement,
    #[conditional]
    #[serde(skip_serializing_if = "UpdateData::is_default")]
    pub update_data: UpdateData,
    #[conditional]
    #[serde(skip_serializing_if = "is_zero")]
    pub guid_count: u32,
    #[depends_on(guid_count)]
    #[conditional]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub guids: Vec<PackedGuid>,
}

impl Block {
    fn guid(instance: &mut Self) -> bool {
        matches!(
            instance.block_type.0,
            BlockType::VALUES |
            BlockType::MOVEMENT |
            BlockType::CREATE_OBJECT |
            BlockType::CREATE_OBJECT2
        )
    }

    fn object_type_id(instance: &mut Self) -> bool {
        matches!(
            instance.block_type.0,
            BlockType::CREATE_OBJECT |
            BlockType::CREATE_OBJECT2
        )
    }

    fn movement(instance: &mut Self) -> bool {
        matches!(
            instance.block_type.0,
            BlockType::MOVEMENT |
            BlockType::CREATE_OBJECT |
            BlockType::CREATE_OBJECT2
        )
    }

    fn update_data(instance: &mut Self) -> bool {
        matches!(
            instance.block_type.0,
            BlockType::VALUES |
            BlockType::CREATE_OBJECT |
            BlockType::CREATE_OBJECT2
        )
    }

    fn guid_count(instance: &mut Self) -> bool {
        matches!(
            instance.block_type.0,
            BlockType::NEAR_OBJECTS |
            BlockType::OUT_OF_RANGE_OBJECTS
        )
    }

    fn guids(instance: &mut Self) -> bool {
        matches!(
            instance.block_type.0,
            BlockType::NEAR_OBJECTS |
            BlockType::OUT_OF_RANGE_OBJECTS
        )
    }
}

pub struct Handler;

#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (Incoming { blocks, .. }, json) = {
            if input.opcode == Opcode::SMSG_UPDATE_OBJECT {
                Incoming::from_binary(&input.data)?
            } else {
                Incoming::from_compressed_binary(&input.data)?
            }
        };

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_opcode_name(input.opcode as u32)
                .unwrap_or(format!("Unknown opcode: {}", input.opcode)),
            Some(json),
        ));

        for block in blocks {
            let mask = {
                let value = match block.update_data.object_fields.get(&ObjectField::Type) {
                    Some(FieldValue::Integer(mask)) => *mask,
                    _ => 0,
                };

                ObjectTypeMask::from_bits(value).unwrap_or_default()
            };

            let PackedGuid(guid) = block.guid;

            let mut guard = input.data_storage.lock().await;

            let is_player_guid = guard.players_map.contains_key(&guid);
            let is_unit_guid = guard.units_map.contains_key(&guid);
            let is_item_guid = guard.items_map.contains_key(&guid);
            let is_game_object_guid = guard.game_objects_map.contains_key(&guid);
            let is_dynamic_object_guid = guard.dynamic_objects_map.contains_key(&guid);
            let is_container_guid = guard.containers_map.contains_key(&guid);
            let is_corpse_guid = guard.corpses_map.contains_key(&guid);

            match mask {
                m if m.contains(ObjectTypeMask::PLAYER) || is_player_guid => {
                    response.push(HandlerOutput::UpdatePlayer(guid));
                    Self::update_or_insert(&mut guard.players_map, guid, Object {
                        update_data: block.update_data,
                        movement: block.movement,
                        guid,
                        ..Object::default()
                    });
                }
                m if m.contains(ObjectTypeMask::UNIT) || is_unit_guid => {
                    response.push(HandlerOutput::UpdateNPC(guid));
                    Self::update_or_insert(&mut guard.units_map, guid, Object {
                        update_data: block.update_data,
                        movement: block.movement,
                        guid,
                        ..Object::default()
                    });
                }
                m if m.contains(ObjectTypeMask::ITEM) || is_item_guid => {
                    let is_my_item = {
                        if let Some(my_guid) = input.session.lock().await.my_guid {
                            match block.update_data.item_fields.get(&ItemField::Owner) {
                                Some(FieldValue::Long(guid)) => *guid == my_guid,
                                _ => false,
                            }
                        } else {
                            false
                        }
                    };

                    if is_my_item {
                        match block.update_data.object_fields.get(&ObjectField::Entry) {
                            Some(FieldValue::Integer(entry)) => {
                                let mut guard = input.session.lock().await;
                                guard.inventory.push(*entry);
                            }
                            _ => {}
                        }
                    }

                    response.push(HandlerOutput::UpdateItem(guid));
                    Self::update_or_insert(&mut guard.items_map, guid, Object {
                        update_data: block.update_data,
                        movement: block.movement,
                        guid,
                        ..Object::default()
                    });
                }
                m if m.contains(ObjectTypeMask::GAMEOBJECT) || is_game_object_guid => {
                    response.push(HandlerOutput::UpdateGameObject(guid));
                    Self::update_or_insert(&mut guard.game_objects_map, guid, Object {
                        update_data: block.update_data,
                        movement: block.movement,
                        guid,
                        ..Object::default()
                    });
                }
                m if m.contains(ObjectTypeMask::DYNAMICOBJECT) || is_dynamic_object_guid => {
                    response.push(HandlerOutput::UpdateDynamicObject(guid));
                    Self::update_or_insert(&mut guard.dynamic_objects_map, guid, Object {
                        update_data: block.update_data,
                        movement: block.movement,
                        guid,
                        ..Object::default()
                    });
                }
                m if m.contains(ObjectTypeMask::CONTAINER) || is_container_guid => {
                    response.push(HandlerOutput::UpdateContainer(guid));
                    Self::update_or_insert(&mut guard.containers_map, guid, Object {
                        update_data: block.update_data,
                        movement: block.movement,
                        guid,
                        ..Object::default()
                    });
                }
                m if m.contains(ObjectTypeMask::CORPSE) || is_corpse_guid => {
                    response.push(HandlerOutput::UpdateCorpse(guid));
                    Self::update_or_insert(&mut guard.corpses_map, guid, Object {
                        update_data: block.update_data,
                        movement: block.movement,
                        guid,
                        ..Object::default()
                    });
                }
                _ => {}
            }
        }

        Ok(response)
    }
}

impl Handler {
    fn update_or_insert(map: &mut HashMap<u64, Object>, guid: u64, mut object: Object) {
        map.entry(guid)
            .and_modify(|o| o.update_data.update_fields(&mut object.update_data))
            .or_insert(object);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use tentacli_traits::types::custom_fields::PackedGuid;
    use tentacli_traits::types::movement::{
        Movement, MovementExtraFlags, MovementFlags, MovementInfo, ObjectUpdateFlags, UnitMoveType,
    };
    use tentacli_traits::types::opcodes::Opcode;
    use tentacli_traits::types::update_data::{BlockType, ObjectTypeID, ObjectTypeMask, UpdateData};
    use tentacli_traits::types::update_fields::{FieldValue, ObjectField, PlayerField, UnitField};

    use crate::features::wotlk_realm::object::update_object::{Block, Incoming};

    #[test]
    fn test_packet_building() -> anyhow::Result<()> {
        const GUID: u64 = 123;
        const SCALE_X: f32 = 3.;
        const AURA_STATE: i32 = 35;
        const HEALTH: i32 = 52;
        const XP: i32 = 152;

        const CONSTANT_SPEED: f32 = 10.;

        let block_type = BlockType::new(BlockType::CREATE_OBJECT);
        let object_type_id = ObjectTypeID::new(ObjectTypeID::PLAYER);

        let block = Block {
            block_type: block_type.clone(),
            guid: PackedGuid(GUID),
            object_type_id: object_type_id.clone(),
            movement: {
                let mut movement = Movement::default();
                let movement_info = MovementInfo {
                    movement_flags: MovementFlags::NONE,
                    movement_extra_flags: MovementExtraFlags::NONE,
                    time: 0,
                    location: Default::default(),
                    taxi_info: None,
                    fall_time: 0,
                    jump_info: None,
                };

                movement.set_movement_info(movement_info);
                movement.movement_speed = {
                    let mut movement_speed: BTreeMap<u8, f32> = BTreeMap::new();
                    for move_type in [
                        UnitMoveType::MOVE_WALK,
                        UnitMoveType::MOVE_RUN,
                        UnitMoveType::MOVE_RUN_BACK,
                        UnitMoveType::MOVE_SWIM,
                        UnitMoveType::MOVE_SWIM_BACK,
                        UnitMoveType::MOVE_FLIGHT,
                        UnitMoveType::MOVE_FLIGHT_BACK,
                        UnitMoveType::MOVE_TURN_RATE,
                        UnitMoveType::MOVE_PITCH_RATE,
                    ] {
                        movement_speed.insert(move_type, CONSTANT_SPEED);
                    }

                    Some(movement_speed)
                };

                movement
            },
            update_data: UpdateData {
                object_fields: {
                    let obj_type = ObjectTypeMask::PLAYER
                        | ObjectTypeMask::UNIT
                        | ObjectTypeMask::OBJECT;

                    let mut map: BTreeMap<ObjectField, FieldValue> = BTreeMap::new();
                    map.insert(ObjectField::Guid, FieldValue::Long(GUID));
                    map.insert(ObjectField::Type, FieldValue::Integer(obj_type.bits()));
                    map.insert(ObjectField::ScaleX, FieldValue::Float(SCALE_X));

                    map
                },
                unit_fields: {
                    let mut map: BTreeMap<UnitField, FieldValue> = BTreeMap::new();
                    map.insert(UnitField::AuraState, FieldValue::Integer(AURA_STATE));
                    map.insert(UnitField::Charm, FieldValue::Long(GUID));
                    map.insert(UnitField::Health, FieldValue::Integer(HEALTH));

                    map
                },
                player_fields: {
                    let mut map: BTreeMap<PlayerField, FieldValue> = BTreeMap::new();
                    map.insert(PlayerField::Xp, FieldValue::Integer(XP));

                    map
                },
                ..UpdateData::default()
            },
            ..Block::default()
        };

        let blocks = vec![block.clone(), block.clone(), block];

        let packet = Incoming {
            blocks_amount: blocks.len() as u32,
            blocks,
        }.to_binary_with_server_opcode(Opcode::SMSG_UPDATE_OBJECT).unwrap();

        let (Incoming { blocks, .. }, _) = Incoming::from_binary(&packet[4..])?;

        assert_eq!(blocks[0].block_type, block_type);
        assert_eq!(blocks[1].block_type, block_type);
        assert_eq!(blocks[2].block_type, block_type);
        assert_eq!(blocks[0].guid, GUID);
        assert_eq!(blocks[1].guid, GUID);
        assert_eq!(blocks[2].guid, GUID);
        assert_eq!(blocks[0].object_type_id, object_type_id);
        assert_eq!(blocks[1].object_type_id, object_type_id);
        assert_eq!(blocks[2].object_type_id, object_type_id);

        assert_eq!(blocks[0].movement.movement_speed.is_some(), true);
        if let Some(movement_speed) = blocks[0].clone().movement.movement_speed {
            assert_eq!(movement_speed.get(&UnitMoveType::MOVE_SWIM), Some(&CONSTANT_SPEED));
        }

        assert_eq!(
            blocks[0].movement.object_update_flags.contains(ObjectUpdateFlags::LIVING),
            true
        );

        assert_eq!(
            blocks[0].update_data.object_fields.get(&ObjectField::Guid),
            Some(&FieldValue::Long(GUID))
        );
        assert_eq!(
            blocks[0].update_data.object_fields.get(&ObjectField::ScaleX),
            Some(&FieldValue::Float(SCALE_X))
        );
        assert_eq!(
            blocks[0].update_data.unit_fields.get(&UnitField::AuraState),
            Some(&FieldValue::Integer(AURA_STATE))
        );
        assert_eq!(
            blocks[0].update_data.player_fields.get(&PlayerField::Xp),
            Some(&FieldValue::Integer(XP))
        );

        Ok(())
    }
}