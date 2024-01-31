use std::str::FromStr;
use async_trait::async_trait;

use crate::primary::types::{
    HandlerInput,
    HandlerOutput,
    HandlerResult
};
use crate::primary::traits::packet_handler::PacketHandler;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let realm_address = input.session
            .lock()
            .await
            .selected_realm.as_ref()
            .unwrap()
            .address.to_string();

        // https://rust-lang.github.io/rust-clippy/master/index.html#single_char_pattern
        let connection_data: Vec<&str> = realm_address.split(':').collect();

        let host = connection_data[0].to_string();
        let port = u16::from_str(connection_data[1]).unwrap();

        response.push(HandlerOutput::ConnectionRequest(host, port));

        Ok(response)
    }
}