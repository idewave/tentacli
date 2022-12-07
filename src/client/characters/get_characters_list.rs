use async_trait::async_trait;

use crate::types::{
    HandlerInput,
    HandlerOutput,
    HandlerResult,
};
use crate::traits::packet_handler::PacketHandler;
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
        let Income { characters } = Income::from_binary(input.data.as_ref().unwrap());

        let me_exists = {
            let guard = input.session.lock().unwrap();
            guard.me.is_some()
        };

        if me_exists {
            return Ok(HandlerOutput::Void);
        }

        if characters.is_empty() {
            return Ok(HandlerOutput::Void);
        }

        input.dialog_income.send_characters_dialog(characters);

        Ok(HandlerOutput::Freeze)
    }
}