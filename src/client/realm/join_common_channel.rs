use std::io::Write;
use byteorder::{LittleEndian, WriteBytesExt};
use async_trait::async_trait;

use crate::client::Opcode;
use crate::network::packet::OutcomePacket;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::types::traits::PacketHandler;

const CHANNEL_ID: u32 = 1;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let channel_name = input.session
            .lock()
            .unwrap()
            .get_config()
            .unwrap()
            .channels.common.to_string();

        input.message_income.send_debug_message(format!("JOINING '{}' channel", &channel_name));

        let mut body = Vec::new();
        body.write_u32::<LittleEndian>(CHANNEL_ID)?;
        body.write_u8(0)?;
        body.write_u8(0)?;
        body.write_all(channel_name.as_bytes())?;
        body.write_u8(0)?;

        Ok(HandlerOutput::Data(
            OutcomePacket::from(Opcode::CMSG_JOIN_CHANNEL, Some(body))
        ))
    }
}