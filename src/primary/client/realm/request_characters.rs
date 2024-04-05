use async_trait::async_trait;

use crate::primary::client::player::globals::CharacterEnumOutcome;
use crate::primary::types::{
    HandlerInput,
    HandlerOutput,
    HandlerResult
};
use crate::primary::traits::PacketHandler;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, _: &mut HandlerInput) -> HandlerResult {
        let response = vec![HandlerOutput::Data(CharacterEnumOutcome::default().unpack()?)];

        Ok(response)
    }
}