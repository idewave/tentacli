use std::io::{Error, ErrorKind};
use async_trait::async_trait;

use crate::{with_opcode};
use crate::client::opcodes::Opcode;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::traits::packet_handler::PacketHandler;

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
        let me_exists = {
            let guard = input.session.lock().unwrap();
            guard.me.is_some()
        };
        
        if !me_exists {
            return Err(Error::new(
                ErrorKind::NotFound,
                "No character selected ! Seems like char list is empty !"
            ));
        }

        let my_guid = {
            input.session.lock().unwrap().me.as_ref().unwrap().guid
        };

        Ok(HandlerOutput::Data(Outcome { guid: my_guid }.unpack()))
    }
}