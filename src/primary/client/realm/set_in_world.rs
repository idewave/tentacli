use async_trait::async_trait;

use crate::primary::shared::session::types::StateFlags;
use crate::primary::types::{HandlerInput, HandlerResult};
use crate::primary::traits::PacketHandler;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let response = Vec::new();

        input.session.lock().await.state_flags.set(StateFlags::IN_WORLD, true);

        Ok(response)
    }
}