use std::io::{Cursor, Error};
use std::sync::{Arc, Mutex as SyncMutex};

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use tentacli_crypto::{Decryptor, Encryptor, WardenCrypt};
use tentacli_traits::types::{IncomingPacket, OutgoingPacket};
use tentacli_traits::types::opcodes::Opcode;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

cfg_if! {
    if #[cfg(feature = "wotlk_login")] {
        use crate::features::wotlk_login::{
            LoginChallengeResponse, LoginProofResponse, RealmlistResponse
        };
    } else {
        panic!("Login feature must be enabled !");
    }
}

pub struct Reader {
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
        decryptor: Option<Decryptor>,
    ) -> Self {
        let buf_reader = BufReader::new(reader);

        Self {
            _stream: buf_reader,
            _decryptor: decryptor,
            _warden_crypt: warden_crypt,
            _need_sync: need_sync,
        }
    }

    pub async fn read(&mut self) -> anyhow::Result<IncomingPacket> {
        let (opcode, header, body) = if let Some(decryptor) = self._decryptor.as_mut() {
            let mut header = vec![0u8; 4];
            self._stream.read_exact(&mut header[..]).await?;

            if !self._need_sync {
                decryptor.decrypt(&mut header)
            } else {
                self._need_sync = false;
            }

            let is_long_packet = header[0] >= 0x80;

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

            // 2 is opcode length for incoming world packet
            let mut body = vec![0u8; size - 2];
            self._stream.read_exact(&mut body).await?;

            if opcode as u32 == Opcode::CMSG_WARDEN_DATA {
                self._warden_crypt.lock().unwrap().as_mut().unwrap().encrypt(&mut body);
            }

            (opcode, header, body)
        } else {
            let opcode = self._stream.read_u8().await?;
            let body = match opcode {
                Opcode::LOGIN_CHALLENGE => {
                    LoginChallengeResponse::from_stream(&mut self._stream)
                        .await
                        .map_err(|e| Error::new(std::io::ErrorKind::Other, e))?
                }
                Opcode::LOGIN_PROOF => {
                    LoginProofResponse::from_stream(&mut self._stream)
                        .await
                        .map_err(|e| Error::new(std::io::ErrorKind::Other, e))?
                }
                Opcode::REALM_LIST => {
                    RealmlistResponse::from_stream(&mut self._stream)
                        .await
                        .map_err(|e| Error::new(std::io::ErrorKind::Other, e))?
                }
                _ => vec![],
            };

            (opcode as u16, vec![opcode], body)
        };

        Ok(IncomingPacket { opcode, header, body })
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
        encryptor: Option<Encryptor>,
    ) -> Self {
        Self {
            _stream: writer,
            _encryptor: encryptor,
            _warden_crypt: warden_crypt,
            _need_sync: need_sync,
        }
    }

    pub async fn write(&mut self, packet: &mut OutgoingPacket) -> anyhow::Result<usize> {
        let OutgoingPacket { opcode, data, .. } = packet;

        if let Some(encryptor) = self._encryptor.as_mut() {
            if self._need_sync {
                self._need_sync = false;
            } else {
                encryptor.encrypt(&mut data[..6]);

                if *opcode == Opcode::CMSG_WARDEN_DATA {
                    self._warden_crypt
                        .lock()
                        .unwrap()
                        .as_mut()
                        .unwrap()
                        .encrypt(&mut data[6..]);
                }
            }
        }

        match self._stream.write(&data).await {
            Ok(bytes_amount) => Ok(bytes_amount),
            Err(err) => Err(err.into()),
        }
    }
}