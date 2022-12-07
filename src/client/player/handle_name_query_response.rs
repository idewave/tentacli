use async_trait::async_trait;

use crate::client::{Player};
use crate::types::{HandlerInput, HandlerOutput, HandlerResult, PackedGuid};
use crate::traits::packet_handler::PacketHandler;

#[derive(WorldPacket, Serialize, Deserialize, Debug)]
#[options(no_opcode)]
struct Income {
    packed_guid: PackedGuid,
    unknown: u8,
    name: String,
    realm: String,
    race: u8,
    gender: u8,
    class: u8,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let Income { packed_guid, name, race, class, .. } = Income::from_binary(
            input.data.as_ref().unwrap(),
        );
        let PackedGuid(guid) = packed_guid;

        let my_guid = {
            input.session.lock().unwrap().me.as_ref().unwrap().guid
        };

        // modify/insert only another players
        // current player stored inside Session instance
        if my_guid != guid {
            input.data_storage.lock().unwrap().players_map.entry(guid).and_modify(|p| {
                p.name = name.to_string();
                p.race = race;
                p.class = class;
            }).or_insert_with(|| Player::new(guid, name, race, class));
        }

        Ok(HandlerOutput::Void)
    }
}