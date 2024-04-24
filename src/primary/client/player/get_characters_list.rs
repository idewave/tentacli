use anyhow::bail;
use async_trait::async_trait;
use regex::Regex;
use tentacli_traits::{CharacterListError, PacketHandler};
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::opcodes::Opcode;
use tentacli_traits::types::player::Player;

use crate::primary::client::player::globals::CharacterEnumOutcome;
use crate::primary::client::player::packet::CharCreateOutcome;
use crate::primary::client::player::traits::CharacterCreateToolkit;

#[derive(WorldPacket, Serialize, Deserialize, Debug)]
struct Income {
    characters_count: u8,
    #[depends_on(characters_count)]
    characters: Vec<Player>,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (Income { characters, .. }, json) = Income::from_binary(&input.data)?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_opcode_name(input.opcode as u32)
                .unwrap_or(format!("Unknown opcode: {}", input.opcode)),
            Some(json),
        ));

        let me_exists = {
            let guard = input.session.lock().await;
            guard.me.is_some()
        };

        if me_exists {
            return Ok(response);
        }

        let auto_create_character_for_new_account = {
            let guard = input.session.lock().await;
            let config = guard.get_config()?;
            config.common.auto_create_character_for_new_account
        };

        if characters.is_empty() {
            return if auto_create_character_for_new_account {
                let random_name = Self::generate_random_string(true);
                response.push(HandlerOutput::ResponseMessage(
                    format!("Creating character with name \"{}\"", random_name),
                    None,
                ));

                response.push(HandlerOutput::Data(CharCreateOutcome {
                    name: format!("{}\0", random_name),
                    race: Self::get_random_race(),
                    class: Self::get_random_class(),
                    gender: Self::get_random_gender(),
                    skin: 0,
                    face: 0,
                    hair_style: 0,
                    hair_color: 0,
                    facial_hair: 0,
                    outfit_id: 0,
                }.unpack_with_client_opcode(Opcode::CMSG_CHAR_CREATE)?));

                response.push(
                    HandlerOutput::Data(
                        CharacterEnumOutcome::default()
                            .unpack_with_client_opcode(Opcode::CMSG_CHAR_ENUM)?
                    )
                );

                Ok(response)
            } else {
                Ok(response)
            }
        }

        let name_pattern = {
            let guard = input.session.lock().await;
            let config = guard.get_config()?;
            config.connection_data.autoselect_character_name.to_string()
        };

        let autoselect_character: bool = !name_pattern.is_empty();

        if !autoselect_character {
            response.push(HandlerOutput::TransferCharactersList(characters));
            response.push(HandlerOutput::Freeze);
        } else {
            let re = Regex::new(&name_pattern).unwrap();
            if let Some(character) = characters.into_iter().find(|item| re.is_match(&item.name[..]))
            {
                response.push(HandlerOutput::DebugMessage(
                    format!("Selected \"{}\" Character", character.name),
                    None,
                ));
                input.session.lock().await.me = Some(character);
            } else if !auto_create_character_for_new_account {
                bail!(CharacterListError::NotFound);
            }
        }

        Ok(response)
    }
}

impl CharacterCreateToolkit for Handler {}