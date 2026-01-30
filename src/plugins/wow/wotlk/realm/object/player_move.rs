use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use binrw::BinRead;
use serde::Serialize;
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::realm::object::types::movement::{Movement, MovementInfo};
use crate::plugins::wow::wotlk::realm::object::types::packed_guid::PackedGuid;
use crate::plugins::wow::wotlk::realm::object::update_object::Object;

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
struct Incoming {
    guid: PackedGuid,
    movement_info: MovementInfo,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(
        &mut self,
        packet: &mut Packet,
        _: Arc<RwLock<CtxMap>>
    ) -> anyhow::Result<Vec<HandlerOutput>> {
        let incoming = Incoming::unpack(packet)?;

        let guid = incoming.guid;
        let movement_info = incoming.movement_info;

        Ok(vec![
            HandlerOutput::Requests(vec![
                Request::SetContext(Some(Box::new(move |ctx: &mut CtxMap| {
                    let Some(objects) = ctx.get_mut::<HashMap<PackedGuid, Object>>() else {
                        return;
                    };

                    let Some(object) = objects.get_mut(&guid) else {
                        return;
                    };

                    match object.movement.as_mut() {
                        Some(movement) => {
                            movement.movement_info = Some(movement_info);
                        }
                        None => {
                            object.movement = Some(Movement {
                                movement_info: Some(movement_info),
                                ..Default::default()
                            });
                        }
                    }
                })))
            ])
        ])
    }
}
