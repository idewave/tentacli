use async_trait::async_trait;

use crate::primary::types::{HandlerInput, HandlerResult};

#[async_trait]
pub trait PacketHandler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult;
}