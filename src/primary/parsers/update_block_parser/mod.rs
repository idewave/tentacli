use std::collections::{BTreeMap};
use std::io::{BufRead, Error, ErrorKind};
use byteorder::{LittleEndian, ReadBytesExt};

pub mod types;

use crate::primary::client::{FieldType, FieldValue, MovementFlags, ObjectField, PlayerField, SplineFlags, UnitField, UnitMoveType};
use crate::primary::parsers::movement_parser::MovementParser;
use crate::primary::parsers::position_parser::PositionParser;
use crate::primary::parsers::update_block_parser::types::{MovementData, ObjectUpdateFlags, ObjectUpdateType, ParsedBlock, UpdateFields};

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
                Err(err) => {
                    return Err(err);
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
    ) -> Result<UpdateFields, Error> {
        let blocks_amount = reader.read_u8()?;
        let mut update_blocks: BTreeMap<u32, u32> = BTreeMap::new();
        let mut update_fields: UpdateFields = BTreeMap::new();

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
                        update_blocks.insert(index, reader.read_u32::<LittleEndian>()?);
                    }
                    bitmask >>= 1;
                    index += 1;
                }
            }

            for (k, v) in update_blocks.clone().into_iter() {
                let field_type = if k < ObjectField::LIMIT {
                    ObjectField::get_field_type(k)
                } else if k < UnitField::LIMIT {
                    UnitField::get_field_type(k)
                } else {
                    PlayerField::get_field_type(k)
                };

                let value = match field_type {
                    FieldType::Integer => {
                        Some(FieldValue::Integer(v))
                    },
                    FieldType::Bytes => {
                        Some(FieldValue::Bytes(v))
                    },
                    FieldType::Long => {
                        if let Some(next_v) = update_blocks.get(&(k + 1)) {
                            Some(FieldValue::Long((u64::from(*next_v) << 32) | u64::from(v)))
                        } else {
                            Some(FieldValue::Long(u64::from(v)))
                        }
                    },
                    FieldType::Float => {
                        Some(FieldValue::Float(f32::from_bits(v)))
                    },
                    FieldType::TwoShorts => {
                        let first: u16 = (v & 0xFFFF) as u16;
                        let second: u16 = ((v >> 16) & 0xFFFF) as u16;
                        Some(FieldValue::TwoShorts(first, second))
                    },
                    FieldType::None => None,
                };

                if let Some(value) = value {
                    update_fields.insert(k, value);
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
                }

                if spline_flags.contains(SplineFlags::FINAL_TARGET) {
                    let _spline_facing_target_guid = reader.read_u64::<LittleEndian>()?;
                }

                if spline_flags.contains(SplineFlags::FINAL_POINT) {
                    let _spline_facing_point_x = reader.read_f32::<LittleEndian>()?;
                    let _spline_facing_point_y = reader.read_f32::<LittleEndian>()?;
                    let _spline_facing_point_z = reader.read_f32::<LittleEndian>()?;
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

                let _ = reader.read_f32::<LittleEndian>()?;
                let _ = reader.read_f32::<LittleEndian>()?;
                let _ = reader.read_f32::<LittleEndian>()?;

                // transport offset x, y, z, orientation
                let _position = PositionParser::parse(reader)?;
                let _ = reader.read_f32::<LittleEndian>()?;
            }
            if object_update_flags.contains(ObjectUpdateFlags::STATIONARY_POSITION) {
                let _position = PositionParser::parse(reader)?;
            }
        }

        if object_update_flags.contains(ObjectUpdateFlags::LOWGUID) {
            movement_data.low_guid = reader.read_u32::<LittleEndian>().ok();
        }

        if object_update_flags.contains(ObjectUpdateFlags::HIGHGUID) {
            movement_data.high_guid = reader.read_u32::<LittleEndian>().ok();
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

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use flate2::read::DeflateDecoder;

    use crate::player::{ObjectField, PlayerField, UnitField};
    use crate::primary::client::FieldValue;
    use crate::primary::parsers::update_block_parser::UpdateBlocksParser;

    const COMPRESSED_PACKET: [u8; 339] = [99, 100, 96, 96, 96,
        226, 223, 188, 53, 140, 157, 165, 16, 200, 132, 1, 227, 199, 181, 110, 179, 100, 235, 220,
        106, 18, 155, 29, 127, 199, 100, 58, 64, 196, 21, 128, 244, 3, 32, 158, 224, 80, 40, 51, 29,
        72, 35, 248, 15, 248, 61, 29, 14, 127, 245, 112, 208, 7, 42, 212, 18, 101, 104, 16, 22, 103,
        60, 240, 227, 62, 195, 71, 70, 14, 6, 6, 127, 54, 6, 22, 54, 160, 56, 185, 96, 71, 238, 237,
        109, 185, 36, 104, 86, 120, 84, 207, 192, 224, 0, 212, 192, 2, 215, 196, 196, 240, 15, 196,
        118, 0, 11, 52, 128, 72, 123, 32, 6, 249, 90, 18, 196, 97, 104, 176, 231, 98, 101, 96, 112,
        7, 178, 182, 3, 113, 10, 16, 195, 216, 47, 152, 33, 252, 79, 111, 30, 58, 48, 2, 197, 189,
        128, 62, 1, 250, 9, 74, 56, 48, 92, 96, 103, 128, 227, 201, 18, 71, 236, 24, 24, 14, 216,
        151, 216, 48, 48, 128, 48, 16, 56, 0, 17, 16, 55, 216, 11, 2, 57, 98, 64, 44, 12, 196, 82,
        80, 182, 14, 144, 246, 4, 98, 19, 32, 102, 0, 26, 14, 52, 138, 129, 7, 136, 239, 108, 211,
        117, 184, 179, 45, 215, 1, 164, 143, 133, 147, 157, 17, 104, 37, 227, 4, 160, 188, 25, 88,
        25, 43, 131, 5, 144, 102, 5, 66, 63, 40, 29, 15, 21, 7, 5, 146, 14, 163, 14, 99, 7, 148,
        223, 9, 229, 47, 130, 242, 159, 64, 233, 249, 64, 179, 24, 129, 48, 149, 9, 98, 206, 23,
        32, 13, 226, 3, 41, 134, 34, 145, 5, 14, 51, 119, 47, 112, 120, 93, 60, 31, 140, 173, 239,
        111, 117, 192, 133, 193, 225, 224, 192, 192, 144, 7, 212, 7, 114, 43, 46, 252, 31, 8, 2,
        128, 42, 68, 129, 24, 20, 6, 226, 64, 44, 1, 196, 160, 176, 7, 133, 5, 0, 2, 177, 96, 33];

    const TEST_GUID: u64 = 123123123;
    const TEST_HEALTH: u32 = 71;
    const TEST_XP: u32 = 400;

    #[test]
    fn test_parsing_compressed_packet() {
        let mut buffer = Vec::new();
        let mut decoder = DeflateDecoder::new(&COMPRESSED_PACKET[..]);
        std::io::Read::read_to_end(&mut decoder, &mut buffer).expect("Cannot read");

        let parsed = UpdateBlocksParser::parse(&mut Cursor::new(buffer)).expect("Cannot parse");
        assert_eq!(parsed[0].guid, Some(TEST_GUID));

        if let Some(FieldValue::Long(guid)) = parsed[0].update_fields.get(&ObjectField::GUID) {
            assert_eq!(*guid, TEST_GUID);
        } else {
            panic!("GUID was not parsed correctly !");
        }

        if let Some(FieldValue::Integer(health)) = parsed[0].update_fields.get(&UnitField::HEALTH) {
            assert_eq!(*health, TEST_HEALTH);
        } else {
            panic!("HEALTH was not parsed correctly !");
        }

        if let Some(FieldValue::Integer(xp)) = parsed[0]
            .update_fields.get(&PlayerField::NEXT_LEVEL_XP)
        {
            assert_eq!(*xp, TEST_XP);
        } else {
            panic!("NEXT_LEVEL_XP was not parsed correctly !");
        }
    }
}