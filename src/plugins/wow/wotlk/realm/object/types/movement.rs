use binrw::{BinRead, BinResult, BinWrite, Endian};
use bitflags::bitflags;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::Serialize;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};

use crate::client::prelude::*;
use crate::plugins::wow::wotlk::realm::object::types::packed_guid::PackedGuid;

#[derive(BinRead, BinWrite, FieldsMetadata, Serialize, Default, PartialEq, Debug)]
pub struct Movement {
    pub object_update_flags: ObjectUpdateFlags,
    #[br(if(object_update_flags.contains(ObjectUpdateFlags::LIVING)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement_info: Option<MovementInfo>,
    #[br(if(object_update_flags.contains(ObjectUpdateFlags::LIVING)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement_speed: Option<MovementSpeed>,
    #[br(if(
        object_update_flags.contains(ObjectUpdateFlags::LIVING) &&
        movement_info
            .as_ref()
            .map(|m| m.movement_flags.contains(MovementFlags::SPLINE_ENABLED))
            .unwrap_or(false)
    ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spline_info: Option<SplineInfo>,
    #[br(if(
        !object_update_flags.contains(ObjectUpdateFlags::LIVING) &&
        object_update_flags.contains(ObjectUpdateFlags::POSITION)
    ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_info: Option<PositionInfo>,

    #[br(if(
        !object_update_flags.contains(ObjectUpdateFlags::LIVING) &&
        object_update_flags.contains(ObjectUpdateFlags::HAS_POSITION) &&
        !object_update_flags.contains(ObjectUpdateFlags::POSITION) &&
        object_update_flags.contains(ObjectUpdateFlags::TRANSPORT)
    ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_object_position: Option<OrientedPoint3D>,

    #[br(if(
        !object_update_flags.contains(ObjectUpdateFlags::LIVING) &&
        object_update_flags.contains(ObjectUpdateFlags::HAS_POSITION) &&
        !object_update_flags.contains(ObjectUpdateFlags::POSITION) &&
        !object_update_flags.contains(ObjectUpdateFlags::TRANSPORT)
    ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_object_position: Option<OrientedPoint3D>,

    #[br(if(object_update_flags.contains(ObjectUpdateFlags::LOWGUID)))]
    pub low_guid: Option<u32>,

    #[br(if(object_update_flags.contains(ObjectUpdateFlags::HIGHGUID)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_guid: Option<u32>,

    #[br(if(object_update_flags.contains(ObjectUpdateFlags::HAS_ATTACKING_TARGET)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_guid: Option<PackedGuid>,

    #[br(if(object_update_flags.contains(ObjectUpdateFlags::TRANSPORT)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_timer: Option<u32>,

    #[br(if(object_update_flags.contains(ObjectUpdateFlags::VEHICLE)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_id: Option<u32>,

    #[br(if(object_update_flags.contains(ObjectUpdateFlags::VEHICLE)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_orientation: Option<f32>,

    #[br(if(object_update_flags.contains(ObjectUpdateFlags::ROTATION)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_object_rotation: Option<i64>,
}

impl Movement {
    /// Merge a partial UPDATE_OBJECT movement block into the accumulated object state.
    pub(crate) fn merge_from(&mut self, incoming: Movement) {
        let flags = incoming.object_update_flags;
        self.object_update_flags |= flags;

        if flags.contains(ObjectUpdateFlags::LIVING) {
            self.movement_info = incoming.movement_info;
            self.movement_speed = incoming.movement_speed;
            self.spline_info = incoming.spline_info;
            self.position_info = None;
            self.game_object_position = None;
            self.world_object_position = None;
        } else if flags.contains(ObjectUpdateFlags::POSITION) {
            self.position_info = incoming.position_info;
            self.game_object_position = None;
            self.world_object_position = None;
        } else if flags.contains(ObjectUpdateFlags::HAS_POSITION) {
            self.position_info = None;
            if flags.contains(ObjectUpdateFlags::TRANSPORT) {
                self.game_object_position = incoming.game_object_position;
                self.world_object_position = None;
            } else {
                self.world_object_position = incoming.world_object_position;
                self.game_object_position = None;
            }
        }

        if flags.contains(ObjectUpdateFlags::LOWGUID) {
            self.low_guid = incoming.low_guid;
        }
        if flags.contains(ObjectUpdateFlags::HIGHGUID) {
            self.high_guid = incoming.high_guid;
        }
        if flags.contains(ObjectUpdateFlags::HAS_ATTACKING_TARGET) {
            self.target_guid = incoming.target_guid;
        }
        if flags.contains(ObjectUpdateFlags::TRANSPORT) {
            self.transport_timer = incoming.transport_timer;
        }
        if flags.contains(ObjectUpdateFlags::VEHICLE) {
            self.vehicle_id = incoming.vehicle_id;
            self.vehicle_orientation = incoming.vehicle_orientation;
        }
        if flags.contains(ObjectUpdateFlags::ROTATION) {
            self.game_object_rotation = incoming.game_object_rotation;
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct MovementSpeed(pub BTreeMap<UnitMoveType, f32>);

const MOVE_ORDER: [UnitMoveType; 9] = [
    UnitMoveType::Walk,
    UnitMoveType::Run,
    UnitMoveType::RunBack,
    UnitMoveType::Swim,
    UnitMoveType::SwimBack,
    UnitMoveType::Flight,
    UnitMoveType::FlightBack,
    UnitMoveType::TurnRate,
    UnitMoveType::PitchRate,
];

impl BinRead for MovementSpeed {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<Self> {
        let mut map = BTreeMap::new();
        for key in MOVE_ORDER {
            let v = f32::read_options(reader, endian, ())?;
            map.insert(key, v);
        }
        Ok(Self(map))
    }
}

impl BinWrite for MovementSpeed {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<()> {
        for key in MOVE_ORDER {
            let v = *self.0.get(&key).unwrap_or(&0.0);
            f32::write_options(&v, writer, endian, ())?;
        }
        Ok(())
    }
}

impl CalculateMetadata for MovementSpeed {
    fn calculate<'a>(&self, ctx: &'a mut MetadataContext) -> &'a mut MetadataContext {
        let prev = ctx.current_key.clone();

        for key in MOVE_ORDER {
            if let Some(v) = self.0.get(&key) {
                ctx.current_key = format!("{}/{}", prev, key);
                v.calculate(ctx);
            }
        }

        ctx.current_key = prev;
        ctx
    }
}

#[derive(BinRead, BinWrite, FieldsMetadata, Serialize, PartialEq, Debug)]
pub struct MovementInfo {
    pub movement_flags: MovementFlags,
    pub movement_extra_flags: MovementExtraFlags,
    pub time: u32,
    pub location: OrientedPoint3D,
    #[br(if(movement_flags.contains(MovementFlags::TAXI)), args(movement_extra_flags))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxi_info: Option<TaxiInfo>,
    pub fall_time: u32,
    #[br(if(movement_flags.contains(MovementFlags::JUMPING)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jump_info: Option<JumpInfo>,
}

#[derive(BinRead, BinWrite, FieldsMetadata, Serialize, PartialEq, Debug)]
pub struct JumpInfo {
    pub vertical_speed: f32,
    pub sin_angle: f32,
    pub cos_angle: f32,
    pub horizontal_speed: f32,
}

#[derive(BinRead, BinWrite, FieldsMetadata, Serialize, PartialEq, Debug, Default)]
pub struct SplineInfo {
    pub spline_flags: SplineFlags,
    #[br(parse_with = SplineInfo::read_facing, args(spline_flags))]
    #[bw(write_with = SplineInfo::write_facing)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facing: Option<SplineFacing>,
    pub time_passed: u32,
    pub duration: u32,
    pub spline_id: u32,
    pub duration_mod: f32,
    pub duration_mod_next: f32,
    pub vertical_acceleration: f32,
    pub parabolic_start_time: i32,
    pub nodes_count: u32,
    #[br(count = nodes_count)]
    pub path: Vec<Point3D>,
    pub evaluation_mode: u8,
    pub final_destination: Point3D,
}

#[derive(Debug, Serialize, PartialEq)]
pub enum SplineFacing {
    Angle(f32),
    Target(u64),
    Point(Point3D),
}

impl SplineInfo {
    fn read_facing<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        (flags,): (SplineFlags,),
    ) -> BinResult<Option<SplineFacing>> {
        if flags.contains(SplineFlags::FINAL_ANGLE) {
            Ok(Some(SplineFacing::Angle(f32::read_options(
                reader,
                endian,
                (),
            )?)))
        } else if flags.contains(SplineFlags::FINAL_TARGET) {
            Ok(Some(SplineFacing::Target(u64::read_options(
                reader,
                endian,
                (),
            )?)))
        } else if flags.contains(SplineFlags::FINAL_POINT) {
            Ok(Some(SplineFacing::Point(Point3D::read_options(
                reader,
                endian,
                (),
            )?)))
        } else {
            Ok(None)
        }
    }

    fn write_facing<W: Write + Seek>(
        facing: &Option<SplineFacing>,
        writer: &mut W,
        endian: Endian,
        _: (),
    ) -> BinResult<()> {
        if let Some(facing) = facing {
            match facing {
                SplineFacing::Angle(v) => f32::write_options(v, writer, endian, ()),
                SplineFacing::Target(v) => u64::write_options(v, writer, endian, ()),
                SplineFacing::Point(p) => Point3D::write_options(p, writer, endian, ()),
            }
        } else {
            Ok(())
        }
    }
}

impl CalculateMetadata for SplineFacing {
    fn calculate<'a>(&self, ctx: &'a mut MetadataContext) -> &'a mut MetadataContext {
        let size = match self {
            SplineFacing::Angle(_) => size_of::<f32>(),
            SplineFacing::Target(_) => size_of::<u64>(),
            SplineFacing::Point(_) => size_of::<Point3D>(),
        };

        ctx.metadata.insert(
            ctx.current_key.clone(),
            MetadataValue {
                size,
                offset: ctx.offset,
            },
        );

        ctx.offset += size;
        ctx
    }
}

#[derive(BinRead, BinWrite, FieldsMetadata, Serialize, PartialEq, Debug)]
#[br(import(extra_flags: MovementExtraFlags))]
pub struct TaxiInfo {
    pub guid: PackedGuid,
    pub location: OrientedPoint3D,
    pub time: u32,
    pub seat: u8,
    #[br(if(extra_flags.contains(MovementExtraFlags::INTERPOLATED_MOVEMENT)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time2: Option<u32>,
}

#[derive(BinRead, BinWrite, FieldsMetadata, Serialize, PartialEq, Debug)]
pub struct PositionInfo {
    pub transport_guid: PackedGuid,
    pub world_object_point: Point3D,
    pub location: OrientedPoint3D,
    pub corpse_direction: f32,
}

#[derive(BinRead, BinWrite, FieldsMetadata, Serialize, PartialEq, Copy, Clone, Default, Debug)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(BinRead, BinWrite, FieldsMetadata, Serialize, PartialEq, Debug)]
pub struct OrientedPoint3D {
    pub point: Point3D,
    pub direction: f32,
}

#[derive(
    BinRead,
    BinWrite,
    IntoPrimitive,
    TryFromPrimitive,
    Serialize,
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Ord,
    PartialOrd,
)]
#[repr(u8)]
#[br(repr = u8)]
#[bw(repr = u8)]
pub enum UnitMoveType {
    Walk = 0,
    Run = 1,
    RunBack = 2,
    Swim = 3,
    SwimBack = 4,
    TurnRate = 5,
    Flight = 6,
    FlightBack = 7,
    PitchRate = 8,
}

impl core::fmt::Display for UnitMoveType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use UnitMoveType::*;
        let s = match self {
            Walk => "Walk",
            Run => "Run",
            RunBack => "RunBack",
            Swim => "Swim",
            SwimBack => "SwimBack",
            TurnRate => "TurnRate",
            Flight => "Flight",
            FlightBack => "FlightBack",
            PitchRate => "PitchRate",
        };
        f.write_str(s)
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, BitflagExtras)]
    #[bitflags_repr(u32)]
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

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, BitflagExtras)]
    #[bitflags_repr(u16)]
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

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, BitflagExtras)]
    #[bitflags_repr(u32)]
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

impl SplineFlags {
    #[inline]
    pub fn is_catmull_rom(self) -> bool {
        self.intersects(SplineFlags::FLYING | SplineFlags::CATMULLROM)
    }
}

impl Default for SplineFlags {
    fn default() -> Self {
        Self::NONE
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, BitflagExtras)]
    #[bitflags_repr(u16)]
    pub struct ObjectUpdateFlags: u16 {
        const NONE = 0x0000;
        const SELF = 0x0001;
        const TRANSPORT = 0x0002;
        const HAS_ATTACKING_TARGET = 0x0004;
        const LOWGUID = 0x0008;
        const HIGHGUID = 0x0010;
        const LIVING = 0x0020;
        const HAS_POSITION = 0x0040;
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
