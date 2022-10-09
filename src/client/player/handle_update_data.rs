use std::io::{Cursor, Error, ErrorKind};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use async_trait::async_trait;

use crate::client::{ObjectField, Player};
use crate::client::opcodes::Opcode;
use crate::crypto::decryptor::INCOMING_HEADER_LENGTH;
use crate::network::packet::{ObjectTypeMask, OutcomePacket, ParsedUpdatePacket};
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::types::traits::PacketHandler;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        // omit size
        let mut reader = Cursor::new(input.data.as_ref().unwrap()[2..].to_vec());
        let opcode = reader.read_u16::<LittleEndian>()?;

        let data = &input.data.as_ref().unwrap()[(INCOMING_HEADER_LENGTH as usize)..];

        let parsed_packet: Result<ParsedUpdatePacket, Error> = match opcode {
            Opcode::SMSG_UPDATE_OBJECT => Ok(ParsedUpdatePacket::new(data)),
            Opcode::SMSG_COMPRESSED_UPDATE_OBJECT => Ok(ParsedUpdatePacket::from_compressed(data)),
            _ => Err(Error::new(ErrorKind::InvalidInput, "Wrong opcode"))
        };

        input.message_income.send_debug_message(String::from("Handling update packet"));

        let my_guid = input.session.lock().unwrap().me.as_ref().unwrap().guid;

        let players_map = &mut input.data_storage.lock().unwrap().players_map;

        for parsed_block in parsed_packet.unwrap().parsed_blocks {
            let guid = parsed_block.guid.unwrap();

            if my_guid != guid {
                match parsed_block.update_fields.get(&ObjectField::OBJECT_FIELD_TYPE) {
                    Some(type_mask) => {
                        match *type_mask {
                            ObjectTypeMask::IS_PLAYER => {
                                if players_map.get(&guid).is_none() {
                                    let mut body = Vec::new();
                                    body.write_u64::<LittleEndian>(guid)?;

                                    let mut player = Player::default();
                                    player.guid = guid;

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

                                    players_map.insert(guid, player);

                                    let mut body = Vec::new();
                                    body.write_u64::<LittleEndian>(guid)?;

                                    return Ok(
                                        HandlerOutput::Data(
                                            OutcomePacket::from(Opcode::CMSG_NAME_QUERY, Some(body))
                                        )
                                    );
                                }
                            },
                            ObjectTypeMask::IS_UNIT => {},
                            _ => {},
                        }
                    },
                    None => {
                        if players_map.get(&guid).is_none() {
                            let mut body = Vec::new();
                            body.write_u64::<LittleEndian>(guid)?;

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

                            players_map.insert(guid, player);

                            let mut body = Vec::new();
                            body.write_u64::<LittleEndian>(guid)?;

                            return Ok(
                                HandlerOutput::Data(
                                    OutcomePacket::from(Opcode::CMSG_NAME_QUERY, Some(body))
                                )
                            );
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