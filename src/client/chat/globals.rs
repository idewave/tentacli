use crate::packet;
use crate::client::Opcode;
use crate::types::{TerminatedString};

packet! {
    @option[world_opcode=Opcode::CMSG_JOIN_CHANNEL]
    pub struct JoinChannelOutcome {
        pub channel_id: u32,
        pub unknown: u8,
        pub unknown1: u8,
        pub channel_name: TerminatedString,
    }
}