use async_trait::async_trait;
use binrw::BinRead;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::realm::object::lifecycle::{
    ObjectLifecycleRegistry, ObjectRemovalReason, now_millis,
};
use crate::plugins::wow::wotlk::realm::object::{ObjectMap, PackedGuid};

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
        let removed_at = now_millis();

        Ok(vec![HandlerOutput::Requests(vec![Request::SetContext(
            Some(Box::new(move |ctx: &mut CtxMap| {
                apply_destroy(ctx, PackedGuid(guid), removed_at);
            })),
        )])])
    }
}

fn apply_destroy(ctx: &mut CtxMap, guid: PackedGuid, removed_at: u64) {
    if let Some(objects) = ctx.get_mut::<ObjectMap>() {
        objects.remove(&guid);
    }

    if let Some(registry) = ctx.get_mut::<ObjectLifecycleRegistry>()
        && let Some(lifecycle) = registry.get_mut(&guid)
    {
        lifecycle.mark_removed(removed_at, ObjectRemovalReason::Destroyed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    use crate::plugins::wow::wotlk::realm::object::lifecycle::ObjectLifecycle;
    use crate::plugins::wow::wotlk::realm::object::types::update_data::ObjectTypeMask;
    use crate::plugins::wow::wotlk::realm::object::{Object, ObjectTypeId};

    fn unit_object(guid: PackedGuid) -> Object {
        Object {
            guid,
            object_type_id: ObjectTypeId::Unit,
            object_type_mask: ObjectTypeMask::OBJECT | ObjectTypeMask::UNIT,
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
    fn destroy_removes_object_and_marks_existing_lifecycle_as_destroyed() {
        let guid = PackedGuid(11);
        let mut ctx = CtxMap::default();

        let mut objects = ObjectMap::default();
        objects.insert(guid, unit_object(guid));
        ctx.insert(objects);

        let mut registry = ObjectLifecycleRegistry::default();
        registry.insert(guid, ObjectLifecycle::created(100));
        ctx.insert(registry);

        apply_destroy(&mut ctx, guid, 250);

        assert!(!ctx.get::<ObjectMap>().unwrap().contains_key(&guid));
        let lifecycle = &ctx.get::<ObjectLifecycleRegistry>().unwrap()[&guid];
        assert_eq!(lifecycle.removed_at(), Some(250));
        assert_eq!(
            lifecycle.removal_reason(),
            Some(ObjectRemovalReason::Destroyed)
        );
    }

    #[test]
    fn destroy_does_not_overwrite_an_earlier_out_of_range_removal() {
        let guid = PackedGuid(12);
        let mut ctx = CtxMap::default();
        ctx.insert(ObjectMap::default());

        let mut lifecycle = ObjectLifecycle::created(100);
        lifecycle.mark_removed(200, ObjectRemovalReason::OutOfRange);
        let mut registry = ObjectLifecycleRegistry::default();
        registry.insert(guid, lifecycle);
        ctx.insert(registry);

        apply_destroy(&mut ctx, guid, 300);

        let lifecycle = &ctx.get::<ObjectLifecycleRegistry>().unwrap()[&guid];
        assert_eq!(lifecycle.removed_at(), Some(200));
        assert_eq!(
            lifecycle.removal_reason(),
            Some(ObjectRemovalReason::OutOfRange)
        );
    }
}
