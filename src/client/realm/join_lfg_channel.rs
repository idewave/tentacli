use async_trait::async_trait;

use crate::client::chat::globals::JoinChannelOutcome;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult, TerminatedString};
use crate::traits::packet_handler::PacketHandler;

const CHANNEL_ID: u32 = 26;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let channel_name = input.session
            .lock()
            .unwrap()
            .get_config()
            .unwrap()
            .channels.lfg.to_string();

        Ok(HandlerOutput::Data(JoinChannelOutcome {
            channel_id: CHANNEL_ID,
            channel_name: TerminatedString::from(channel_name),
            ..JoinChannelOutcome::default()
        }.unpack()))
    }
}