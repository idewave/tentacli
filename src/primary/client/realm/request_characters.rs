use async_trait::async_trait;

use crate::{with_opcode};
use crate::primary::client::opcodes::Opcode;
use crate::primary::types::{
    HandlerInput,
    HandlerOutput,
    HandlerResult
};
use crate::primary::traits::packet_handler::PacketHandler;

with_opcode! {
    @world_opcode(Opcode::CMSG_CHAR_ENUM)
    #[derive(WorldPacket, Serialize, Deserialize, Debug, Default)]
    struct Outcome {}
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