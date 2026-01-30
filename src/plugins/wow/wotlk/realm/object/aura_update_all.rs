use std::sync::Arc;
use async_trait::async_trait;
use binrw::BinRead;
use bitflags::bitflags;
use serde::Serialize;
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::realm::object::types::packed_guid::PackedGuid;

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
struct Incoming {
    target_guid: PackedGuid,
    #[br(parse_with = binrw::helpers::until_eof)]
    aura_info: Vec<AuraInfo>,
}

#[derive(BinRead, Serialize, Default, Clone, PartialEq, FieldsMetadata)]
struct AuraInfo {
    aura_slot: u8,
    aura_id: u32,
    aura_flags: AuraFlags,
    aura_level: u8,
    #[br(if(!aura_flags.contains(AuraFlags::NOT_CASTER)))]
    caster_guid: PackedGuid,
    #[br(if(aura_flags.contains(AuraFlags::DURATION)))]
    aura_max_duration: u32,
    #[br(if(aura_flags.contains(AuraFlags::DURATION)))]
    aura_duration: u32,
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

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Default, BitflagExtras)]
    #[bitflags_repr(u8)]
    pub struct AuraFlags: u8 {
        const NONE = 0x00;
        const EFF_INDEX_0 = 0x01;
        const EFF_INDEX_1 = 0x02;
        const EFF_INDEX_2 = 0x04;
        const NOT_CASTER = 0x08;
        const POSITIVE = 0x10;
        const DURATION = 0x20;
        const UNK2 = 0x40;
        const NEGATIVE = 0x80;
    }
}