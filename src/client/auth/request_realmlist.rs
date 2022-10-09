use byteorder::{LittleEndian, WriteBytesExt};
use async_trait::async_trait;

use super::opcodes::Opcode;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::types::traits::PacketHandler;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, _: &mut HandlerInput) -> HandlerResult {
        let mut header = Vec::new();
        header.write_u8(Opcode::REALM_LIST)?;

        let mut body = Vec::new();
        body.write_i32::<LittleEndian>(0)?;

        Ok(HandlerOutput::Data((Opcode::REALM_LIST as u32, header, body)))
    }
}