use std::fmt::{Debug, Formatter};
use hmacsha::HmacSha;
use sha1::Sha1;

use super::rc4::RC4;

const ENCRYPTION_KEY: [u8; 16] = [
    0xC2, 0xB3, 0x72, 0x3C, 0xC6, 0xAE, 0xD9, 0xB5, 0x34, 0x3C, 0x53, 0xEE, 0x2F, 0x43, 0x67, 0xCE
];

pub struct Encryptor {
    instance: RC4,
}

impl Encryptor {
    pub fn new(secret: &[u8]) -> Self {
        let sync = vec![0; 1024];

        let mut encryptor = RC4::new(
            HmacSha::new(&ENCRYPTION_KEY, secret, Sha1::default()).compute_digest().to_vec()
        );

        let _ = &encryptor.encrypt(&sync);

        Self {
            instance: encryptor,
        }
    }

    pub fn encrypt(&mut self, data: &[u8]) -> Vec<u8> {
        self.instance.encrypt(data)
    }
}

impl Debug for Encryptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Encryptor")
    }
}