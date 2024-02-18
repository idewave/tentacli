use anyhow::bail;
use async_trait::async_trait;
use regex::Regex;

use crate::primary::client::{Opcode, Player};
use crate::primary::errors::CharacterListError;
use crate::primary::types::{
    HandlerInput,
    HandlerOutput,
    HandlerResult,
};
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
            return Ok(vec![]);
        }

        if characters.is_empty() {
            return Ok(vec![]);
        }

        let autoselect_character_name = {
            let guard = input.session.lock().await;
            let config = guard.get_config()?;
            config.connection_data.autoselect_character_name.to_string()
        };

        if autoselect_character_name.is_empty() {
            response.push(HandlerOutput::TransferCharactersList(characters));
            response.push(HandlerOutput::Freeze);
        } else {
            let re = Regex::new(&autoselect_character_name).unwrap();
            if let Some(character) = characters.into_iter().find(|item| re.is_match(&item.name[..])) {
                response.push(HandlerOutput::DebugMessage(
                    format!("Selected \"{}\" Character", character.name),
                    None,
                ));
                input.session.lock().await.me = Some(character);
            } else {
                bail!(CharacterListError::NotFound);
            }
        }

        Ok(response)
    }
}