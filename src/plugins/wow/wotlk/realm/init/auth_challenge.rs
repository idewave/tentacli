use std::io::Write;
use std::sync::Arc;
use async_trait::async_trait;
use binrw::{BinRead, BinWrite};
use byteorder::{LittleEndian, WriteBytesExt};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::config::Config;
use crate::plugins::wow::wotlk::login::{Secret, ServerId};
use crate::plugins::wow::wotlk::opcodes::Opcode;

const SEED_SIZE: usize = 4;

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
struct Incoming {
    skip: u32,
    server_seed: [u8; SEED_SIZE],
    seed: [u8; 32],
}

#[derive(Packet, BinWrite, Serialize, FieldsMetadata)]
#[name(CMSG_AUTH_SESSION)]
#[opcode(U32(Opcode::CMSG_AUTH_SESSION))]
#[bw(little)]
struct Outgoing {
    build: u32,
    unknown: u32,
    #[bw(write_with = crate::client::packet::helpers::null_terminated)]
    account: NullTerminated<String>,
    unknown2: u32,
    client_seed: [u8; SEED_SIZE],
    unknown3: u64,
    server_id: u32,
    unknown4: u64,
    digest: [u8; 20],
    addons_count: u32,
    addons: Vec<u8>,
}

pub struct Handler;

#[async_trait]
impl PacketHandler for Handler {
    async fn handle(
        &mut self,
        packet: &mut Packet,
        context: Arc<RwLock<CtxMap>>
    ) -> anyhow::Result<Vec<HandlerOutput>> {
        let session_key = context.read().await.get::<Secret>()
            .map(|secret: &Secret| secret.0.to_vec())
            .unwrap_or_else(Vec::new);

        let server_id = context.read().await.get::<ServerId>()
            .map(|server_id: &ServerId| server_id.0 as u32)
            .unwrap_or_default();

        let Incoming { server_seed, .. } = Incoming::unpack(packet)?;

        let config: Config = ConfigParser::parse_from_file("wow/wotlk/connection.toml")?;
        let connection = config.connection.ok_or_else(|| anyhow::anyhow!("Missing [connection]"))?;
        let game = config.game.ok_or_else(|| anyhow::anyhow!("Missing [game]"))?;

        let config: AddonsConfig = ConfigParser::parse_from_file("wow/wotlk/addons.toml")?;
        let addon_info = AddonInfo::build_addon_info(&config.addons, config.timestamp)?;

        let client_seed: [u8; SEED_SIZE] = rand::random();

        let digest = Sha1::new()
            .chain(connection.account_name.to_uppercase())
            .chain(vec![0, 0, 0, 0])
            .chain(client_seed)
            .chain(server_seed)
            .chain(session_key)
            .finalize()
            .to_vec();

        Ok(vec![
            HandlerOutput::Packets(vec![
                Outgoing {
                    build: game.build as u32,
                    unknown: 0,
                    account: connection.account_name.to_uppercase().into(),
                    unknown2: 0,
                    client_seed,
                    unknown3: 0,
                    server_id,
                    unknown4: 0,
                    digest: digest.try_into().unwrap(),
                    addons_count: addon_info.len() as u32,
                    addons: compress(&addon_info)?,
                }.pack()?
            ]),
            HandlerOutput::Messages(vec![
                Message {
                    msg_type: MsgType::Success,
                    text: "Authentication successfully finished".to_string(),
                }
            ])
        ])
    }
}

pub fn compress(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(data)?;

    encoder.finish().map_err(|e| anyhow::anyhow!("Error on compress: {}", e))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddonInfo {
    pub name: String,
    pub flags: u8,
    pub modulus_crc: u32,
    pub urlcrc_crc: u32,
}

impl AddonInfo {
    pub fn build_addon_info(addons: &[AddonInfo], timestamp: u32) -> anyhow::Result<Vec<u8>> {
        let mut out = Vec::with_capacity(
            4 // u32: number of addons at the start of the buffer
                + addons.iter()
                .map(|a|
                         a.name.len() // addon name bytes
                             + 1 // C-string NUL terminator after the name
                             + 1 // flags: u8
                             + 4 // modulus_crc: u32 (little-endian)
                             + 4 // urlcrc_crc: u32 (little-endian)
                )
                .sum::<usize>()
                + 4 // trailing timestamp: u32 (little-endian)
        );

        // Write addon count (u32, LE)
        out.write_u32::<LittleEndian>(addons.len() as u32)?;

        for addon in addons {
            // Name as a C-string: raw bytes + NUL terminator
            out.write_all(addon.name.as_bytes())?;
            out.write_u8(0)?; // NUL terminator

            // Fixed-size addon fields
            out.write_u8(addon.flags)?;                        // flags (u8)
            out.write_u32::<LittleEndian>(addon.modulus_crc)?; // modulus_crc (u32, LE)
            out.write_u32::<LittleEndian>(addon.urlcrc_crc)?;  // urlcrc_crc (u32, LE)
        }

        // Final "last modified" timestamp (u32, LE)
        out.write_u32::<LittleEndian>(timestamp)?;
        Ok(out)
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddonsConfig {
    #[serde(default)]
    pub addons: Vec<AddonInfo>,
    #[serde(default = "default_addons_timestamp")]
    pub timestamp: u32,
}

fn default_addons_timestamp() -> u32 { 1636457673 }
