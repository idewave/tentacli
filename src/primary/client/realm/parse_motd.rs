use async_trait::async_trait;

use crate::primary::client::Opcode;
use crate::primary::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::primary::traits::packet_handler::PacketHandler;

#[derive(WorldPacket, Serialize, Deserialize, Debug)]
#[options(no_opcode)]
struct Income {
    skip: u32,
    message: String,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (Income { .. }, json) = Income::from_binary(&input.data)?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_server_opcode_name(input.opcode),
            Some(json),
        ));

        Ok(response)
    }
}