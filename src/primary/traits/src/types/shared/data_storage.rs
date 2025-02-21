use std::collections::HashMap;

use serde::Serialize;

use crate::types::movement::Movement;
use crate::types::update_data::UpdateData;

#[derive(Serialize, Clone, Default, Debug)]
pub struct Object {
    pub guid: u64,
    pub name: String,
    #[serde(skip_serializing_if = "Movement::is_default")]
    pub movement: Movement,
    #[serde(skip_serializing_if = "UpdateData::is_default")]
    pub update_data: UpdateData,
}

#[derive(Debug, Default)]
pub struct DataStorage {
    pub players_map: HashMap<u64, Object>,
    pub units_map: HashMap<u64, Object>,
    pub items_map: HashMap<u64, Object>,
    pub containers_map: HashMap<u64, Object>,
    pub game_objects_map: HashMap<u64, Object>,
    pub dynamic_objects_map: HashMap<u64, Object>,
    pub corpses_map: HashMap<u64, Object>,
}