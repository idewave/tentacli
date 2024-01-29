use std::collections::{BTreeMap};
use std::io::{BufRead, Error, ErrorKind};
use byteorder::{LittleEndian, ReadBytesExt};

pub mod types;

use crate::primary::client::{FieldType, FieldValue, MovementFlags, ObjectField, PlayerField, SplineFlags, UnitField, UnitMoveType};
use crate::primary::parsers::movement_parser::MovementParser;
use crate::primary::parsers::position_parser::PositionParser;
use crate::primary::parsers::update_block_parser::types::{MovementData, ObjectUpdateFlags, ObjectUpdateType, ParsedBlock};

use crate::primary::utils::read_packed_guid;

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

pub struct UpdateBlocksParser;

impl UpdateBlocksParser {
    pub fn parse<R: BufRead>(reader: &mut R) -> Result<Vec<ParsedBlock>, Error> {
        // sometimes blocks_amount is 0
        let blocks_amount = reader.read_u32::<LittleEndian>()?;
        let mut parsed_blocks: Vec<ParsedBlock> = Vec::new();

        for _ in 0..blocks_amount {
            match Self::parse_block(reader) {
                Ok(parsed_block) => {
                    // TODO: need to investigate why empty block comes from server
                    if !ParsedBlock::is_empty(&parsed_block) {
                        parsed_blocks.push(parsed_block);
                    }
                },
                _ => {
                    break;
                },
            }
        }

        Ok(parsed_blocks)
    }

    fn parse_block<R: BufRead>(reader: &mut R) -> Result<ParsedBlock, Error> {
        let block_type = reader.read_u8()?;

        let mut parsed_block = ParsedBlock::new();

        match block_type {
            ObjectUpdateType::VALUES => {
                let guid = read_packed_guid(reader);

                parsed_block.guid = Some(guid);

                match Self::parse_updated_values(reader) {
                    Ok(update_fields) => {
                        parsed_block.update_fields = update_fields;
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            ObjectUpdateType::MOVEMENT => {
                let guid = read_packed_guid(reader);

                parsed_block.guid = Some(guid);

                match Self::parse_movement_data(reader) {
                    Ok(movement_data) => {
                        parsed_block.movement_data = Some(movement_data);
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            ObjectUpdateType::CREATE_OBJECT |
            ObjectUpdateType::CREATE_OBJECT2 => {
                let guid = read_packed_guid(reader);

                parsed_block.guid = Some(guid);

                let _object_type_id = reader.read_u8().ok();

                match Self::parse_movement_data(reader) {
                    Ok(movement_data) => {
                        parsed_block.movement_data = Some(movement_data);
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }

                match Self::parse_updated_values(reader) {
                    Ok(update_fields) => {
                        parsed_block.update_fields = update_fields;
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            ObjectUpdateType::OUT_OF_RANGE_OBJECTS => {
                let guid_amount = reader.read_u32::<LittleEndian>()?;
                let mut guids: Vec<u64> = Vec::new();
                for _ in 0..guid_amount {
                    let guid = read_packed_guid(reader);
                    guids.push(guid);
                }

                parsed_block.out_of_range_guids = guids;
            }
            ObjectUpdateType::NEAR_OBJECTS => {
                let guid_amount = reader.read_u32::<LittleEndian>()?;
                let mut guids: Vec<u64> = Vec::new();
                for _ in 0..guid_amount {
                    let guid = read_packed_guid(reader);
                    guids.push(guid);
                }

                parsed_block.near_object_guids = guids;
            }
            _ => {
                return Err(Error::new(ErrorKind::InvalidData, "Wrong block type"));
            }
        }

        Ok(parsed_block)
    }

    fn parse_updated_values<R: BufRead>(
        reader: &mut R
    ) -> Result<BTreeMap<u32, FieldValue>, Error> {
        let blocks_amount = reader.read_u8().unwrap_or(0);
        let mut update_fields: BTreeMap<u32, FieldValue> = BTreeMap::new();

        if blocks_amount > 0 {
            let mut update_mask = vec![0i32; blocks_amount as usize];

            for i in 0..blocks_amount {
                update_mask[i as usize] = reader.read_i32::<LittleEndian>()?;
            }

            let mut index = 0;
            for i in 0..blocks_amount {
                let mut bitmask = update_mask[i as usize];

                for _ in 0..32 {
                    if bitmask & 1 != 0 {
                        let field_type = if index < ObjectField::LIMIT {
                            ObjectField::get_field_type(index)
                        } else if index < UnitField::LIMIT {
                            UnitField::get_field_type(index)
                        } else {
                            PlayerField::get_field_type(index)
                        };

                        if let Some(value) = match field_type {
                            FieldType::LONG => {
                                Some(FieldValue::LONG(reader.read_u64::<LittleEndian>()?))
                            },
                            FieldType::INT => {
                                Some(FieldValue::INT(reader.read_u32::<LittleEndian>()?))
                            },
                            FieldType::FLOAT => {
                                Some(FieldValue::FLOAT(reader.read_f32::<LittleEndian>()?))
                            },
                            _ => None,
                        } {
                            update_fields.insert(index, value);
                        }
                    }
                    bitmask >>= 1;
                    index += 1;
                }
            }

            return Ok(update_fields);
        }

        Err(Error::new(ErrorKind::Other, "No blocks for update. Just ignore."))
    }
    fn parse_movement_data<R: BufRead>(reader: &mut R) -> Result<MovementData, Error> {
        let mut movement_data = MovementData::new();

        let object_update_flags = ObjectUpdateFlags::from_bits(
            reader.read_u16::<LittleEndian>()?
        ).unwrap_or(ObjectUpdateFlags::NONE);

        if object_update_flags.contains(ObjectUpdateFlags::SELF)  {
            // current player movement
        }

        if object_update_flags.contains(ObjectUpdateFlags::LIVING) {

            let movement_info = MovementParser::parse(reader)?;

            let mut movement_speed: BTreeMap<u8, f32> = BTreeMap::new();
            for move_type in MOVE_TYPES {
                movement_speed.insert(move_type, reader.read_f32::<LittleEndian>()?);
            }

            movement_data.movement_speed = movement_speed;

            if movement_info.movement_flags.contains(MovementFlags::SPLINE_ENABLED) {
                let spline_flags = SplineFlags::from_bits(
                    reader.read_u32::<LittleEndian>()?
                ).unwrap_or(SplineFlags::NONE);

                if spline_flags.contains(SplineFlags::FINAL_ANGLE) {
                    let _spline_facing_angle = reader.read_f32::<LittleEndian>()?;
                } else if spline_flags.contains(SplineFlags::FINAL_TARGET) {
                    let _spline_facing_target_guid = reader.read_u64::<LittleEndian>()?;
                } else if spline_flags.contains(SplineFlags::FINAL_POINT) {
                    let _spline_facing_point_x = reader.read_f32::<LittleEndian>()?;
                    let _spline_facing_point_y = reader.read_f32::<LittleEndian>()?;
                    let _spline_facing_point_z = reader.read_f32::<LittleEndian>()?;
                }

                let _spline_time_passed = reader.read_i32::<LittleEndian>()?;
                let _spline_duration = reader.read_i32::<LittleEndian>()?;
                let _spline_id = reader.read_u32::<LittleEndian>()?;

                // omit
                reader.read_f64::<LittleEndian>()?;

                let _spline_vertical_acceleration = reader.read_i32::<LittleEndian>()?;

                let _spline_effect_start_time = reader.read_i32::<LittleEndian>()?;

                let spline_amount = reader.read_u32::<LittleEndian>()?;
                for _ in 0..spline_amount {
                    let _spline_point_x = reader.read_f32::<LittleEndian>()?;
                    let _spline_point_y = reader.read_f32::<LittleEndian>()?;
                    let _spline_point_z = reader.read_f32::<LittleEndian>()?;
                }

                let _spline_evaluation_mode = reader.read_u8()?;
                let _spline_end_point_x = reader.read_f32::<LittleEndian>()?;
                let _spline_end_point_y = reader.read_f32::<LittleEndian>()?;
                let _spline_end_point_z = reader.read_f32::<LittleEndian>()?;
            }

            movement_data.movement_info = Some(movement_info);

        } else {
            if object_update_flags.contains(ObjectUpdateFlags::POSITION) {
                let _transport_guid = read_packed_guid(reader);

                let _position_x = reader.read_f32::<LittleEndian>()?;
                let _position_y = reader.read_f32::<LittleEndian>()?;
                let _position_z = reader.read_f32::<LittleEndian>()?;

                // transport offset x, y, z, orientation
                let _position = PositionParser::parse(reader)?;
                let _corpse_orientation = reader.read_f32::<LittleEndian>()?;
            }
            if object_update_flags.contains(ObjectUpdateFlags::STATIONARY_POSITION) {
                let _position = PositionParser::parse(reader)?;
            }
        }

        if object_update_flags.contains(ObjectUpdateFlags::HIGHGUID) {
            movement_data.high_guid = reader.read_u32::<LittleEndian>().ok();
        }
        if object_update_flags.contains(ObjectUpdateFlags::LOWGUID) {
            movement_data.low_guid = reader.read_u32::<LittleEndian>().ok();
        }
        if object_update_flags.contains(ObjectUpdateFlags::HAS_TARGET) {
            let target_guid = read_packed_guid(reader);
            movement_data.target_guid = Some(target_guid);
        }
        if object_update_flags.contains(ObjectUpdateFlags::TRANSPORT) {
            let _transport_timer = reader.read_u32::<LittleEndian>()?;
        }
        if object_update_flags.contains(ObjectUpdateFlags::VEHICLE) {
            let _vehicle_id = reader.read_u32::<LittleEndian>()?;
            let _vehicle_orientation = reader.read_f32::<LittleEndian>()?;
        }
        if object_update_flags.contains(ObjectUpdateFlags::ROTATION) {
            let _go_rotation = reader.read_i64::<LittleEndian>()?;
        }

        Ok(movement_data)
    }
}