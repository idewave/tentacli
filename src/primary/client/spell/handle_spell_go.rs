use async_trait::async_trait;

use crate::primary::client::Opcode;
use crate::primary::shared::session::types::ActionFlags;
use crate::primary::types::{HandlerInput, HandlerOutput, HandlerResult, PackedGuid};
use crate::primary::traits::PacketHandler;

#[derive(WorldPacket, Serialize, Deserialize, Debug)]
#[options(no_opcode)]
struct Income {
    cast_item_guid: PackedGuid,
    caster_guid: PackedGuid,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (Income { caster_guid, .. }, json) = Income::from_binary(&input.data)?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_opcode_name(input.opcode as u32)
                .unwrap_or(format!("Unknown opcode: {}", input.opcode)),
            Some(json),
        ));

        let my_guid: Option<u64> = {
            let guard = input.session.lock().await;
            guard.me.as_ref().map(|player| player.guid)
        };

        if my_guid.is_some() {
            input.session.lock().await.action_flags.set(
                ActionFlags::IS_CASTING, my_guid.unwrap() == caster_guid
            );
        }

        Ok(response)
    }
}