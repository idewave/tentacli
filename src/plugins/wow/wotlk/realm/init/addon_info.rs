use std::sync::Arc;
use async_trait::async_trait;
use binrw::BinRead;
use serde::Serialize;
use tokio::sync::RwLock;

use crate::client::prelude::*;

const MODULUS_LEN: usize = 128;

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
pub struct Incoming {
    #[br(parse_with = binrw::helpers::until_eof)]
    pub addons: Vec<AddonEntry>,
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

#[derive(BinRead, Serialize, FieldsMetadata, Debug)]
#[br(little)]
pub struct AddonEntry {
    /// Always 2
    pub entry_type: u8,

    /// Either:
    ///  - fingerprint byte
    ///  - or flags
    pub flags_or_fingerprint: u8,

    /// Present when fingerprint branch OR when flags != 0
    #[br(if(flags_or_fingerprint != 0))]
    pub has_modulus: Option<u8>,

    /// Only when has_modulus == 1
    #[br(if(has_modulus == Some(1)), count = MODULUS_LEN)]
    pub modulus: Option<Vec<u8>>,

    /// Only when flags_or_fingerprint != 0
    #[br(if(flags_or_fingerprint != 0))]
    pub unknown: Option<u32>,

    /// Always 0 (never update URL)
    pub url_flag: u8,
}