use async_trait::async_trait;

use crate::packet;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::traits::packet_handler::PacketHandler;
use super::opcodes::Opcode;

packet! {
    @option[login_opcode=Opcode::REALM_LIST]
    struct Outcome {
        unknown: i32,
    }
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, _: &mut HandlerInput) -> HandlerResult {
        Ok(HandlerOutput::Data(Outcome::default().unpack()))
    }
}