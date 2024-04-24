use std::io::{BufRead, Error};
use bitflags::{bitflags};
use byteorder::{LittleEndian, ReadBytesExt};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::BinaryConverter;

use crate::errors::FieldError;
use crate::types::custom_fields::read_packed_guid;
use crate::types::position::Position;

#[derive(Clone, Default, Debug)]
pub struct MovementInfo {
    pub movement_flags: MovementFlags,
    pub movement_flags_extra: MovementFlagsExtra,
    pub time: u32,
    pub position: Position,
    pub fall_time: u32,
    pub jump_info: JumpInfo,
}

impl MovementInfo {
    pub fn parse<R: BufRead>(reader: &mut R) -> Result<MovementInfo, Error> {
        let movement_flags = MovementFlags::from_bits(
            reader.read_u32::<LittleEndian>()?
        ).unwrap_or(MovementFlags::NONE);

        let movement_flags_extra = MovementFlagsExtra::from_bits(
            reader.read_u16::<LittleEndian>()?
        ).unwrap_or(MovementFlagsExtra::NONE);

        let time = reader.read_u32::<LittleEndian>()?;

        let position = Position::parse(reader)?;

        if movement_flags.contains(MovementFlags::TAXI) {
            let _transport_guid = read_packed_guid(reader);

            // transport x, y, z, orientation
            let _position = Position::parse(reader);

            let _transport_time = reader.read_u32::<LittleEndian>()?;
            let _transport_seat = reader.read_u8()?;

            if movement_flags_extra.contains(MovementFlagsExtra::INTERPOLATED_MOVEMENT) {
                let _transport_time = reader.read_u32::<LittleEndian>()?;
            }
        }

        if movement_flags.contains(MovementFlags::SWIMMING)  ||
            movement_flags.contains(MovementFlags::FLYING) ||
            movement_flags_extra.contains(MovementFlagsExtra::ALWAYS_ALLOW_PITCHING) {
            let _pitch = reader.read_f32::<LittleEndian>()?;
        }

        let fall_time = reader.read_u32::<LittleEndian>()?;

        let mut jump_vertical_speed = 0.0;
        let mut jump_sin_angle = 0.0;
        let mut jump_cos_angle = 0.0;
        let mut jump_horizontal_speed = 0.0;

        if movement_flags.contains(MovementFlags::JUMPING) {
            jump_vertical_speed = reader.read_f32::<LittleEndian>()?;
            jump_sin_angle = reader.read_f32::<LittleEndian>()?;
            jump_cos_angle = reader.read_f32::<LittleEndian>()?;
            jump_horizontal_speed = reader.read_f32::<LittleEndian>()?;
        }

        if movement_flags.contains(MovementFlags::SPLINE_ELEVATION) {
            let _spline_elevation = reader.read_f32::<LittleEndian>()?;
        }

        let movement_info = MovementInfo {
            movement_flags,
            movement_flags_extra,
            time,
            position,
            fall_time,
            jump_info: JumpInfo {
                jump_vertical_speed,
                jump_sin_angle,
                jump_cos_angle,
                jump_horizontal_speed
            },
        };

        Ok(movement_info)
    }
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

impl BinaryConverter for MovementInfo {
    fn write_into(&mut self, _buffer: &mut Vec<u8>) -> Result<(), FieldError> {
        todo!()
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> Result<Self, FieldError> {
        Self::parse(reader).map_err(|e| FieldError::CannotRead(e, "MovementInfo".to_string()))
    }

    fn to_bytes(&self) -> Vec<u8> {
        todo!()
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

bitflags! {
    #[derive(Default, Clone, Debug)]
    pub struct MovementFlags: u32 {
        const NONE = 0x00000000;
        const FORWARD = 0x00000001;
        const BACKWARD = 0x00000002;
        const STRAFE_LEFT = 0x00000004;
        const STRAFE_RIGHT = 0x00000008;
        const LEFT = 0x00000010;
        const RIGHT = 0x00000020;
        const PITCH_UP = 0x00000040;
        const PITCH_DOWN = 0x00000080;
        const WALKING = 0x00000100;
        const TAXI = 0x00000200;
        const DISABLE_GRAVITY = 0x00000400;
        const ROOT = 0x00000800;
        const JUMPING = 0x00001000;
        const FALLING_FAR = 0x00002000;
        const PENDING_STOP = 0x00004000;
        const PENDING_STRAFE_STOP = 0x00008000;
        const PENDING_FORWARD = 0x00010000;
        const PENDING_BACKWARD = 0x00020000;
        const PENDING_STRAFE_LEFT = 0x00040000;
        const PENDING_STRAFE_RIGHT = 0x00080000;
        const PENDING_ROOT = 0x00100000;
        const SWIMMING = 0x00200000;
        const ASCENDING = 0x00400000;
        const DESCENDING = 0x00800000;
        const CAN_FLY = 0x01000000;
        const FLYING = 0x02000000;
        const SPLINE_ELEVATION = 0x04000000;
        const SPLINE_ENABLED = 0x08000000;
        const WATERWALKING = 0x10000000;
        const FALLING_SLOW = 0x20000000;
        const HOVER = 0x40000000;
    }
}

bitflags! {
    #[derive(Default, Clone, Debug)]
    pub struct MovementFlagsExtra: u16 {
        const NONE = 0x00000000;
        const NO_STRAFE = 0x00000001;
        const NO_JUMPING = 0x00000002;
        const UNK3 = 0x00000004;
        const FULL_SPEED_TURNING = 0x00000008;
        const FULL_SPEED_PITCHING = 0x00000010;
        const ALWAYS_ALLOW_PITCHING = 0x00000020;
        const UNK7 = 0x00000040;
        const UNK8 = 0x00000080;
        const UNK9 = 0x00000100;
        const UNK10 = 0x00000200;
        const INTERPOLATED_MOVEMENT = 0x00000400;
        const INTERPOLATED_TURNING = 0x00000800;
        const INTERPOLATED_PITCHING = 0x00001000;
        const UNK14 = 0x00002000;
        const UNK15 = 0x00004000;
        const UNK16 = 0x00008000;
    }
}

#[non_exhaustive]
pub struct UnitMoveType;
impl UnitMoveType {
    pub const MOVE_WALK: u8 = 0;
    pub const MOVE_RUN: u8 = 1;
    pub const MOVE_RUN_BACK: u8 = 2;
    pub const MOVE_SWIM: u8 = 3;
    pub const MOVE_SWIM_BACK: u8 = 4;
    pub const MOVE_TURN_RATE: u8 = 5;
    pub const MOVE_FLIGHT: u8 = 6;
    pub const MOVE_FLIGHT_BACK: u8 = 7;
    pub const MOVE_PITCH_RATE: u8 = 8;
}

bitflags! {
    pub struct SplineFlags: u32 {
        const NONE = 0x00000000;
        const DONE = 0x00000100;
        const FALLING = 0x00000200;
        const NO_SPLINE = 0x00000400;
        const PARABOLIC = 0x00000800;
        const WALKMODE = 0x00001000;
        const FLYING = 0x00002000;
        const ORIENTATION_FIXED = 0x00004000;
        const FINAL_POINT = 0x00008000;
        const FINAL_TARGET = 0x00010000;
        const FINAL_ANGLE = 0x00020000;
        const CATMULLROM = 0x00040000;
        const CYCLIC = 0x00080000;
        const ENTER_CYCLE = 0x00100000;
        const ANIMATION = 0x00200000;
        const FROZEN = 0x00400000;
        const TRANSPORT_ENTER = 0x00800000;
        const TRANSPORT_EXIT = 0x01000000;
        const UNKNOWN7 = 0x02000000;
        const UNKNOWN8 = 0x04000000;
        const ORIENTATION_INVERSED = 0x08000000;
        const UNKNOWN10 = 0x10000000;
        const UNKNOWN11 = 0x20000000;
        const UNKNOWN12 = 0x40000000;
        const UNKNOWN13 = 0x80000000;
    }
}