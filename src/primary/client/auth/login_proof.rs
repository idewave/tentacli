use async_trait::async_trait;
use serde::{Serialize};
use tentacli_crypto::Srp;
use tentacli_traits::PacketHandler;
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::opcodes::Opcode;
use tentacli_utils::encode_hex;

#[derive(LoginPacket, Serialize, Debug)]
#[options(with_async)]
pub struct LoginChallengeResponse {
    unknown: u8,
    code: u8,
    #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
    server_ephemeral: [u8; 32],
    g_len: u8,
    #[depends_on(g_len)]
    g: Vec<u8>,
    n_len: u8,
    #[depends_on(n_len)]
    n: Vec<u8>,
    #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
    salt: [u8; 32],
    version_challenge: [u8; 16],
    unknown2: u8,
}

#[derive(LoginPacket, Serialize, Debug)]
struct Outcome {
    #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
    public_ephemeral: [u8; 32],
    #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
    client_proof: [u8; 20],
    #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
    crc_hash: [u8; 20],
    keys_count: u8,
    security_flags: u8,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (LoginChallengeResponse {
            n,
            g,
            server_ephemeral,
            salt,
            ..
        }, json) = LoginChallengeResponse::from_binary(&input.data)?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_opcode_name(input.opcode as u32)
                .unwrap_or(format!("Unknown opcode: {}", input.opcode)),
            Some(json),
        ));

        let mut session = input.session.lock().await;
        let (account, password) = {
            let config = session.get_config()?;
            (&config.connection_data.account, &config.connection_data.password)
        };

        let mut srp_client = Srp::new(&n, &g, &server_ephemeral, salt);
        srp_client.calculate_session_key(account, password);

        let client_proof: [u8; 20] = srp_client.calculate_proof(account);
        let crc_hash: [u8; 20]  = [
            0xCD, 0xCB, 0xBD, 0x51, 0x88, 0x31, 0x5E, 0x6B,
            0x4D, 0x19, 0x44, 0x9D, 0x49, 0x2D, 0xBC, 0xFA,
            0xF1, 0x56, 0xA3, 0x47
        ];

        response.push(HandlerOutput::DebugMessage(
            String::from("Session key created"),
            Some(encode_hex(&srp_client.session_key())),
        ));

        response.push(HandlerOutput::Data(Outcome {
            public_ephemeral: srp_client.public_ephemeral(),
            client_proof,
            crc_hash,
            keys_count: 0,
            security_flags: 0
        }.unpack_with_opcode(Opcode::LOGIN_PROOF)?));

        session.srp = Some(srp_client);

        Ok(response)
    }
}