use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeStruct;

use crate::primary::client::{FieldValue, ObjectField, PlayerField, UnitField};
use crate::primary::parsers::movement_parser::types::MovementInfo;

#[derive(Clone, Default, Debug)]
pub struct MovementData {
    pub movement_info: Option<MovementInfo>,
    pub high_guid: Option<u32>,
    pub low_guid: Option<u32>,
    pub target_guid: Option<u64>,
    pub movement_speed: BTreeMap<u8, f32>,
}

impl MovementData {
    pub fn new() -> Self {
        Self {
            movement_info: None,
            high_guid: None,
            low_guid: None,
            target_guid: None,
            movement_speed: BTreeMap::new(),
        }
    }
}

impl<'de> Deserialize<'de> for MovementData {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        todo!()
    }
}

impl Serialize for MovementData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        const FIELDS_AMOUNT: usize = 5;
        let mut state = serializer.serialize_struct("MovementData", FIELDS_AMOUNT)?;
        state.serialize_field("movement_info", &self.movement_info)?;
        state.serialize_field("high_guid", &self.high_guid)?;
        state.serialize_field("low_guid", &self.low_guid)?;
        state.serialize_field("target_guid", &self.target_guid)?;
        state.serialize_field("movement_speed", &self.movement_speed)?;
        state.end()
    }
}

#[derive(Clone, Default)]
pub struct ParsedBlock {
    pub guid: Option<u64>,
    pub out_of_range_guids: Vec<u64>,
    pub near_object_guids: Vec<u64>,
    pub update_fields: BTreeMap<u32, FieldValue>,
    pub movement_data: Option<MovementData>,
}

impl<'de> Deserialize<'de> for ParsedBlock {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        todo!()
    }
}

impl Serialize for ParsedBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut update_fields: BTreeMap<String, FieldValue> = BTreeMap::new();
        for (k, v) in &self.update_fields {
            let key = if k < &ObjectField::LIMIT {
                ObjectField::get_field_name(*k)
            } else if k < &UnitField::LIMIT {
                UnitField::get_field_name(*k)
            } else {
                PlayerField::get_field_name(*k)
            };

            update_fields.insert(key, v.clone());
        }

        const FIELDS_AMOUNT: usize = 3;
        let mut state = serializer.serialize_struct("ParsedBlock", FIELDS_AMOUNT)?;
        state.serialize_field("guid", &self.guid)?;
        state.serialize_field("out_of_range_guids", &self.out_of_range_guids)?;
        state.serialize_field("near_object_guids", &self.near_object_guids)?;
        state.serialize_field("update_fields", &update_fields)?;
        state.serialize_field("movement_data", &self.movement_data)?;
        state.end()
    }
}

impl ParsedBlock {
    pub fn new() -> Self {
        Self {
            guid: None,
            out_of_range_guids: Vec::new(),
            near_object_guids: Vec::new(),
            update_fields: BTreeMap::new(),
            movement_data: None,
        }
    }

    pub fn is_empty(parsed_block: &ParsedBlock) -> bool {
        parsed_block.guid.is_none()
            && parsed_block.update_fields.is_empty()
            && parsed_block.movement_data.is_none()
            && parsed_block.out_of_range_guids.is_empty()
            && parsed_block.near_object_guids.is_empty()
    }
}

impl Debug for ParsedBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "guid: {:?}, update_fields: {:?}",
            self.guid,
            self.update_fields,
        )
    }
}

#[non_exhaustive]
pub struct ObjectUpdateType;

#[allow(dead_code)]
impl ObjectUpdateType {
    pub const VALUES: u8 = 0;
    pub const MOVEMENT: u8 = 1;
    pub const CREATE_OBJECT: u8 = 2;
    pub const CREATE_OBJECT2: u8 = 3;
    pub const OUT_OF_RANGE_OBJECTS: u8 = 4;
    pub const NEAR_OBJECTS: u8 = 5;
}

#[non_exhaustive]
pub struct ObjectTypeID;

#[allow(dead_code)]
impl ObjectTypeID {
    pub const TYPEID_OBJECT: u8 = 0;
    pub const TYPEID_ITEM: u8 = 1;
    pub const TYPEID_CONTAINER: u8 = 2;
    pub const TYPEID_UNIT: u8 = 3;
    pub const TYPEID_PLAYER: u8 = 4;
    pub const TYPEID_GAMEOBJECT: u8 = 5;
    pub const TYPEID_DYNAMICOBJECT: u8 = 6;
    pub const TYPEID_CORPSE: u8 = 7;
}

#[non_exhaustive]
pub struct ObjectTypeMask;

#[allow(dead_code)]
impl ObjectTypeMask {
    pub const TYPEMASK_OBJECT: u64 = 0x0001;
    pub const TYPEMASK_ITEM: u64 = 0x0002;
    pub const TYPEMASK_CONTAINER: u64 = 0x0004;
    pub const TYPEMASK_UNIT: u64 = 0x0008;
    pub const TYPEMASK_PLAYER: u64 = 0x0010;
    pub const TYPEMASK_GAMEOBJECT: u64 = 0x0020;
    pub const TYPEMASK_DYNAMICOBJECT: u64 = 0x0040;
    pub const TYPEMASK_CORPSE: u64 = 0x0080;

    pub const IS_UNIT: u64 = ObjectTypeMask::TYPEMASK_OBJECT | ObjectTypeMask::TYPEMASK_UNIT;
    pub const IS_PLAYER: u64 = ObjectTypeMask::IS_UNIT | ObjectTypeMask::TYPEMASK_PLAYER;
}

bitflags! {
    pub struct ObjectUpdateFlags: u16 {
        const NONE = 0x0000;
        const SELF = 0x0001;
        const TRANSPORT = 0x0002;
        const HAS_TARGET = 0x0004;
        const HIGHGUID = 0x0008;
        const LOWGUID = 0x0010;
        const LIVING = 0x0020;
        const STATIONARY_POSITION = 0x0040;
        const VEHICLE = 0x0080;
        const POSITION = 0x0100;
        const ROTATION = 0x0200;
    }
}