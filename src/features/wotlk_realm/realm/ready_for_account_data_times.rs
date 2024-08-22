use async_trait::async_trait;
use tentacli_traits::PacketHandler;
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::opcodes::Opcode;

#[derive(WorldPacket, Serialize, Debug, Default)]
struct Outgoing {}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, _: &mut HandlerInput) -> HandlerResult {
        let response = vec![
            HandlerOutput::Data(
                Outgoing::default()
                    .unpack_with_client_opcode(Opcode::CMSG_READY_FOR_ACCOUNT_DATA_TIMES)?
            )
        ];

        Ok(response)
    }
}