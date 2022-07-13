use std::fmt::{Debug, Formatter};
use crate::client::{MovementFlags, MovementFlagsExtra};

#[derive(Copy, Clone)]
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

pub struct MovementInfo {
    pub movement_flags: MovementFlags,
    pub movement_flags_extra: MovementFlagsExtra,
    pub time: u32,
    pub position: Position,
    pub fall_time: u32,
    pub jump_info: JumpInfo,
}

pub struct JumpInfo {
    pub jump_vertical_speed: f32,
    pub jump_sin_angle: f32,
    pub jump_cos_angle: f32,
    pub jump_horizontal_speed: f32,
}