use std::io::{Error, ErrorKind};
use byteorder::{LittleEndian, WriteBytesExt};
use async_trait::async_trait;

use crate::client::opcodes::Opcode;
use crate::network::packet::OutcomePacket;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::types::traits::PacketHandler;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let me_exists = input.session.lock().unwrap().me.is_some();
        if !me_exists {
            return Err(Error::new(
                ErrorKind::NotFound,
                "No character selected ! Seems like char list is empty !"
            ));
        }

        let my_guid = input.session.lock().unwrap().me.as_ref().unwrap().guid;

        let mut body = Vec::new();
        body.write_u64::<LittleEndian>(my_guid)?;

        Ok(HandlerOutput::Data(OutcomePacket::from(Opcode::CMSG_PLAYER_LOGIN, Some(body))))
    }
}