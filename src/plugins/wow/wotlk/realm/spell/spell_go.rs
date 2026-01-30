use std::sync::Arc;
use async_trait::async_trait;
use binrw::BinRead;
use fields_metadata::FieldsMetadata;
use serde::Serialize;
use tokio::sync::RwLock;

use crate::enum_field;
use crate::client::prelude::*;
use crate::plugins::wow::wotlk::realm::object::types::packed_guid::PackedGuid;
use crate::plugins::wow::wotlk::realm::spell::types::{
    AdjustMissile, AmmoInfo, CastFlags, PredictedRunes,
    SpellTargets, VisualChain, VisualChainTargets
};

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
struct Incoming {
    // who initiated the spell
    source_guid: PackedGuid,
    // who animated when cast
    caster_guid: PackedGuid,
    // pending spell cast
    cast_count: u8,
    spell_id: u32,
    cast_flags: CastFlags,
    // delay?
    timestamp: u32,
    spell_go_targets: SpellGOTargets,
    targets: SpellTargets,
    #[br(if(cast_flags.contains(CastFlags::PREDICTED_POWER)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    predicted_power: Option<u32>,
    #[br(if(cast_flags.contains(CastFlags::PREDICTED_RUNES)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    runes: Option<PredictedRunes>,
    #[br(if(cast_flags.contains(CastFlags::ADJUST_MISSILE)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    missile: Option<AdjustMissile>,
    #[br(if(cast_flags.contains(CastFlags::AMMO)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    ammo: Option<AmmoInfo>,
    #[br(if(cast_flags.contains(CastFlags::VISUAL_CHAIN)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    visual_chain: Option<VisualChain>,
    #[br(if(targets.has_dest_location))]
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_loc_counter: Option<u8>,
    #[br(if(targets.has_visual_chain_targets))]
    #[serde(skip_serializing_if = "Option::is_none")]
    visual_chain_targets: Option<VisualChainTargets>,
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

enum_field! {
    pub enum SpellMissInfo: u8 {
        None     = 0,
        Miss     = 1,
        Resist  = 2,
        Dodge   = 3,
        Parry   = 4,
        Block   = 5,
        Evade   = 6,
        Immune  = 7,
        Immune2 = 8,
        Deflect = 9,
        Absorb  = 10,
        Reflect = 11,
    }
}

#[derive(BinRead, Debug, Clone, FieldsMetadata, Serialize)]
pub struct SpellGOTargets {
    pub hit_count: u8,

    #[br(count = hit_count)]
    pub hit_guids: Vec<u64>,

    pub miss_count: u8,

    #[br(count = miss_count)]
    pub misses: Vec<MissEntry>,
}

#[derive(BinRead, Debug, Clone, FieldsMetadata, Serialize)]
pub struct MissEntry {
    pub target_guid: u64,
    pub miss_condition: SpellMissInfo,

    #[br(if(matches!(miss_condition, SpellMissInfo::Reflect)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reflect_result: Option<u8>,
}