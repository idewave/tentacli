use std::io::{Cursor};
use byteorder::{ReadBytesExt};

mod check_proof_code;
mod connect_to_realm;
mod get_realmlist;
mod login_challenge;
mod login_proof;
mod request_realmlist;
mod types;

// TODO: remove this (need to think how better refactor this part)
pub use login_challenge::handler as login_challenge;

use crate::primary::client::Opcode;
use crate::primary::traits::processor::Processor;
use crate::primary::types::{HandlerInput, ProcessorResult};

pub struct AuthProcessor;

impl Processor for AuthProcessor {
    fn process_input(input: &mut HandlerInput) -> ProcessorResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap());
        let opcode = reader.read_u8().unwrap();

        let handlers: ProcessorResult = match opcode {
            Opcode::LOGIN_CHALLENGE => {
                input.opcode = Some(opcode as u16);
                vec![
                    Box::new(check_proof_code::Handler),
                    Box::new(login_proof::Handler),
                ]
            },
            Opcode::LOGIN_PROOF => {
                input.opcode = Some(opcode as u16);
                vec![Box::new(request_realmlist::Handler)]
            },
            Opcode::REALM_LIST => {
                input.opcode = Some(opcode as u16);
                vec![
                    Box::new(get_realmlist::Handler),
                    Box::new(connect_to_realm::Handler),
                ]
            }
            _ => vec![],
        };

        handlers
    }
}