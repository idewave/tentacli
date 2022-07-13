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

        let handlers: Vec<HandlerFunction> = match opcode {
            Opcode::SMSG_AUTH_CHALLENGE => {
                println!("RECEIVE SMSG_AUTH_CHALLENGE");
                vec![
                    Box::new(auth_challenge::handler),
                    Box::new(set_encryption::handler),
                ]
            },
            Opcode::SMSG_AUTH_RESPONSE => {
                println!("RECEIVE SMSG_AUTH_RESPONSE");
                vec![
                    Box::new(ready_for_account_data_times::handler),
                    Box::new(request_characters::handler),
                    Box::new(realm_split::handler),
                ]
            },
            Opcode::SMSG_ADDON_INFO => {
                println!("RECEIVE SMSG_ADDON_INFO");
                vec![]
            },
            Opcode::SMSG_CLIENTCACHE_VERSION => {
                println!("RECEIVE SMSG_CLIENTCACHE_VERSION");
                vec![]
            },
            Opcode::SMSG_TUTORIAL_FLAGS => {
                println!("RECEIVE SMSG_TUTORIAL_FLAGS");
                vec![]
            },
            Opcode::SMSG_LOGIN_VERIFY_WORLD => {
                println!("RECEIVE SMSG_LOGIN_VERIFY_WORLD");
                vec![]
            },
            Opcode::SMSG_CHAR_ENUM => {
                println!("RECEIVE SMSG_CHAR_ENUM");
                vec![]
            },
            Opcode::SMSG_ACCOUNT_DATA_TIMES => {
                println!("RECEIVE SMSG_ACCOUNT_DATA_TIMES");
                vec![]
            },
            Opcode::SMSG_REALM_SPLIT => {
                println!("RECEIVE SMSG_REALM_SPLIT");
                vec![]
            },
            Opcode::SMSG_MOTD => {
                println!("RECEIVE SMSG_MOTD");
                vec![Box::new(parse_motd::handler)]
            },
            _ => {
                vec![]
            },
        };

        Self::collect_responses(handlers, input)
    }
}