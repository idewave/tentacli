use std::io::{BufRead, Cursor};
use async_trait::async_trait;

use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::types::traits::PacketHandler;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap()[8..].to_vec());

        let mut message = Vec::new();
        reader.read_until(0, &mut message)?;

        Ok(HandlerOutput::Void)
    }
}