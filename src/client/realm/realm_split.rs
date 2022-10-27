use async_trait::async_trait;

use crate::packet;
use crate::client::opcodes::Opcode;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::traits::packet_handler::PacketHandler;

packet! {
    @option[world_opcode=Opcode::CMSG_REALM_SPLIT]
    struct Outcome {
        unknown: [u8; 4],
    }
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, _: &mut HandlerInput) -> HandlerResult {
        Ok(HandlerOutput::Data(Outcome {
            unknown: [0xFF, 0xFF, 0xFF, 0xFF]
        }.unpack()))
    }
}