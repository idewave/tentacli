use std::sync::Arc;
use async_trait::async_trait;
use binrw::BinWrite;
use serde::Serialize;
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::opcodes::Opcode;

#[derive(Packet, BinWrite, Serialize, FieldsMetadata, Default)]
#[name(CMSG_READY_FOR_ACCOUNT_DATA_TIMES)]
#[opcode(U32(Opcode::CMSG_READY_FOR_ACCOUNT_DATA_TIMES))]
#[bw(little)]
pub struct Outgoing {}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(
        &mut self,
        _: &mut Packet,
        _: Arc<RwLock<CtxMap>>
    ) -> anyhow::Result<Vec<HandlerOutput>> {
        Ok(vec![
            HandlerOutput::Packets(vec![
                Outgoing::default().pack()?
            ])
        ])
    }
}