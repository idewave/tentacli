use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};

use crate::client::MovementInfo;

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