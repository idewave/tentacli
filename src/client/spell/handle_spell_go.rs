use async_trait::async_trait;

use crate::ipc::session::types::ActionFlags;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult, PackedGuid};
use crate::traits::packet_handler::PacketHandler;

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
        let Income { caster_guid, .. } = Income::from_binary(input.data.as_ref().unwrap());

        let my_guid = {
            input.session.lock().unwrap().me.as_ref().unwrap().guid
        };

        input.session.lock().unwrap().action_flags.set(
            ActionFlags::IS_CASTING, my_guid == caster_guid
        );

        Ok(HandlerOutput::Void)
    }
}