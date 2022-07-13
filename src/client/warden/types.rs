// use std::fs::File;
use std::io::{Cursor, Read};
use byteorder::{LittleEndian, ReadBytesExt};
// use flate2::write::ZlibDecoder;

use crate::crypto::rc4::RC4;
use crate::utils::{decompress, encode_hex};

pub struct WardenModuleInfo {
    #[allow(dead_code)]
    md5: Vec<u8>,
    decoder: RC4,
    compressed_size: u32,
    binary: Vec<u8>,
    #[allow(dead_code)]
    seed: Option<Vec<u8>>,
}

impl WardenModuleInfo {
    pub fn new(md5: Vec<u8>, decrypt_key: Vec<u8>, compressed_size: u32) -> Self {
        Self {
            md5,
            decoder: RC4::new(decrypt_key),
            compressed_size,
            binary: Vec::new(),
            seed: None,
        }
    }

    pub fn set_seed(&mut self, seed: Vec<u8>) {
        self.seed = Some(seed);
    }

    pub fn add_binary(&mut self, partial: Vec<u8>) {
        self.binary.extend(partial);
    }

    pub fn loaded(&mut self) -> bool {
        self.binary.len() == self.compressed_size as usize
    }

    pub fn assemble(&mut self) {
        let decoded_binary = self.decoder.encrypt(&self.binary);

        let mut reader = Cursor::new(&decoded_binary);
        let module_size = reader.read_u32::<LittleEndian>().unwrap();

        let mut compressed_module = Vec::new();
        reader.read_to_end(&mut compressed_module).unwrap();

        let decompressed_data = decompress(&compressed_module);
        let module_name = format!("{}.mod", encode_hex(&self.md5));
        println!("{:?}, {:?}, {:?}", module_size, decompressed_data, module_name);

        // let mut file = File::create(format!("./{}", module_name)).unwrap();
        // file.write_all(&decompressed_data).unwrap();
    }
}