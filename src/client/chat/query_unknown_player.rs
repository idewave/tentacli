use async_trait::async_trait;

use crate::packet;
use crate::client::player::globals::NameQueryOutcome;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::traits::packet_handler::PacketHandler;

packet! {
    struct Income {
        skip: [u8; 5],
        sender_guid: u64,
    }
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let Income { sender_guid, .. } = Income::from_binary(input.data.as_ref().unwrap());

        let players_map = &mut input.data_storage.lock().unwrap().players_map;
        if players_map.get(&sender_guid).is_none() {
            return Ok(HandlerOutput::Data(NameQueryOutcome { guid: sender_guid }.unpack()));
        }

        Ok(HandlerOutput::Void)
    }
}
