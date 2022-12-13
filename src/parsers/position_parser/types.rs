use std::fmt::{Debug, Formatter};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Copy, Clone, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub orientation: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32, orientation: f32) -> Self {
        Self { x, y, z, orientation }
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x: {:?}, y: {:?}, z: {:?}, orientation: {:?}",
            self.x,
            self.y,
            self.z,
            self.orientation,
        )
    }
}

impl<'de> Deserialize<'de> for Position {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        todo!()
    }
}

impl Serialize for Position {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        const FIELDS_AMOUNT: usize = 4;
        let mut state = serializer.serialize_struct("Position", FIELDS_AMOUNT)?;
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        state.serialize_field("z", &self.z)?;
        state.serialize_field("orientation", &self.orientation)?;
        state.end()
    }
}