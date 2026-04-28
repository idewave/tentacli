use binrw::BinRead;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::login::auth::{
    login_proof::Incoming as LoginProofIncoming, select_realm::Incoming as RealmlistIncoming,
    validate_proof::Incoming as ValidateProofIncoming,
};
use crate::plugins::wow::wotlk::opcodes::Opcode;

const HEADER_SIZE: usize = 1;

#[derive(Default)]
pub struct PacketReader;
impl BytesRead for PacketReader {
    fn read(&mut self, buffer: &mut [u8], _: &CtxMap) -> anyhow::Result<Packet> {
        let mut reader = std::io::Cursor::new(&mut *buffer);

        let opcode = byteorder::ReadBytesExt::read_u8(&mut reader)?;
        match opcode {
            Opcode::LOGIN_CHALLENGE => {
                LoginProofIncoming::read(&mut reader)?;
            }
            Opcode::LOGIN_PROOF => {
                ValidateProofIncoming::read(&mut reader)?;
            }
            Opcode::REALM_LIST => {
                RealmlistIncoming::read(&mut reader)?;
            }
            _ => unreachable!("Unknown opcode: {}", opcode),
        }

        let pos = reader.position() as usize;
        let body = buffer[HEADER_SIZE..pos].to_vec();

        let mut packet = Packet::default();
        packet.set_type(PacketType::Incoming);
        packet.set_opcode(PacketOpcode::U8(opcode));
        packet.set_packet_name(Opcode::get_opcode_name(opcode as u32).unwrap_or_default());
        packet.set_packet_size(body.len() + HEADER_SIZE);
        packet.set_body(body);

        Ok(packet)
    }
}

pub struct PacketSerializer;
impl Serializer for PacketSerializer {
    fn serialize(&mut self, packet: &Packet, _: &CtxMap) -> anyhow::Result<Vec<u8>> {
        let opcode = match packet.metadata.opcode {
            PacketOpcode::U8(opcode) => opcode,
            _ => anyhow::bail!("Wrong opcode type !"),
        };

        let mut buffer = Vec::with_capacity(1 + packet.content.body.len());
        buffer.push(opcode);
        buffer.extend_from_slice(&packet.content.body);

        Ok(buffer)
    }
}
