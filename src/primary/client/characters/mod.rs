use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

use crate::primary::client::opcodes::Opcode;
use crate::primary::client::player::player_login;
use crate::primary::traits::processor::Processor;
use crate::primary::types::{HandlerInput, ProcessorResult};

pub struct CharactersProcessor;

impl Processor for CharactersProcessor {
    fn process_input(input: &mut HandlerInput) -> ProcessorResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap()[2..].to_vec());
        let opcode = reader.read_u16::<LittleEndian>().unwrap();

        let handlers: ProcessorResult = match opcode {
            Opcode::SMSG_CHAR_ENUM => {
                input.opcode = Some(opcode);
                vec![
                    Box::new(get_characters_list::Handler),
                    Box::new(player_login::Handler),
                ]
            },
            _ => vec![],
        };

        handlers
    }
}