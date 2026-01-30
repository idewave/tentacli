use std::sync::Arc;
use async_trait::async_trait;
use binrw::BinRead;
use serde::Serialize;
use tokio::sync::RwLock;

use crate::client::prelude::*;

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
struct Incoming {
    map_id: u32,
    zone_id: u32,
    area_id: u32,
    blocks_amount: u16,
    #[br(count = blocks_amount)]
    world_states: Vec<WorldState>,
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

#[derive(BinRead, Serialize, FieldsMetadata)]
pub struct WorldState {
    pub state: u32,
    pub value: u32,
}