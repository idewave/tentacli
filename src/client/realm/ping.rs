use byteorder::{LittleEndian, WriteBytesExt};
use async_trait::async_trait;

use crate::client::opcodes::Opcode;
use crate::network::packet::OutcomePacket;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::types::traits::PacketHandler;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, _: &mut HandlerInput) -> HandlerResult {
        let mut body = Vec::new();
        body.write_u32::<LittleEndian>(0)?;
        body.write_u32::<LittleEndian>(0)?;

        Ok(HandlerOutput::Data(OutcomePacket::from(Opcode::CMSG_PING, Some(body))))
    }
}