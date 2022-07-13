use byteorder::{LittleEndian, WriteBytesExt};

use crate::client::opcodes::Opcode;
use crate::network::packet::OutcomePacket;
use crate::types::{
    HandlerInput,
    HandlerOutput,
    HandlerResult
};

#[allow(dead_code)]
pub fn handler(_: &mut HandlerInput) -> HandlerResult {
    let mut body = Vec::new();
    body.write_u32::<LittleEndian>(0)?;
    body.write_u32::<LittleEndian>(0)?;

    println!("SEND CMSG_PING");

    Ok(HandlerOutput::Data(OutcomePacket::new(Opcode::CMSG_PING, Some(body))))
}