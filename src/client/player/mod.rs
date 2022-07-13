use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

mod handle_name_query_response;
mod handle_update_data;
mod party_invite;
pub mod types;

use crate::client::opcodes::Opcode;
use crate::traits::Processor;
use crate::types::{HandlerFunction, HandlerInput, ProcessorResult};

pub struct PlayerProcessor;

impl Processor for PlayerProcessor {
    fn process_input(input: HandlerInput) -> ProcessorResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap()[2..].to_vec());
        let opcode = reader.read_u16::<LittleEndian>().unwrap();

        let handlers: Vec<HandlerFunction> = match opcode {
            Opcode::SMSG_COMPRESSED_UPDATE_OBJECT => {
                println!("RECEIVE SMSG_COMPRESSED_UPDATE_OBJECT");
                vec![
                    Box::new(handle_update_data::handler),
                ]
            },
            Opcode::SMSG_UPDATE_OBJECT => {
                println!("RECEIVE SMSG_UPDATE_OBJECT");
                vec![Box::new(handle_update_data::handler)]
            },
            Opcode::SMSG_GROUP_INVITE => {
                println!("RECEIVE SMSG_GROUP_INVITE");
                vec![Box::new(party_invite::handler)]
            },
            Opcode::SMSG_NAME_QUERY_RESPONSE => {
                println!("RECEIVE SMSG_NAME_QUERY_RESPONSE");
                vec![Box::new(handle_name_query_response::handler)]
            },
            Opcode::SMSG_SET_PCT_SPELL_MODIFIER => {
                println!("RECEIVE SMSG_SET_PCT_SPELL_MODIFIER");
                vec![]
            },
            Opcode::SMSG_TALENT_UPDATE => {
                println!("RECEIVE SMSG_TALENT_UPDATE");
                vec![]
            },
            Opcode::SMSG_INITIAL_SPELLS => {
                println!("RECEIVE SMSG_INITIAL_SPELLS");
                vec![]
            },
            Opcode::MSG_SET_DUNGEON_DIFFICULTY => {
                println!("RECEIVE MSG_SET_DUNGEON_DIFFICULTY");
                vec![]
            },
            _ => vec![],
        };

        Self::collect_responses(handlers, input)
    }
}