use std::io::{Cursor};
use byteorder::{LittleEndian, ReadBytesExt};
use async_trait::async_trait;

use crate::packet;
use crate::client::{ObjectField, Player};
use crate::client::opcodes::Opcode;
use crate::client::player::globals::NameQueryOutcome;
use crate::parsers::update_block_parser::types::{ObjectTypeMask, ParsedBlock};
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::traits::packet_handler::PacketHandler;

packet! {
    @option[compressed_if: Opcode::SMSG_COMPRESSED_UPDATE_OBJECT]
    struct Income {
        parsed_blocks: Vec<ParsedBlock>,
    }
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let Income { parsed_blocks } = Income::from_binary(
            input.data.as_ref().unwrap(),
        );

        input.message_income.send_debug_message(String::from("Handling update packet"));

        let my_guid = {
            input.session.lock().unwrap().me.as_ref().unwrap().guid
        };

        let mut players_map = {
            let guard = input.data_storage.lock().unwrap();
            guard.players_map.clone()
        };

        for parsed_block in parsed_blocks {
            let guid = parsed_block.guid.unwrap();

            if my_guid != guid {
                match parsed_block.update_fields.get(&ObjectField::OBJECT_FIELD_TYPE) {
                    Some(type_mask) => {
                        match *type_mask {
                            ObjectTypeMask::IS_PLAYER => {
                                if players_map.get(&guid).is_none() {
                                    let mut player = Player {
                                        guid,
                                        .. Player::default()
                                    };

                                    if let Some(movement_data) = parsed_block.movement_data {
                                        if let Some(movement_info) = movement_data.movement_info {
                                            player.position = Some(movement_info.position);
                                        }

                                        if !movement_data.movement_speed.is_empty() {
                                            player.movement_speed = movement_data.movement_speed;
                                        }
                                    }

                                    if !parsed_block.update_fields.is_empty() {
                                        player.fields = parsed_block.update_fields;
                                    }

                                    input.data_storage.lock()
                                        .unwrap().players_map.insert(guid, player);

                                    return Ok(
                                        HandlerOutput::Data(NameQueryOutcome { guid }.unpack())
                                    );
                                }
                            },
                            ObjectTypeMask::IS_UNIT => {},
                            _ => {},
                        }
                    },
                    None => {
                        if players_map.get(&guid).is_none() {
                            let mut player = Player::new(guid, String::new(), 0, 0);

                            if let Some(movement_data) = parsed_block.movement_data {
                                if let Some(movement_info) = movement_data.movement_info {
                                    player.position = Some(movement_info.position);
                                }

                                if !movement_data.movement_speed.is_empty() {
                                    player.movement_speed = movement_data.movement_speed;
                                }
                            }

                            if !parsed_block.update_fields.is_empty() {
                                player.fields = parsed_block.update_fields;
                            }

                            input.data_storage.lock().unwrap().players_map.insert(guid, player);

                            return Ok(HandlerOutput::Data(NameQueryOutcome { guid }.unpack()));
                        } else {
                            players_map.entry(guid).and_modify(|p| {
                                if let Some(movement_data) = parsed_block.movement_data {
                                    if let Some(movement_info) = movement_data.movement_info {
                                        p.position = Some(movement_info.position);
                                    }

                                    if !movement_data.movement_speed.is_empty() {
                                        p.movement_speed = movement_data.movement_speed;
                                    }
                                }
                            });
                        }
                    },
                }
            } else {
                if let Some(movement_data) = parsed_block.movement_data {
                    if let Some(movement_info) = movement_data.movement_info {
                        input.session.lock().unwrap().me.as_mut().unwrap().position = Some(movement_info.position);
                    }

                    if !movement_data.movement_speed.is_empty() {
                        input.session.lock().unwrap().me.as_mut().unwrap().movement_speed = movement_data.movement_speed;
                    }
                }

                if !parsed_block.update_fields.is_empty() {
                    input.session.lock().unwrap().me.as_mut().unwrap().fields = parsed_block.update_fields;
                }
            }
        }

        Ok(HandlerOutput::Void)
    }
}