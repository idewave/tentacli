use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use crate::{with_opcode};
use crate::client::auth::types::AuthLogonResult;
use crate::client::Opcode;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::traits::packet_handler::PacketHandler;

with_opcode! {
    @login_opcode(Opcode::LOGIN_PROOF)
    #[derive(LoginPacket, Serialize, Deserialize, Debug)]
    struct Income {
        unknown: u8,
        code: u8,
    }
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let (Income { code, .. }, _) = Income::from_binary(
            input.data.as_ref().unwrap()
        )?;

        if code != AuthLogonResult::AUTH_LOGON_SUCCESS {
            match code {
                AuthLogonResult::AUTH_LOGON_FAILED_UNKNOWN0
                | AuthLogonResult::AUTH_LOGON_FAILED_UNKNOWN1
                | AuthLogonResult::AUTH_LOGON_FAILED_INVALID_SERVER
                | AuthLogonResult::AUTH_LOGON_FAILED_FAIL_NOACCESS => {
                    input.message_income.send_error_message("Unable to connect".to_string(), None);
                },
                AuthLogonResult::AUTH_LOGON_FAILED_BANNED => {
                    input.message_income.send_error_message("Account was banned".to_string(), None);
                },
                AuthLogonResult::AUTH_LOGON_FAILED_SUSPENDED => {
                    input.message_income.send_error_message(
                        "Account was temporary suspended".to_string(),
                        None
                    );
                },
                AuthLogonResult::AUTH_LOGON_FAILED_UNKNOWN_ACCOUNT
                | AuthLogonResult::AUTH_LOGON_FAILED_INCORRECT_PASSWORD => {
                    input.message_income.send_error_message(
                        "Credentials not valid".to_string(),
                        None
                    );
                },
                AuthLogonResult::AUTH_LOGON_FAILED_ALREADY_ONLINE => {
                    input.message_income.send_error_message(
                        "Account already online".to_string(),
                        None
                    );
                },
                AuthLogonResult::AUTH_LOGON_FAILED_DB_BUSY => {
                    input.message_income.send_error_message(
                        "Cannot login at this time, try again later".to_string(),
                        None
                    );
                },
                _ => {},
            }

            return Ok(HandlerOutput::Drop);
        }

        Ok(HandlerOutput::Void)
    }
}