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

        let mut message = String::new();

        let handlers: ProcessorResult = match opcode {
            Opcode::SMSG_AUTH_CHALLENGE => {
                message = String::from("SMSG_AUTH_CHALLENGE");
                vec![
                    Box::new(auth_challenge::Handler),
                ]
            },
            Opcode::SMSG_AUTH_RESPONSE => {
                message = String::from("SMSG_AUTH_RESPONSE");
                vec![
                    Box::new(ready_for_account_data_times::Handler),
                    Box::new(request_characters::Handler),
                    Box::new(realm_split::Handler),
                ]
            },
            Opcode::SMSG_ADDON_INFO => {
                message = String::from("SMSG_ADDON_INFO");
                vec![]
            },
            Opcode::SMSG_CLIENTCACHE_VERSION => {
                message = String::from("SMSG_CLIENTCACHE_VERSION");
                vec![]
            },
            Opcode::SMSG_TUTORIAL_FLAGS => {
                message = String::from("SMSG_TUTORIAL_FLAGS");
                vec![]
            },
            Opcode::SMSG_LOGIN_VERIFY_WORLD => {
                message = String::from("SMSG_LOGIN_VERIFY_WORLD");
                vec![
                    Box::new(join_lfg_channel::Handler),
                    Box::new(join_common_channel::Handler),
                    Box::new(join_trade_channel::Handler),
                    Box::new(set_in_world::Handler),
                ]
            },
            Opcode::SMSG_CHAR_ENUM => {
                message = String::from("SMSG_CHAR_ENUM");
                vec![]
            },
            Opcode::SMSG_ACCOUNT_DATA_TIMES => {
                message = String::from("SMSG_ACCOUNT_DATA_TIMES");
                vec![]
            },
            Opcode::SMSG_REALM_SPLIT => {
                message = String::from("SMSG_REALM_SPLIT");
                vec![]
            },
            Opcode::SMSG_MOTD => {
                message = String::from("SMSG_MOTD");
                vec![Box::new(parse_motd::Handler)]
            },
            Opcode::SMSG_LOGIN_SETTIMESPEED => {
                message = String::from("SMSG_LOGIN_SETTIMESPEED");
                vec![Box::new(parse_motd::Handler)]
            },
            Opcode::SMSG_SET_FORCED_REACTIONS => {
                message = String::from("SMSG_SET_FORCED_REACTIONS");
                vec![Box::new(parse_motd::Handler)]
            },
            _ => {
                vec![]
            },
        };

        input.message_income.send_server_message(message);

        handlers
    }
}