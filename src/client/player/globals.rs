use crate::packet;
use crate::client::opcodes::Opcode;

packet! {
    @option[world_opcode=Opcode::CMSG_NAME_QUERY]
    pub struct NameQueryOutcome {
        pub guid: u64,
    }
}