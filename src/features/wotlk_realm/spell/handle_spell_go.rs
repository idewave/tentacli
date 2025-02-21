use async_trait::async_trait;
use tentacli_traits::PacketHandler;
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::custom_fields::PackedGuid;
use tentacli_traits::types::opcodes::Opcode;
use tentacli_traits::types::shared::ActionFlags;

#[derive(WorldPacket, Serialize, Debug)]
struct Incoming {
    cast_item_guid: PackedGuid,
    caster_guid: PackedGuid,
}

pub struct Handler;

#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (Incoming { caster_guid, .. }, json) = Incoming::from_binary(&input.data)?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_opcode_name(input.opcode as u32)
                .unwrap_or(format!("Unknown opcode: {}", input.opcode)),
            Some(json),
        ));

        let my_guid: Option<u64> = input.session.lock().await.my_guid;

        if my_guid.is_some() {
            input.session.lock().await.action_flags.set(
                ActionFlags::IS_CASTING, my_guid.unwrap() == caster_guid,
            );
        }

        Ok(response)
    }
}