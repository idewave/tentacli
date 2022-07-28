use byteorder::{LittleEndian, WriteBytesExt};
use crate::logger::types::LoggerOutput;

use crate::types::{HandlerInput, HandlerOutput, HandlerResult};

use super::opcodes::Opcode;

pub fn handler(input: &mut HandlerInput) -> HandlerResult {
    let mut header = Vec::new();
    header.write_u8(Opcode::REALM_LIST)?;

    let mut body = Vec::new();
    body.write_i32::<LittleEndian>(0)?;

    input.output_sender.send(LoggerOutput::Client("REALM_LIST".to_string())).unwrap();

    Ok(HandlerOutput::Data((Opcode::REALM_LIST as u32, header, body)))
}