use std::io::Cursor;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use crate::client::Opcode;

mod opcodes;
mod send_data;
pub mod types;

use crate::traits::processor::Processor;
use crate::types::{HandlerInput, ProcessorResult};

pub struct WardenProcessor;

impl Processor for WardenProcessor {
    fn process_input(input: &mut HandlerInput) -> ProcessorResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap());
        let _size = reader.read_u16::<BigEndian>().unwrap();
        let opcode = reader.read_u16::<LittleEndian>().unwrap();
        
        let mut message = String::new();

        let handlers: ProcessorResult = match opcode {
            Opcode::SMSG_WARDEN_DATA => {
                message = String::from("RECEIVE SMSG_WARDEN_DATA");
                vec![Box::new(send_data::Handler)]
            },
            _ => vec![],
        };

        input.message_income.send_server_message(message);

        handlers
    }
}