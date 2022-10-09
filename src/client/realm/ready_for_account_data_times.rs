use async_trait::async_trait;

use crate::client::opcodes::Opcode;
use crate::network::packet::OutcomePacket;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::types::traits::PacketHandler;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, _: &mut HandlerInput) -> HandlerResult {
        Ok(HandlerOutput::Data(
            OutcomePacket::from(Opcode::CMSG_READY_FOR_ACCOUNT_DATA_TIMES, None)
        ))
    }
}