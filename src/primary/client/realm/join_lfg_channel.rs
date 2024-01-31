use async_trait::async_trait;

use crate::primary::client::chat::globals::JoinChannelOutcome;
use crate::primary::types::{HandlerInput, HandlerOutput, HandlerResult, TerminatedString};
use crate::primary::traits::packet_handler::PacketHandler;

const CHANNEL_ID: u32 = 26;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let channel_name = input.session
            .lock()
            .await
            .get_config()
            .unwrap()
            .channel_labels.lfg.to_string();

        response.push(HandlerOutput::Data(JoinChannelOutcome {
            channel_id: CHANNEL_ID,
            channel_name: TerminatedString::from(channel_name),
            ..JoinChannelOutcome::default()
        }.unpack()?));

        Ok(response)
    }
}