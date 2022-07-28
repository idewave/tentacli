use std::io::{Error, ErrorKind};
use byteorder::{LittleEndian, WriteBytesExt};

use crate::client::opcodes::Opcode;
use crate::logger::types::LoggerOutput;
use crate::network::packet::OutcomePacket;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};

pub fn handler(input: &mut HandlerInput) -> HandlerResult {
    if input.session.me.is_none() {
        return Err(Error::new(
            ErrorKind::NotFound,
            "No character selected ! Seems like char list is empty !"
        ));
    }

    let me = input.session.me.as_ref().unwrap();

    let mut body = Vec::new();
    body.write_u64::<LittleEndian>(me.guid)?;

    input.output_sender.send(LoggerOutput::Client(String::from("CMSG_PLAYER_LOGIN"))).unwrap();

    Ok(HandlerOutput::Data(OutcomePacket::from(Opcode::CMSG_PLAYER_LOGIN, Some(body))))
}