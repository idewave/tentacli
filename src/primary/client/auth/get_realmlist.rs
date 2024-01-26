use async_trait::async_trait;

use crate::{with_opcode};
use crate::primary::client::{Realm, Opcode};
use crate::primary::traits::packet_handler::PacketHandler;
use crate::primary::types::{
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
        let mut response = Vec::new();

        let (Income { realms, .. }, json) = Income::from_binary(input.data.as_ref().unwrap())?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_server_opcode_name(input.opcode.unwrap()),
            Some(json),
        ));

        response.push(HandlerOutput::TransferRealmsList(realms));
        response.push(HandlerOutput::Freeze);

        Ok(response)
    }
}