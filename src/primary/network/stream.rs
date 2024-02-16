use std::io::{Cursor, Error};
use std::sync::{Arc, Mutex as SyncMutex};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

use crate::primary::client::Opcode;
use crate::primary::crypto::decryptor::{Decryptor};
use crate::primary::crypto::encryptor::{Encryptor};
use crate::primary::crypto::warden_crypt::WardenCrypt;
use crate::primary::types::{IncomePacket, OutcomePacket};

pub const INCOME_WORLD_OPCODE_LENGTH: usize = 2;
pub const OUTCOME_WORLD_PACKET_HEADER_LENGTH: usize = 6;

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

    pub async fn read(&mut self) -> Result<IncomePacket, Error> {
        let (opcode, body) = if let Some(decryptor) = self._decryptor.as_mut() {
            if self._need_sync {
                self._need_sync = false;

                let mut buffer = [0u8; 65536];
                let bytes_count = self._stream.read(&mut buffer).await?;

                let mut header_reader = Cursor::new(&buffer[2..4]);
                let opcode = ReadBytesExt::read_u16::<LittleEndian>(&mut header_reader)?;

                (opcode, buffer[4..bytes_count].to_vec())
            } else {
                let mut buffer = [0u8; 1];
                self._stream.read_exact(&mut buffer).await?;

                let mut first_byte = buffer[0];
                first_byte = decryptor.decrypt(&[first_byte])[0];

                let is_long_packet = first_byte >= 0x80;

                let buffer_size = if is_long_packet { 4 } else { 3 };
                let mut buffer = vec![0u8; buffer_size];
                self._stream.read_exact(&mut buffer).await?;

                let mut header = decryptor.decrypt(&buffer);
                if is_long_packet {
                    first_byte = first_byte & 0x7Fu8;
                }
                header.insert(0, first_byte);

                let mut header_reader = Cursor::new(&header);
                let size = if is_long_packet {
                    ReadBytesExt::read_u24::<BigEndian>(&mut header_reader)? as usize
                } else {
                    ReadBytesExt::read_u16::<BigEndian>(&mut header_reader)? as usize
                };

                let opcode = ReadBytesExt::read_u16::<LittleEndian>(&mut header_reader).unwrap();

                let mut body = vec![0u8; size - INCOME_WORLD_OPCODE_LENGTH];
                self._stream.read_exact(&mut body).await?;

                if opcode == Opcode::SMSG_WARDEN_DATA {
                    body = self._warden_crypt.lock().unwrap().as_mut().unwrap().decrypt(&body);
                }

                (opcode, body)
            }
        } else {
            let mut buffer = [0u8; 65536];
            let bytes_count = self._stream.read(&mut buffer).await?;
            let body = buffer[1..bytes_count].to_vec();
            let opcode = buffer[0] as u16;

            (opcode, body)
        };

        Ok(IncomePacket { opcode, body })
    }
}

pub struct Writer {
    _stream: OwnedWriteHalf,
    _encryptor: Option<Encryptor>,
    _warden_crypt: Arc<SyncMutex<Option<WardenCrypt>>>,
    _need_sync: bool,
}

impl Writer {
    pub fn new(writer: OwnedWriteHalf) -> Self {
        Self {
            _stream: writer,
            _encryptor: None,
            _warden_crypt: Arc::new(SyncMutex::new(None)),
            _need_sync: false,
        }
    }

    pub fn init(&mut self, session_key: &[u8], warden_crypt: Arc<SyncMutex<Option<WardenCrypt>>>) {
        self._encryptor = Some(Encryptor::new(session_key));
        self._warden_crypt = warden_crypt;
        self._need_sync = true;
    }

    pub async fn write(&mut self, packet: &OutcomePacket) -> Result<usize, Error> {
        let packet_bytes = match self._encryptor.as_mut() {
            Some(encryptor) => {
                if self._need_sync {
                    self._need_sync = false;
                    packet.data.to_vec()
                } else {
                    let header = encryptor.encrypt(
                        &packet.data[..OUTCOME_WORLD_PACKET_HEADER_LENGTH]
                    );

                    let body = if packet.opcode == Opcode::CMSG_WARDEN_DATA {
                        self._warden_crypt.lock().unwrap().as_mut().unwrap()
                            .encrypt(&packet.data[OUTCOME_WORLD_PACKET_HEADER_LENGTH..])
                    } else {
                        packet.data[OUTCOME_WORLD_PACKET_HEADER_LENGTH..].to_vec()
                    };

                    [header.to_vec(), body.to_vec()].concat()
                }
            },
            _ => packet.data.to_vec(),
        };

        match self._stream.write(&packet_bytes).await {
            Ok(bytes_amount) => {
                let _ = &self._stream.flush().await.unwrap();
                Ok(bytes_amount)
            },
            Err(err) => Err(err),
        }
    }
}