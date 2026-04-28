use async_trait::async_trait;
use std::sync::{Arc, Mutex};

mod check_proof_error;
pub mod login_challenge;
pub mod login_proof;
mod request_realmlist;
pub mod select_realm;
pub mod validate_proof;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::login::srp::Srp;
use crate::plugins::wow::wotlk::opcodes::Opcode;

#[derive(Default)]
pub struct AuthProcessor {
    srp: Arc<Mutex<Srp>>,
}

#[async_trait]
impl Processor for AuthProcessor {
    fn get_handlers(
        &mut self,
        opcode: &PacketOpcode,
        _: &CtxMap,
    ) -> anyhow::Result<Vec<Box<dyn PacketHandler>>> {
        let opcode: u8 = opcode.try_into()?;

        Ok(match opcode {
            Opcode::LOGIN_CHALLENGE => vec![
                Box::new(check_proof_error::Handler),
                Box::new(login_proof::Handler {
                    srp: self.srp.clone(),
                }),
            ],
            Opcode::LOGIN_PROOF => vec![
                Box::new(validate_proof::Handler {
                    srp: self.srp.clone(),
                }),
                Box::new(request_realmlist::Handler),
            ],
            Opcode::REALM_LIST => vec![Box::new(select_realm::Handler)],
            _ => vec![],
        })
    }
}
