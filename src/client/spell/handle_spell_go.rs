use async_trait::async_trait;

use crate::client::Opcode;
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
        let (Income { caster_guid, .. }, json) = Income::from_binary(input.data.as_ref().unwrap());

        input.message_income.send_server_message(
            Opcode::get_server_opcode_name(input.opcode.unwrap()),
            Some(json),
        );

        let my_guid = {
            input.session.lock().unwrap().me.as_ref().unwrap().guid
        };

        input.session.lock().unwrap().action_flags.set(
            ActionFlags::IS_CASTING, my_guid == caster_guid
        );

        Ok(HandlerOutput::Void)
    }
}