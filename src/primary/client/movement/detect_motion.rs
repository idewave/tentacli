use async_trait::async_trait;

use crate::primary::client::Opcode;
use crate::primary::client::player::globals::NameQueryOutcome;
use crate::primary::types::{HandlerInput, HandlerOutput, HandlerResult, PackedGuid};
use crate::primary::parsers::movement_parser::types::{MovementInfo};
use crate::primary::traits::packet_handler::PacketHandler;

#[derive(WorldPacket, Serialize, Deserialize, Debug)]
#[options(no_opcode)]
struct Income {
    packed_guid: PackedGuid,
    movement_info: MovementInfo,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (Income { packed_guid, movement_info }, json) = Income::from_binary(&input.data)?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_opcode_name(input.opcode as u32)
                .unwrap_or(format!("Unknown opcode: {}", input.opcode)),
            Some(json),
        ));

        let PackedGuid(guid) = packed_guid;

        {
            input.data_storage.lock().unwrap().players_map.entry(guid).and_modify(|p| {
                p.position = Some(movement_info.position);
            });
        }

        let my_guid = {
            input.session.lock().await.me.as_ref().unwrap().guid
        };

        if my_guid != guid {
            let players_map = &mut input.data_storage.lock().unwrap().players_map;
            let player = players_map.get(&guid);

            if player.is_none() {
                response.push(HandlerOutput::Data(NameQueryOutcome { guid }.unpack()?));

                return Ok(response);
            }
        }

        Ok(response)
    }
}