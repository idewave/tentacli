use async_trait::async_trait;

use crate::primary::macros::with_opcode;
use crate::primary::client::opcodes::Opcode;
use crate::primary::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::primary::traits::packet_handler::PacketHandler;

with_opcode! {
    @world_opcode(Opcode::CMSG_REALM_SPLIT)
    #[derive(WorldPacket, Serialize, Deserialize, Debug)]
    struct Outcome {
        #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
        unknown: [u8; 4],
    }
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, _: &mut HandlerInput) -> HandlerResult {
        let response = vec![
            HandlerOutput::Data(Outcome {
                unknown: [0xFF, 0xFF, 0xFF, 0xFF]
            }.unpack()?)
        ];

        Ok(response)
    }
}