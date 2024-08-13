use async_trait::async_trait;
use tentacli_traits::PacketHandler;
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};

use crate::primary::client::Opcode;
use crate::primary::client::player::globals::NameQueryOutgoing;

#[derive(WorldPacket, Serialize, Debug)]
struct Incoming {
    skip: [u8; 5],
    sender_guid: u64,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (Incoming { sender_guid, .. }, json) = Incoming::from_binary(&input.data)?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_opcode_name(input.opcode as u32)
                .unwrap_or(format!("Unknown opcode: {}", input.opcode)),
            Some(json),
        ));

        let players_map = &mut input.data_storage.lock().unwrap().players_map;
        if players_map.get(&sender_guid).is_none() {
            response.push(HandlerOutput::Data(
                NameQueryOutgoing { guid: sender_guid }
                    .unpack_with_client_opcode(Opcode::CMSG_NAME_QUERY)?
            ));

            return Ok(response);
        }

        Ok(response)
    }
}
