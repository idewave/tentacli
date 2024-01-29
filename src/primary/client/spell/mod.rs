use std::io::Cursor;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

mod handle_initial_spells;
mod handle_spell_go;
pub mod types;

use crate::primary::client::opcodes::Opcode;
use crate::primary::traits::processor::Processor;
use crate::primary::types::{HandlerInput, ProcessorResult};

pub struct SpellProcessor;

impl Processor for SpellProcessor {
    fn process_input(input: &mut HandlerInput) -> ProcessorResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap());
        let _size = reader.read_u16::<BigEndian>().unwrap();
        let opcode = reader.read_u16::<LittleEndian>().unwrap();

        let handlers: ProcessorResult = match opcode {
            Opcode::SMSG_SPELL_GO => {
                input.opcode = Some(opcode);
                vec![Box::new(handle_spell_go::Handler)]
            },
            Opcode::SMSG_INITIAL_SPELLS => {
                input.opcode = Some(opcode);
                vec![
                    Box::new(handle_initial_spells::Handler),
                ]
            },
            _ => vec![]
        };

        handlers
    }
}