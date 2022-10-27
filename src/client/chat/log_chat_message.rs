use async_trait::async_trait;

use crate::packet;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult, TerminatedString};
use crate::traits::packet_handler::PacketHandler;

packet! {
    struct Income {
        message_type: u8,
        language: u32,
        skip: u32,
        // if @message_type == MessageType::CHANNEL
        channel_name: TerminatedString,
        target_guid: u64,
        message_length: u32,
        // length=@message_length
        message: String,
    }
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let Income { .. } = Income::from_binary(input.data.as_ref().unwrap());

        Ok(HandlerOutput::Void)
    }
}