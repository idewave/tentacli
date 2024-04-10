use std::net::Ipv4Addr;
use anyhow::{Result as AnyResult};
use tentacli_traits::types::custom_fields::TerminatedString;
use tentacli_traits::types::opcodes::Opcode;
use tentacli_traits::types::OutgoingPacket;

#[derive(LoginPacket, Serialize, Deserialize, Debug)]
struct Outcome {
    unknown: u8,
    packet_size: u16,
    game_name: TerminatedString,
    #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
    version: [u8; 3],
    build: u16,
    platform: TerminatedString,
    os: TerminatedString,
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

    let (opcode, data, json_details) = Outcome {
        unknown: 0,
        packet_size,
        game_name: TerminatedString::from("WoW"),
        version: [3, 3, 5],
        build: 12340,
        platform: TerminatedString::from("68x"),
        os: TerminatedString::from("niW"),
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