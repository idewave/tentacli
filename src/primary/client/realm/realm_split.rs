use async_trait::async_trait;
use tentacli_traits::PacketHandler;
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::opcodes::Opcode;

#[derive(WorldPacket, Serialize, Debug)]
struct Outcome {
    #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
    unknown: [u8; 4],
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, _: &mut HandlerInput) -> HandlerResult {
        let response = vec![
            HandlerOutput::Data(Outcome {
                unknown: [0xFF, 0xFF, 0xFF, 0xFF]
            }.unpack_with_client_opcode(Opcode::CMSG_REALM_SPLIT)?)
        ];

        Ok(response)
    }
}