use async_trait::async_trait;

use crate::packet;
use crate::client::Realm;
use crate::traits::packet_handler::PacketHandler;
use crate::types::{
    HandlerInput,
    HandlerOutput,
    HandlerResult,
};
use super::opcodes::Opcode;

packet! {
    @option[login_opcode=Opcode::REALM_LIST]
    struct Income {
        skip: [u8; 6],
        realms: Vec<Realm>,
    }
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let Income { realms, .. } = Income::from_binary(input.data.as_ref().unwrap());

        input.dialog_income.send_realm_dialog(realms);

        Ok(HandlerOutput::Freeze)
    }
}