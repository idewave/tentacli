use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::login::Secret;
use crate::plugins::wow::wotlk::opcodes::Opcode;
use crate::plugins::wow::wotlk::realm::rc4::{Decryptor, Encryptor};

const OPCODE_SIZE: usize = 2;

#[derive(Default)]
pub struct PacketReader {
    decryptor: Option<Decryptor>,
    is_decrypted: bool,
}

impl BytesRead for PacketReader {
    fn read(&mut self, buffer: &mut [u8], context: &CtxMap) -> anyhow::Result<Packet> {
        if buffer.len() < 4 {
            anyhow::bail!("Incomplete header... continue reading.");
        }

        let mut header = buffer[..4].to_vec();
        let mut is_long_packet = false;

        if !self.is_decrypted
            && let Some(decryptor) = self.decryptor.as_mut()
        {
            decryptor.decrypt(&mut header[..1]);

            is_long_packet = (header[0] & 0x80) != 0;

            if is_long_packet {
                if buffer.len() < 5 {
                    anyhow::bail!("Incomplete header (long)... continue reading.");
                }

                let extra_byte = buffer[4];
                header.push(extra_byte);
            }

            decryptor.decrypt(&mut header[1..]);

            buffer[..header.len()].copy_from_slice(&header);
            self.is_decrypted = true;
        }

        let mut header_reader = Cursor::new(&header);

        let size = if is_long_packet {
            header_reader.read_u24::<BigEndian>()? as usize
        } else {
            header_reader.read_u16::<BigEndian>()? as usize
        };

        let opcode = ReadBytesExt::read_u16::<LittleEndian>(&mut header_reader)?;
        let size = size - OPCODE_SIZE;

        let mut reader = Cursor::new(&mut buffer[header.len()..]);
        let mut body = vec![0u8; size];
        reader.read_exact(&mut body)?;

        if self.decryptor.is_none()
            && let Some(secret) = context.get::<Secret>()
        {
            self.decryptor = Some(Decryptor::new(&secret.0.to_vec()));
        }

        self.is_decrypted = false;

        let mut packet = Packet::default();
        packet.set_type(PacketType::Incoming);
        packet.set_opcode(PacketOpcode::U16(opcode));
        packet.set_packet_name(Opcode::get_opcode_name(opcode as u32).unwrap_or_default());
        packet.set_packet_size(body.len() + header.len());
        packet.set_body(body);

        Ok(packet)
    }
}

#[derive(Default)]
pub struct PacketSerializer {
    encryptor: Option<Encryptor>,
}

impl Serializer for PacketSerializer {
    fn serialize(&mut self, packet: &Packet, context: &CtxMap) -> anyhow::Result<Vec<u8>> {
        let opcode = match packet.metadata.opcode {
            PacketOpcode::U32(opcode) => opcode,
            _ => anyhow::bail!("Wrong opcode type !"),
        };

        let size = (4 + packet.content.body.len()) as u16;

        let mut buffer = Vec::with_capacity(2 + size as usize);
        buffer.extend_from_slice(&size.to_be_bytes());
        buffer.extend_from_slice(&opcode.to_le_bytes());

        if let Some(encryptor) = self.encryptor.as_mut() {
            encryptor.encrypt(&mut buffer);
        }

        buffer.extend_from_slice(&packet.content.body);

        if self.encryptor.is_none()
            && let Some(secret) = context.get::<Secret>()
        {
            self.encryptor = Some(Encryptor::new(&secret.0.to_vec()));
        }

        Ok(buffer)
    }
}
