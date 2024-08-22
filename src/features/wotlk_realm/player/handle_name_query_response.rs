use async_trait::async_trait;
use tentacli_traits::PacketHandler;
use tentacli_traits::types::custom_fields::{PackedGuid};
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::opcodes::Opcode;
use tentacli_traits::types::player::Player;

#[derive(WorldPacket, Serialize, Debug)]
struct CheckEmptyIncoming {
    packed_guid: PackedGuid,
    unknown: u8,
}

#[derive(WorldPacket, Serialize, Debug)]
struct Incoming {
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
        let mut response = Vec::new();

        let (CheckEmptyIncoming { unknown, .. }, _) = CheckEmptyIncoming::from_binary(&input.data)?;

        if unknown == 1 {
            response.push(HandlerOutput::ErrorMessage("Player not exists".to_string(), None));

            return Ok(response);
        }

        let (Incoming {
            packed_guid,
            name,
            race,
            class,
            gender,
            ..
        }, json) = Incoming::from_binary(&input.data)?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_opcode_name(input.opcode as u32)
                .unwrap_or(format!("Unknown opcode: {}", input.opcode)),
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
            }).or_insert_with(|| Player::new(guid, name.to_string(), race, class, gender));
        }

        Ok(response)
    }
}