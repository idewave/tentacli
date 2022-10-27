use sha1::{Sha1};
use async_trait::async_trait;

use crate::packet;
use super::opcodes::Opcode;
use crate::crypto::srp::Srp;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::traits::packet_handler::PacketHandler;

packet! {
    @option[login_opcode=Opcode::LOGIN_PROOF]
    struct Income {
        unknown: u8,
        code: u8,
        server_ephemeral: [u8; 32],
        g_len: u8,
        g: [u8; 32],
        n_len: u8,
        n: [u8; 1],
        salt: [u8; 32],
    }
}

packet! {
    @option[login_opcode=Opcode::LOGIN_PROOF]
    struct Outcome {
        public_ephemeral: Vec<u8>,
        proof: Vec<u8>,
        crc_hash: [u8; 20],
        keys_count: u8,
        security_flags: u8,
    }
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let Income { n, g, server_ephemeral, salt, .. } = Income::from_binary(
            input.data.as_ref().unwrap()
        );

        let mut session = input.session.lock().unwrap();
        let config = session.get_config().unwrap();

        let mut srp_client = Srp::new(
            n.to_vec(),
            g.to_vec(),
            server_ephemeral.to_vec()
        );
        let proof = srp_client.calculate_proof::<Sha1>(
            &config.connection_data.account,
            &config.connection_data.password,
            &salt
        );
        let crc_hash: [u8; 20] = rand::random();

        session.session_key = Some(srp_client.session_key());

        input.message_income.send_debug_message(String::from("Session key created"));

        Ok(HandlerOutput::Data(Outcome {
            public_ephemeral: srp_client.public_ephemeral(),
            proof,
            crc_hash,
            keys_count: 0,
            security_flags: 0
        }.unpack()))
    }
}