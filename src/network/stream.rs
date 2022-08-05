use std::io::{Cursor, Error, ErrorKind};
use std::sync::{Arc, Mutex as SyncMutex};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use tokio::io::{AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use crate::client::Opcode;

use crate::crypto::decryptor::{Decryptor, INCOMING_HEADER_LENGTH, INCOMING_OPCODE_LENGTH};
use crate::crypto::encryptor::Encryptor;
use crate::crypto::warden_crypt::WardenCrypt;

pub struct Reader {
    stream: OwnedReadHalf,
    decryptor: Option<Decryptor>,
    warden_crypt: Arc<SyncMutex<Option<WardenCrypt>>>
}

impl Reader {
    pub fn new(reader: OwnedReadHalf) -> Self {
        Self {
            stream: reader,
            decryptor: None,
            warden_crypt: Arc::new(SyncMutex::new(None))
        }
    }

    pub fn init(&mut self, session_key: &[u8], warden_crypt: Arc<SyncMutex<Option<WardenCrypt>>>) {
        self.decryptor = Some(Decryptor::new(session_key));
        self.warden_crypt = warden_crypt;
    }

    pub fn read(&mut self) -> Result<Vec<Vec<u8>>, Error> {
        let mut buffer = [0u8; 65536];

        match self.stream.try_read(&mut buffer) {
            Ok(bytes_count) => {
                let result = match self.decryptor.as_mut() {
                    Some(decryptor) => {
                        let mut warden_crypt = self.warden_crypt.lock().unwrap();

                        Self::parse_packets(
                            buffer[..bytes_count].to_vec(),
                            decryptor,
                            warden_crypt.as_mut().unwrap(),
                        )
                    },
                    _ => {
                        vec![buffer[..bytes_count].to_vec()]
                    },
                };

                Ok(result)
            },
            Err(err) => {
                let critical_errors: Vec<ErrorKind> = vec![
                    ErrorKind::ConnectionReset,
                    ErrorKind::ConnectionAborted,
                ];

                if critical_errors.contains(&err.kind()) {
                    panic!("[CRITICAL ERROR] on read: {:?}", err.to_string());
                }

                Err(Error::new(ErrorKind::NotFound, "No data read"))
            },
        }
    }

    fn parse_packets(
        raw_data: Vec<u8>,
        decryptor: &mut Decryptor,
        warden_crypt: &mut WardenCrypt
    ) -> Vec<Vec<u8>> {
        let mut reader = Cursor::new(&raw_data);

        let mut packets = Vec::new();
        while reader.position() < (raw_data.len() as u64) {
            let mut header = [0u8; INCOMING_HEADER_LENGTH as usize];
            std::io::Read::read_exact(&mut reader, &mut header).unwrap();

            let mut header_reader = Cursor::new(decryptor.decrypt(&header));
            let size = ReadBytesExt::read_u16::<BigEndian>(&mut header_reader).unwrap();
            let opcode = ReadBytesExt::read_u16::<LittleEndian>(&mut header_reader).unwrap();

            let mut body = vec![0u8; (size - INCOMING_OPCODE_LENGTH) as usize];
            // TODO: need to refactor this function to return Result and forward error to UI
            std::io::Read::read_exact(&mut reader, &mut body) .unwrap_or_else(
                |_| panic!("Cannot read raw data for opcode {} and size {}", opcode, size)
            );

            if opcode == Opcode::SMSG_WARDEN_DATA {
                body = warden_crypt.decrypt(&body);
            }

            // match reader.read_exact(&mut body) {
            //     Ok(_) => {},
            //     Err(_) => {
            //         break;
            //     }
            // }
            // if opcode == 742 {
            //     body = warden_crypt.decrypt(&body);
            // }

            let mut packet: Vec<u8> = Vec::new();
            packet.append(&mut size.to_be_bytes().to_vec());
            packet.append(&mut opcode.to_le_bytes().to_vec());
            packet.append(&mut body);

            packets.push(packet);
        }

        packets
    }
}

pub struct Writer {
    stream: OwnedWriteHalf,
    encryptor: Option<Encryptor>,
}

impl Writer {
    pub fn new(writer: OwnedWriteHalf) -> Self {
        Self {
            stream: writer,
            encryptor: None,
        }
    }

    pub fn init(&mut self, session_key: &[u8]) {
        self.encryptor = Some(Encryptor::new(session_key));
    }

    pub async fn write(&mut self, packet: &[u8]) -> Result<usize, Error> {
        let packet = match self.encryptor.as_mut() {
            Some(encryptor) => encryptor.encrypt(packet),
            _ => packet.to_vec(),
        };

        return match self.stream.write(&packet).await {
            Ok(bytes_amount) => {
                let _ = &self.stream.flush().await.unwrap();
                Ok(bytes_amount)
            },
            Err(err) => Err(err),
        }
    }
}