use async_trait::async_trait;
use binrw::BinRead;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::realm::object::types::packed_guid::PackedGuid;

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
struct Incoming {
    target_guid: PackedGuid,
    caster_guid: PackedGuid,
    spell_id: u32,
    damage: u32,
    overheal: u32,
    absorb: u32,
    #[br(map = |v: u8| v != 0)]
    critical: bool,
    unknown: u8,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(
        &mut self,
        packet: &mut Packet,
        _: Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<Vec<HandlerOutput>> {
        let _ = Incoming::unpack(packet)?;
        Ok(vec![])
    }
}
