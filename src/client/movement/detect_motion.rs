use async_trait::async_trait;

use crate::packet;
use crate::client::player::globals::NameQueryOutcome;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult, PackedGuid};
use crate::parsers::movement_parser::types::{MovementInfo};
use crate::traits::packet_handler::PacketHandler;

packet! {
    struct Income {
        packed_guid: PackedGuid,
        movement_info: MovementInfo,
    }
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let Income { packed_guid, movement_info } = Income::from_binary(
            &input.data.as_ref().unwrap().to_vec()
        );

        let PackedGuid(guid) = packed_guid;

        {
            input.data_storage.lock().unwrap().players_map.entry(guid).and_modify(|p| {
                p.position = Some(movement_info.position);
            });
        }

        let my_guid = {
            input.session.lock().unwrap().me.as_ref().unwrap().guid
        };

        if my_guid != guid {
            let players_map = &mut input.data_storage.lock().unwrap().players_map;
            let player = players_map.get(&guid);

            if player.is_none() {
                return Ok(HandlerOutput::Data(NameQueryOutcome { guid }.unpack()));
            }
        }

        Ok(HandlerOutput::Void)
    }
}