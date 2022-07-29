use byteorder::{LittleEndian, WriteBytesExt};

use crate::client::opcodes::Opcode;
use crate::network::packet::OutcomePacket;
use crate::types::{
    HandlerInput,
    HandlerOutput,
    HandlerResult
};

#[allow(dead_code)]
pub fn handler(input: &mut HandlerInput) -> HandlerResult {
    let mut body = Vec::new();
    body.write_u32::<LittleEndian>(0)?;
    body.write_u32::<LittleEndian>(0)?;

    input.message_sender.send_client_message(String::from("CMSG_PING"));

    Ok(HandlerOutput::Data(OutcomePacket::from(Opcode::CMSG_PING, Some(body))))
}