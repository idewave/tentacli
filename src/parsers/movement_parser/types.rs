use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::client::{MovementFlags, MovementFlagsExtra};
use crate::parsers::position_parser::types::Position;

#[derive(Clone, Default, Debug)]
pub struct MovementInfo {
    pub movement_flags: MovementFlags,
    pub movement_flags_extra: MovementFlagsExtra,
    pub time: u32,
    pub position: Position,
    pub fall_time: u32,
    pub jump_info: JumpInfo,
}

impl<'de> Deserialize<'de> for MovementInfo {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        todo!()
    }
}

impl Serialize for MovementInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        const FIELDS_AMOUNT: usize = 6;
        let mut state = serializer.serialize_struct("MovementInfo", FIELDS_AMOUNT)?;
        state.serialize_field("movement_flags", &self.movement_flags.bits())?;
        state.serialize_field("movement_flags_extra", &self.movement_flags_extra.bits())?;
        state.serialize_field("time", &self.time)?;
        state.serialize_field("position", &self.position)?;
        state.serialize_field("fall_time", &self.fall_time)?;
        state.serialize_field("jump_info", &self.jump_info)?;
        state.end()
    }
}

#[derive(Clone, Default, Debug)]
pub struct JumpInfo {
    pub jump_vertical_speed: f32,
    pub jump_sin_angle: f32,
    pub jump_cos_angle: f32,
    pub jump_horizontal_speed: f32,
}

impl<'de> Deserialize<'de> for JumpInfo {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        todo!()
    }
}

impl Serialize for JumpInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        const FIELDS_AMOUNT: usize = 4;
        let mut state = serializer.serialize_struct("JumpInfo", FIELDS_AMOUNT)?;
        state.serialize_field("jump_vertical_speed", &self.jump_vertical_speed)?;
        state.serialize_field("jump_sin_angle", &self.jump_sin_angle)?;
        state.serialize_field("jump_cos_angle", &self.jump_cos_angle)?;
        state.serialize_field("jump_horizontal_speed", &self.jump_horizontal_speed)?;
        state.end()
    }
}