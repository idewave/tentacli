use std::io::BufRead;
use sha1::{Sha1};
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use tokio::io::{AsyncBufRead, AsyncReadExt};

use crate::primary::macros::with_opcode;
use crate::primary::client::Opcode;
use crate::primary::crypto::srp::Srp;
use crate::primary::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::primary::traits::PacketHandler;
use crate::primary::utils::encode_hex;

with_opcode! {
    @login_opcode(Opcode::LOGIN_PROOF)
    #[derive(LoginPacket, Serialize, Deserialize, Debug)]
    pub struct LoginChallengeResponse {
        unknown: u8,
        code: u8,
        #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
        server_ephemeral: [u8; 32],
        g_len: u8,
        #[dynamic_field]
        g: Vec<u8>,
        n_len: u8,
        #[dynamic_field]
        n: Vec<u8>,
        #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
        salt: [u8; 32],
        version_challenge: [u8; 16],
        unknown2: u8,
    }

    impl LoginChallengeResponse {
        fn g<R: BufRead>(mut reader: R, cache: &mut Self) -> Vec<u8> {
            let mut buffer = vec![0u8; cache.g_len as usize];
            reader.read_exact(&mut buffer).unwrap();
            buffer
        }

        async fn async_g<R>(stream: &mut R, cache: &mut Self) -> Vec<u8>
            where R: AsyncBufRead + Unpin + Send
        {
            let mut buffer = vec![0u8; cache.g_len as usize];
            stream.read_exact(&mut buffer).await.unwrap();
            buffer
        }

        fn n<R: BufRead>(mut reader: R, cache: &mut Self) -> Vec<u8> {
            let mut buffer = vec![0u8; cache.n_len as usize];
            reader.read_exact(&mut buffer).unwrap();
            buffer
        }

        async fn async_n<R>(stream: &mut R, cache: &mut Self) -> Vec<u8>
            where R: AsyncBufRead + Unpin + Send
        {
            let mut buffer = vec![0u8; cache.n_len as usize];
            stream.read_exact(&mut buffer).await.unwrap();
            buffer
        }
    }
}

with_opcode! {
    @login_opcode(Opcode::LOGIN_PROOF)
    #[derive(LoginPacket, Serialize, Deserialize, Debug)]
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
        srp_client.calculate_session_key::<Sha1>(account, password);

        let client_proof: [u8; 20] = srp_client.calculate_proof::<Sha1>(account);
        let crc_hash: [u8; 20] = rand::random();

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
        }.unpack()?));

        session.srp = Some(srp_client);

        Ok(response)
    }
}