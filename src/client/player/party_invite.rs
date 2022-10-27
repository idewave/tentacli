use async_trait::async_trait;

use crate::packet;
use crate::client::opcodes::Opcode;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::traits::packet_handler::PacketHandler;

packet! {
    @option[world_opcode=Opcode::CMSG_GROUP_ACCEPT]
    // @option[world_opcode=Opcode::CMSG_GROUP_DECLINE]
    struct Outcome {
        unknown: u32,
    }
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, _input: &mut HandlerInput) -> HandlerResult {
        Ok(HandlerOutput::Data(Outcome::default().unpack()))
    }
}