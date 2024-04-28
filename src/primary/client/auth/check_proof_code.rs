use async_trait::async_trait;
use tentacli_traits::PacketHandler;
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::auth::AuthLogonResult;

#[derive(LoginPacket, Serialize, Debug)]
struct Income {
    unknown: u8,
    code: u8,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (Income { code, .. }, _) = Income::from_binary(&input.data)?;

        if code != AuthLogonResult::AUTH_LOGON_SUCCESS {
            let message = match code {
                AuthLogonResult::AUTH_LOGON_FAILED_UNKNOWN0
                | AuthLogonResult::AUTH_LOGON_FAILED_UNKNOWN1
                | AuthLogonResult::AUTH_LOGON_FAILED_INVALID_SERVER
                | AuthLogonResult::AUTH_LOGON_FAILED_FAIL_NOACCESS => {
                    HandlerOutput::ErrorMessage("Unable to connect".to_string(), None)
                },
                AuthLogonResult::AUTH_LOGON_FAILED_BANNED => {
                    HandlerOutput::ErrorMessage("Account was banned".to_string(), None)
                },
                AuthLogonResult::AUTH_LOGON_FAILED_SUSPENDED => {
                    HandlerOutput::ErrorMessage(
                        "Account was temporary suspended".to_string(),
                        None
                    )
                },
                AuthLogonResult::AUTH_LOGON_FAILED_UNKNOWN_ACCOUNT
                | AuthLogonResult::AUTH_LOGON_FAILED_INCORRECT_PASSWORD => {
                    HandlerOutput::ErrorMessage(
                        "Credentials not valid".to_string(),
                        None
                    )
                },
                AuthLogonResult::AUTH_LOGON_FAILED_ALREADY_ONLINE => {
                    HandlerOutput::ErrorMessage(
                        "Account already online".to_string(),
                        None
                    )
                },
                AuthLogonResult::AUTH_LOGON_FAILED_DB_BUSY => {
                    HandlerOutput::ErrorMessage(
                        "Cannot login at this time, try again later".to_string(),
                        None
                    )
                },
                _ => {
                    HandlerOutput::ErrorMessage(
                        format!("Unknown error with code: \"{}\"", code),
                        None
                    )
                },
            };

            response.push(message);
            response.push(HandlerOutput::Drop);

            return Ok(response);
        }

        Ok(response)
    }
}