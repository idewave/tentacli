use std::io::{Write};
use std::net::Ipv4Addr;
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};

use super::opcodes::Opcode;

// TODO: need to refactor endianness converting for strings
pub fn handler(username: &str) -> Vec<u8> {
    let username_length = username.chars().count() as i8;
    let size = 30 + username_length;

    let mut packet: Vec<u8> = Vec::new();
    packet.write_u8(Opcode::LOGIN_CHALLENGE).unwrap();

    packet.write_u8(0).unwrap();
    packet.write_u16::<LittleEndian>(size as u16).unwrap();
    packet.write_all(String::from("WoW\0").as_bytes()).unwrap();
    packet.write_all(&vec![3, 3, 5]).unwrap();
    packet.write_u16::<LittleEndian>(12340).unwrap();
    packet.write_all(String::from("68x\0").as_bytes()).unwrap();
    packet.write_all(String::from("niW\0").as_bytes()).unwrap();
    packet.write_all(String::from("URur\0").as_bytes()).unwrap();
    packet.write_u24::<LittleEndian>(0).unwrap();
    packet.write_u32::<BigEndian>(Ipv4Addr::new(127, 0, 0, 1).into()).unwrap();
    packet.write_i8(username_length).unwrap();
    packet.write_all(username.to_uppercase().as_bytes()).unwrap();

    packet
}