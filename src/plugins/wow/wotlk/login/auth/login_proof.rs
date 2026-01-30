use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use binrw::{BinRead, BinWrite};
use serde::Serialize;
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::config::Config;
use crate::plugins::wow::wotlk::login::srp::Srp;
use crate::plugins::wow::wotlk::opcodes::Opcode;

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
pub struct Incoming {
    unknown: u8,
    code: u8,
    server_ephemeral: [u8; 32],
    g_len: u8,
    #[br(count = g_len)]
    g: Vec<u8>,
    n_len: u8,
    #[br(count = n_len)]
    n: Vec<u8>,
    salt: [u8; 32],
    version_challenge: [u8; 16],
    unknown2: u8,
}

#[derive(Packet, BinWrite, Serialize, FieldsMetadata)]
#[name(LOGIN_PROOF)]
#[opcode(U8(Opcode::LOGIN_PROOF))]
#[bw(little)]
struct Outgoing {
    public_ephemeral: [u8; 32],
    client_proof: [u8; 20],
    crc_hash: [u8; 20],
    keys_count: u8,
    security_flags: u8,
}

pub struct Handler {
    pub srp: Arc<Mutex<Srp>>,
}

#[async_trait]
impl PacketHandler for Handler {
    async fn handle(
        &mut self,
        packet: &mut Packet,
        _: Arc<RwLock<CtxMap>>
    ) -> anyhow::Result<Vec<HandlerOutput>> {
        let Incoming {
            n,
            g,
            server_ephemeral,
            salt,
            ..
        } = Incoming::unpack(packet)?;

        let config: Config = ConfigParser::parse_from_file("wow/wotlk/connection.toml")?;
        let connection = config.connection.ok_or_else(|| anyhow::anyhow!("Missing [connection]"))?;

        let (account, password) = {
            (connection.account_name.to_uppercase(), connection.password.to_uppercase())
        };

        let mut srp_client = self.srp.lock().map_err(|_| anyhow::anyhow!("Mutex poisoned"))?;
        srp_client.init(&n, &g, &server_ephemeral, salt);
        srp_client.calculate_session_key(&account, &password);

        let client_proof: [u8; 20] = srp_client.calculate_proof(&account);
        let crc_hash: [u8; 20] = [
            0xCD, 0xCB, 0xBD, 0x51, 0x88, 0x31, 0x5E, 0x6B,
            0x4D, 0x19, 0x44, 0x9D, 0x49, 0x2D, 0xBC, 0xFA,
            0xF1, 0x56, 0xA3, 0x47
        ];

        Ok(vec![
            HandlerOutput::Messages(vec![
                Message {
                    msg_type: Default::default(),
                    text: "SRP secret created".to_string(),
                }
            ]),
            HandlerOutput::Packets(
                vec![
                    Outgoing {
                        public_ephemeral: srp_client.public_ephemeral(),
                        client_proof,
                        crc_hash,
                        keys_count: 0,
                        security_flags: 0,
                    }.pack()?,
                ]
            ),
        ])
    }
}