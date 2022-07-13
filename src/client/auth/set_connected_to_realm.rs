use crate::types::{HandlerInput, HandlerOutput, HandlerResult, State};

pub fn handler(_input: &mut HandlerInput) -> HandlerResult {
    Ok(HandlerOutput::UpdateState(State::SetConnectedToRealm(true)))
}