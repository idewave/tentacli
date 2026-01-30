use async_trait::async_trait;
use binrw::BinWrite;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::Arc;
use serde::Serialize;
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::config::Config;
use crate::plugins::wow::wotlk::opcodes::Opcode;

const BASE_PACKET_SIZE: u16 = 30;

#[derive(Packet, BinWrite, Serialize, FieldsMetadata)]
#[name(LOGIN_CHALLENGE)]
#[opcode(U8(Opcode::LOGIN_CHALLENGE))]
#[bw(little)]
struct Outgoing {
    unknown: u8,
    packet_size: u16,
    #[bw(write_with = crate::client::packet::helpers::null_terminated)]
    game_name: NullTerminated<String>,
    version: [u8; 3],
    build: u16,
    #[bw(write_with = crate::client::packet::helpers::reversed_null_terminated)]
    platform: NullTerminated<String>,
    #[bw(write_with = crate::client::packet::helpers::reversed_null_terminated)]
    os: NullTerminated<String>,
    #[bw(write_with = crate::client::packet::helpers::reversed)]
    locale: String,
    timezone: u32,
    ip: u32,
    account_length: u8,
    #[bw(write_with = crate::client::packet::helpers::just_bytes)]
    account: String,
}

#[derive(Default)]
pub struct Builder;
#[async_trait]
impl OutputBuilder for Builder {
    async fn build(&mut self, _: Arc<RwLock<CtxMap>>) -> anyhow::Result<Vec<HandlerOutput>> {
        let config: Config = ConfigParser::parse_from_file("wow/wotlk/connection.toml")?;
        let connection = config.connection.ok_or_else(|| anyhow::anyhow!("Missing [connection]"))?;
        let game = config.game.ok_or_else(|| anyhow::anyhow!("Missing [game]"))?;

        Ok(vec![
            HandlerOutput::Packets(vec![
                Outgoing {
                    unknown: 0,
                    packet_size: BASE_PACKET_SIZE + connection.account_name.len() as u16,
                    game_name: game.name.into(),
                    version: game.version,
                    build: game.build,
                    platform: game.platform.into(),
                    os: game.os.into(),
                    locale: game.locale,
                    timezone: game.timezone,
                    ip: Ipv4Addr::from_str(&game.my_ip)?.into(),
                    account_length: connection.account_name.len() as u8,
                    account: connection.account_name.to_uppercase(),
                }.pack()?,
            ]),
            HandlerOutput::Messages(vec![
                Message {
                    msg_type: MsgType::Info,
                    text: format!(
                        "Trying to login as \"{}\"", connection.account_name.to_uppercase()
                    ),
                }
            ])
        ])
    }
}