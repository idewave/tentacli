use std::io::BufRead;
use sha1::{Sha1};
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use crate::primary::macros::with_opcode;
use crate::primary::client::Opcode;
use crate::primary::crypto::srp::Srp;
use crate::primary::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::primary::traits::packet_handler::PacketHandler;
use crate::primary::utils::encode_hex;

with_opcode! {
    @login_opcode(Opcode::LOGIN_PROOF)
    #[derive(LoginPacket, Serialize, Deserialize, Debug)]
    struct Income {
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
    }

    impl Income {
        fn g<R: BufRead>(mut reader: R, initial: &mut Self) -> Vec<u8> {
            let mut buffer = vec![0u8; initial.g_len as usize];
            reader.read_exact(&mut buffer).unwrap();
            buffer
        }

        fn n<R: BufRead>(mut reader: R, initial: &mut Self) -> Vec<u8> {
            let mut buffer = vec![0u8; initial.n_len as usize];
            reader.read_exact(&mut buffer).unwrap();
            buffer
        }
    }
}

with_opcode! {
    @login_opcode(Opcode::LOGIN_PROOF)
    #[derive(LoginPacket, Serialize, Deserialize, Debug)]
    struct Outcome {
        #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
        public_ephemeral: Vec<u8>,
        #[serde(serialize_with = "crate::primary::serializers::array_serializer::serialize_array")]
        proof: Vec<u8>,
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

        let (Income { n, g, server_ephemeral, salt, .. }, json) = Income::from_binary(
            input.data.as_ref().unwrap()
        )?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_server_opcode_name(input.opcode.unwrap()),
            Some(json),
        ));

        let mut session = input.session.lock().unwrap();
        let (account, password) = {
            let config = session.get_config()?;
            (&config.connection_data.account, &config.connection_data.password)
        };

        let mut srp_client = Srp::new(&n, &g, &server_ephemeral);
        let proof = srp_client.calculate_proof::<Sha1>(account, password, &salt);
        let crc_hash: [u8; 20] = rand::random();

        session.session_key = Some(srp_client.session_key());

        response.push(HandlerOutput::DebugMessage(
            String::from("Session key created"),
            Some(encode_hex(&srp_client.session_key())),
        ));

        response.push(HandlerOutput::Data(Outcome {
            public_ephemeral: srp_client.public_ephemeral(),
            proof,
            crc_hash,
            keys_count: 0,
            security_flags: 0
        }.unpack()?));

        Ok(response)
    }
}