use async_trait::async_trait;

use crate::packet;
use super::opcodes::Opcode;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::traits::packet_handler::PacketHandler;

packet! {
    @option[login_opcode=Opcode::REALM_LIST]
    struct Outcome {
        unknown: u32,
    }
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, _: &mut HandlerInput) -> HandlerResult {
        Ok(HandlerOutput::Data(Outcome::default().unpack()))
    }
}