use std::io::{Error, ErrorKind};
use byteorder::{LittleEndian, WriteBytesExt};

use crate::client::opcodes::Opcode;
use crate::network::packet::OutcomePacket;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};

pub fn handler(input: &mut HandlerInput) -> HandlerResult {
    let session = input.session.lock().unwrap();
    if session.me.is_none() {
        return Err(Error::new(
            ErrorKind::NotFound,
            "No character selected ! Seems like char list is empty !"
        ));
    }

    let me = session.me.as_ref().unwrap();

    let mut body = Vec::new();
    body.write_u64::<LittleEndian>(me.guid)?;

    input.message_income.send_client_message(String::from("CMSG_PLAYER_LOGIN"));

    Ok(HandlerOutput::Data(OutcomePacket::from(Opcode::CMSG_PLAYER_LOGIN, Some(body))))
}