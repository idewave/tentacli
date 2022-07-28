use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

mod handle_emote;
mod handle_order;
pub mod types;

use crate::client::opcodes::Opcode;
use crate::logger::types::LoggerOutput;
use crate::traits::Processor;
use crate::types::{HandlerFunction, HandlerInput, ProcessorResult};

pub struct ChatProcessor;

impl Processor for ChatProcessor {
    fn process_input(input: HandlerInput) -> ProcessorResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap()[2..].to_vec());
        let opcode = reader.read_u16::<LittleEndian>().unwrap();

        let mut message = String::new();

        let handlers: Vec<HandlerFunction> = match opcode {
            Opcode::SMSG_MESSAGECHAT => {
                message = String::from("SMSG_MESSAGECHAT");
                vec![Box::new(handle_order::handler)]
            },
            Opcode::SMSG_TEXT_EMOTE => {
                message = String::from("SMSG_TEXT_EMOTE");
                vec![Box::new(handle_emote::handler)]
            },
            _ => vec![]
        };

        input.output_sender.send(LoggerOutput::Server(message)).unwrap();

        Self::collect_responses(handlers, input)
    }
}