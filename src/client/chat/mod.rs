use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

pub mod globals;
mod handle_emote;
mod handle_order;
mod log_chat_message;
mod query_unknown_player;
pub mod types;

use crate::client::opcodes::Opcode;
use crate::traits::processor::Processor;
use crate::types::{HandlerInput, ProcessorResult};

pub struct ChatProcessor;

impl Processor for ChatProcessor {
    fn process_input(input: &mut HandlerInput) -> ProcessorResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap()[2..].to_vec());
        let opcode = reader.read_u16::<LittleEndian>().unwrap();

        let handlers: ProcessorResult = match opcode {
            Opcode::SMSG_MESSAGECHAT => {
                input.opcode = Some(opcode);
                vec![
                    Box::new(query_unknown_player::Handler),
                    Box::new(handle_order::Handler),
                    Box::new(log_chat_message::Handler),
                ]
            },
            Opcode::SMSG_TEXT_EMOTE => {
                input.opcode = Some(opcode);
                vec![
                    Box::new(handle_emote::Handler),
                ]
            },
            _ => vec![]
        };

        handlers
    }
}