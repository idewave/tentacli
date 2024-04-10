use std::fmt::{Debug, Formatter};
use hmacsha::HmacSha;
use sha1::Sha1;

const ENCRYPTION_KEY: [u8; 16] = [
    0xC2, 0xB3, 0x72, 0x3C, 0xC6, 0xAE, 0xD9, 0xB5, 0x34, 0x3C, 0x53, 0xEE, 0x2F, 0x43, 0x67, 0xCE
];

const DECRYPTION_KEY: [u8; 16] = [
    0xCC, 0x98, 0xAE, 0x04, 0xE8, 0x97, 0xEA, 0xCA, 0x12, 0xDD, 0xC0, 0x93, 0x42, 0x91, 0x53, 0x57
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
        self.instance.encrypt(data)
    }
}

impl Debug for Decryptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Decryptor")
    }
}

#[derive(Debug)]
pub struct RC4 {
    i: u8,
    j: u8,
    pub state: [u8; 256],
}

impl RC4 {
    pub fn new(key: Vec<u8>) -> Self {
        assert!(!key.is_empty() && key.len() <= 256);

        let mut state = [0u8; 256];
        let mut j: u8 = 0;

        for (i, x) in state.iter_mut().enumerate() {
            *x = i as u8;
        }
        for i in 0..256 {
            j = j.wrapping_add(state[i]).wrapping_add(key[i % key.len()]);
            state.swap(i, j as usize);
        }

        Self {
            i: 0,
            j: 0,
            state,
        }
    }

    // prga
    pub fn next(&mut self) -> u8 {
        self.i = self.i.wrapping_add(1);
        self.j = self.j.wrapping_add(self.state[self.i as usize]);
        self.state.swap(self.i as usize, self.j as usize);
        self.state[(self.state[self.i as usize].wrapping_add(self.state[self.j as usize])) as usize]
    }

    pub fn encrypt(&mut self, data: &[u8]) -> Vec<u8> {
        let mut encrypted = Vec::new();
        for x in data.iter() {
            encrypted.push(x ^ self.next());
        }

        encrypted
    }
}