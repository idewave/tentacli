use std::net::Ipv4Addr;
use anyhow::{Result as AnyResult};

use crate::primary::client::Opcode;
use crate::primary::macros::with_opcode;
use crate::primary::types::{OutcomePacket, TerminatedString};

with_opcode! {
    @login_opcode(Opcode::LOGIN_CHALLENGE)
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
}

const PACKET_LENGTH_WITHOUT_ACCOUNT: u16 = 30;

// TODO: need to refactor endianness converting for strings
pub fn handler(account: &str) -> AnyResult<OutcomePacket> {
    let account_length = account.chars().count() as u8;
    let packet_size = PACKET_LENGTH_WITHOUT_ACCOUNT + account_length as u16;

    Outcome {
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
    }.unpack()
}