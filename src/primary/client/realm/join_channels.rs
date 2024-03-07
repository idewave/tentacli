use async_trait::async_trait;

use crate::primary::client::chat::globals::JoinChannelOutcome;
use crate::primary::types::{HandlerInput, HandlerOutput, HandlerResult, TerminatedString};
use crate::primary::traits::PacketHandler;

const COMMON_CHANNEL_ID: u32 = 1;
const LFG_CHANNEL_ID: u32 = 26;
const TRADE_CHANNEL_ID: u32 = 2;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let channel_labels = &input.session.lock().await.get_config().unwrap().channel_labels.clone();

        response.push(HandlerOutput::Data(JoinChannelOutcome {
            channel_id: COMMON_CHANNEL_ID,
            channel_name: TerminatedString::from(channel_labels.common.to_string()),
            ..JoinChannelOutcome::default()
        }.unpack()?));

        response.push(HandlerOutput::Data(JoinChannelOutcome {
            channel_id: LFG_CHANNEL_ID,
            channel_name: TerminatedString::from(channel_labels.common.to_string()),
            ..JoinChannelOutcome::default()
        }.unpack()?));

        response.push(HandlerOutput::Data(JoinChannelOutcome {
            channel_id: TRADE_CHANNEL_ID,
            channel_name: TerminatedString::from(channel_labels.common.to_string()),
            ..JoinChannelOutcome::default()
        }.unpack()?));

        Ok(response)
    }
}