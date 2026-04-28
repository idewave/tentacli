use async_trait::async_trait;
use binrw::BinRead;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::realm::object::types::packed_guid::PackedGuid;
use crate::plugins::wow::wotlk::realm::object::update_object::Object;

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
struct Incoming {
    guid: u64,
    #[br(map = |v: u8| v != 0)]
    has_anim: bool,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(
        &mut self,
        packet: &mut Packet,
        _: Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<Vec<HandlerOutput>> {
        let Incoming { guid, .. } = Incoming::unpack(packet)?;

        Ok(vec![HandlerOutput::Requests(vec![Request::SetContext(
            Some(Box::new(move |ctx: &mut CtxMap| {
                let Some(objects) = ctx.get_mut::<HashMap<PackedGuid, Object>>() else {
                    return;
                };

                let packed = PackedGuid(guid);
                objects.remove(&packed);
            })),
        )])])
    }
}
