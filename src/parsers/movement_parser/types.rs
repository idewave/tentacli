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

#[derive(Clone, Default, Debug)]
pub struct JumpInfo {
    pub jump_vertical_speed: f32,
    pub jump_sin_angle: f32,
    pub jump_cos_angle: f32,
    pub jump_horizontal_speed: f32,
}