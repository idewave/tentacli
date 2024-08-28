use std::collections::BTreeMap;

use crate::types::object::{Container, Corpse, DynamicObject, GameObject, Item, Unit};
use crate::types::player::Player;

#[derive(Debug, Default)]
pub struct DataStorage {
    pub players_map: BTreeMap<u64, Player>,
    pub units_map: BTreeMap<u64, Unit>,
    pub items_map: BTreeMap<u64, Item>,
    pub containers_map: BTreeMap<u64, Container>,
    pub game_objects_map: BTreeMap<u64, GameObject>,
    pub dynamic_objects_map: BTreeMap<u64, DynamicObject>,
    pub corpses_map: BTreeMap<u64, Corpse>,
}