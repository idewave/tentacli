use std::io::BufRead;
use sha1::{Sha1};
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use crate::{with_opcode};
use crate::client::Opcode;
use crate::crypto::srp::Srp;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::traits::packet_handler::PacketHandler;
use crate::utils::encode_hex;

// TODO: check LOGIN_PROOF code before parsing rest packet

with_opcode! {
    @login_opcode(Opcode::LOGIN_PROOF)
    #[derive(LoginPacket, Serialize, Deserialize, Debug)]
    struct Income {
        unknown: u8,
        code: u8,
        #[serde(serialize_with = "crate::serializers::array_serializer::serialize_array")]
        server_ephemeral: [u8; 32],
        g_len: u8,
        #[dynamic_field]
        g: Vec<u8>,
        n_len: u8,
        #[dynamic_field]
        n: Vec<u8>,
        #[serde(serialize_with = "crate::serializers::array_serializer::serialize_array")]
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
        #[serde(serialize_with = "crate::serializers::array_serializer::serialize_array")]
        public_ephemeral: Vec<u8>,
        #[serde(serialize_with = "crate::serializers::array_serializer::serialize_array")]
        proof: Vec<u8>,
        #[serde(serialize_with = "crate::serializers::array_serializer::serialize_array")]
        crc_hash: [u8; 20],
        keys_count: u8,
        security_flags: u8,
    }
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let (Income { n, g, server_ephemeral, salt, .. }, json) = Income::from_binary(
            input.data.as_ref().unwrap()
        );

        input.message_income.send_server_message(
            Opcode::get_server_opcode_name(input.opcode.unwrap()),
            Some(json),
        );

        let mut session = input.session.lock().unwrap();
        let config = session.get_config().unwrap();

        let mut srp_client = Srp::new(&n, &g, &server_ephemeral);
        let proof = srp_client.calculate_proof::<Sha1>(
            &config.connection_data.account,
            &config.connection_data.password,
            &salt
        );
        let crc_hash: [u8; 20] = rand::random();

        session.session_key = Some(srp_client.session_key());

        input.message_income.send_debug_message(
            String::from("Session key created"),
            Some(encode_hex(&srp_client.session_key())),
        );

        Ok(HandlerOutput::Data(Outcome {
            public_ephemeral: srp_client.public_ephemeral(),
            proof,
            crc_hash,
            keys_count: 0,
            security_flags: 0
        }.unpack()))
    }
}