use anyhow::{Result as AnyResult};
use std::collections::BTreeMap;
use std::io::{BufRead, Cursor};
use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use crate::{BinaryConverter, impl_serialize_for_flags};
use crate::types::custom_fields::PackedGuid;
use crate::types::position::{Point3D, Vector3D};

#[derive(Clone, Default, Debug)]
pub struct Movement {
    pub object_update_flags: ObjectUpdateFlags,
    pub movement_info: Option<MovementInfo>,
    pub high_guid: u32,
    pub low_guid: u32,
    pub target_guid: Option<PackedGuid>,
    pub movement_speed: BTreeMap<u8, f32>,
    pub spline_info: Option<SplineInfo>,
    pub position_info: Option<PositionInfo>,
}

impl BinaryConverter for Movement {
    fn write_into(&mut self, _: &mut Vec<u8>) -> AnyResult<()> {
        todo!()
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
        let mut instance = Self::default();

        instance.object_update_flags = ObjectUpdateFlags::from_bits(
            reader.read_u16::<LittleEndian>()?
        ).unwrap();

        if instance.object_update_flags.contains(ObjectUpdateFlags::SELF)  {}

        if instance.object_update_flags.contains(ObjectUpdateFlags::LIVING) {
            let movement_info = MovementInfo::read_from(reader, &mut vec![])?;

            instance.movement_speed = {
                let mut movement_speed: BTreeMap<u8, f32> = BTreeMap::new();
                for move_type in [
                    UnitMoveType::MOVE_WALK,
                    UnitMoveType::MOVE_RUN,
                    UnitMoveType::MOVE_RUN_BACK,
                    UnitMoveType::MOVE_SWIM,
                    UnitMoveType::MOVE_SWIM_BACK,
                    UnitMoveType::MOVE_FLIGHT,
                    UnitMoveType::MOVE_FLIGHT_BACK,
                    UnitMoveType::MOVE_TURN_RATE,
                    UnitMoveType::MOVE_PITCH_RATE,
                ] {
                    movement_speed.insert(move_type, reader.read_f32::<LittleEndian>()?);
                }

                movement_speed
            };

            if movement_info.movement_flags.contains(MovementFlags::SPLINE_ENABLED) {
                instance.spline_info = Some(SplineInfo::parse(reader)?);
            }

            instance.movement_info = Some(movement_info);

        } else {
            if instance.object_update_flags.contains(ObjectUpdateFlags::POSITION) {
                instance.position_info = Some(PositionInfo::read_from(reader, &mut vec![])?);
            }

            if instance.object_update_flags.contains(ObjectUpdateFlags::STATIONARY_POSITION) {
                let _ = Vector3D::read_from(reader, &mut vec![])?;
            }
        }

        if instance.object_update_flags.contains(ObjectUpdateFlags::LOWGUID) {
            instance.low_guid = reader.read_u32::<LittleEndian>()?;
        }

        if instance.object_update_flags.contains(ObjectUpdateFlags::HIGHGUID) {
            instance.high_guid = reader.read_u32::<LittleEndian>()?;
        }

        if instance.object_update_flags.contains(ObjectUpdateFlags::HAS_TARGET) {
            instance.target_guid = {
                let target_guid = PackedGuid::read_from(reader, &mut vec![])?;
                let PackedGuid(guid) = target_guid;
                if guid == 0 { None } else { Some(target_guid) }
            };
        }

        if instance.object_update_flags.contains(ObjectUpdateFlags::TRANSPORT) {
            let _transport_timer = reader.read_u32::<LittleEndian>()?;
        }

        if instance.object_update_flags.contains(ObjectUpdateFlags::VEHICLE) {
            let _vehicle_id = reader.read_u32::<LittleEndian>()?;
            let _vehicle_orientation = reader.read_f32::<LittleEndian>()?;
        }

        if instance.object_update_flags.contains(ObjectUpdateFlags::ROTATION) {
            let _go_rotation = reader.read_i64::<LittleEndian>()?;
        }

        Ok(instance)
    }

    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

impl Serialize for Movement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        const FIELDS_AMOUNT: usize = 8;
        let mut state = serializer.serialize_struct("Movement", FIELDS_AMOUNT)?;
        state.serialize_field("object_update_flags", &self.object_update_flags)?;
        state.serialize_field("movement_info", &self.movement_info)?;
        state.serialize_field("high_guid", &self.high_guid)?;
        state.serialize_field("low_guid", &self.low_guid)?;
        state.serialize_field("target_guid", &self.target_guid)?;
        state.serialize_field("movement_speed", &self.movement_speed)?;
        state.serialize_field("spline_info", &self.spline_info)?;
        state.serialize_field("position_info", &self.position_info)?;
        state.end()
    }
}

#[derive(Clone, Default, Debug)]
pub struct MovementInfo {
    pub movement_flags: MovementFlags,
    pub movement_extra_flags: MovementExtraFlags,
    pub time: u32,
    pub location: Vector3D,
    pub taxi_info: Option<TaxiInfo>,
    pub fall_time: u32,
    pub jump_info: Option<JumpInfo>,
}

impl BinaryConverter for MovementInfo {
    fn write_into(&mut self, _: &mut Vec<u8>) -> AnyResult<()> {
        todo!()
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
        let mut instance = Self::default();

        instance.movement_flags = MovementFlags::from_bits(
            reader.read_u32::<LittleEndian>()?
        ).unwrap();

        instance.movement_extra_flags = MovementExtraFlags::from_bits(
            reader.read_u16::<LittleEndian>()?
        ).unwrap();

        instance.time = reader.read_u32::<LittleEndian>()?;
        instance.location = Vector3D::read_from(reader, &mut vec![])?;

        if instance.movement_flags.contains(MovementFlags::TAXI) {
            let mut dependencies = instance.movement_extra_flags.bits().to_le_bytes().to_vec();
            instance.taxi_info = Some(TaxiInfo::read_from(reader, &mut dependencies)?);
        }

        if instance.movement_flags.contains(MovementFlags::SWIMMING)  ||
            instance.movement_flags.contains(MovementFlags::FLYING) ||
            instance.movement_extra_flags.contains(MovementExtraFlags::ALWAYS_ALLOW_PITCHING) {
            let _pitch = reader.read_f32::<LittleEndian>()?;
        }

        instance.fall_time = reader.read_u32::<LittleEndian>()?;

        if instance.movement_flags.contains(MovementFlags::JUMPING) {
            instance.jump_info = Some(JumpInfo::parse(reader)?);
        }

        if instance.movement_flags.contains(MovementFlags::SPLINE_ELEVATION) {
            let _ = reader.read_f32::<LittleEndian>()?;
        }

        Ok(instance)
    }

    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

impl Serialize for MovementInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        const FIELDS_AMOUNT: usize = 7;
        let mut state = serializer.serialize_struct("MovementInfo", FIELDS_AMOUNT)?;
        state.serialize_field("movement_flags", &self.movement_flags)?;
        state.serialize_field("movement_flags_extra", &self.movement_extra_flags)?;
        state.serialize_field("time", &self.time)?;
        state.serialize_field("location", &self.location)?;
        state.serialize_field("taxi_info", &self.taxi_info)?;
        state.serialize_field("fall_time", &self.fall_time)?;
        state.serialize_field("jump_info", &self.jump_info)?;
        state.end()
    }
}

#[derive(Clone, Default, Debug)]
pub struct JumpInfo {
    pub vertical_speed: f32,
    pub sin_angle: f32,
    pub cos_angle: f32,
    pub horizontal_speed: f32,
}

impl JumpInfo {
    pub fn parse<R: BufRead>(reader: &mut R) -> AnyResult<Self> {
        let vertical_speed = reader.read_f32::<LittleEndian>()?;
        let sin_angle = reader.read_f32::<LittleEndian>()?;
        let cos_angle = reader.read_f32::<LittleEndian>()?;
        let horizontal_speed = reader.read_f32::<LittleEndian>()?;

        Ok(Self {
            vertical_speed,
            sin_angle,
            cos_angle,
            horizontal_speed,
        })
    }
}

impl Serialize for JumpInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        const FIELDS_AMOUNT: usize = 4;
        let mut state = serializer.serialize_struct("JumpInfo", FIELDS_AMOUNT)?;
        state.serialize_field("vertical_speed", &self.vertical_speed)?;
        state.serialize_field("sin_angle", &self.sin_angle)?;
        state.serialize_field("cos_angle", &self.cos_angle)?;
        state.serialize_field("horizontal_speed", &self.horizontal_speed)?;
        state.end()
    }
}

#[derive(Clone, Default, Debug)]
pub struct SplineInfo {
    pub spline_flags: SplineFlags,
}

impl SplineInfo {
    pub fn parse<R: BufRead>(reader: &mut R) -> AnyResult<Self> {
        let mut instance = Self::default();

        let spline_flags = SplineFlags::from_bits(
            reader.read_u32::<LittleEndian>()?
        ).unwrap_or(SplineFlags::NONE);

        instance.spline_flags = spline_flags;

        if spline_flags.contains(SplineFlags::FINAL_ANGLE) {
            let _spline_facing_angle = reader.read_f32::<LittleEndian>()?;
        }

        if spline_flags.contains(SplineFlags::FINAL_TARGET) {
            let _spline_facing_target_guid = reader.read_u64::<LittleEndian>()?;
        }

        if spline_flags.contains(SplineFlags::FINAL_POINT) {
            let _spline_facing_point = Point3D::read_from(reader, &mut vec![])?;
        }

        let _ = reader.read_u32::<LittleEndian>()?;
        let _ = reader.read_u32::<LittleEndian>()?;
        let _ = reader.read_u32::<LittleEndian>()?;

        let _ = reader.read_u32::<LittleEndian>()?;
        let _ = reader.read_u32::<LittleEndian>()?;
        let _ = reader.read_u32::<LittleEndian>()?;
        let _ = reader.read_u32::<LittleEndian>()?;

        let spline_amount = reader.read_u32::<LittleEndian>()?;

        for _ in 0..spline_amount {
            let _spline_point = Point3D::read_from(reader, &mut vec![])?;
        }

        let _spline_evaluation_mode = reader.read_u8()?;
        let _spline_end_point = Point3D::read_from(reader, &mut vec![])?;

        Ok(instance)
    }
}

impl Serialize for SplineInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        const FIELDS_AMOUNT: usize = 1;
        let mut state = serializer.serialize_struct("SplineInfo", FIELDS_AMOUNT)?;
        state.serialize_field("spline_flags", &self.spline_flags)?;
        state.end()
    }
}

#[derive(Clone, Default, Debug)]
pub struct TaxiInfo {
    pub guid: PackedGuid,
    pub location: Vector3D,
    pub time: u32,
    pub seat: u8,
    pub time2: Option<u32>,
}

impl BinaryConverter for TaxiInfo {
    fn write_into(&mut self, _: &mut Vec<u8>) -> AnyResult<()> {
        todo!()
    }

    fn read_from<R: BufRead>(reader: &mut R, dependencies: &mut Vec<u8>) -> AnyResult<Self> {
        let mut instance = Self::default();

        instance.guid = PackedGuid::read_from(reader, &mut vec![])?;
        instance.location = Vector3D::read_from(reader, &mut vec![])?;
        instance.time = reader.read_u32::<LittleEndian>()?;
        instance.seat = reader.read_u8()?;

        let mut deps_reader = Cursor::new(&dependencies);
        let extra_flags = deps_reader.read_u16::<LittleEndian>()?;

        if extra_flags == MovementExtraFlags::INTERPOLATED_MOVEMENT.bits() {
            instance.time2 = Some(reader.read_u32::<LittleEndian>()?);
        }

        Ok(instance)
    }

    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

impl Serialize for TaxiInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        const FIELDS_AMOUNT: usize = 5;
        let mut state = serializer.serialize_struct("TaxiInfo", FIELDS_AMOUNT)?;
        state.serialize_field("guid", &self.guid)?;
        state.serialize_field("location", &self.location)?;
        state.serialize_field("time", &self.time)?;
        state.serialize_field("seat", &self.seat)?;
        state.serialize_field("time2", &self.time2)?;
        state.end()
    }
}

#[derive(Clone, Default, Debug)]
pub struct PositionInfo {
    pub transport_guid: Option<PackedGuid>,
    pub location: Vector3D,
}

impl BinaryConverter for PositionInfo {
    fn write_into(&mut self, _: &mut Vec<u8>) -> AnyResult<()> {
        todo!()
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
        let transport_guid = PackedGuid::read_from(reader, &mut vec![])?;
        let _ = Point3D::read_from(reader, &mut vec![])?;
        let location = Vector3D::read_from(reader, &mut vec![])?;
        let _ = reader.read_f32::<LittleEndian>()?;

        Ok(Self {
            transport_guid: {
                let PackedGuid(guid) = transport_guid;
                if guid == 0 { None } else { Some(transport_guid) }
            },
            location,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

impl Serialize for PositionInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        const FIELDS_AMOUNT: usize = 2;
        let mut state = serializer.serialize_struct("PositionInfo", FIELDS_AMOUNT)?;
        state.serialize_field("transport_guid", &self.transport_guid)?;
        state.serialize_field("location", &self.location)?;
        state.end()
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
    #[derive(Copy, Clone, Debug)]
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

impl Default for MovementFlags {
    fn default() -> Self {
        Self::NONE
    }
}

impl_serialize_for_flags!(MovementFlags);

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct MovementExtraFlags: u16 {
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

impl Default for MovementExtraFlags {
    fn default() -> Self {
        Self::NONE
    }
}

impl_serialize_for_flags!(MovementExtraFlags);

bitflags! {
    #[derive(Copy, Clone, Debug)]
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

impl Default for SplineFlags {
    fn default() -> Self {
        Self::NONE
    }
}

impl_serialize_for_flags!(SplineFlags);

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct ObjectUpdateFlags: u16 {
        const NONE = 0x0000;
        const SELF = 0x0001;
        const TRANSPORT = 0x0002;
        const HAS_TARGET = 0x0004;
        const HIGHGUID = 0x0008;
        const LOWGUID = 0x0010;
        const LIVING = 0x0020;
        const STATIONARY_POSITION = 0x0040;
        const VEHICLE = 0x0080;
        const POSITION = 0x0100;
        const ROTATION = 0x0200;
    }
}

impl Default for ObjectUpdateFlags {
    fn default() -> Self {
        Self::NONE
    }
}

impl_serialize_for_flags!(ObjectUpdateFlags);