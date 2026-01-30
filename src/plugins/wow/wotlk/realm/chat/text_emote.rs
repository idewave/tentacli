use std::sync::Arc;
use async_trait::async_trait;
use binrw::BinRead;
use serde::Serialize;
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::realm::chat::types::{Emote, TextEmote};

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
struct Incoming {
    guid: u64,
    text_emote: TextEmote,
    // is it correct ??? or should be just u32 ???
    emote: Emote,
    target_name_len: u32,
    #[br(
        if(target_name_len > 1),
        count = target_name_len,
        map = |bytes: Vec<u8>| {
            let s = std::str::from_utf8(&bytes)
                .unwrap_or("")
                .trim_end_matches('\0')
                .to_string();
            Some(s)
        }
    )]
    target_name: Option<String>,
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