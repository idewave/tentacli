use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

mod handle_emote;
mod handle_message;
pub mod types;

use crate::client::opcodes::Opcode;
use crate::traits::Processor;
use crate::types::{HandlerFunction, HandlerInput, ProcessorResult};

pub struct ChatProcessor;

impl Processor for ChatProcessor {
    fn process_input(input: HandlerInput) -> ProcessorResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap()[2..].to_vec());
        let opcode = reader.read_u16::<LittleEndian>().unwrap();

        let handlers: Vec<HandlerFunction> = match opcode {
            Opcode::SMSG_MESSAGECHAT => {
                println!("RECEIVE SMSG_MESSAGECHAT");
                vec![Box::new(handle_message::handler)]
            },
            Opcode::SMSG_TEXT_EMOTE => {
                println!("RECEIVE SMSG_TEXT_EMOTE");
                vec![Box::new(handle_emote::handler)]
            },
            _ => vec![]
        };

        Self::collect_responses(handlers, input)
    }
}