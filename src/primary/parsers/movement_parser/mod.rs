use std::io::{BufRead, Error};
use byteorder::{LittleEndian, ReadBytesExt};

use crate::primary::client::{MovementFlags, MovementFlagsExtra};

pub mod types;

use crate::primary::parsers::movement_parser::types::{JumpInfo, MovementInfo};
use crate::primary::parsers::position_parser::PositionParser;
use crate::primary::utils::read_packed_guid;

pub struct MovementParser;

impl MovementParser {
    pub fn parse<R: BufRead>(reader: &mut R) -> Result<MovementInfo, Error> {
        let movement_flags = MovementFlags::from_bits(
            reader.read_u32::<LittleEndian>()?
        ).unwrap_or(MovementFlags::NONE);

        let movement_flags_extra = MovementFlagsExtra::from_bits(
            reader.read_u16::<LittleEndian>()?
        ).unwrap_or(MovementFlagsExtra::NONE);

        let time = reader.read_u32::<LittleEndian>()?;

        let position = PositionParser::parse(reader)?;

        if movement_flags.contains(MovementFlags::TAXI) {
            let _transport_guid = read_packed_guid(reader);

            // transport x, y, z, orientation
            let _position = PositionParser::parse(reader);

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