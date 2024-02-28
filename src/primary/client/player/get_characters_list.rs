use anyhow::bail;
use async_trait::async_trait;
use regex::Regex;

use crate::packet::player::CharCreateOutcome;
use crate::primary::client::{Opcode, Player};
use crate::primary::client::player::globals::CharacterEnumOutcome;
use crate::primary::client::player::traits::CharacterCreateToolkit;
use crate::primary::errors::CharacterListError;
use crate::primary::types::{HandlerInput, HandlerOutput, HandlerResult, TerminatedString};
use crate::primary::traits::packet_handler::PacketHandler;

#[derive(WorldPacket, Serialize, Deserialize, Debug)]
#[options(no_opcode)]
struct Income {
    characters: Vec<Player>,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (Income { characters }, json) = Income::from_binary(&input.data)?;

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
                    name: TerminatedString::from(random_name),
                    race: Self::get_random_race(),
                    class: Self::get_random_class(),
                    gender: Self::get_random_gender(),
                    skin: 0,
                    face: 0,
                    hair_style: 0,
                    hair_color: 0,
                    facial_hair: 0,
                    outfit_id: 0,
                }.unpack()?));

                response.push(HandlerOutput::Data(CharacterEnumOutcome::default().unpack()?));

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