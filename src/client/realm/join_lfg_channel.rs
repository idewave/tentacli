use async_trait::async_trait;

use crate::client::chat::globals::JoinChannelOutcome;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
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

        input.message_income.send_debug_message(format!("JOINING '{}' channel", &channel_name));

        Ok(HandlerOutput::Data(JoinChannelOutcome {
            channel_id: CHANNEL_ID,
            channel_name: format!("{}\0", channel_name),
            ..JoinChannelOutcome::default()
        }.unpack()))
    }
}