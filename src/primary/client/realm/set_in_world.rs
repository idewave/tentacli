use async_trait::async_trait;
use tentacli_traits::PacketHandler;
use tentacli_traits::types::{HandlerInput, HandlerResult};
use tentacli_traits::types::shared::StateFlags;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let response = Vec::new();

        input.session.lock().await.state_flags.set(StateFlags::IN_WORLD, true);

        Ok(response)
    }
}