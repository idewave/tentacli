use async_trait::async_trait;
use tentacli_traits::PacketHandler;
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::opcodes::Opcode;

use crate::features::wotlk_realm::globals::CharacterEnumOutgoing;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, _: &mut HandlerInput) -> HandlerResult {
        let response = vec![
            HandlerOutput::Data(
                CharacterEnumOutgoing::default().unpack_with_client_opcode(Opcode::CMSG_CHAR_ENUM)?
            )
        ];

        Ok(response)
    }
}