use crate::client::opcodes::Opcode;
use crate::network::packet::OutcomePacket;
use crate::types::{
    HandlerInput,
    HandlerOutput,
    HandlerResult
};


pub fn handler(_: &mut HandlerInput) -> HandlerResult {
    Ok(HandlerOutput::Data(OutcomePacket::from(Opcode::CMSG_READY_FOR_ACCOUNT_DATA_TIMES, None)))
}