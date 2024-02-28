mod check_proof_code;
mod connect_to_realm;
mod get_realmlist;
mod login_challenge;
mod login_proof;
mod request_realmlist;
mod types;
mod validate_proof;

// TODO: remove this (need to think how better refactor this part)
pub use login_challenge::handler as login_challenge;

use crate::primary::client::Opcode;
use crate::primary::traits::processor::Processor;
use crate::primary::types::{HandlerInput, ProcessorResult};

pub struct AuthProcessor;

impl Processor for AuthProcessor {
    fn get_handlers(input: &mut HandlerInput) -> ProcessorResult {
        let opcode = input.opcode as u8;

        let handlers: ProcessorResult = match opcode {
            Opcode::LOGIN_CHALLENGE => {
                vec![
                    Box::new(check_proof_code::Handler),
                    Box::new(login_proof::Handler),
                ]
            },
            Opcode::LOGIN_PROOF => {
                vec![
                    Box::new(validate_proof::Handler),
                    Box::new(request_realmlist::Handler),
                ]
            },
            Opcode::REALM_LIST => {
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