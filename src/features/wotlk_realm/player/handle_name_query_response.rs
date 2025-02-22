use async_trait::async_trait;
use tentacli_traits::PacketHandler;
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::custom_fields::PackedGuid;
use tentacli_traits::types::opcodes::Opcode;
use tentacli_traits::types::shared::Object;

#[derive(WorldPacket, Serialize, Debug)]
struct CheckEmpty {
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

        let (CheckEmpty { unknown, .. }, _) = CheckEmpty::from_binary(&input.data)?;

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

        input.data_storage.lock().await.players_map.entry(guid).and_modify(|p| {
            p.name = name.to_string();
        }).or_insert_with(|| {
            Object {
                guid,
                name,
                ..Object::default()
            }
        });

        Ok(response)
    }
}