use std::{num::ParseIntError};
use std::io::{Read, Write};
use anyhow::{anyhow, Result as AnyResult};
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::read::DeflateDecoder;
use flate2::write::ZlibEncoder;
use rand::Rng;
use rand::distributions::{Distribution, Standard};

#[allow(dead_code)]
pub fn generate_random_number<T>() -> T
    where
        Standard: Distribution<T>,
{
    let mut rng = rand::thread_rng();
    rng.gen()
}

#[allow(dead_code)]
pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    let str = s.replace(' ', "");
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

pub fn zlib_decompress(data: &[u8]) -> AnyResult<Vec<u8>> {
    let mut buffer = Vec::new();
    let mut decoder = ZlibDecoder::new(data);
    decoder.read_to_end(&mut buffer)?;

    Ok(buffer)
}

pub fn deflate_decompress(data: &[u8]) -> AnyResult<Vec<u8>> {
    let mut buffer = Vec::new();
    let mut decoder = DeflateDecoder::new(data);
    decoder.read_to_end(&mut buffer)?;

    Ok(buffer)
}

pub fn compress(data: &[u8]) -> AnyResult<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(data)?;

    encoder.finish().map_err(|e| anyhow!("Error on compress: {}", e))
}

pub fn camel_to_upper_snake_case(name: &str) -> String {
    let mut result = String::new();
    let mut previous_was_upper = true;

    for c in name.chars() {
        if c.is_uppercase() && !previous_was_upper {
            result.push('_');
        }
        result.push(c.to_uppercase().next().unwrap());
        previous_was_upper = c.is_uppercase() || c.is_numeric();
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{decode_hex, zlib_decompress, compress, encode_hex};

    #[test]
    fn test_decompress() {
        let origin = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let compressed = compress(&origin).unwrap();

        assert_eq!(origin, zlib_decompress(&compressed).unwrap());
    }

    #[test]
    fn test_encode_decode() {
        const ORIGIN: [u8; 5] = [255, 12, 3, 45, 5];
        let encoded = encode_hex(&ORIGIN);
        assert_eq!("FF 0C 03 2D 05", encoded);

        assert_eq!(ORIGIN.to_vec(), decode_hex(&encoded).unwrap());
    }
}