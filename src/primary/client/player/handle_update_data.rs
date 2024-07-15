use async_trait::async_trait;
use tentacli_traits::{PacketHandler};
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::custom_fields::PackedGuid;
use tentacli_traits::types::opcodes::Opcode;
use tentacli_traits::types::player::{Gender, Player};
use tentacli_traits::types::update_data::{ObjectTypeMask};
use tentacli_traits::types::update_fields::{FieldValue, ObjectField};

use crate::primary::client::player::globals::NameQueryOutcome;
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

        let mut players_map = {
            let guard = input.data_storage.lock().unwrap();
            guard.players_map.clone()
        };

        for block in blocks {
            if block.guid == 0 {
                continue;
            }

            let PackedGuid(guid) = block.guid;

            if my_guid != guid {
                match block.update_data.object_fields.get(&ObjectField::Type) {
                    Some(type_mask) => {

                        if let FieldValue::Integer(mask) = type_mask {
                            match *mask {
                                ObjectTypeMask::IS_PLAYER => {
                                    if players_map.get(&guid).is_none() {
                                        let mut player = Player {
                                            guid,
                                            .. Player::default()
                                        };

                                        // if let Some(movement) = block.movement {
                                            if let Some(movement_info) = block.movement.movement_info {
                                                player.location = Some(movement_info.location);
                                            }

                                            if !block.movement.movement_speed.is_empty() {
                                                player.movement_speed = block.movement.movement_speed;
                                            }
                                        // }

                                        // if !block.update_data.update_fields.is_empty() {
                                        //     player.fields = block.update_data.update_fields;
                                        // }

                                        input.data_storage.lock()
                                            .unwrap().players_map.insert(guid, player);

                                        return Ok(
                                            vec![HandlerOutput::Data(
                                                NameQueryOutcome { guid }
                                                    .unpack_with_client_opcode(
                                                        Opcode::CMSG_NAME_QUERY
                                                    )?
                                            )]
                                        );
                                    }
                                },
                                ObjectTypeMask::IS_UNIT => {},
                                _ => {},
                            }
                        }
                    },
                    None => {
                        if players_map.get(&guid).is_none() {
                            let mut player = Player::new(
                                guid, String::new(), 0, 0, Gender::GENDER_NONE, 1
                            );

                            // if let Some(movement_data) = block.movement {
                                if let Some(movement_info) = block.movement.movement_info {
                                    player.location = Some(movement_info.location);
                                }

                                if !block.movement.movement_speed.is_empty() {
                                    player.movement_speed = block.movement.movement_speed;
                                }
                            // }

                            // if !block.update_data.update_fields.is_empty() {
                            //     player.fields = block.update_data.update_fields;
                            // }

                            input.data_storage.lock().unwrap().players_map.insert(guid, player);

                            return Ok(
                                vec![
                                    HandlerOutput::Data(
                                        NameQueryOutcome { guid }
                                            .unpack_with_client_opcode(Opcode::CMSG_NAME_QUERY)?
                                    )
                                ]
                            );
                        } else {
                            players_map.entry(guid).and_modify(|p| {
                                // if let Some(movement_data) = block.movement_data {
                                    if let Some(movement_info) = block.movement.movement_info {
                                        p.location = Some(movement_info.location);
                                    }

                                    if !block.movement.movement_speed.is_empty() {
                                        p.movement_speed = block.movement.movement_speed;
                                    }
                                // }
                            });
                        }
                    },
                }
            } else {
                if let Some(movement_info) = block.movement.movement_info {
                    input.session.lock().await
                        .me.as_mut().unwrap().location = Some(movement_info.location);
                }

                if !block.movement.movement_speed.is_empty() {
                    input.session.lock().await
                        .me.as_mut().unwrap().movement_speed = block.movement.movement_speed;
                }

                // if !block.update_data.update_fields.is_empty() {
                //     input.session.lock().await
                //         .me.as_mut().unwrap().fields = block.update_data.update_fields;
                // }

                let me = input.session.lock().await.me.clone().unwrap();
                response.push(HandlerOutput::UpdatePlayer(me));
            }
        }

        Ok(response)
    }
}