use std::io::Cursor;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

mod handle_spell_go;
pub mod types;

use crate::client::opcodes::Opcode;
use crate::traits::Processor;
use crate::types::{HandlerFunction, HandlerInput, ProcessorResult};

pub struct SpellProcessor;

impl Processor for SpellProcessor {
    fn process_input(mut input: HandlerInput) -> ProcessorResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap());
        let _size = reader.read_u16::<BigEndian>().unwrap();
        let opcode = reader.read_u16::<LittleEndian>().unwrap();

        let mut message = String::new();

        let handlers: Vec<HandlerFunction> = match opcode {
            Opcode::SMSG_SPELL_START => {
                message = String::from("SMSG_SPELL_START");
                vec![]
            },
            Opcode::SMSG_SPELL_GO => {
                message = String::from("SMSG_SPELL_GO");
                vec![Box::new(handle_spell_go::handler)]
            },
            Opcode::SMSG_SPELL_FAILURE => {
                message = String::from("SMSG_SPELL_FAILURE");
                vec![]
            },
            Opcode::SMSG_SPELL_FAILED_OTHER => {
                message = String::from("SMSG_SPELL_FAILED_OTHER");
                vec![]
            },
            Opcode::SMSG_SPELL_DELAYED => {
                message = String::from("SMSG_SPELL_DELAYED");
                vec![]
            },
            Opcode::SMSG_SPELLHEALLOG => {
                message = String::from("SMSG_SPELLHEALLOG");
                vec![]
            },
            _ => vec![]
        };

        input.message_sender.send_server_message(message);

        Self::collect_responses(handlers, input)
    }
}