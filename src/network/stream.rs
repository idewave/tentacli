use std::io::{Cursor, Error};
use std::sync::{Arc, Mutex as SyncMutex};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

use crate::client::Opcode;
use crate::crypto::decryptor::{Decryptor, INCOMING_HEADER_LENGTH, INCOMING_OPCODE_LENGTH};
use crate::crypto::encryptor::Encryptor;
use crate::crypto::warden_crypt::WardenCrypt;

pub struct Reader {
    _stream: BufReader<OwnedReadHalf>,
    _decryptor: Option<Decryptor>,
    _warden_crypt: Arc<SyncMutex<Option<WardenCrypt>>>,
    _need_sync: bool,
}

impl Reader {
    pub fn new(reader: OwnedReadHalf) -> Self {
        Self {
            _stream: BufReader::new(reader),
            _decryptor: None,
            _warden_crypt: Arc::new(SyncMutex::new(None)),
            _need_sync: false,
        }
    }

    pub fn init(&mut self, session_key: &[u8], warden_crypt: Arc<SyncMutex<Option<WardenCrypt>>>) {
        self._decryptor = Some(Decryptor::new(session_key));
        self._warden_crypt = warden_crypt;
        self._need_sync = true;
    }

    pub async fn read(&mut self) -> Result<Vec<u8>, Error> {
        if let Some(decryptor) = self._decryptor.as_mut() {
            if self._need_sync {
                self._need_sync = false;
            } else {
                let mut header = [0u8; INCOMING_HEADER_LENGTH];
                self._stream.read_exact(&mut header).await?;

                let mut header_reader = Cursor::new(decryptor.decrypt(&header));
                let size = ReadBytesExt::read_u16::<BigEndian>(&mut header_reader)?;
                let opcode = ReadBytesExt::read_u16::<LittleEndian>(&mut header_reader)?;

                let mut body = vec![0u8; size as usize - INCOMING_OPCODE_LENGTH];
                self._stream.read_exact(&mut body).await?;

                if opcode == Opcode::SMSG_WARDEN_DATA {
                    body = self._warden_crypt.lock().unwrap().as_mut().unwrap().decrypt(&body);
                }

                let mut packet: Vec<u8> = Vec::new();
                packet.append(&mut size.to_be_bytes().to_vec());
                packet.append(&mut opcode.to_le_bytes().to_vec());
                packet.append(&mut body);

                return Ok(packet);
            }
        }

        let mut buffer = [0u8; 65536];
        match self._stream.read(&mut buffer).await {
            Ok(bytes_count) => {
                Ok(buffer[..bytes_count].to_vec())
            },
            Err(err) => Err(err),
        }
    }
}

pub struct Writer {
    _stream: OwnedWriteHalf,
    _encryptor: Option<Encryptor>,
    _need_sync: bool,
}

impl Writer {
    pub fn new(writer: OwnedWriteHalf) -> Self {
        Self {
            _stream: writer,
            _encryptor: None,
            _need_sync: false,
        }
    }

    pub fn init(&mut self, session_key: &[u8]) {
        self._encryptor = Some(Encryptor::new(session_key));
        self._need_sync = true;
    }

    pub async fn write(&mut self, packet: &[u8]) -> Result<usize, Error> {
        let packet = match self._encryptor.as_mut() {
            Some(encryptor) => {
                if self._need_sync {
                    self._need_sync = false;
                    packet.to_vec()
                } else {
                    encryptor.encrypt(packet)
                }
            },
            _ => packet.to_vec(),
        };

        return match self._stream.write(&packet).await {
            Ok(bytes_amount) => {
                let _ = &self._stream.flush().await.unwrap();
                Ok(bytes_amount)
            },
            Err(err) => Err(err),
        }
    }
}