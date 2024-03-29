use crate::primary::macros::with_opcode;
use crate::primary::client::opcodes::Opcode;

with_opcode! {
    @world_opcode(Opcode::CMSG_NAME_QUERY)
    #[derive(WorldPacket, Serialize, Deserialize, Debug)]
    pub struct NameQueryOutcome {
        pub guid: u64,
    }
}

with_opcode! {
    @world_opcode(Opcode::CMSG_CHAR_ENUM)
    #[derive(WorldPacket, Serialize, Deserialize, Debug, Default)]
    pub struct CharacterEnumOutcome {}
}