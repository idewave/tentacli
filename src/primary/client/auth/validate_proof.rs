use async_trait::async_trait;
use tentacli_traits::PacketHandler;
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};

#[derive(LoginPacket, Serialize, Deserialize, Debug)]
#[options(with_async)]
pub struct LoginProofResponse {
    error: u8,
    server_proof: [u8; 20],
    account_flags: u32,
    survey_id: u32,
    unknown_flags: u16,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();
        let (LoginProofResponse {
            server_proof, ..
        }, _) = LoginProofResponse::from_binary(&input.data)?;

        let mut guard = input.session.lock().await;
        let is_valid_proof = guard.srp.as_mut().unwrap().validate_proof(server_proof);
        if !is_valid_proof {
            response.push(HandlerOutput::ErrorMessage("Proof is not valid".into(), None));
            response.push(HandlerOutput::Drop);
        }

        Ok(response)
    }
}