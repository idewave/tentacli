use std::sync::Arc;
use async_trait::async_trait;
use binrw::BinWrite;
use serde::Serialize;
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::opcodes::Opcode;

const COMMON_CHANNEL_ID: u32 = 1;
const LFG_CHANNEL_ID: u32 = 26;
const TRADE_CHANNEL_ID: u32 = 2;

#[derive(Packet, BinWrite, Serialize, FieldsMetadata, Default)]
#[name(CMSG_JOIN_CHANNEL)]
#[opcode(U32(Opcode::CMSG_JOIN_CHANNEL))]
#[bw(little)]
pub struct Outgoing {
    channel_id: u32,
    unknown: u8,
    unknown1: u8,
    #[bw(write_with = crate::client::packet::helpers::null_terminated)]
    channel_name: NullTerminated<String>,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(
        &mut self,
        _: &mut Packet,
        _: Arc<RwLock<CtxMap>>
    ) -> anyhow::Result<Vec<HandlerOutput>> {
        let make_packet = |channel_id: u32, channel_name: &str| -> anyhow::Result<Packet> {
            Outgoing {
                channel_id,
                channel_name: channel_name.into(),
                ..Default::default()
            }.pack()
        };

        Ok(vec![
            HandlerOutput::Packets(vec![
                make_packet(COMMON_CHANNEL_ID, "Common")?,
                make_packet(LFG_CHANNEL_ID, "LookingForGroup")?,
                make_packet(TRADE_CHANNEL_ID, "Trade")?,
            ])
        ])
    }
}