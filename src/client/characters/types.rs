use std::fmt::{Debug, Formatter};
use crate::parsers::position_parser::types::Position;

#[derive(Clone, Default)]
pub struct Character {
    pub guid: u64,
    pub name: String,
    pub race: u8,
    pub class: u8,
    pub gender: u8,
    pub level: u8,
    pub position: Position,
}

impl Debug for Character {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "guid: {:?}, name: {:?}, race: {:?}, level: {:?}, position: {:?}",
            self.guid,
            self.name,
            self.race,
            self.level,
            self.position,
        )
    }
}