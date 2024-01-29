use bitflags::bitflags;

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