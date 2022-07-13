use byteorder::{LittleEndian, WriteBytesExt};

use crate::types::{HandlerInput, HandlerOutput, HandlerResult};

use super::opcodes::Opcode;

pub fn handler(_input: &mut HandlerInput) -> HandlerResult {
    let mut header = Vec::new();
    header.write_u8(Opcode::REALM_LIST)?;

    let mut body = Vec::new();
    body.write_i32::<LittleEndian>(0)?;

    Ok(HandlerOutput::Data((Opcode::REALM_LIST as u32, header, body)))
}