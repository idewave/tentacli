use async_trait::async_trait;

use crate::packet;
use crate::client::opcodes::Opcode;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::traits::packet_handler::PacketHandler;

packet! {
    struct Outcome {
        ping: u32,
        latency: u32,
    }
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, _: &mut HandlerInput) -> HandlerResult {
        let packet = Outcome::default().to_binary();

        Ok(HandlerOutput::Data((Opcode::CMSG_PING, packet)))
    }
}