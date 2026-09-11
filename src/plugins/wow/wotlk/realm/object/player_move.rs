use async_trait::async_trait;
use binrw::BinRead;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::realm::object::lifecycle::{ObjectLifecycleRegistry, now_millis};
use crate::plugins::wow::wotlk::realm::object::types::movement::{Movement, MovementInfo};
use crate::plugins::wow::wotlk::realm::object::types::packed_guid::PackedGuid;
use crate::plugins::wow::wotlk::realm::object::ObjectMap;

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
        _: Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<Vec<HandlerOutput>> {
        let incoming = Incoming::unpack(packet)?;

        let guid = incoming.guid;
        let movement_info = incoming.movement_info;
        let updated_at = now_millis();

        Ok(vec![HandlerOutput::Requests(vec![Request::SetContext(
            Some(Box::new(move |ctx: &mut CtxMap| {
                apply_player_movement(ctx, guid, movement_info, updated_at);
            })),
        )])])
    }
}

fn apply_player_movement(
    ctx: &mut CtxMap,
    guid: PackedGuid,
    movement_info: MovementInfo,
    updated_at: u64,
) {
    {
        let Some(objects) = ctx.get_mut::<ObjectMap>() else {
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
    }

    if let Some(registry) = ctx.get_mut::<ObjectLifecycleRegistry>()
        && let Some(lifecycle) = registry.get_mut(&guid)
    {
        lifecycle.mark_updated(updated_at);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    use crate::plugins::wow::wotlk::realm::object::lifecycle::ObjectLifecycle;
    use crate::plugins::wow::wotlk::realm::object::types::movement::{
        MovementExtraFlags, MovementFlags, OrientedPoint3D, Point3D,
    };
    use crate::plugins::wow::wotlk::realm::object::types::update_data::ObjectTypeMask;
    use crate::plugins::wow::wotlk::realm::object::{Object, ObjectTypeId};

    fn player_object(guid: PackedGuid) -> Object {
        Object {
            guid,
            object_type_id: ObjectTypeId::Player,
            object_type_mask: ObjectTypeMask::OBJECT
                | ObjectTypeMask::UNIT
                | ObjectTypeMask::PLAYER,
            movement: None,
            object_fields: BTreeMap::new(),
            unit_fields: BTreeMap::new(),
            player_fields: BTreeMap::new(),
            item_fields: BTreeMap::new(),
            container_fields: BTreeMap::new(),
            game_object_fields: BTreeMap::new(),
            dynamic_object_fields: BTreeMap::new(),
            corpse_fields: BTreeMap::new(),
        }
    }

    #[test]
    fn player_movement_updates_position_and_lifecycle() {
        let guid = PackedGuid(101);
        let mut ctx = CtxMap::default();

        let mut objects = ObjectMap::default();
        objects.insert(guid, player_object(guid));
        ctx.insert(objects);

        let mut registry = ObjectLifecycleRegistry::default();
        registry.insert(guid, ObjectLifecycle::created(100));
        ctx.insert(registry);

        let movement_info = MovementInfo {
            movement_flags: MovementFlags::NONE,
            movement_extra_flags: MovementExtraFlags::NONE,
            time: 123,
            location: OrientedPoint3D {
                point: Point3D {
                    x: 1.0,
                    y: 2.0,
                    z: 3.0,
                },
                direction: 1.5,
            },
            taxi_info: None,
            fall_time: 0,
            jump_info: None,
        };

        apply_player_movement(&mut ctx, guid, movement_info, 250);

        let object = &ctx.get::<ObjectMap>().unwrap()[&guid];
        let movement_info = object
            .movement
            .as_ref()
            .and_then(|movement| movement.movement_info.as_ref())
            .unwrap();
        assert_eq!(movement_info.location.point.x, 1.0);
        assert_eq!(movement_info.location.direction, 1.5);

        let lifecycle = &ctx.get::<ObjectLifecycleRegistry>().unwrap()[&guid];
        assert_eq!(lifecycle.created_at(), 100);
        assert_eq!(lifecycle.updated_at(), 250);
    }
}
