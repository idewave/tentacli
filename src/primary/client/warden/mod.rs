use std::io::Cursor;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use crate::primary::client::Opcode;

mod opcodes;
mod send_data;
pub mod types;

use crate::primary::traits::processor::Processor;
use crate::primary::types::{HandlerInput, ProcessorResult};

pub struct WardenProcessor;

impl Processor for WardenProcessor {
    fn process_input(input: &mut HandlerInput) -> ProcessorResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap());
        let _size = reader.read_u16::<BigEndian>().unwrap();
        let opcode = reader.read_u16::<LittleEndian>().unwrap();

        let handlers: ProcessorResult = match opcode {
            Opcode::SMSG_WARDEN_DATA => {
                input.opcode = Some(opcode);
                vec![Box::new(send_data::Handler)]
            },
            _ => vec![],
        };

        handlers
    }
}