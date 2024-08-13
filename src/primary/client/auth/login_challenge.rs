use std::net::Ipv4Addr;
use anyhow::{Result as AnyResult};
use tentacli_traits::types::opcodes::Opcode;
use tentacli_traits::types::OutgoingPacket;

#[derive(LoginPacket, Serialize, Debug)]
struct Outgoing {
    unknown: u8,
    packet_size: u16,
    game_name: String,
    #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
    version: [u8; 3],
    build: u16,
    platform: String,
    os: String,
    locale: String,
    timezone: u32,
    ip: u32,
    account_length: u8,
    account: String,
}

const PACKET_LENGTH_WITHOUT_ACCOUNT: u16 = 30;

// TODO: need to refactor endianness converting for strings
pub fn handler(account: &str) -> AnyResult<OutgoingPacket> {
    let account_length = account.chars().count() as u8;
    let packet_size = PACKET_LENGTH_WITHOUT_ACCOUNT + account_length as u16;

    let (opcode, data, json_details) = Outgoing {
        unknown: 0,
        packet_size,
        game_name: "WoW\0".to_string(),
        version: [3, 3, 5],
        build: 12340,
        platform: "68x\0".to_string(),
        os: "niW\0".to_string(),
        locale: String::from("URur"),
        timezone: 0,
        ip: Ipv4Addr::new(127, 0, 0, 1).into(),
        account_length,
        account: account.to_string(),
    }.unpack_with_opcode(Opcode::LOGIN_CHALLENGE)?;

    Ok(OutgoingPacket {
        opcode,
        data,
        json_details,
    })
}