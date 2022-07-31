use crate::client::opcodes::Opcode;
use crate::network::packet::OutcomePacket;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};

pub fn handler(input: &mut HandlerInput) -> HandlerResult {
    input.message_income.send_client_message(String::from("CMSG_READY_FOR_ACCOUNT_DATA_TIMES"));
    Ok(HandlerOutput::Data(OutcomePacket::from(Opcode::CMSG_READY_FOR_ACCOUNT_DATA_TIMES, None)))
}