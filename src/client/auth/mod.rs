use std::io::{Cursor};
use byteorder::{ReadBytesExt};

mod connect_to_realm;
mod get_realmlist;
mod login_challenge;
mod login_proof;
mod opcodes;
mod request_realmlist;
mod set_connected_to_realm;

// TODO: remove this (need to think how better refactor this part)
pub use login_challenge::handler as login_challenge;

use crate::client::auth::opcodes::Opcode;
use crate::types::traits::Processor;
use crate::types::{HandlerFunction, HandlerInput, ProcessorResult};


pub struct AuthProcessor;

impl Processor for AuthProcessor {
    fn process_input(input: &mut HandlerInput) -> ProcessorResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap());
        let opcode = reader.read_u8().unwrap();

        let mut message = String::new();

        let handlers: Vec<HandlerFunction> = match opcode {
            Opcode::LOGIN_CHALLENGE => {
                message = String::from("LOGIN_CHALLENGE");
                vec![Box::new(login_proof::handler)]
            },
            Opcode::LOGIN_PROOF => {
                message = String::from("LOGIN_PROOF");
                vec![Box::new(request_realmlist::handler)]
            },
            Opcode::REALM_LIST => {
                message = String::from("REALM_LIST");
                vec![
                    Box::new(get_realmlist::handler),
                    Box::new(connect_to_realm::handler),
                    Box::new(set_connected_to_realm::handler),
                ]
            }
            _ => vec![],
        };

        input.message_income.send_server_message(message);

        handlers
    }
}