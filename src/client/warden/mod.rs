use std::io::Cursor;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

mod opcodes;
mod send_data;
pub mod types;

use crate::client::opcodes::Opcode;
use crate::types::traits::Processor;
use crate::types::{HandlerFunction, HandlerInput, ProcessorResult};

pub struct WardenProcessor;

impl Processor for WardenProcessor {
    fn process_input(input: &mut HandlerInput) -> ProcessorResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap());
        let _size = reader.read_u16::<BigEndian>().unwrap();
        let opcode = reader.read_u16::<LittleEndian>().unwrap();
        
        let mut message = String::new();

        let handlers: Vec<HandlerFunction> = match opcode {
            Opcode::SMSG_WARDEN_DATA => {
                message = String::from("RECEIVE SMSG_WARDEN_DATA");
                vec![Box::new(send_data::handler)]
            },
            _ => vec![],
        };

        input.message_income.send_server_message(message);

        handlers
    }
}