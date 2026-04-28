mod emote;
mod log_chat_message;
mod text_emote;
mod types;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::opcodes::Opcode;

#[derive(Default)]
pub struct ChatProcessor;

impl Processor for ChatProcessor {
    fn get_handlers(
        &mut self,
        opcode: &PacketOpcode,
        _: &CtxMap,
    ) -> anyhow::Result<Vec<Box<dyn PacketHandler>>> {
        let opcode: u16 = opcode.try_into()?;

        Ok(match opcode {
            Opcode::SMSG_MESSAGECHAT => vec![Box::new(log_chat_message::Handler)],
            Opcode::SMSG_EMOTE => vec![Box::new(emote::Handler)],
            Opcode::SMSG_TEXT_EMOTE => vec![Box::new(text_emote::Handler)],
            _ => vec![],
        })
    }
}
