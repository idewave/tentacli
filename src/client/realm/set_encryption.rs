use crate::logger::types::LoggerOutput;
use crate::types::{
    HandlerInput,
    HandlerOutput,
    HandlerResult,
    State
};

pub fn handler(input: &mut HandlerInput) -> HandlerResult {
    let session_key = input.session.session_key.as_ref().unwrap();

    input.output_sender.send(LoggerOutput::Debug(String::from("Set encryption"))).unwrap();

    Ok(HandlerOutput::UpdateState(State::SetEncryption(session_key.to_vec())))
}