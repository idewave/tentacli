use async_trait::async_trait;

use crate::ipc::session::types::StateFlags;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::types::traits::PacketHandler;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        input.session.lock().unwrap().state_flags.set(StateFlags::IN_WORLD, true);

        Ok(HandlerOutput::Void)
    }
}