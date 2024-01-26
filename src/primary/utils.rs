use std::{num::ParseIntError};
use std::io::{BufRead, Read};
use byteorder::ReadBytesExt;
use flate2::read::ZlibDecoder;

#[allow(dead_code)]
pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    let str = s.replace(" ", "");
    (0..str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&str[i..i + 2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut items: Vec<String> = Vec::new();
    for &b in bytes {
        items.push(format!("{:02x}", b).to_uppercase());
    }
    items.join(" ")
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

#[allow(dead_code)]
pub fn crop(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

#[cfg(test)]
mod tests {
    use std::io::{Write};
    use flate2::Compression;
    use flate2::write::ZlibEncoder;

    use crate::primary::utils::{decode_hex, decompress, encode_hex};

    #[test]
    fn test_decompress() {
        let origin = vec![1, 2, 3, 4, 5, 6, 7, 8];

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(&origin).unwrap();

        assert_eq!(origin, decompress(&encoder.finish().unwrap()));
    }

    #[test]
    fn test_encode_decode() {
        const ORIGIN: [u8; 5] = [255, 12, 3, 45, 5];
        let encoded = encode_hex(&ORIGIN);
        assert_eq!("FF 0C 03 2D 05", encoded);

        assert_eq!(ORIGIN.to_vec(), decode_hex(&encoded).unwrap());
    }
}