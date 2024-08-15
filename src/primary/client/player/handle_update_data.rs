use async_trait::async_trait;
use tentacli_traits::{PacketHandler};
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::opcodes::Opcode;
use tentacli_traits::types::player::{Player};

use crate::primary::client::player::packet::UpdateDataIncoming;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (UpdateDataIncoming { blocks, .. }, json) = if input.opcode == Opcode::SMSG_UPDATE_OBJECT {
            UpdateDataIncoming::from_binary(&input.data)?
        } else {
            UpdateDataIncoming::from_compressed_binary(&input.data)?
        };

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_opcode_name(input.opcode as u32)
                .unwrap_or(format!("Unknown opcode: {}", input.opcode)),
            Some(json),
        ));

        let my_guid = {
            input.session.lock().await.me.as_ref().unwrap().guid
        };

        for block in blocks {
            if block.guid == 0 {
                continue;
            }

            if block.guid == my_guid {
                let mut guard = input.session.lock().await;
                let me = guard.me.as_mut().unwrap();
                me.update_data = block.update_data;

                if let Some(movement_info) = block.movement.movement_info {
                    me.movement_info = movement_info;
                }
            } else {
                let mut player = Player::default();
                player.update_data = block.update_data;

                if let Some(movement_info) = block.movement.movement_info {
                    player.movement_info = movement_info;
                }

                let mut guard = input.data_storage.lock().unwrap();
                guard.players_map.insert(block.guid.0, player);
            }
        }

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::{Result as AnyResult};
    use std::collections::BTreeMap;
    use tentacli_traits::types::custom_fields::PackedGuid;
    use tentacli_traits::types::movement::{Movement, MovementExtraFlags, MovementFlags, MovementInfo, ObjectUpdateFlags, UnitMoveType};
    use tentacli_traits::types::opcodes::Opcode;
    use tentacli_traits::types::update_data::{BlockType, ObjectTypeID, UpdateData};
    use tentacli_traits::types::update_fields::{FieldValue, ObjectField, PlayerField, UnitField};
    use crate::primary::client::player::packet::{Block, UpdateDataIncoming};

    #[test]
    fn test_packet_building() -> AnyResult<()> {
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
                    let mut map: BTreeMap<ObjectField, FieldValue> = BTreeMap::new();
                    map.insert(ObjectField::Guid, FieldValue::Long(GUID));
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
                }
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