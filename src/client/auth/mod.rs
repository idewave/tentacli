use std::io::{Cursor};
use byteorder::{ReadBytesExt};

mod get_realmlist;
mod login_challenge;
mod login_proof;
mod opcodes;
mod request_realmlist;
mod set_connected_to_realm;

// TODO: remove this (need to think how better refactor this part)
pub use login_challenge::handler as login_challenge;

use crate::client::auth::opcodes::Opcode;
use crate::traits::Processor;
use crate::types::{
    HandlerFunction,
    HandlerInput,
    ProcessorResult
};


pub struct AuthProcessor;

impl Processor for AuthProcessor {
    fn process_input(input: HandlerInput) -> ProcessorResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap());
        let opcode = reader.read_u8().unwrap();

        let handlers: Vec<HandlerFunction> = match opcode {
            Opcode::LOGIN_CHALLENGE => {
                println!("RESPOND TO LOGIN_CHALLENGE");
                vec![Box::new(login_proof::handler)]
            },
            Opcode::LOGIN_PROOF => {
                println!("RESPOND TO LOGIN_PROOF");
                vec![Box::new(request_realmlist::handler)]
            },
            Opcode::REALM_LIST => {
                println!("RESPOND TO REALM_LIST");
                vec![
                    Box::new(get_realmlist::handler),
                    Box::new(set_connected_to_realm::handler)
                ]
            }
            _ => vec![],
        };

        Self::collect_responses(handlers, input)
    }
}