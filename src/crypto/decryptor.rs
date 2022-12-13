use std::fmt::{Debug, Formatter};
use hmacsha::HmacSha;
use sha1::Sha1;

use super::rc4::RC4;

const DECRYPTION_KEY: [u8; 16] = [
    0xCC, 0x98, 0xAE, 0x04, 0xE8, 0x97, 0xEA, 0xCA, 0x12, 0xDD, 0xC0, 0x93, 0x42, 0x91, 0x53, 0x57
];

pub const INCOMING_HEADER_LENGTH: usize = 4;
pub const INCOMING_OPCODE_LENGTH: usize = 2;

pub struct Decryptor {
    instance: RC4,
}

impl Decryptor {
    pub fn new(secret: &[u8]) -> Self {
        let sync = vec![0; 1024];

        let mut decryptor = RC4::new(
            HmacSha::new(&DECRYPTION_KEY, secret, Sha1::default()).compute_digest().to_vec()
        );

        let _ = &decryptor.encrypt(&sync);

        Self {
            instance: decryptor,
        }
    }

    pub fn decrypt(&mut self, data: &[u8]) -> Vec<u8> {
        let header = self.instance.encrypt(&data[..INCOMING_HEADER_LENGTH]);
        [header, data[INCOMING_HEADER_LENGTH..].to_vec()].concat().to_vec()
    }
}

impl Debug for Decryptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Decryptor")
    }
}