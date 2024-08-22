use tentacli_traits::Processor;
use tentacli_traits::types::{ProcessorResult};
use tentacli_traits::types::opcodes::Opcode;

mod check_proof_code;
mod connect_to_realm;
mod get_realmlist;
mod login_challenge;
mod login_proof;
mod request_realmlist;
mod validate_proof;

pub use login_proof::LoginChallengeResponse;
pub use validate_proof::LoginProofResponse;
pub use get_realmlist::RealmlistResponse;

pub struct AuthProcessor;

impl Processor for AuthProcessor {
    fn get_handlers(opcode: u16) -> ProcessorResult {
        let opcode = opcode as u8;

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

    fn get_initial_handlers(_opcode: u16) -> ProcessorResult {
        vec![
            Box::new(login_challenge::Handler)
        ]
    }
}