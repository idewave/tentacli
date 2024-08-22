use anyhow::{Result as AnyResult};
use std::io::{Cursor, Error};
use std::sync::{Arc, Mutex as SyncMutex};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tentacli_crypto::{Decryptor, Encryptor, WardenCrypt};
use tentacli_traits::types::{IncomingPacket, OutgoingPacket};
use tentacli_traits::types::opcodes::Opcode;
// use tokio_util::io::InspectReader;
cfg_if! {
    if #[cfg(feature = "wotlk_login")] {
        use crate::features::wotlk_login::{
            LoginChallengeResponse, LoginProofResponse, RealmlistResponse
        };
    } else {
        panic!("Login feature must be enabled !");
    }
}

pub const INCOME_WORLD_OPCODE_LENGTH: usize = 2;
pub const OUTCOME_WORLD_PACKET_HEADER_LENGTH: usize = 6;

pub struct Reader {
    // _stream: BufReader<InspectReader<OwnedReadHalf, fn(&[u8])>>,
    _stream: BufReader<OwnedReadHalf>,
    _decryptor: Option<Decryptor>,
    _warden_crypt: Arc<SyncMutex<Option<WardenCrypt>>>,
    _need_sync: bool,
}

impl Reader {
    pub fn new(
        reader: OwnedReadHalf,
        warden_crypt: Arc<SyncMutex<Option<WardenCrypt>>>,
        need_sync: bool,
        decryptor: Option<Decryptor>
    ) -> Self {
        // let inspect_fn: fn(&[u8]) = |bytes| println!("READ: {bytes:?}");
        // let inspect_reader = InspectReader::new(reader, inspect_fn);
        let buf_reader = BufReader::new(reader);

        Self {
            _stream: buf_reader,
            _decryptor: decryptor,
            _warden_crypt: warden_crypt,
            _need_sync: need_sync,
        }
    }

    pub async fn read(&mut self) -> AnyResult<IncomingPacket> {
        let (opcode, body) = if let Some(decryptor) = self._decryptor.as_mut() {
            let mut header = vec![0u8; 4];
            self._stream.read_exact(&mut header[..]).await?;

            if !self._need_sync {
                header = decryptor.decrypt(&header);
            } else {
                self._need_sync = false;
            }

            let first_byte = header[0];
            let is_long_packet = first_byte >= 0x80;

            if is_long_packet {
                let extra_byte = self._stream.read_u8().await?;
                header.push(extra_byte);
            }

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
        } else {
            let opcode = self._stream.read_u8().await?;
            let body = match opcode {
                Opcode::LOGIN_CHALLENGE => {
                    LoginChallengeResponse::from_stream(&mut self._stream)
                        .await
                        .map_err(|e| Error::new(std::io::ErrorKind::Other, e))?
                },
                Opcode::LOGIN_PROOF => {
                    LoginProofResponse::from_stream(&mut self._stream)
                        .await
                        .map_err(|e| Error::new(std::io::ErrorKind::Other, e))?
                },
                Opcode::REALM_LIST => {
                    RealmlistResponse::from_stream(&mut self._stream)
                        .await
                        .map_err(|e| Error::new(std::io::ErrorKind::Other, e))?
                },
                _ => vec![],
            };

            (opcode as u16, body)
        };

        Ok(IncomingPacket { opcode, body })
    }
}

pub struct Writer {
    _stream: OwnedWriteHalf,
    _encryptor: Option<Encryptor>,
    _warden_crypt: Arc<SyncMutex<Option<WardenCrypt>>>,
    _need_sync: bool,
}

impl Writer {
    pub fn new(
        writer: OwnedWriteHalf,
        warden_crypt: Arc<SyncMutex<Option<WardenCrypt>>>,
        need_sync: bool,
        encryptor: Option<Encryptor>
    ) -> Self {
        Self {
            _stream: writer,
            _encryptor: encryptor,
            _warden_crypt: warden_crypt,
            _need_sync: need_sync,
        }
    }

    pub async fn write(&mut self, packet: &OutgoingPacket) -> AnyResult<usize> {
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
            Err(err) => Err(err.into()),
        }
    }
}