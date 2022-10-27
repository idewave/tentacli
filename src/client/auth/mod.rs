use std::io::{Cursor};
use byteorder::{ReadBytesExt};

mod connect_to_realm;
mod get_realmlist;
mod login_challenge;
mod login_proof;
mod opcodes;
mod request_realmlist;

// TODO: remove this (need to think how better refactor this part)
pub use login_challenge::handler as login_challenge;

use crate::client::auth::opcodes::Opcode;
use crate::traits::processor::Processor;
use crate::types::{HandlerInput, ProcessorResult};

pub struct AuthProcessor;

impl Processor for AuthProcessor {
    fn process_input(input: &mut HandlerInput) -> ProcessorResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap());
        let opcode = reader.read_u8().unwrap();

        let mut message = String::new();

        let handlers: ProcessorResult = match opcode {
            Opcode::LOGIN_CHALLENGE => {
                message = String::from("LOGIN_CHALLENGE");
                vec![Box::new(login_proof::Handler)]
            },
            Opcode::LOGIN_PROOF => {
                message = String::from("LOGIN_PROOF");
                vec![Box::new(request_realmlist::Handler)]
            },
            Opcode::REALM_LIST => {
                message = String::from("REALM_LIST");
                vec![
                    Box::new(get_realmlist::Handler),
                    Box::new(connect_to_realm::Handler),
                ]
            }
            _ => vec![],
        };

        input.message_income.send_server_message(message);

        handlers
    }
}