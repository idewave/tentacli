use async_trait::async_trait;

use crate::primary::client::Opcode;
use crate::primary::shared::session::types::ActionFlags;
use crate::primary::types::{HandlerInput, HandlerOutput, HandlerResult, PackedGuid};
use crate::primary::traits::packet_handler::PacketHandler;

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
            Opcode::get_server_opcode_name(input.opcode),
            Some(json),
        ));

        let my_guid = {
            input.session.lock().await.me.as_ref().unwrap().guid
        };

        input.session.lock().await.action_flags.set(
            ActionFlags::IS_CASTING, my_guid == caster_guid
        );

        Ok(response)
    }
}