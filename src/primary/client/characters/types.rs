use std::fmt::{Debug, Formatter};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeStruct;

use crate::primary::parsers::position_parser::types::Position;

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

impl<'de> Deserialize<'de> for Character {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        todo!()
    }
}

impl Serialize for Character {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        const FIELDS_AMOUNT: usize = 7;
        let mut state = serializer.serialize_struct("Character", FIELDS_AMOUNT)?;
        state.serialize_field("guid", &self.guid)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("race", &self.race)?;
        state.serialize_field("class", &self.class)?;
        state.serialize_field("gender", &self.gender)?;
        state.serialize_field("level", &self.level)?;
        state.serialize_field("position", &self.position)?;
        state.end()
    }
}