use async_trait::async_trait;

use crate::{with_opcode};
use crate::client::opcodes::Opcode;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::traits::packet_handler::PacketHandler;

with_opcode! {
    @world_opcode(Opcode::CMSG_READY_FOR_ACCOUNT_DATA_TIMES)
    #[derive(WorldPacket, Serialize, Deserialize, Debug, Default)]
    struct Outcome {}
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, _: &mut HandlerInput) -> HandlerResult {
        Ok(HandlerOutput::Data(Outcome::default().unpack()))
    }
}