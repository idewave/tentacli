use async_trait::async_trait;

use crate::client::{FieldValue, ObjectField, Player};
use crate::client::opcodes::Opcode;
use crate::client::player::globals::NameQueryOutcome;
use crate::parsers::update_block_parser::types::{ObjectTypeMask, ParsedBlock};
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::traits::packet_handler::PacketHandler;

#[derive(WorldPacket, Serialize, Deserialize, Debug)]
#[options(no_opcode)]
struct Income {
    parsed_blocks: Vec<ParsedBlock>,
}

#[derive(WorldPacket, Serialize, Deserialize, Debug)]
#[options(no_opcode, compressed)]
struct CompressedIncome {
    parsed_blocks: Vec<ParsedBlock>,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let (parsed_blocks, json) = if input.opcode == Some(Opcode::SMSG_UPDATE_OBJECT) {
            let (Income { parsed_blocks }, json) = Income::from_binary(
                input.data.as_ref().unwrap()
            );
            (parsed_blocks, json)
        } else {
            let (CompressedIncome { parsed_blocks }, json) = CompressedIncome::from_binary(
                input.data.as_ref().unwrap()
            );
            (parsed_blocks, json)
        };

        input.message_income.send_server_message(
            Opcode::get_server_opcode_name(input.opcode.unwrap()),
            Some(json),
        );

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
                match parsed_block.update_fields.get(&ObjectField::TYPE) {
                    Some(type_mask) => {
                        if let FieldValue::LONG(mask) = type_mask {
                            match *mask {
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