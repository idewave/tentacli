use std::io::{BufRead, Cursor};

use crate::types::{HandlerInput, HandlerOutput, HandlerResult};

pub fn handler(input: &mut HandlerInput) -> HandlerResult {
    let mut reader = Cursor::new(input.data.as_ref().unwrap()[8..].to_vec());

    let mut message = Vec::new();
    reader.read_until(0, &mut message)?;

    Ok(HandlerOutput::Void)
}