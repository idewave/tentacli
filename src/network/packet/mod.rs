use std::io::{Read};
use flate2::read::DeflateDecoder;
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};

mod parsers;
pub mod types;

use parsers::update_packet_parser::UpdatePacketParser;

use crate::crypto::encryptor::OUTCOMING_OPCODE_LENGTH;
use crate::network::packet::parsers::types::ParsedBlock;

pub struct OutcomePacket {}

impl OutcomePacket {
    pub fn new(opcode: u32, body: Option<Vec<u8>>) -> (u32, Vec<u8>, Vec<u8>) {
        let body = body.unwrap_or(vec![]);

        let mut header = Vec::new();
        header.write_u16::<BigEndian>((body.len() as u16) + OUTCOMING_OPCODE_LENGTH).unwrap();
        header.write_u32::<LittleEndian>(opcode).unwrap();

        // let mut packet = Vec::new();
        // packet.write(&header).unwrap();
        // packet.write(&body).unwrap();

        (opcode, header, body)
    }
}

pub struct ParsedUpdatePacket {
    pub parsed_blocks: Vec<ParsedBlock>,
}

impl ParsedUpdatePacket {
    pub fn new(data: &[u8]) -> Self {
        Self {
            parsed_blocks: UpdatePacketParser::parse(data.to_vec()),
        }
    }

    pub fn from_compressed(data: &[u8]) -> Self {
        let mut buffer = Vec::new();

        // omit 4 bytes uncompressed size + 2 bytes used by zlib
        let omit_bytes_amount: usize = 6;
        let mut decoder = DeflateDecoder::new(&data[omit_bytes_amount..]);
        decoder.read_to_end(&mut buffer).unwrap();

        Self {
            parsed_blocks: UpdatePacketParser::parse(buffer.to_vec()),
        }
    }
}