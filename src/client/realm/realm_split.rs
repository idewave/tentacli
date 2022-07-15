use byteorder::{WriteBytesExt};

use crate::client::opcodes::Opcode;
use crate::network::packet::OutcomePacket;
use crate::types::{
    HandlerInput,
    HandlerOutput,
    HandlerResult
};


pub fn handler(_: &mut HandlerInput) -> HandlerResult {
    let mut body = Vec::new();
    body.write_u8(0xFF)?;
    body.write_u8(0xFF)?;
    body.write_u8(0xFF)?;
    body.write_u8(0xFF)?;

    println!("SEND CMSG_REALM_SPLIT");

    Ok(HandlerOutput::Data(OutcomePacket::from(Opcode::CMSG_REALM_SPLIT, Some(body))))
}