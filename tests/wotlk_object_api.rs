#![cfg(feature = "wow-wotlk")]

use std::collections::BTreeMap;

use tentacli::client::prelude::CtxMap;
use tentacli::plugins::wow::wotlk::realm::object::types::movement::{
    Movement, OrientedPoint3D, Point3D,
};
use tentacli::plugins::wow::wotlk::realm::object::types::update_data::ObjectTypeMask;
use tentacli::plugins::wow::wotlk::realm::object::types::update_fields::{
    FieldValue, GameObjectField, ObjectField, PlayerField, UnitField,
};
use tentacli::plugins::wow::wotlk::realm::object::{
    Class, Gender, Object, ObjectLifecycleRegistry, ObjectMap, ObjectTypeId, PackedGuid, PowerType,
    Race, object_lifecycle, objects, objects_mut,
};

fn empty_object(guid: PackedGuid, object_type_id: ObjectTypeId, mask: ObjectTypeMask) -> Object {
    Object {
        guid,
        object_type_id,
        object_type_mask: mask,
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
fn wotlk_object_api_is_available_to_external_consumers() {
    let mut ctx = CtxMap::default();
    ctx.insert(ObjectMap::default());
    ctx.insert(ObjectLifecycleRegistry::default());

    assert!(objects(&ctx).is_some());
    assert!(objects_mut(&mut ctx).is_some());
    assert!(object_lifecycle(&ctx).is_some());

    let _ = std::mem::size_of::<Object>();
    let _ = ObjectTypeId::Player;
    let _ = PackedGuid(1);
}

#[test]
fn typed_player_accessors_hide_raw_update_field_representation() {
    let guid = PackedGuid(42);
    let mut object = empty_object(
        guid,
        ObjectTypeId::Player,
        ObjectTypeMask::OBJECT | ObjectTypeMask::UNIT | ObjectTypeMask::PLAYER,
    );

    object.object_fields.insert(ObjectField::Entry, FieldValue::Integer(123));
    object.object_fields.insert(ObjectField::ScaleX, FieldValue::Float(1.25));

    let bytes0 = u8::from(Race::Human) as u32
        | ((u8::from(Class::Mage) as u32) << 8)
        | ((u8::from(Gender::Female) as u32) << 16)
        | ((u8::from(PowerType::Mana) as u32) << 24);
    object
        .unit_fields
        .insert(UnitField::Bytes0, FieldValue::Bytes(bytes0));
    object
        .unit_fields
        .insert(UnitField::Health, FieldValue::Integer(900));
    object
        .unit_fields
        .insert(UnitField::MaxHealth, FieldValue::Integer(1000));
    object.unit_fields.insert(
        UnitField::Powers,
        FieldValue::IntegerArray(vec![Some(700), None, None, None, None, None, None]),
    );
    object.unit_fields.insert(
        UnitField::MaxPowers,
        FieldValue::IntegerArray(vec![Some(1000), None, None, None, None, None, None]),
    );
    object
        .unit_fields
        .insert(UnitField::Level, FieldValue::Integer(80));
    object
        .unit_fields
        .insert(UnitField::Target, FieldValue::Long(77));
    object.player_fields.insert(
        PlayerField::VisibleItems,
        FieldValue::CustomArray(vec![vec![
            Some(FieldValue::Integer(12345)),
            Some(FieldValue::TwoShorts((7, 8))),
        ]]),
    );

    assert_eq!(object.guid(), guid);
    assert_eq!(object.object_type_id(), ObjectTypeId::Player);
    assert_eq!(object.entry_id(), Some(123));
    assert_eq!(object.scale(), Some(1.25));

    let unit = object.as_unit().unwrap();
    assert_eq!(unit.health(), Some(900));
    assert_eq!(unit.max_health(), Some(1000));
    assert_eq!(unit.power_type(), Some(PowerType::Mana));
    assert_eq!(unit.power(), Some(700));
    assert_eq!(unit.max_power(), Some(1000));
    assert_eq!(unit.level(), Some(80));
    assert_eq!(unit.target_guid(), Some(PackedGuid(77)));

    let player = object.as_player().unwrap();
    assert_eq!(player.race(), Some(Race::Human));
    assert_eq!(player.class(), Some(Class::Mage));
    assert_eq!(player.gender(), Some(Gender::Female));
    assert_eq!(player.visible_item_entry(0), Some(12345));
    assert_eq!(player.visible_item_enchantment(0), Some((7, 8)));
}

#[test]
fn typed_accessors_return_none_for_missing_or_wrong_object_fields() {
    let object = empty_object(
        PackedGuid(5),
        ObjectTypeId::GameObject,
        ObjectTypeMask::OBJECT | ObjectTypeMask::GAMEOBJECT,
    );

    assert!(object.as_unit().is_none());
    let game_object = object.as_game_object().unwrap();
    assert_eq!(game_object.display_id(), None);
    assert_eq!(game_object.state(), None);
    assert_eq!(game_object.game_object_type(), None);
}

#[test]
fn game_object_bytes_are_exposed_as_state_and_type() {
    let mut object = empty_object(
        PackedGuid(6),
        ObjectTypeId::GameObject,
        ObjectTypeMask::OBJECT | ObjectTypeMask::GAMEOBJECT,
    );
    object
        .game_object_fields
        .insert(GameObjectField::DisplayId, FieldValue::Integer(999));
    object
        .game_object_fields
        .insert(GameObjectField::Bytes1, FieldValue::Bytes(1 | (3 << 8)));

    let game_object = object.as_game_object().unwrap();
    assert_eq!(game_object.display_id(), Some(999));
    assert_eq!(game_object.state(), Some(1));
    assert_eq!(game_object.game_object_type(), Some(3));
}

#[test]
fn typed_unit_and_game_object_accessors_cover_flags_and_movement() {
    let mut unit_object = empty_object(
        PackedGuid(80),
        ObjectTypeId::Unit,
        ObjectTypeMask::OBJECT | ObjectTypeMask::UNIT,
    );
    unit_object
        .unit_fields
        .insert(UnitField::FactionTemplate, FieldValue::Integer(35));
    unit_object
        .unit_fields
        .insert(UnitField::NpcFlags, FieldValue::Integer(0x10));
    unit_object
        .unit_fields
        .insert(UnitField::Flags, FieldValue::Integer(0x20));
    unit_object
        .unit_fields
        .insert(UnitField::Flags2, FieldValue::Integer(0x40));
    unit_object
        .unit_fields
        .insert(UnitField::DynamicFlags, FieldValue::Integer(0x80));
    unit_object
        .unit_fields
        .insert(UnitField::AuraState, FieldValue::Integer(0x100));
    unit_object.movement = Some(Movement {
        world_object_position: Some(OrientedPoint3D {
            point: Point3D {
                x: 11.0,
                y: 12.0,
                z: 13.0,
            },
            direction: 1.25,
        }),
        ..Default::default()
    });

    let unit = unit_object.as_unit().unwrap();
    assert_eq!(unit.faction_template(), Some(35));
    assert_eq!(unit.npc_flags(), Some(0x10));
    assert_eq!(unit.flags(), Some(0x20));
    assert_eq!(unit.flags2(), Some(0x40));
    assert_eq!(unit.dynamic_flags(), Some(0x80));
    assert_eq!(unit.aura_state(), Some(0x100));
    assert_eq!(unit.position().unwrap().x, 11.0);
    assert_eq!(unit.facing(), Some(1.25));

    let mut game_object = empty_object(
        PackedGuid(81),
        ObjectTypeId::GameObject,
        ObjectTypeMask::OBJECT | ObjectTypeMask::GAMEOBJECT,
    );
    game_object
        .game_object_fields
        .insert(GameObjectField::Flags, FieldValue::Integer(0x200));
    game_object
        .game_object_fields
        .insert(GameObjectField::Faction, FieldValue::Integer(14));
    game_object.game_object_fields.insert(
        GameObjectField::ParentRotation,
        FieldValue::FloatArray(vec![Some(0.0), Some(0.5), None, Some(1.0)]),
    );
    game_object.movement = Some(Movement {
        game_object_position: Some(OrientedPoint3D {
            point: Point3D {
                x: 21.0,
                y: 22.0,
                z: 23.0,
            },
            direction: 2.5,
        }),
        game_object_rotation: Some(123456),
        ..Default::default()
    });

    let game_object = game_object.as_game_object().unwrap();
    assert_eq!(game_object.flags(), Some(0x200));
    assert_eq!(game_object.faction(), Some(14));
    assert_eq!(game_object.position().unwrap().z, 23.0);
    assert_eq!(game_object.facing(), Some(2.5));
    assert_eq!(game_object.rotation(), Some(123456));
    assert_eq!(
        game_object.parent_rotation(),
        Some(&[Some(0.0_f32), Some(0.5_f32), None, Some(1.0_f32)][..])
    );
}

#[test]
fn typed_accessors_return_none_for_wrong_field_value_variants() {
    let mut object = empty_object(
        PackedGuid(90),
        ObjectTypeId::Player,
        ObjectTypeMask::OBJECT | ObjectTypeMask::UNIT | ObjectTypeMask::PLAYER,
    );

    object
        .object_fields
        .insert(ObjectField::Entry, FieldValue::Float(123.0));
    object
        .object_fields
        .insert(ObjectField::ScaleX, FieldValue::Integer(1));
    object
        .unit_fields
        .insert(UnitField::Health, FieldValue::Float(900.0));
    object
        .unit_fields
        .insert(UnitField::Target, FieldValue::Integer(77));
    object
        .unit_fields
        .insert(UnitField::Bytes0, FieldValue::Integer(0));
    object.player_fields.insert(
        PlayerField::VisibleItems,
        FieldValue::IntegerArray(vec![Some(12345)]),
    );

    assert_eq!(object.entry_id(), None);
    assert_eq!(object.scale(), None);

    let unit = object.as_unit().unwrap();
    assert_eq!(unit.health(), None);
    assert_eq!(unit.target_guid(), None);
    assert_eq!(unit.power_type(), None);

    let player = object.as_player().unwrap();
    assert_eq!(player.race(), None);
    assert_eq!(player.class(), None);
    assert_eq!(player.gender(), None);
    assert_eq!(player.visible_item_entry(0), None);
}

#[test]
fn typed_enum_accessors_return_none_for_unknown_protocol_values() {
    let mut object = empty_object(
        PackedGuid(91),
        ObjectTypeId::Player,
        ObjectTypeMask::OBJECT | ObjectTypeMask::UNIT | ObjectTypeMask::PLAYER,
    );

    let unknown_bytes0 = 0xFF_FF_FF_FF;
    object
        .unit_fields
        .insert(UnitField::Bytes0, FieldValue::Bytes(unknown_bytes0));

    let unit = object.as_unit().unwrap();
    assert_eq!(unit.power_type(), None);
    assert_eq!(unit.power(), None);
    assert_eq!(unit.max_power(), None);

    let player = object.as_player().unwrap();
    assert_eq!(player.race(), None);
    assert_eq!(player.class(), None);
    assert_eq!(player.gender(), None);
}

#[test]
fn typed_accessors_handle_missing_array_slots_without_panicking() {
    let mut object = empty_object(
        PackedGuid(92),
        ObjectTypeId::Player,
        ObjectTypeMask::OBJECT | ObjectTypeMask::UNIT | ObjectTypeMask::PLAYER,
    );

    let bytes0 = u8::from(Race::Human) as u32
        | ((u8::from(Class::Mage) as u32) << 8)
        | ((u8::from(Gender::Male) as u32) << 16)
        | ((u8::from(PowerType::Mana) as u32) << 24);
    object
        .unit_fields
        .insert(UnitField::Bytes0, FieldValue::Bytes(bytes0));
    object.unit_fields.insert(
        UnitField::Powers,
        FieldValue::IntegerArray(vec![None]),
    );
    object.unit_fields.insert(
        UnitField::MaxPowers,
        FieldValue::IntegerArray(Vec::new()),
    );
    object.player_fields.insert(
        PlayerField::VisibleItems,
        FieldValue::CustomArray(vec![vec![Some(FieldValue::Integer(12345))]]),
    );

    let unit = object.as_unit().unwrap();
    assert_eq!(unit.power(), None);
    assert_eq!(unit.max_power(), None);

    let player = object.as_player().unwrap();
    assert_eq!(player.visible_item_entry(99), None);
    assert_eq!(player.visible_item_enchantment(0), None);
    assert_eq!(player.visible_item_enchantment(99), None);
}

#[test]
fn typed_views_reject_incompatible_object_types() {
    let unit = empty_object(
        PackedGuid(93),
        ObjectTypeId::Unit,
        ObjectTypeMask::OBJECT | ObjectTypeMask::UNIT,
    );
    assert!(unit.as_unit().is_some());
    assert!(unit.as_player().is_none());
    assert!(unit.as_game_object().is_none());

    let game_object = empty_object(
        PackedGuid(94),
        ObjectTypeId::GameObject,
        ObjectTypeMask::OBJECT | ObjectTypeMask::GAMEOBJECT,
    );
    assert!(game_object.as_unit().is_none());
    assert!(game_object.as_player().is_none());
    assert!(game_object.as_game_object().is_some());
}
