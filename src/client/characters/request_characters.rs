use crate::client::opcodes::Opcode;
use crate::network::packet::OutcomePacket;
use crate::types::{
    HandlerInput,
    HandlerOutput,
    HandlerResult
};


pub fn handler(input: &mut HandlerInput) -> HandlerResult {
    input.message_sender.send_client_message(String::from("CMSG_CHAR_ENUM"));

    Ok(HandlerOutput::Data(OutcomePacket::from(Opcode::CMSG_CHAR_ENUM, None)))
}