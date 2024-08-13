// Opcode::CMSG_NAME_QUERY
#[derive(WorldPacket, Serialize, Debug)]
pub struct NameQueryOutgoing {
    pub guid: u64,
}

// Opcode::CMSG_CHAR_ENUM
#[derive(WorldPacket, Serialize, Debug, Default)]
pub struct CharacterEnumOutgoing {}