use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use async_trait::async_trait;

use crate::client::Opcode;
use crate::network::packet::OutcomePacket;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::types::traits::PacketHandler;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let players_map = &mut input.data_storage.lock().unwrap().players_map;
        let mut reader = Cursor::new(input.data.as_ref().unwrap()[9..].to_vec());

        let sender_guid = reader.read_u64::<LittleEndian>()?;

        if players_map.get(&sender_guid).is_none() {
            let mut body = Vec::new();
            body.write_u64::<LittleEndian>(sender_guid)?;

            return Ok(
                HandlerOutput::Data(
                    OutcomePacket::from(Opcode::CMSG_NAME_QUERY, Some(body))
                )
            );
        }

        Ok(HandlerOutput::Void)
    }
}
