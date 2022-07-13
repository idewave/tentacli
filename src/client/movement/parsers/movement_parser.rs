use std::cell::RefCell;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

use crate::client::movement::parsers::position_parser::PositionParser;
use crate::client::movement::parsers::types::{JumpInfo, MovementInfo};

use crate::client::movement::types::{
    MovementFlags,
    MovementFlagsExtra,
};
use crate::utils::read_packed_guid;

pub struct MovementParser;

impl MovementParser {
    pub fn parse(reader: RefCell<Cursor<Vec<u8>>>) -> (MovementInfo, u64) {
        let movement_flags = MovementFlags::from_bits(
            reader.borrow_mut().read_u32::<LittleEndian>().unwrap()
        ).unwrap();

        let movement_flags_extra = MovementFlagsExtra::from_bits(
            reader.borrow_mut().read_u16::<LittleEndian>().unwrap()
        ).unwrap();

        let time = reader.borrow_mut().read_u32::<LittleEndian>().unwrap();

        let (position, cursor_position) = PositionParser::parse(RefCell::clone(&reader));
        reader.borrow_mut().set_position(cursor_position);

        if movement_flags.contains(MovementFlags::TAXI) {
            let (_transport_guid, cursor_position) = read_packed_guid(RefCell::clone(&reader));
            reader.borrow_mut().set_position(cursor_position);

            // transport x, y, z, orientation
            let (_position, cursor_position) = PositionParser::parse(RefCell::clone(&reader));
            reader.borrow_mut().set_position(cursor_position);

            let _transport_time = reader.borrow_mut().read_u32::<LittleEndian>().unwrap();
            let _transport_seat = reader.borrow_mut().read_u8().unwrap();

            if movement_flags_extra.contains(MovementFlagsExtra::INTERPOLATED_MOVEMENT) {
                let _transport_time = reader.borrow_mut().read_u32::<LittleEndian>().unwrap();
            }
        }

        if movement_flags.contains(MovementFlags::SWIMMING)  ||
            movement_flags.contains(MovementFlags::FLYING) ||
            movement_flags_extra.contains(MovementFlagsExtra::ALWAYS_ALLOW_PITCHING) {
            let _pitch = reader.borrow_mut().read_f32::<LittleEndian>().unwrap();
        }

        let fall_time = reader.borrow_mut().read_u32::<LittleEndian>().unwrap();

        let mut jump_vertical_speed = 0.0;
        let mut jump_sin_angle = 0.0;
        let mut jump_cos_angle = 0.0;
        let mut jump_horizontal_speed = 0.0;

        if movement_flags.contains(MovementFlags::JUMPING) {
            jump_vertical_speed = reader.borrow_mut().read_f32::<LittleEndian>().unwrap();
            jump_sin_angle = reader.borrow_mut().read_f32::<LittleEndian>().unwrap();
            jump_cos_angle = reader.borrow_mut().read_f32::<LittleEndian>().unwrap();
            jump_horizontal_speed = reader.borrow_mut().read_f32::<LittleEndian>().unwrap();
        }

        if movement_flags.contains(MovementFlags::SPLINE_ELEVATION) {
            let _spline_elevation = reader.borrow_mut().read_f32::<LittleEndian>().unwrap();
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

        (movement_info, reader.borrow_mut().position())
    }
}