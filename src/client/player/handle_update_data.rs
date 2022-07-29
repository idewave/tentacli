use std::io::{Cursor, Error, ErrorKind};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::client::{ObjectField, Player};
use crate::client::opcodes::Opcode;
use crate::crypto::decryptor::INCOMING_HEADER_LENGTH;
use crate::network::packet::{OutcomePacket, ParsedUpdatePacket};
use crate::network::packet::types::{ObjectTypeMask};
use crate::types::{
    HandlerInput,
    HandlerOutput,
    HandlerResult
};

pub fn handler(input: &mut HandlerInput) -> HandlerResult {
    // omit size
    let mut reader = Cursor::new(input.data.as_ref().unwrap()[2..].to_vec());
    let opcode = reader.read_u16::<LittleEndian>()?;

    let data = &input.data.as_ref().unwrap()[(INCOMING_HEADER_LENGTH as usize)..];

    let parsed_packet: Result<ParsedUpdatePacket, Error> = match opcode {
        Opcode::SMSG_UPDATE_OBJECT => Ok(ParsedUpdatePacket::new(data)),
        Opcode::SMSG_COMPRESSED_UPDATE_OBJECT => Ok(ParsedUpdatePacket::from_compressed(data)),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Wrong opcode"))
    };

    input.message_sender.send_debug_message(String::from("Handling update packet"));

    let me = input.session.me.as_mut().unwrap();

    for parsed_block in parsed_packet.unwrap().parsed_blocks {
        let guid = parsed_block.guid.unwrap();

        if me.guid != guid {
            match parsed_block.update_fields.get(&ObjectField::OBJECT_FIELD_TYPE) {
                Some(type_mask) => {
                    match *type_mask {
                        ObjectTypeMask::IS_PLAYER => {
                            if let Some(_player) = input.data_storage.players_map.get(&guid) {
                                // ...
                            } else {
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

                                input.data_storage.players_map.insert(guid, player);

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
                    if input.data_storage.players_map.get(&guid).is_none() {
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

                        input.data_storage.players_map.insert(guid, player);

                        return Ok(
                            HandlerOutput::Data(
                                OutcomePacket::from(Opcode::CMSG_NAME_QUERY, Some(body))
                            )
                        );
                    } else {
                        input.data_storage.players_map.entry(guid).and_modify(|p| {
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
                    me.position = Some(movement_info.position);
                }

                if !movement_data.movement_speed.is_empty() {
                    me.movement_speed = movement_data.movement_speed;
                }
            }

            if !parsed_block.update_fields.is_empty() {
                me.fields = parsed_block.update_fields;
            }
        }
    }

    Ok(HandlerOutput::Void)
}