use async_trait::async_trait;

use crate::primary::client::{Opcode, Player};
use crate::primary::types::{HandlerInput, HandlerOutput, HandlerResult, PackedGuid, TerminatedString};
use crate::primary::traits::packet_handler::PacketHandler;

#[derive(WorldPacket, Serialize, Deserialize, Debug)]
#[options(no_opcode)]
struct CheckEmptyIncome {
    packed_guid: PackedGuid,
    unknown: u8,
}

#[derive(WorldPacket, Serialize, Deserialize, Debug)]
#[options(no_opcode)]
struct Income {
    packed_guid: PackedGuid,
    unknown: u8,
    name: TerminatedString,
    realm: TerminatedString,
    race: u8,
    gender: u8,
    class: u8,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (CheckEmptyIncome { unknown, .. }, _) = CheckEmptyIncome::from_binary(
            input.data.as_ref().unwrap(),
        )?;

        if unknown == 1 {
            response.push(HandlerOutput::ErrorMessage("Player not exists".to_string(), None));

            return Ok(response);
        }

        let (Income { packed_guid, name, race, class, gender, .. }, json) = Income::from_binary(
            input.data.as_ref().unwrap(),
        )?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_server_opcode_name(input.opcode.unwrap()),
            Some(json),
        ));

        let PackedGuid(guid) = packed_guid;

        let my_guid = {
            input.session.lock().await.me.as_ref().unwrap().guid
        };

        // modify/insert only another players
        // current player stored inside Session instance
        if my_guid != guid {
            input.data_storage.lock().unwrap().players_map.entry(guid).and_modify(|p| {
                p.name = name.to_string();
                p.race = race;
                p.class = class;
                p.gender = gender;
            }).or_insert_with(|| Player::new(guid, name.to_string(), race, class, gender, 1));
        }

        Ok(response)
    }
}