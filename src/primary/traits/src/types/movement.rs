use anyhow::{Result as AnyResult};
use std::collections::BTreeMap;
use std::io::{BufRead, Cursor};
use bitflags::{bitflags};
use byteorder::{LittleEndian, ReadBytesExt};
use serde::{Serialize, Serializer};

use crate::{BinaryConverter, impl_serialize_for_flags};
use crate::types::custom_fields::PackedGuid;
use crate::types::position::{Point3D, Vector3D};

#[derive(Serialize, Clone, Default, Debug, PartialEq)]
pub struct Movement {
    pub object_update_flags: ObjectUpdateFlags,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement_info: Option<MovementInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_guid: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_guid: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_guid: Option<PackedGuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_timer: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_orientation: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_object_rotation: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement_speed: Option<BTreeMap<u8, f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spline_info: Option<SplineInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_info: Option<PositionInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_object_position: Option<Vector3D>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_object_position: Option<Vector3D>,
}

impl Movement {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }

    pub fn set_movement_info(&mut self, movement_info: MovementInfo) {
        self.object_update_flags.set(ObjectUpdateFlags::LIVING, true);
        self.movement_info = Some(movement_info);
    }

    pub fn set_position_info(&mut self, position_info: PositionInfo) {
        self.object_update_flags.set(ObjectUpdateFlags::LIVING, false);
        self.object_update_flags.set(ObjectUpdateFlags::POSITION, true);
        self.position_info = Some(position_info);
    }

    pub fn set_stationary_position(&mut self, vector: Vector3D, is_transport: bool) {
        self.object_update_flags.set(ObjectUpdateFlags::LIVING, false);
        self.object_update_flags.set(ObjectUpdateFlags::STATIONARY_POSITION, true);

        if is_transport {
            self.object_update_flags.set(ObjectUpdateFlags::TRANSPORT, true);
            self.game_object_position = Some(vector);
        } else {
            self.world_object_position = Some(vector);
        }
    }

    pub fn set_high_guid(&mut self, high_guid: u32) {
        self.object_update_flags.set(ObjectUpdateFlags::HIGHGUID, true);
        self.high_guid = Some(high_guid);
    }

    pub fn set_low_guid(&mut self, low_guid: u32) {
        self.object_update_flags.set(ObjectUpdateFlags::LOWGUID, true);
        self.low_guid = Some(low_guid);
    }

    pub fn set_target_guid(&mut self, target_guid: u64) {
        self.object_update_flags.set(ObjectUpdateFlags::HAS_TARGET, true);
        self.target_guid = Some(PackedGuid(target_guid));
    }

    pub fn set_transport_timer(&mut self, transport_timer: u32) {
        self.object_update_flags.set(ObjectUpdateFlags::TRANSPORT, true);
        self.transport_timer = Some(transport_timer);
    }

    pub fn set_vehicle(&mut self, vehicle_id: u32, vehicle_orientation: f32) {
        self.object_update_flags.set(ObjectUpdateFlags::VEHICLE, true);
        self.vehicle_id = Some(vehicle_id);
        self.vehicle_orientation = Some(vehicle_orientation);
    }

    pub fn set_game_object_rotation(&mut self, game_object_rotation: i64) {
        self.object_update_flags.set(ObjectUpdateFlags::ROTATION, true);
        self.game_object_rotation = Some(game_object_rotation);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use anyhow::{Result as AnyResult};
    use crate::BinaryConverter;
    use crate::types::movement::{Movement, MovementExtraFlags, MovementFlags, MovementInfo, ObjectUpdateFlags, UnitMoveType};
    use crate::types::position::Vector3D;

    #[test]
    fn test_movement_building() -> AnyResult<()> {
        let mut movement = Movement::default();
        let movement_info = MovementInfo {
            movement_flags: MovementFlags::NONE,
            movement_extra_flags: MovementExtraFlags::NONE,
            time: 0,
            location: Default::default(),
            taxi_info: None,
            fall_time: 0,
            jump_info: None,
        };

        movement.set_movement_info(movement_info);
        movement.movement_speed = {
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
                movement_speed.insert(move_type, 10.);
            }

            Some(movement_speed)
        };

        let mut buffer = vec![];
        movement.write_into(&mut buffer)?;

        let mut reader = std::io::Cursor::new(&buffer);
        let movement = Movement::read_from(&mut reader, &mut vec![])?;

        assert_eq!(movement.movement_speed.is_some(), true);
        if let Some(movement_speed) = movement.movement_speed {
            assert_eq!(movement_speed.get(&UnitMoveType::MOVE_FLIGHT), Some(&10.));
        }

        assert_eq!(movement.movement_info.is_some(), true);
        if let Some(movement_info) = movement.movement_info {
            assert_eq!(movement_info.movement_flags, MovementFlags::NONE);
            assert_eq!(movement_info.movement_extra_flags, MovementExtraFlags::NONE);
            assert_eq!(movement_info.time, 0);
            assert_eq!(movement_info.location, Vector3D::default());
        }

        assert_eq!(movement.object_update_flags.contains(ObjectUpdateFlags::LIVING), true);

        Ok(())
    }

    #[test]
    fn test_movement_low_guid() -> AnyResult<()> {
        const LOW_GUID: u32 = 123;

        let mut movement = Movement::default();
        movement.set_low_guid(LOW_GUID);

        assert_eq!(movement.low_guid.is_some(), true);
        assert_eq!(movement.low_guid, Some(LOW_GUID));

        let mut buffer = vec![];
        movement.write_into(&mut buffer)?;

        let mut reader = std::io::Cursor::new(&buffer);
        let movement = Movement::read_from(&mut reader, &mut vec![])?;

        assert_eq!(movement.low_guid.is_some(), true);
        assert_eq!(movement.low_guid, Some(LOW_GUID));
        assert_eq!(movement.object_update_flags.contains(ObjectUpdateFlags::LOWGUID), true);

        Ok(())
    }

    #[test]
    fn test_movement_high_guid() -> AnyResult<()> {
        const HIGH_GUID: u32 = 123;

        let mut movement = Movement::default();
        movement.set_high_guid(HIGH_GUID);

        assert_eq!(movement.high_guid.is_some(), true);
        assert_eq!(movement.high_guid, Some(HIGH_GUID));

        let mut buffer = vec![];
        movement.write_into(&mut buffer)?;

        let mut reader = std::io::Cursor::new(&buffer);
        let movement = Movement::read_from(&mut reader, &mut vec![])?;

        assert_eq!(movement.high_guid.is_some(), true);
        assert_eq!(movement.high_guid, Some(HIGH_GUID));
        assert_eq!(movement.object_update_flags.contains(ObjectUpdateFlags::HIGHGUID), true);

        Ok(())
    }
}

impl BinaryConverter for Movement {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        self.object_update_flags.bits().write_into(buffer)?;

        if let Some(mut movement_info) = self.movement_info {
            movement_info.write_into(buffer)?;
        }

        if let Some(movement_speed) = self.movement_speed.clone() {
            let mut speed_info: Vec<f32> = movement_speed.values().copied().collect();
            speed_info.write_into(buffer)?;
        }

        if let Some(mut spline_info) = self.spline_info.clone() {
            spline_info.write_into(buffer)?;
        }

        if let Some(mut position_info) = self.position_info {
            position_info.write_into(buffer)?;
        }

        if let Some(mut position) = self.game_object_position {
            position.write_into(buffer)?;
        } else if let Some(mut position) = self.world_object_position {
            position.write_into(buffer)?;
        }

        if let Some(mut low_guid) = self.low_guid {
            low_guid.write_into(buffer)?;
        }

        if let Some(mut high_guid) = self.high_guid {
            high_guid.write_into(buffer)?;
        }

        if let Some(mut target_guid) = self.target_guid {
            target_guid.write_into(buffer)?;
        }

        Ok(())
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

                Some(movement_speed)
            };

            if movement_info.movement_flags.contains(MovementFlags::SPLINE_ENABLED) {
                instance.spline_info = Some(SplineInfo::read_from(reader, &mut vec![])?);
            }

            instance.movement_info = Some(movement_info);

        } else {
            if instance.object_update_flags.contains(ObjectUpdateFlags::POSITION) {
                instance.position_info = Some(PositionInfo::read_from(reader, &mut vec![])?);
            } else if instance.object_update_flags.contains(ObjectUpdateFlags::STATIONARY_POSITION) {
                let stationary_position = Vector3D::read_from(reader, &mut vec![])?;

                if instance.object_update_flags.contains(ObjectUpdateFlags::TRANSPORT) {
                    instance.game_object_position = Some(stationary_position);
                } else {
                    instance.world_object_position = Some(stationary_position);
                }
            }
        }

        if instance.object_update_flags.contains(ObjectUpdateFlags::LOWGUID) {
            let low_guid = reader.read_u32::<LittleEndian>()?;
            instance.low_guid = Some(low_guid);
        }

        if instance.object_update_flags.contains(ObjectUpdateFlags::HIGHGUID) {
            let high_guid = reader.read_u32::<LittleEndian>()?;
            instance.high_guid = Some(high_guid);
        }

        if instance.object_update_flags.contains(ObjectUpdateFlags::HAS_TARGET) {
            instance.target_guid = {
                let target_guid = PackedGuid::read_from(reader, &mut vec![])?;
                let PackedGuid(guid) = target_guid;
                if guid == 0 { None } else { Some(target_guid) }
            };
        }

        if instance.object_update_flags.contains(ObjectUpdateFlags::TRANSPORT) {
            let transport_timer = reader.read_u32::<LittleEndian>()?;
            instance.transport_timer = Some(transport_timer);
        }

        if instance.object_update_flags.contains(ObjectUpdateFlags::VEHICLE) {
            let vehicle_id = reader.read_u32::<LittleEndian>()?;
            instance.vehicle_id = Some(vehicle_id);
            let vehicle_orientation = reader.read_f32::<LittleEndian>()?;
            instance.vehicle_orientation = Some(vehicle_orientation);
        }

        if instance.object_update_flags.contains(ObjectUpdateFlags::ROTATION) {
            let go_rotation = reader.read_i64::<LittleEndian>()?;
            instance.game_object_rotation = Some(go_rotation);
        }

        Ok(instance)
    }
}

#[derive(Serialize, Clone, Default, Debug, Copy, PartialEq)]
pub struct MovementInfo {
    #[serde(skip_serializing_if = "MovementFlags::is_empty")]
    pub movement_flags: MovementFlags,
    #[serde(skip_serializing_if = "MovementExtraFlags::is_empty")]
    pub movement_extra_flags: MovementExtraFlags,
    pub time: u32,
    pub location: Vector3D,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxi_info: Option<TaxiInfo>,
    pub fall_time: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jump_info: Option<JumpInfo>,
}

impl MovementInfo {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

impl BinaryConverter for MovementInfo {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        self.movement_flags.bits().write_into(buffer)?;
        self.movement_extra_flags.bits().write_into(buffer)?;
        self.time.write_into(buffer)?;
        self.location.write_into(buffer)?;

        if let Some(mut taxi_info) = self.taxi_info {
            taxi_info.write_into(buffer)?;
        }

        self.fall_time.write_into(buffer)?;

        if let Some(mut jump_info) = self.jump_info {
            jump_info.write_into(buffer)?;
        }

        Ok(())
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
            instance.jump_info = Some(JumpInfo::read_from(reader, &mut vec![])?);
        }

        if instance.movement_flags.contains(MovementFlags::SPLINE_ELEVATION) {
            let _ = reader.read_f32::<LittleEndian>()?;
        }

        Ok(instance)
    }
}

#[derive(Serialize, Clone, Default, Debug, Copy, PartialEq)]
pub struct JumpInfo {
    pub vertical_speed: f32,
    pub sin_angle: f32,
    pub cos_angle: f32,
    pub horizontal_speed: f32,
}

impl BinaryConverter for JumpInfo {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        self.vertical_speed.write_into(buffer)?;
        self.sin_angle.write_into(buffer)?;
        self.cos_angle.write_into(buffer)?;
        self.horizontal_speed.write_into(buffer)?;

        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
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

#[derive(Serialize, Clone, Default, Debug, PartialEq)]
pub struct SplineInfo {
    pub spline_flags: SplineFlags,
    pub facing_angle: Option<f32>,
    pub facing_target_guid: Option<u64>,
    pub facing_point: Option<Point3D>,
    pub time_passed: u32,
    pub duration: u32,
    pub spline_id: u32,
    pub duration_mod: f32,
    pub duration_mod_next: f32,
    pub vertical_acceleration: f32,
    pub parabolic_start_time: i32,
    pub nodes_count: u32,
    pub path: Vec<Point3D>,
    pub evaluation_mode: u8,
    pub final_destination: Point3D,
}

impl BinaryConverter for SplineInfo {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        self.spline_flags.bits().write_into(buffer)?;

        if let Some(mut value) = self.facing_angle {
            value.write_into(buffer)?;
        }

        if let Some(mut value) = self.facing_target_guid {
            value.write_into(buffer)?;
        }

        if let Some(mut point) = self.facing_point {
            point.write_into(buffer)?;
        }

        self.time_passed.write_into(buffer)?;
        self.duration.write_into(buffer)?;
        self.spline_id.write_into(buffer)?;
        self.duration_mod.write_into(buffer)?;
        self.duration_mod_next.write_into(buffer)?;
        self.vertical_acceleration.write_into(buffer)?;
        self.parabolic_start_time.write_into(buffer)?;
        self.nodes_count.write_into(buffer)?;
        self.path.write_into(buffer)?;
        self.evaluation_mode.write_into(buffer)?;
        self.final_destination.write_into(buffer)?;

        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
        let mut instance = Self::default();

        let spline_flags = SplineFlags::from_bits(
            reader.read_u32::<LittleEndian>()?
        ).unwrap_or(SplineFlags::NONE);

        instance.spline_flags = spline_flags;

        if spline_flags.contains(SplineFlags::FINAL_ANGLE) {
            let facing_angle = reader.read_f32::<LittleEndian>()?;
            instance.facing_angle = Some(facing_angle);
        }

        if spline_flags.contains(SplineFlags::FINAL_TARGET) {
            let facing_target_guid = reader.read_u64::<LittleEndian>()?;
            instance.facing_target_guid = Some(facing_target_guid);
        }

        if spline_flags.contains(SplineFlags::FINAL_POINT) {
            let facing_point = Point3D::read_from(reader, &mut vec![])?;
            instance.facing_point = Some(facing_point);
        }

        instance.time_passed = reader.read_u32::<LittleEndian>()?;
        instance.duration = reader.read_u32::<LittleEndian>()?;
        instance.spline_id = reader.read_u32::<LittleEndian>()?;

        instance.duration_mod = reader.read_f32::<LittleEndian>()?;
        instance.duration_mod_next = reader.read_f32::<LittleEndian>()?;
        instance.vertical_acceleration = reader.read_f32::<LittleEndian>()?;
        instance.parabolic_start_time = reader.read_i32::<LittleEndian>()?;

        let nodes_count = reader.read_u32::<LittleEndian>()?;

        let mut path: Vec<Point3D> = vec![];
        for _ in 0..nodes_count {
            let point = Point3D::read_from(reader, &mut vec![])?;
            path.push(point);
        }

        instance.evaluation_mode = reader.read_u8()?;
        instance.final_destination = Point3D::read_from(reader, &mut vec![])?;

        Ok(instance)
    }
}

#[derive(Serialize, Clone, Default, Debug, Copy, PartialEq)]
pub struct TaxiInfo {
    pub guid: PackedGuid,
    pub location: Vector3D,
    pub time: u32,
    pub seat: u8,
    pub time2: Option<u32>,
}

impl BinaryConverter for TaxiInfo {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        self.guid.write_into(buffer)?;
        self.location.write_into(buffer)?;
        self.time.write_into(buffer)?;
        self.seat.write_into(buffer)?;

        if let Some(mut value) = self.time2 {
            value.write_into(buffer)?;
        }

        Ok(())
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
}

#[derive(Serialize, Clone, Default, Debug, Copy, PartialEq)]
pub struct PositionInfo {
    pub transport_guid: PackedGuid,
    pub world_object_point: Point3D,
    pub location: Vector3D,
    pub corpse_direction: f32,
}

impl BinaryConverter for PositionInfo {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        self.transport_guid.write_into(buffer)?;
        self.world_object_point.write_into(buffer)?;
        self.location.write_into(buffer)?;
        self.corpse_direction.write_into(buffer)?;

        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
        let transport_guid = PackedGuid::read_from(reader, &mut vec![])?;
        let world_object_point = Point3D::read_from(reader, &mut vec![])?;
        // according to mangos, when transport guid exists, location contain transport offset
        let location = Vector3D::read_from(reader, &mut vec![])?;
        let corpse_direction = reader.read_f32::<LittleEndian>()?;

        Ok(Self {
            transport_guid,
            world_object_point,
            location,
            corpse_direction,
        })
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
    #[derive(Copy, Clone, Debug, PartialEq)]
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
    #[derive(Copy, Clone, Debug, PartialEq)]
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
    #[derive(Copy, Clone, Debug, PartialEq)]
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
    #[derive(Copy, Clone, Debug, PartialEq)]
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