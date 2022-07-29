use crate::types::{
    HandlerInput,
    HandlerOutput,
    HandlerResult,
    State
};

pub fn handler(input: &mut HandlerInput) -> HandlerResult {
    let session_key = input.session.session_key.as_ref().unwrap();

    input.message_sender.send_debug_message(String::from("Set encryption"));

    Ok(HandlerOutput::UpdateState(State::SetEncryption(session_key.to_vec())))
}