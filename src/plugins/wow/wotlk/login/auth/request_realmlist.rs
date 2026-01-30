use std::sync::Arc;
use async_trait::async_trait;
use binrw::BinWrite;
use serde::Serialize;
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::opcodes::Opcode;

#[derive(Packet, BinWrite, Serialize, FieldsMetadata, Default)]
#[name(REALM_LIST)]
#[opcode(U8(Opcode::REALM_LIST))]
#[bw(little)]
struct Outgoing {
    unknown: i32,
}

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
            ]),
        ])
    }
}