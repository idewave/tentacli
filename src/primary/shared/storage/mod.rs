use std::collections::BTreeMap;

use crate::primary::client::Player;

#[derive(Debug)]
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