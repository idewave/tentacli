use byteorder::{LittleEndian, WriteBytesExt};

use crate::types::{HandlerInput, HandlerOutput, HandlerResult};

use super::opcodes::Opcode;

pub fn handler(input: &mut HandlerInput) -> HandlerResult {
    let mut header = Vec::new();
    header.write_u8(Opcode::REALM_LIST)?;

    let mut body = Vec::new();
    body.write_i32::<LittleEndian>(0)?;

    input.message_income.send_client_message(
        String::from("REALM_LIST")
    );

    Ok(HandlerOutput::Data((Opcode::REALM_LIST as u32, header, body)))
}