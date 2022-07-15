use std::cell::{RefCell};
use std::collections::{BTreeMap};
use std::io::{Cursor, Error, ErrorKind};
use byteorder::{LittleEndian, ReadBytesExt};

use crate::client::{
    MovementFlags,
    MovementParser,
    PositionParser,
    SplineFlags,
    UnitMoveType
};
use crate::network::packet::parsers::types::{MovementData, ParsedBlock};
use crate::network::packet::types::{ObjectUpdateFlags, ObjectUpdateType};
use crate::utils::read_packed_guid;

const MOVE_TYPES: [u8; 9] = [
    UnitMoveType::MOVE_WALK,
    UnitMoveType::MOVE_RUN,
    UnitMoveType::MOVE_RUN_BACK,
    UnitMoveType::MOVE_SWIM,
    UnitMoveType::MOVE_SWIM_BACK,
    UnitMoveType::MOVE_FLIGHT,
    UnitMoveType::MOVE_FLIGHT_BACK,
    UnitMoveType::MOVE_TURN_RATE,
    UnitMoveType::MOVE_PITCH_RATE,
];

pub struct UpdatePacketParser;

impl UpdatePacketParser {
    pub fn parse(buffer: Vec<u8>) -> Vec<ParsedBlock> {
        let reader = RefCell::new(Cursor::new(buffer));

        let blocks_amount = reader.borrow_mut().read_u32::<LittleEndian>().unwrap();
        let mut parsed_blocks: Vec<ParsedBlock> = Vec::new();

        for _ in 0..blocks_amount {
            match Self::parse_block(RefCell::clone(&reader)) {
                Ok((position, parsed_block)) => {
                    reader.borrow_mut().set_position(position);
                    if !ParsedBlock::is_empty(&parsed_block) {
                        parsed_blocks.push(parsed_block);
                    }
                },
                Err(err) => {
                    println!("Update Packet parser error: {}", err);
                    break;
                },
            }
        }

        parsed_blocks
    }

    fn parse_block(reader: RefCell<Cursor<Vec<u8>>>) -> Result<(u64, ParsedBlock), Error> {
        let block_type = reader.borrow_mut().read_u8()?;

        let mut parsed_block = ParsedBlock::new();

        match block_type {
            ObjectUpdateType::VALUES => {
                let (guid, position) = read_packed_guid(RefCell::clone(&reader));
                reader.borrow_mut().set_position(position);
                println!("UPDATETYPE_VALUES {}", guid);

                parsed_block.guid = Some(guid);

                match Self::parse_updated_values(RefCell::clone(&reader)) {
                    Ok((position, update_fields)) => {
                        reader.borrow_mut().set_position(position);
                        parsed_block.update_fields = update_fields;
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            ObjectUpdateType::MOVEMENT => {
                println!("UPDATETYPE_MOVEMENT");
                let (guid, position) = read_packed_guid(RefCell::clone(&reader));
                reader.borrow_mut().set_position(position);

                parsed_block.guid = Some(guid);

                match Self::parse_movement_data(RefCell::clone(&reader)) {
                    Ok((position, movement_data)) => {
                        reader.borrow_mut().set_position(position);
                        parsed_block.movement_data = Some(movement_data);
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            ObjectUpdateType::CREATE_OBJECT |
            ObjectUpdateType::CREATE_OBJECT2 => {
                let (guid, position) = read_packed_guid(RefCell::clone(&reader));
                reader.borrow_mut().set_position(position);

                parsed_block.guid = Some(guid);
                println!("UPDATETYPE_CREATE_OBJECT: {:?}", guid);

                let _object_type_id = reader.borrow_mut().read_u8().ok();

                match Self::parse_movement_data(RefCell::clone(&reader)) {
                    Ok((position, movement_data)) => {
                        reader.borrow_mut().set_position(position);
                        parsed_block.movement_data = Some(movement_data);
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }

                match Self::parse_updated_values(RefCell::clone(&reader)) {
                    Ok((position, update_fields)) => {
                        reader.borrow_mut().set_position(position);
                        parsed_block.update_fields = update_fields;
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            ObjectUpdateType::OUT_OF_RANGE_OBJECTS => {
                println!("OUT OF RANGE");
                let guid_amount = reader.borrow_mut().read_u32::<LittleEndian>()?;
                for _ in 0..guid_amount {
                    let (guid, position) = read_packed_guid(RefCell::clone(&reader));
                    reader.borrow_mut().set_position(position);

                    println!("FAR GUID: {}", guid);
                }
            }
            ObjectUpdateType::NEAR_OBJECTS => {
                println!("NEAR");
                let guid_amount = reader.borrow_mut().read_u32::<LittleEndian>()?;
                for _ in 0..guid_amount {
                    let (guid, position) = read_packed_guid(RefCell::clone(&reader));
                    reader.borrow_mut().set_position(position);

                    println!("NEAR GUID: {}", guid);
                }
            }
            _ => {
                return Err(Error::new(ErrorKind::InvalidData, "Wrong block type"));
            }
        }

        Ok((reader.borrow_mut().position(), parsed_block))
    }

    fn parse_updated_values(
        reader: RefCell<Cursor<Vec<u8>>>
    ) -> Result<(u64, BTreeMap<u32, u32>), Error> {
        let blocks_amount = reader.borrow_mut().read_u8().unwrap();

        if blocks_amount > 0 {
            let mut update_mask = vec![0i32; blocks_amount as usize];

            for i in 0..blocks_amount {
                update_mask[i as usize] = reader.borrow_mut().read_i32::<LittleEndian>()?;
            }

            let mut index = 0;
            let mut update_fields: BTreeMap<u32, u32> = BTreeMap::new();

            for i in 0..blocks_amount {
                let mut bitmask = update_mask[i as usize];

                for _ in 0..32 {
                    if bitmask & 1 != 0 {
                        match reader.borrow_mut().read_u32::<LittleEndian>() {
                            Ok(value) => {
                                update_fields.insert(index, value);
                            },
                            _ => {
                                println!("Error on {} position", &index);
                                return Err(Error::new(ErrorKind::InvalidData, "Cannot read data"));
                            },
                        }
                    }
                    bitmask >>= 1;
                    index += 1;
                }
            }

            Ok((reader.borrow_mut().position(), update_fields))
        } else {
            Err(Error::new(ErrorKind::Other, "No blocks for update. Just ignore."))
        }
    }
    fn parse_movement_data(reader: RefCell<Cursor<Vec<u8>>>) -> Result<(u64, MovementData), Error> {
        let mut movement_data = MovementData::new();

        let object_update_flags = ObjectUpdateFlags::from_bits(
            reader.borrow_mut().read_u16::<LittleEndian>()?
        ).unwrap();

        if object_update_flags.contains(ObjectUpdateFlags::SELF)  {
            println!("UPDATING MOVEMENT INFO FOR CURRENT CHARACTER");
        }

        if object_update_flags.contains(ObjectUpdateFlags::LIVING) {

            let (movement_info, position) = MovementParser::parse(RefCell::clone(&reader));
            reader.borrow_mut().set_position(position);

            let mut movement_speed: BTreeMap<u8, f32> = BTreeMap::new();
            for move_type in MOVE_TYPES {
                movement_speed.insert(move_type, reader.borrow_mut().read_f32::<LittleEndian>()?);
            }

            movement_data.movement_speed = movement_speed;

            if movement_info.movement_flags.contains(MovementFlags::SPLINE_ENABLED) {
                let spline_flags = SplineFlags::from_bits(
                    reader.borrow_mut().read_u32::<LittleEndian>()?
                ).unwrap();

                if spline_flags.contains(SplineFlags::FINAL_ANGLE) {
                    let _spline_facing_angle = reader.borrow_mut()
                        .read_f32::<LittleEndian>()?;
                } else if spline_flags.contains(SplineFlags::FINAL_TARGET) {
                    let _spline_facing_target_guid = reader.borrow_mut()
                        .read_u64::<LittleEndian>()?;
                } else if spline_flags.contains(SplineFlags::FINAL_POINT) {
                    let _spline_facing_point_x = reader.borrow_mut()
                        .read_f32::<LittleEndian>()?;
                    let _spline_facing_point_y = reader.borrow_mut()
                        .read_f32::<LittleEndian>()?;
                    let _spline_facing_point_z = reader.borrow_mut()
                        .read_f32::<LittleEndian>()?;
                }

                let _spline_time_passed = reader.borrow_mut().read_i32::<LittleEndian>()?;
                let _spline_duration = reader.borrow_mut().read_i32::<LittleEndian>()?;
                let _spline_id = reader.borrow_mut().read_u32::<LittleEndian>()?;

                // omit
                reader.borrow_mut().read_f64::<LittleEndian>()?;

                let _spline_vertical_acceleration = reader.borrow_mut()
                    .read_i32::<LittleEndian>()?;

                let _spline_effect_start_time = reader.borrow_mut()
                    .read_i32::<LittleEndian>()?;

                let spline_amount = reader.borrow_mut().read_u32::<LittleEndian>()?;
                for _ in 0..spline_amount {
                    let _spline_point_x = reader.borrow_mut().read_f32::<LittleEndian>()?;
                    let _spline_point_y = reader.borrow_mut().read_f32::<LittleEndian>()?;
                    let _spline_point_z = reader.borrow_mut().read_f32::<LittleEndian>()?;
                }

                let _spline_evaluation_mode = reader.borrow_mut().read_u8()?;
                let _spline_end_point_x = reader.borrow_mut().read_f32::<LittleEndian>()?;
                let _spline_end_point_y = reader.borrow_mut().read_f32::<LittleEndian>()?;
                let _spline_end_point_z = reader.borrow_mut().read_f32::<LittleEndian>()?;
            }

            movement_data.movement_info = Some(movement_info);

        } else {
            if object_update_flags.contains(ObjectUpdateFlags::POSITION) {
                let (_transport_guid, cursor_position) = read_packed_guid(RefCell::clone(&reader));
                reader.borrow_mut().set_position(cursor_position);

                let _position_x = reader.borrow_mut().read_f32::<LittleEndian>()?;
                let _position_y = reader.borrow_mut().read_f32::<LittleEndian>()?;
                let _position_z = reader.borrow_mut().read_f32::<LittleEndian>()?;

                // transport offset x, y, z, orientation
                let (_position, cursor_position) = PositionParser::parse(RefCell::clone(&reader));
                reader.borrow_mut().set_position(cursor_position);

                let _corpse_orientation = reader.borrow_mut().read_f32::<LittleEndian>()?;
            }
            if object_update_flags.contains(ObjectUpdateFlags::STATIONARY_POSITION) {
                let (_position, cursor_position) = PositionParser::parse(RefCell::clone(&reader));
                reader.borrow_mut().set_position(cursor_position);
            }
        }

        if object_update_flags.contains(ObjectUpdateFlags::HIGHGUID) {
            movement_data.high_guid = reader.borrow_mut().read_u32::<LittleEndian>().ok();
        }
        if object_update_flags.contains(ObjectUpdateFlags::LOWGUID) {
            movement_data.low_guid = reader.borrow_mut().read_u32::<LittleEndian>().ok();
        }
        if object_update_flags.contains(ObjectUpdateFlags::HAS_TARGET) {
            let (target_guid, position) = read_packed_guid(RefCell::clone(&reader));
            reader.borrow_mut().set_position(position);

            movement_data.target_guid = Some(target_guid);
        }
        if object_update_flags.contains(ObjectUpdateFlags::TRANSPORT) {
            let _transport_timer = reader.borrow_mut().read_u32::<LittleEndian>()?;
        }
        if object_update_flags.contains(ObjectUpdateFlags::VEHICLE) {
            let _vehicle_id = reader.borrow_mut().read_u32::<LittleEndian>()?;
            let _vehicle_orientation = reader.borrow_mut().read_f32::<LittleEndian>()?;
        }
        if object_update_flags.contains(ObjectUpdateFlags::ROTATION) {
            let _go_rotation = reader.borrow_mut().read_i64::<LittleEndian>()?;
        }

        Ok((reader.borrow_mut().position(), movement_data))
    }
}