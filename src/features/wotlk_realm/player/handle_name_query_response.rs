use async_trait::async_trait;
use tentacli_traits::PacketHandler;
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::custom_fields::PackedGuid;
use tentacli_traits::types::opcodes::Opcode;
use tentacli_traits::types::shared::Object;

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

        let (Incoming { packed_guid, name, .. }, json) = Incoming::from_binary(&input.data)?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_opcode_name(input.opcode as u32)
                .unwrap_or(format!("Unknown opcode: {}", input.opcode)),
            Some(json),
        ));

        let PackedGuid(guid) = packed_guid;

        let is_my_guid = {
            let guard = input.session.lock().await;
            guard.my_guid.map_or(false, |my_guid| my_guid == guid)
        };

        if !is_my_guid {
            input.data_storage.lock().await.players_map.entry(guid).and_modify(|p| {
                p.name = name.to_string();
            }).or_insert_with(|| {
                Object {
                    guid,
                    name,
                    ..Object::default()
                }
            });
        }

        Ok(response)
    }
}