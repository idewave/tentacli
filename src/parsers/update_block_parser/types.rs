use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use bitflags::bitflags;

use crate::parsers::movement_parser::types::MovementInfo;

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

#[derive(Clone, Default)]
pub struct ParsedBlock {
    pub guid: Option<u64>,
    pub update_fields: BTreeMap<u32, u32>,
    pub movement_data: Option<MovementData>,
}

impl ParsedBlock {
    pub fn new() -> Self {
        Self {
            guid: None,
            update_fields: BTreeMap::new(),
            movement_data: None,
        }
    }

    pub fn is_empty(parsed_block: &ParsedBlock) -> bool {
        parsed_block.guid.is_none()
            && parsed_block.update_fields.is_empty()
            && parsed_block.movement_data.is_none()
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
    pub const TYPEMASK_OBJECT: u32 = 0x0001;
    pub const TYPEMASK_ITEM: u32 = 0x0002;
    pub const TYPEMASK_CONTAINER: u32 = 0x0004;
    pub const TYPEMASK_UNIT: u32 = 0x0008;
    pub const TYPEMASK_PLAYER: u32 = 0x0010;
    pub const TYPEMASK_GAMEOBJECT: u32 = 0x0020;
    pub const TYPEMASK_DYNAMICOBJECT: u32 = 0x0040;
    pub const TYPEMASK_CORPSE: u32 = 0x0080;

    pub const IS_UNIT: u32 = ObjectTypeMask::TYPEMASK_OBJECT | ObjectTypeMask::TYPEMASK_UNIT;
    pub const IS_PLAYER: u32 = ObjectTypeMask::IS_UNIT | ObjectTypeMask::TYPEMASK_PLAYER;
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