use std::io::Cursor;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

pub mod types;

use crate::traits::processor::Processor;
use crate::types::{HandlerInput, ProcessorResult};

pub struct TradeProcessor;

impl Processor for TradeProcessor {
    fn process_input(input: &mut HandlerInput) -> ProcessorResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap());
        let _size = reader.read_u16::<BigEndian>().unwrap();
        let _opcode = reader.read_u16::<LittleEndian>().unwrap();

        // let handlers: Vec<Box<dyn PacketHandler>> = match opcode {
        //     _ => vec![]
        // };

        let handlers: ProcessorResult = vec![];

        handlers
    }
}