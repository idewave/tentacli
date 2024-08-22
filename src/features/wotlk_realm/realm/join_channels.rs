use async_trait::async_trait;
use tentacli_traits::PacketHandler;
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::opcodes::Opcode;

#[derive(WorldPacket, Serialize, Debug, Default)]
pub struct JoinChannelOutgoing {
    pub channel_id: u32,
    pub unknown: u8,
    pub unknown1: u8,
    pub channel_name: String,
}

const COMMON_CHANNEL_ID: u32 = 1;
const LFG_CHANNEL_ID: u32 = 26;
const TRADE_CHANNEL_ID: u32 = 2;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let channel_labels = &input.session.lock().await.get_config().unwrap().channel_labels.clone();

        response.push(HandlerOutput::Data(JoinChannelOutgoing {
            channel_id: COMMON_CHANNEL_ID,
            channel_name: format!("{}\0", channel_labels.common),
            ..JoinChannelOutgoing::default()
        }.unpack_with_client_opcode(Opcode::CMSG_JOIN_CHANNEL)?));

        response.push(HandlerOutput::Data(JoinChannelOutgoing {
            channel_id: LFG_CHANNEL_ID,
            channel_name: format!("{}\0", channel_labels.lfg),
            ..JoinChannelOutgoing::default()
        }.unpack_with_client_opcode(Opcode::CMSG_JOIN_CHANNEL)?));

        response.push(HandlerOutput::Data(JoinChannelOutgoing {
            channel_id: TRADE_CHANNEL_ID,
            channel_name: format!("{}\0", channel_labels.trade),
            ..JoinChannelOutgoing::default()
        }.unpack_with_client_opcode(Opcode::CMSG_JOIN_CHANNEL)?));

        Ok(response)
    }
}