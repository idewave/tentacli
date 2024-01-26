use async_trait::async_trait;

use crate::primary::client::Opcode;
use crate::primary::client::player::globals::NameQueryOutcome;
use crate::primary::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::primary::traits::packet_handler::PacketHandler;

#[derive(WorldPacket, Serialize, Deserialize, Debug)]
#[options(no_opcode)]
struct Income {
    skip: [u8; 5],
    sender_guid: u64,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (Income { sender_guid, .. }, json) = Income::from_binary(input.data.as_ref().unwrap())?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_server_opcode_name(input.opcode.unwrap()),
            Some(json),
        ));

        let players_map = &mut input.data_storage.lock().unwrap().players_map;
        if players_map.get(&sender_guid).is_none() {
            response.push(HandlerOutput::Data(NameQueryOutcome { guid: sender_guid }.unpack()?));

            return Ok(response);
        }

        Ok(response)
    }
}
