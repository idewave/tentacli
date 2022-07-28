use std::io::{Cursor};
use byteorder::{LittleEndian, ReadBytesExt};

mod auth_challenge;
mod parse_motd;
mod ping;
mod ready_for_account_data_times;
mod realm_split;
mod set_encryption;
pub mod types;

use crate::client::opcodes::Opcode;
use crate::client::characters::request_characters;
use crate::logger::types::LoggerOutput;
use crate::traits::Processor;
use crate::types::{
    HandlerFunction,
    HandlerInput,
    ProcessorResult
};

pub struct RealmProcessor;

impl Processor for RealmProcessor {
    fn process_input(input: HandlerInput) -> ProcessorResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap()[2..].to_vec());
        let opcode = reader.read_u16::<LittleEndian>().unwrap();
        
        let mut message = String::new();

        let handlers: Vec<HandlerFunction> = match opcode {
            Opcode::SMSG_AUTH_CHALLENGE => {
                message = String::from("SMSG_AUTH_CHALLENGE");
                vec![
                    Box::new(auth_challenge::handler),
                    Box::new(set_encryption::handler),
                ]
            },
            Opcode::SMSG_AUTH_RESPONSE => {
                message = String::from("SMSG_AUTH_RESPONSE");
                vec![
                    Box::new(ready_for_account_data_times::handler),
                    Box::new(request_characters::handler),
                    Box::new(realm_split::handler),
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
                vec![]
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
                vec![Box::new(parse_motd::handler)]
            },
            _ => {
                vec![]
            },
        };

        input.output_sender.send(LoggerOutput::Server(message)).unwrap();

        Self::collect_responses(handlers, input)
    }
}