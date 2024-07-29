// Opcode::CMSG_JOIN_CHANNEL
#[derive(WorldPacket, Serialize, Debug, Default)]
pub struct JoinChannelOutcome {
    pub channel_id: u32,
    pub unknown: u8,
    pub unknown1: u8,
    pub channel_name: String,
}