use async_trait::async_trait;

use crate::primary::traits::PacketHandler;
use crate::primary::types::{HandlerInput, HandlerResult};
use crate::types::HandlerOutput;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, _: &mut HandlerInput) -> HandlerResult {
        let response = vec![HandlerOutput::ExitConfirmed];

        Ok(response)
    }
}