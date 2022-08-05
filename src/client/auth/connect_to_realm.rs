use std::str::FromStr;

use crate::types::{
    HandlerInput,
    HandlerOutput,
    HandlerResult
};

pub fn handler(input: &mut HandlerInput) -> HandlerResult {
    let session = input.session.lock().unwrap();
    let realm = session.selected_realm.as_ref().unwrap();

    // https://rust-lang.github.io/rust-clippy/master/index.html#single_char_pattern
    let connection_data: Vec<&str> = realm.address.split(':').collect();

    let host = connection_data[0].to_string();
    let port = u16::from_str(connection_data[1]).unwrap();

    Ok(HandlerOutput::ConnectionRequest(host, port))
}