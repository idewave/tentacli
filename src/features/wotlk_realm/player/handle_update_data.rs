use async_trait::async_trait;
use tentacli_traits::{PacketHandler};
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::custom_fields::PackedGuid;
use tentacli_traits::types::movement::Movement;
use tentacli_traits::types::object::{Container, Corpse, DynamicObject, GameObject, Item, Unit};
use tentacli_traits::types::opcodes::Opcode;
use tentacli_traits::types::player::{Player};
use tentacli_traits::types::update_data::{BlockType, ObjectTypeID, ObjectTypeMask, UpdateData};
use tentacli_traits::types::update_fields::{FieldValue, ItemField, ObjectField};

#[derive(WorldPacket, Serialize, Debug)]
pub struct UpdateDataIncoming {
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
    pub guids: Vec<PackedGuid>
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

        let (UpdateDataIncoming { blocks, blocks_amount }, _) = {
            if input.opcode == Opcode::SMSG_UPDATE_OBJECT {
                UpdateDataIncoming::from_binary(&input.data)?
            } else {
                UpdateDataIncoming::from_compressed_binary(&input.data)?
            }
        };

        if input.session.lock().await.me.is_none() {
            response.push(HandlerOutput::ErrorMessage(
                "Session player was not initialized ?!!".to_string(),
                None,
            ));
            return Ok(response);
        }

        let my_guid = {
            input.session.lock().await.me.as_ref().unwrap().guid
        };

        let mut refined_blocks: Vec<Block> = vec![];

        for mut block in blocks {
            let mut update_data = UpdateData::default();
            let cloned_block = block.clone();

            let PackedGuid(guid) = block.guid;

            if let Some(FieldValue::Integer(mask)) =
                block.update_data.object_fields.get(&ObjectField::Type)
            {
                match mask {
                    m if m & ObjectTypeMask::PLAYER != 0 => {
                        update_data = block.update_data.clone();
                        let mut object = Player {
                            update_data: block.update_data,
                            guid,
                            ..Player::default()
                        };

                        object.movement = block.movement;

                        if guid == my_guid {
                            let mut guard = input.session.lock().await;
                            let me = guard.me.as_mut().unwrap();
                            *me = object.clone();
                        }

                        let mut guard = input.data_storage.lock().unwrap();
                        guard.players_map.insert(guid, object);
                    },
                    m if m & ObjectTypeMask::UNIT != 0 => {
                        update_data = block.update_data.clone();
                        let mut object = Unit {
                            update_data: block.update_data,
                            guid,
                            ..Unit::default()
                        };

                        object.movement = block.movement;

                        let mut guard = input.data_storage.lock().unwrap();
                        guard.units_map.insert(guid, object);
                    },
                    m if m & ObjectTypeMask::GAMEOBJECT != 0 => {
                        update_data = block.update_data.clone();
                        let mut object = GameObject {
                            update_data: block.update_data,
                            guid,
                            ..GameObject::default()
                        };

                        object.movement = block.movement;

                        let mut guard = input.data_storage.lock().unwrap();
                        guard.game_objects_map.insert(guid, object);
                    },
                    m if m & ObjectTypeMask::DYNAMICOBJECT != 0 => {
                        update_data = block.update_data.clone();
                        let mut object = DynamicObject {
                            update_data: block.update_data,
                            guid,
                            ..DynamicObject::default()
                        };

                        object.movement = block.movement;

                        let mut guard = input.data_storage.lock().unwrap();
                        guard.dynamic_objects_map.insert(guid, object);
                    },
                    m if m & ObjectTypeMask::ITEM != 0 => {
                        update_data = block.update_data.clone();
                        if let Some(FieldValue::Long(guid)) =
                            block.update_data.item_fields.get(&ItemField::Owner)
                        {
                            if my_guid == *guid {
                                let mut guard = input.session.lock().await;
                                let me = guard.me.as_mut().unwrap();
                                me.inventory.push(*guid);
                            }
                        }

                        let mut object = Item {
                            update_data: block.update_data,
                            guid,
                            ..Item::default()
                        };

                        object.movement = block.movement;

                        let mut guard = input.data_storage.lock().unwrap();
                        guard.items_map.insert(guid, object);
                    },
                    m if m & ObjectTypeMask::CONTAINER != 0 => {
                        update_data = block.update_data.clone();
                        if let Some(FieldValue::Long(guid)) =
                            block.update_data.item_fields.get(&ItemField::Owner)
                        {
                            if my_guid == *guid {
                                let mut guard = input.session.lock().await;
                                let me = guard.me.as_mut().unwrap();
                                me.inventory.push(*guid);
                            }
                        }

                        let mut object = Container {
                            update_data: block.update_data,
                            guid,
                            ..Container::default()
                        };

                        object.movement = block.movement;

                        let mut guard = input.data_storage.lock().unwrap();
                        guard.containers_map.insert(guid, object);
                    },
                    m if m & ObjectTypeMask::CORPSE != 0 => {
                        update_data = block.update_data.clone();
                        let mut object = Corpse {
                            update_data: block.update_data,
                            guid,
                            ..Corpse::default()
                        };

                        object.movement = block.movement;

                        let mut guard = input.data_storage.lock().unwrap();
                        guard.corpses_map.insert(guid, object);
                    }
                    _ => {},
                }
            } else {
                let mut guard = input.data_storage.lock().unwrap();

                match guid {
                    g if guard.players_map.contains_key(&g) => {
                        guard.players_map.entry(guid).and_modify(|o| {
                            o.update_data.extend_or_clear_source(&mut block.update_data);
                            update_data = block.update_data.clone();
                        });
                    },
                    g if guard.units_map.contains_key(&g) => {
                        guard.units_map.entry(guid).and_modify(|o| {
                            o.update_data.extend_or_clear_source(&mut block.update_data);
                            update_data = block.update_data.clone();
                        });
                    },
                    g if guard.game_objects_map.contains_key(&g) => {
                        guard.game_objects_map.entry(guid).and_modify(|o| {
                            o.update_data.extend_or_clear_source(&mut block.update_data);
                            update_data = block.update_data.clone();
                        });
                    },
                    g if guard.dynamic_objects_map.contains_key(&g) => {
                        guard.dynamic_objects_map.entry(guid).and_modify(|o| {
                            o.update_data.extend_or_clear_source(&mut block.update_data);
                            update_data = block.update_data.clone();
                        });
                    },
                    g if guard.items_map.contains_key(&g) => {
                        guard.items_map.entry(guid).and_modify(|o| {
                            o.update_data.extend_or_clear_source(&mut block.update_data);
                            update_data = block.update_data.clone();
                        });
                    },
                    g if guard.containers_map.contains_key(&g) => {
                        guard.containers_map.entry(guid).and_modify(|o| {
                            o.update_data.extend_or_clear_source(&mut block.update_data);
                            update_data = block.update_data.clone();
                        });
                    },
                    g if guard.corpses_map.contains_key(&g) => {
                        guard.corpses_map.entry(guid).and_modify(|o| {
                            o.update_data.extend_or_clear_source(&mut block.update_data);
                            update_data = block.update_data.clone();
                        });
                    },
                    _ => {},
                }
            }

            refined_blocks.push(Block {
                update_data,
                ..cloned_block
            });
        }

        let json = UpdateDataIncoming {
            blocks_amount,
            blocks: refined_blocks,
        }.get_json_details()?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_opcode_name(input.opcode as u32)
                .unwrap_or(format!("Unknown opcode: {}", input.opcode)),
            Some(json),
        ));

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::{Result as AnyResult};
    use std::collections::BTreeMap;
    use tentacli_traits::types::custom_fields::PackedGuid;
    use tentacli_traits::types::movement::{
        Movement, MovementExtraFlags, MovementFlags, MovementInfo, ObjectUpdateFlags, UnitMoveType
    };
    use tentacli_traits::types::opcodes::Opcode;
    use tentacli_traits::types::update_data::{BlockType, ObjectTypeID, ObjectTypeMask, UpdateData};
    use tentacli_traits::types::update_fields::{FieldValue, ObjectField, PlayerField, UnitField};
    use crate::features::wotlk_realm::player::handle_update_data::{Block, UpdateDataIncoming};

    #[test]
    fn test_packet_building() -> AnyResult<()> {
        const GUID: u64 = 123;
        const TYPE: i32 = ObjectTypeMask::PLAYER | ObjectTypeMask::UNIT | ObjectTypeMask::OBJECT;
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
                    let mut map: BTreeMap<ObjectField, FieldValue> = BTreeMap::new();
                    map.insert(ObjectField::Guid, FieldValue::Long(GUID));
                    map.insert(ObjectField::Type, FieldValue::Integer(TYPE));
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

        let packet = UpdateDataIncoming {
            blocks_amount: blocks.len() as u32,
            blocks,
        }.to_binary_with_server_opcode(Opcode::SMSG_UPDATE_OBJECT).unwrap();

        let (UpdateDataIncoming { blocks, .. }, _) = UpdateDataIncoming::from_binary(&packet[4..])?;

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