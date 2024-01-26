use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

pub mod globals;
mod handle_name_query_response;
mod handle_update_data;
pub mod types;

use crate::primary::client::opcodes::Opcode;
use crate::primary::traits::processor::Processor;
use crate::primary::types::{HandlerInput, ProcessorResult};

pub struct PlayerProcessor;

impl Processor for PlayerProcessor {
    fn process_input(input: &mut HandlerInput) -> ProcessorResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap()[2..].to_vec());
        let opcode = reader.read_u16::<LittleEndian>().unwrap();

        let handlers: ProcessorResult = match opcode {
            Opcode::SMSG_COMPRESSED_UPDATE_OBJECT => {
                input.opcode = Some(opcode);
                vec![
                    Box::new(handle_update_data::Handler),
                ]
            },
            Opcode::SMSG_UPDATE_OBJECT => {
                input.opcode = Some(opcode);
                vec![
                    Box::new(handle_update_data::Handler),
                ]
            },
            Opcode::SMSG_GROUP_INVITE => {
                input.opcode = Some(opcode);
                vec![
                    // Box::new(party_invite::Handler),
                ]
            },
            Opcode::SMSG_NAME_QUERY_RESPONSE => {
                input.opcode = Some(opcode);
                vec![
                    Box::new(handle_name_query_response::Handler),
                ]
            },
            Opcode::SMSG_SET_PCT_SPELL_MODIFIER => {
                input.opcode = Some(opcode);
                vec![]
            },
            Opcode::SMSG_TALENT_UPDATE => {
                input.opcode = Some(opcode);
                vec![]
            },
            Opcode::MSG_SET_DUNGEON_DIFFICULTY => {
                input.opcode = Some(opcode);
                vec![]
            },
            Opcode::SMSG_QUESTGIVER_STATUS_MULTIPLE => {
                input.opcode = Some(opcode);
                vec![]
            },
            Opcode::SMSG_ACHIEVEMENT_EARNED => {
                input.opcode = Some(opcode);
                vec![]
            },
            _ => vec![],
        };

        handlers
    }
}