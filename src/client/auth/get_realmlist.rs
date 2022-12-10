use async_trait::async_trait;

use crate::{with_opcode};
use crate::client::{Realm, Opcode};
use crate::traits::packet_handler::PacketHandler;
use crate::types::{
    HandlerInput,
    HandlerOutput,
    HandlerResult,
};

with_opcode! {
    @login_opcode(Opcode::REALM_LIST)
    #[derive(LoginPacket, Serialize, Deserialize, Debug, Default)]
    struct Income {
        skip: [u8; 6],
        realms: Vec<Realm>,
    }
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let (Income { realms, .. }, json) = Income::from_binary(input.data.as_ref().unwrap());

        input.message_income.send_server_message(
            Opcode::get_login_opcode_name(input.opcode.unwrap() as u8),
            Some(json),
        );

        input.dialog_income.send_realm_dialog(realms);

        Ok(HandlerOutput::Freeze)
    }
}