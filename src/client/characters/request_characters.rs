use crate::client::opcodes::Opcode;
use crate::logger::types::LoggerOutput;
use crate::network::packet::OutcomePacket;
use crate::types::{
    HandlerInput,
    HandlerOutput,
    HandlerResult
};


pub fn handler(input: &mut HandlerInput) -> HandlerResult {
    input.output_sender.send(LoggerOutput::Client(String::from("CMSG_CHAR_ENUM"))).unwrap();

    Ok(HandlerOutput::Data(OutcomePacket::from(Opcode::CMSG_CHAR_ENUM, None)))
}