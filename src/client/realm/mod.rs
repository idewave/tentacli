use std::io::{Cursor};
use byteorder::{LittleEndian, ReadBytesExt};

mod auth_challenge;
mod join_common_channel;
mod join_lfg_channel;
mod join_trade_channel;
mod parse_motd;
mod ping;
mod ready_for_account_data_times;
mod realm_split;
mod request_characters;
mod set_in_world;
pub mod types;

use crate::client::opcodes::Opcode;
use crate::traits::processor::Processor;
use crate::types::{HandlerInput, ProcessorResult};

pub struct RealmProcessor;

impl Processor for RealmProcessor {
    fn process_input(input: &mut HandlerInput) -> ProcessorResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap()[2..].to_vec());
        let opcode = reader.read_u16::<LittleEndian>().unwrap();

        let handlers: ProcessorResult = match opcode {
            Opcode::SMSG_AUTH_CHALLENGE => {
                input.opcode = Some(opcode);
                vec![
                    Box::new(auth_challenge::Handler),
                ]
            },
            Opcode::SMSG_AUTH_RESPONSE => {
                input.opcode = Some(opcode);
                vec![
                    Box::new(ready_for_account_data_times::Handler),
                    Box::new(request_characters::Handler),
                    Box::new(realm_split::Handler),
                ]
            },
            Opcode::SMSG_ADDON_INFO => {
                input.opcode = Some(opcode);
                vec![]
            },
            Opcode::SMSG_CLIENTCACHE_VERSION => {
                input.opcode = Some(opcode);
                vec![]
            },
            Opcode::SMSG_TUTORIAL_FLAGS => {
                input.opcode = Some(opcode);
                vec![]
            },
            Opcode::SMSG_LOGIN_VERIFY_WORLD => {
                input.opcode = Some(opcode);
                vec![
                    Box::new(join_lfg_channel::Handler),
                    Box::new(join_common_channel::Handler),
                    Box::new(join_trade_channel::Handler),
                    Box::new(set_in_world::Handler),
                ]
            },
            Opcode::SMSG_CHAR_ENUM => {
                input.opcode = Some(opcode);
                vec![]
            },
            Opcode::SMSG_ACCOUNT_DATA_TIMES => {
                input.opcode = Some(opcode);
                vec![]
            },
            Opcode::SMSG_REALM_SPLIT => {
                input.opcode = Some(opcode);
                vec![]
            },
            Opcode::SMSG_MOTD => {
                input.opcode = Some(opcode);
                vec![Box::new(parse_motd::Handler)]
            },
            Opcode::SMSG_LOGIN_SETTIMESPEED => {
                input.opcode = Some(opcode);
                vec![]
            },
            Opcode::SMSG_SET_FORCED_REACTIONS => {
                input.opcode = Some(opcode);
                vec![]
            },
            _ => {
                vec![]
            },
        };

        handlers
    }
}