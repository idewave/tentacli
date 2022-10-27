use std::net::Ipv4Addr;

use crate::packet;
use crate::types::{PacketOutcome};
use super::opcodes::Opcode;
use crate::types::{TerminatedString};

packet! {
    @option[login_opcode=Opcode::LOGIN_CHALLENGE]
    struct Outcome {
        unknown: u8,
        packet_size: u16,
        game_name: TerminatedString,
        version: [u8; 3],
        build: u16,
        platform: TerminatedString,
        os: TerminatedString,
        locale: String,
        timezone: u32,
        ip: crate::types::IpAddr,
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
        game_name: TerminatedString::new("WoW"),
        version: [3, 3, 5],
        build: 12340,
        platform: TerminatedString::new("68x"),
        os: TerminatedString::new("niW"),
        locale: String::from("URur"),
        timezone: 0,
        ip: crate::types::IpAddr(Ipv4Addr::new(127, 0, 0, 1).into()),
        account_length,
        account: account.to_uppercase(),
    }.unpack()
}