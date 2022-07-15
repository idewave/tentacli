use rand::{thread_rng, RngCore};
use std::{fmt::Write, num::ParseIntError};
use std::cell::RefCell;
use std::io::{Cursor, Read};
use byteorder::ReadBytesExt;
use flate2::read::ZlibDecoder;

pub fn random_range(size: usize) -> Vec<u8> {
    let mut range = vec![0u8; size];
    thread_rng().fill_bytes(&mut range[..]);

    range
}

#[allow(dead_code)]
pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

#[allow(dead_code)]
pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s.to_uppercase()
}

pub fn pack_guid(mut guid: u64) -> Vec<u8> {
    let mut pack_guid = vec![0u8; 9];
    let mut size = 1;
    let mut index = 0;

    while guid != 0 {
        if guid & 0xFF > 0 {
            pack_guid[0] |= 1 << index;
            pack_guid[size] = guid as u8;
            size += 1;
        }

        index += 1;
        guid >>= 8;
    }

    pack_guid[..size].to_vec()
}

pub fn read_packed_guid(reader: RefCell<Cursor<Vec<u8>>>) -> (u64, u64) {
    let mut reader = reader.borrow_mut();

    let mask = reader.read_u8().unwrap_or(0);

    if mask == 0 {
        return (0, reader.position());
    }

    let mut guid: u64 = 0;
    let mut i = 0;

    while i < 8 {
        if (mask & (1 << i)) != 0 {
            guid |= (reader.read_u8().unwrap() as u64) << (i * 8);
        }

        i += 1;
    }

    (guid, reader.position())
}

pub fn decompress(data: &[u8]) -> Vec<u8> {
    let mut buffer = Vec::new();

    let mut decoder = ZlibDecoder::new(data);
    decoder.read_to_end(&mut buffer).unwrap();

    buffer
}