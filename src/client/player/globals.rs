use crate::{with_opcode};
use crate::client::opcodes::Opcode;

with_opcode! {
    @world_opcode(Opcode::CMSG_NAME_QUERY)
    #[derive(WorldPacket, Serialize, Deserialize, Debug)]
    pub struct NameQueryOutcome {
        pub guid: u64,
    }
}