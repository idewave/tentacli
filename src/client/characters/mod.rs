use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

mod get_characters_list;
mod player_login;
pub mod types;

use crate::client::opcodes::Opcode;
use crate::types::traits::{Processor};
use crate::types::{HandlerInput, ProcessorResult};

pub struct CharactersProcessor;

impl Processor for CharactersProcessor {
    fn process_input(input: &mut HandlerInput) -> ProcessorResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap()[2..].to_vec());
        let opcode = reader.read_u16::<LittleEndian>().unwrap();

        let mut message = String::new();

        let handlers: ProcessorResult = match opcode {
            Opcode::SMSG_CHAR_ENUM => {
                message = String::from("SMSG_CHAR_ENUM");
                vec![
                    Box::new(get_characters_list::Handler),
                    Box::new(player_login::Handler),
                ]
            },
            _ => vec![],
        };

        input.message_income.send_server_message(message);

        handlers
    }
}