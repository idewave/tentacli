use std::sync::Arc;
use async_trait::async_trait;
use binrw::BinRead;
use serde::Serialize;
use tokio::sync::RwLock;

use crate::client::prelude::*;

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
struct Incoming {
    spell_id: u32,
    unknown: u16,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(
        &mut self,
        packet: &mut Packet,
        _: Arc<RwLock<CtxMap>>
    ) -> anyhow::Result<Vec<HandlerOutput>> {
        let _ = Incoming::unpack(packet)?;
        Ok(vec![])
    }
}