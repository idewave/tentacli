use std::net::Ipv4Addr;

use crate::{with_opcode};
use crate::types::{PacketOutcome};
use super::opcodes::Opcode;

with_opcode! {
    @login_opcode(Opcode::LOGIN_CHALLENGE)
    #[derive(LoginPacket, Serialize, Deserialize, Debug)]
    struct Outcome {
        unknown: u8,
        packet_size: u16,
        game_name: String,
        #[serde(serialize_with = "crate::serializers::array_serializer::serialize_array")]
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
}

const PACKET_LENGTH_WITHOUT_ACCOUNT: u16 = 30;

// TODO: need to refactor endianness converting for strings
pub fn handler(account: &str) -> PacketOutcome {
    let account_length = account.chars().count() as u8;
    let packet_size = PACKET_LENGTH_WITHOUT_ACCOUNT + account_length as u16;

    Outcome {
        unknown: 0,
        packet_size,
        game_name: String::from("WoW\0"),
        version: [3, 3, 5],
        build: 12340,
        platform: String::from("68x\0"),
        os: String::from("niW\0"),
        locale: String::from("URur"),
        timezone: 0,
        ip: Ipv4Addr::new(127, 0, 0, 1).into(),
        account_length,
        account: account.to_string(),
    }.unpack()
}