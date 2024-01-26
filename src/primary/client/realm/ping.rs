use async_trait::async_trait;

use crate::primary::client::opcodes::Opcode;
use crate::primary::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::primary::traits::packet_handler::PacketHandler;
use crate::with_opcode;

with_opcode! {
    @world_opcode(Opcode::CMSG_PING)
    #[derive(WorldPacket, Serialize, Deserialize, Debug, Default)]
    struct Outcome {
        ping: u32,
        latency: u32,
    }
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, _: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();
        response.push(HandlerOutput::Data(Outcome::default().unpack()?));

        Ok(response)
    }
}