use async_trait::async_trait;

use crate::primary::client::Opcode;
use crate::primary::macros::with_opcode;
use crate::primary::traits::PacketHandler;
use crate::primary::types::{HandlerInput, HandlerResult};
use crate::types::HandlerOutput;

with_opcode! {
    @login_opcode(Opcode::LOGIN_PROOF)
    #[derive(LoginPacket, Serialize, Deserialize, Debug)]
    pub struct LoginProofResponse {
        error: u8,
        server_proof: [u8; 20],
        account_flags: u32,
        survey_id: u32,
        unknown_flags: u16,
    }
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();
        let (LoginProofResponse { server_proof, .. }, _) = LoginProofResponse::from_binary(&input.data)?;

        let mut guard = input.session.lock().await;
        let is_valid_proof = guard.srp.as_mut().unwrap().validate_proof(server_proof);
        if !is_valid_proof {
            response.push(HandlerOutput::ErrorMessage("Proof is not valid".into(), None));
            response.push(HandlerOutput::Drop);
        }

        Ok(response)
    }
}