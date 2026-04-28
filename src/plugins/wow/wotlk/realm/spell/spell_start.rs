use async_trait::async_trait;
use binrw::BinRead;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::realm::object::types::packed_guid::PackedGuid;
use crate::plugins::wow::wotlk::realm::spell::types::{
    AmmoInfo, CastFlags, SpellImmunity, SpellTargets,
};

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
struct Incoming {
    cast_item_guid: PackedGuid,
    caster_guid: PackedGuid,
    // pending spell cast
    cast_count: u8,
    spell_id: u32,
    cast_flags: CastFlags,
    // delay?
    timestamp: u32,
    targets: SpellTargets,

    #[br(if(cast_flags.contains(CastFlags::AMMO)))]
    ammo: AmmoInfo,

    #[br(if(cast_flags.contains(CastFlags::IMMUNITY)))]
    immunity: SpellImmunity,
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
