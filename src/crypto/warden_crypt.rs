use sha1::{Digest, Sha1};

use crate::crypto::rc4::RC4;

const KEY_SIZE: usize = 16;

pub struct WardenCrypt {
    encryptor: RC4,
    decryptor: RC4,
}

impl WardenCrypt {
    pub fn new(secret: &[u8]) -> Self {
        let (input_key, output_key) = Self::generate(secret);

        Self {
            encryptor: RC4::new(input_key),
            decryptor: RC4::new(output_key),
        }
    }

    pub fn encrypt(&mut self, data: &[u8]) -> Vec<u8> {
        self.encryptor.encrypt(data)
    }

    pub fn decrypt(&mut self, data: &[u8]) -> Vec<u8> {
        self.decryptor.encrypt(data)
    }

    fn generate(secret: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let half_size = secret.len() / 2;

        let o1 = Sha1::new().chain(&secret[..half_size]).finalize().to_vec();
        let o2 = Sha1::new().chain(&secret[half_size..]).finalize().to_vec();

        let o0 = Self::fill_up(&vec![0u8; half_size], &o1, &o2);

        let input_key = o0[..KEY_SIZE].to_vec();
        let output_key = [
            o0[KEY_SIZE..].to_vec(),
            Self::fill_up(&o0, &o1, &o2)
        ].concat()[..KEY_SIZE].to_vec();

        (input_key, output_key)
    }

    fn fill_up(o0: &[u8], o1: &[u8], o2: &[u8]) -> Vec<u8> {
        Sha1::new().chain(o1).chain(o0).chain(o2).finalize().to_vec()
    }
}