use byteorder::{LittleEndian, WriteBytesExt};

use crate::client::opcodes::Opcode;
use crate::network::packet::OutcomePacket;
use crate::types::{
    HandlerInput,
    HandlerOutput,
    HandlerResult
};

pub fn handler(input: &mut HandlerInput) -> HandlerResult {
    let _data = input.data.as_ref().unwrap();

    let mut body = Vec::new();
    body.write_u32::<LittleEndian>(0)?;

    Ok(HandlerOutput::Data(OutcomePacket::from(Opcode::CMSG_GROUP_ACCEPT, Some(body))))

    // Ok(HandlerOutput::Data(
    //     Opcode::CMSG_GROUP_DECLINE,
    //     OutcomePacket::new(Opcode::CMSG_GROUP_DECLINE, body)
    // ))
}