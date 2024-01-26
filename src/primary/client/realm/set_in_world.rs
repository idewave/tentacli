use async_trait::async_trait;

use crate::primary::ipc::session::types::StateFlags;
use crate::primary::types::{HandlerInput, HandlerResult};
use crate::primary::traits::packet_handler::PacketHandler;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let response = Vec::new();

        input.session.lock().unwrap().state_flags.set(StateFlags::IN_WORLD, true);

        Ok(response)
    }
}