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
            let guard = input.session.lock().unwrap();
            guard.me.is_some()
        };
        
        if !me_exists {
            bail!(CharacterListError::Empty);
        }

        let my_guid = {
            input.session.lock().unwrap().me.as_ref().unwrap().guid
        };

        response.push(HandlerOutput::Data(Outcome { guid: my_guid }.unpack()?));

        Ok(response)
    }
}