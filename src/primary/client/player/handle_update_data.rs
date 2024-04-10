use async_trait::async_trait;
use tentacli_traits::PacketHandler;
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::opcodes::Opcode;
use tentacli_traits::types::parsed_block::{ObjectTypeMask, ParsedBlock};
use tentacli_traits::types::player::{FieldValue, Gender, ObjectField, Player};
use crate::primary::client::player::globals::NameQueryOutcome;


#[derive(WorldPacket, Serialize, Deserialize, Debug)]
struct Income {
    parsed_blocks: Vec<ParsedBlock>,
}

#[derive(WorldPacket, Serialize, Deserialize, Debug)]
#[options(compressed)]
struct CompressedIncome {
    parsed_blocks: Vec<ParsedBlock>,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (parsed_blocks, json) = if input.opcode == Opcode::SMSG_UPDATE_OBJECT {
            let (Income { parsed_blocks }, json) = Income::from_binary(&input.data)?;

            (parsed_blocks, json)
        } else {
            let (CompressedIncome {
                parsed_blocks
            }, json) = CompressedIncome::from_binary(&input.data)?;

            (parsed_blocks, json)
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

        for parsed_block in parsed_blocks {
            if parsed_block.guid.is_none() {
                continue;
            }

            let guid = parsed_block.guid.unwrap();

            if my_guid != guid {
                match parsed_block.update_fields.get(&ObjectField::TYPE) {
                    Some(type_mask) => {

                        if let FieldValue::Integer(mask) = type_mask {
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
                                            vec![HandlerOutput::Data(
                                                NameQueryOutcome { guid }
                                                    .unpack_with_opcode(Opcode::CMSG_NAME_QUERY)?
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

                            return Ok(
                                vec![
                                    HandlerOutput::Data(
                                        NameQueryOutcome { guid }
                                            .unpack_with_opcode(Opcode::CMSG_NAME_QUERY)?
                                    )
                                ]
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
                        input.session.lock().await
                            .me.as_mut().unwrap().position = Some(movement_info.position);
                    }

                    if !movement_data.movement_speed.is_empty() {
                        input.session.lock().await
                            .me.as_mut().unwrap().movement_speed = movement_data.movement_speed;
                    }
                }

                if !parsed_block.update_fields.is_empty() {
                    input.session.lock().await
                        .me.as_mut().unwrap().fields = parsed_block.update_fields;
                }

                let me = input.session.lock().await.me.clone().unwrap();
                response.push(HandlerOutput::UpdatePlayer(me));
            }
        }

        Ok(response)
    }
}