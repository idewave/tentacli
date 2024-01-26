use crate::{with_opcode};
use crate::primary::client::Opcode;
use crate::primary::types::{TerminatedString};

with_opcode! {
    @world_opcode(Opcode::CMSG_JOIN_CHANNEL)
    #[derive(WorldPacket, Serialize, Deserialize, Debug, Default)]
    pub struct JoinChannelOutcome {
        pub channel_id: u32,
        pub unknown: u8,
        pub unknown1: u8,
        pub channel_name: TerminatedString,
    }
}