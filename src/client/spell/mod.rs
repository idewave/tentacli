use std::io::Cursor;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

mod handle_initial_spells;
mod handle_spell_go;
pub mod types;

use crate::client::opcodes::Opcode;
use crate::types::traits::{Processor};
use crate::types::{HandlerInput, ProcessorResult};

pub struct SpellProcessor;

impl Processor for SpellProcessor {
    fn process_input(input: &mut HandlerInput) -> ProcessorResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap());
        let _size = reader.read_u16::<BigEndian>().unwrap();
        let opcode = reader.read_u16::<LittleEndian>().unwrap();

        let mut message = String::new();

        let handlers: ProcessorResult = match opcode {
            Opcode::SMSG_SPELL_GO => {
                message = String::from("SMSG_SPELL_GO");
                vec![Box::new(handle_spell_go::Handler)]
            },
            Opcode::SMSG_INITIAL_SPELLS => {
                message = String::from("SMSG_INITIAL_SPELLS");
                vec![
                    Box::new(handle_initial_spells::Handler),
                ]
            },
            _ => vec![]
        };

        input.message_income.send_server_message(message);

        handlers
    }
}