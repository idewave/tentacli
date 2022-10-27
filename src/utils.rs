use rand::{thread_rng, RngCore};
use std::{fmt::Write, num::ParseIntError};
use std::io::{BufRead, Read};
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

pub fn read_packed_guid<R: BufRead>(reader: &mut R) -> u64 {
    let mask = reader.read_u8().unwrap_or(0);

    if mask == 0 {
        return 0;
    }

    let mut guid: u64 = 0;
    let mut i = 0;

    while i < 8 {
        if (mask & (1 << i)) != 0 {
            guid |= (reader.read_u8().unwrap() as u64) << (i * 8);
        }

        i += 1;
    }

    guid
}

pub fn decompress(data: &[u8]) -> Vec<u8> {
    let mut buffer = Vec::new();

    let mut decoder = ZlibDecoder::new(data);
    decoder.read_to_end(&mut buffer).unwrap();

    buffer
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor, Write};
    use flate2::Compression;
    use flate2::write::ZlibEncoder;

    use crate::utils::{
        decode_hex, decompress, encode_hex,
        pack_guid, random_range, read_packed_guid,
    };

    #[test]
    fn test_random_range() {
        const RANGE_SIZE: usize = 10;

        let range = random_range(RANGE_SIZE);
        assert_eq!(RANGE_SIZE, range.len());
    }

    #[test]
    fn test_decompress() {
        let origin = vec![1, 2, 3, 4, 5, 6, 7, 8];

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(&origin).unwrap();

        assert_eq!(origin, decompress(&encoder.finish().unwrap()));
    }

    // #[test]
    // fn test_packed_guid() {
    //     const ORIGIN_GUID: u64 = 1;
    //
    //     let packed_guid = pack_guid(ORIGIN_GUID);
    //     let mut reader = BufReader::new(packed_guid);
    //
    //     let unpacked_guid = read_packed_guid(&mut reader);
    //
    //     assert_eq!(ORIGIN_GUID, unpacked_guid);
    // }

    #[test]
    fn test_encode_decode() {
        const ORIGIN: [u8; 5] = [255, 12, 3, 45, 5];
        let encoded = encode_hex(&ORIGIN);
        assert_eq!("FF0C032D05", encoded);

        assert_eq!(ORIGIN.to_vec(), decode_hex(&encoded).unwrap());
    }
}