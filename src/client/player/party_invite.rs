use byteorder::{LittleEndian, WriteBytesExt};
use async_trait::async_trait;

use crate::client::opcodes::Opcode;
use crate::network::packet::OutcomePacket;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::types::traits::PacketHandler;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let _data = input.data.as_ref().unwrap();

        let mut body = Vec::new();
        body.write_u32::<LittleEndian>(0)?;

        Ok(HandlerOutput::Data(
            OutcomePacket::from(Opcode::CMSG_GROUP_ACCEPT, Some(body))
        ))

        // Ok(HandlerOutput::Data(
        //     Opcode::CMSG_GROUP_DECLINE,
        //     OutcomePacket::new(Opcode::CMSG_GROUP_DECLINE, body)
        // ))
    }
}