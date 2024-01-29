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
use super::types::Character;

#[derive(WorldPacket, Serialize, Deserialize, Debug)]
#[options(no_opcode)]
struct Income {
    characters: Vec<Character>,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (Income { characters }, json) = Income::from_binary(input.data.as_ref().unwrap())?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_server_opcode_name(input.opcode.unwrap()),
            Some(json),
        ));

        let me_exists = {
            let guard = input.session.lock().unwrap();
            guard.me.is_some()
        };

        if me_exists {
            return Ok(vec![]);
        }

        if characters.is_empty() {
            return Ok(vec![]);
        }

        let autoselect_character_name = {
            let guard = input.session.lock().unwrap();
            let config = guard.get_config()?;
            config.connection_data.autoselect_character_name.to_string()
        };

        if autoselect_character_name.is_empty() {
            response.push(HandlerOutput::TransferCharactersList(characters));
            response.push(HandlerOutput::Freeze);
        } else {
            let re = Regex::new(format!(r#"{}"#, autoselect_character_name).as_str()).unwrap();
            if let Some(character) = characters.into_iter().find(|item| re.is_match(&item.name[..])) {
                input.session.lock().unwrap().me = Some(Player::from(character));
            } else {
                bail!(CharacterListError::NotFound);
            }
        }

        Ok(response)
    }
}