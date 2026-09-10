#![cfg(feature = "wow-wotlk")]

use tentacli::client::prelude::CtxMap;
use tentacli::plugins::wow::wotlk::realm::object::{
    Object, ObjectMap, ObjectTypeId, PackedGuid, objects, objects_mut,
};

#[test]
fn wotlk_object_api_is_available_to_external_consumers() {
    let mut ctx = CtxMap::default();
    ctx.insert(ObjectMap::default());

    assert!(objects(&ctx).is_some());
    assert!(objects_mut(&mut ctx).is_some());

    let _ = std::mem::size_of::<Object>();
    let _ = ObjectTypeId::Player;
    let _ = PackedGuid(1);
}
