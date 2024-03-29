use anyhow::bail;
use async_trait::async_trait;

use crate::primary::macros::with_opcode;
use crate::primary::client::opcodes::Opcode;
use crate::primary::errors::CharacterListError;
use crate::primary::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::primary::traits::packet_handler::PacketHandler;

with_opcode! {
    @world_opcode(Opcode::CMSG_PLAYER_LOGIN)
    #[derive(WorldPacket, Serialize, Deserialize, Debug)]
    struct Outcome {
        guid: u64,
    }
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let me_exists = {
            let guard = input.session.lock().await;
            guard.me.is_some()
        };

        let auto_create_character_for_new_account = {
            let guard = input.session.lock().await;
            let config = guard.get_config()?;
            config.common.auto_create_character_for_new_account
        };
        
        if !me_exists {
            if auto_create_character_for_new_account {
                return Ok(response);
            }

            bail!(CharacterListError::Empty);
        }

        let my_guid = {
            input.session.lock().await.me.as_ref().unwrap().guid
        };

        response.push(HandlerOutput::Data(Outcome { guid: my_guid }.unpack()?));

        Ok(response)
    }
}