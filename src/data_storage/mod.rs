use std::collections::BTreeMap;

use crate::client::Player;

pub struct DataStorage {
    pub players_map: BTreeMap<u64, Player>,
}

impl DataStorage {
    pub fn new() -> Self {
        Self {
            players_map: BTreeMap::new(),
        }
    }
}