use anyhow::{bail, Context, Result as AnyResult};
use std::collections::{BTreeMap};
use std::io::{BufRead, Error, ErrorKind};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde::{Serialize, Serializer};
use serde::ser::{SerializeStruct, SerializeTuple};

use crate::{BinaryConverter};
use crate::types::update_fields::{FieldValue, ObjectField, PlayerField, UnitField};

#[derive(Clone, Default, Debug)]
pub struct UpdateData {
    pub update_mask: Vec<bool>,
    pub object_fields:  BTreeMap<ObjectField, FieldValue>,
    pub unit_fields:  BTreeMap<UnitField, FieldValue>,
    pub player_fields:  BTreeMap<PlayerField, FieldValue>,
}

impl UpdateData {
    pub fn is_empty(instance: &Self) -> bool {
        instance.update_mask.is_empty()
            && instance.object_fields.is_empty()
            && instance.unit_fields.is_empty()
            && instance.player_fields.is_empty()
    }
}

impl BinaryConverter for UpdateData {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        todo!()
        // buffer.write_u8(self.update_fields.len() as u8)
        //     .map_err(|e| FieldError::CannotWrite(e, "u8".to_string()))?;
        //
        // let mut update_mask: Vec<u32> = vec![0; (self.update_fields.len() + 31) / 32];
        // let mut update_values: Vec<u32> = vec![];
        //
        // for (index, &field_value) in self.update_fields.iter() {
        //     let block_index = index / 32;
        //     let bit_index = index % 32;
        //     update_mask[block_index as usize] |= 1 << bit_index;
        //
        //     let value = match field_value {
        //         FieldValue::Integer(value) => vec![value],
        //         FieldValue::Bytes(value) => vec![value],
        //         FieldValue::Float(value) => vec![value.to_bits()],
        //         FieldValue::TwoShorts(first, second) => {
        //             vec![u32::from(first) << 16 | u32::from(second)]
        //         },
        //         FieldValue::Long(value) => {
        //             let higher_bits = (value >> 32) as u32;
        //             let lower_bits = (value & 0xFFFFFFFF) as u32;
        //             vec![higher_bits, lower_bits]
        //         }
        //     };
        //
        //     update_values.extend(value);
        // }
        //
        // for value in update_mask {
        //     buffer.extend(value.to_le_bytes());
        // }
        //
        // for value in update_values {
        //     buffer.extend(value.to_le_bytes());
        // }
        //
        // Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
        let blocks_amount = reader.read_u8()?;

        if blocks_amount > 0 {
            let mut update_blocks: BTreeMap<u32, u32> = BTreeMap::new();

            let mut update_mask = (0..blocks_amount)
                .map(|_| reader.read_u32::<LittleEndian>().context(format!("Failed to read MASK VAL from reader with {}", blocks_amount)))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(|mut mask| {
                    let mut bits = vec![];
                    for _ in 0..32 {
                        bits.push((mask & 1) == 1);
                        mask >>= 1;
                    }
                    bits
                })
                .flatten()
                .collect::<Vec<bool>>();

            let update_indexes: Vec<u32> = update_mask.iter()
                .enumerate()
                .filter_map(|(index, &value)| if value { Some(index as u32) } else { None })
                .collect();

            for index in update_indexes {
                update_blocks.insert(index, reader.read_u32::<LittleEndian>()
                    .context(format!("Failed to read UPD BLOCK from reader with {} blocks on index {}", blocks_amount, index))?);
            }

            let mut unit_blocks = update_blocks.split_off(&ObjectField::get_limit());
            let mut player_blocks = unit_blocks.split_off(&UnitField::get_limit());
            let mut object_blocks = update_blocks.clone();

            let object_values = object_blocks.values().cloned().collect::<Vec<u32>>();
            let unit_values = unit_blocks.values().cloned().collect::<Vec<u32>>();
            let player_values = player_blocks.values().cloned().collect::<Vec<u32>>();

            let object_fields = ObjectField::read_from(object_values, &mut update_mask)?;
            let unit_fields = UnitField::read_from(unit_values, &mut update_mask)?;
            let player_fields = PlayerField::read_from(player_values, &mut update_mask)?;

            Ok(Self {
                update_mask,
                object_fields,
                unit_fields,
                player_fields
            })
        } else {
            Ok(Self::default())
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

impl Serialize for UpdateData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut object_fields: BTreeMap<String, FieldValue> = BTreeMap::new();
        let mut unit_fields: BTreeMap<String, FieldValue> = BTreeMap::new();
        let mut player_fields: BTreeMap<String, FieldValue> = BTreeMap::new();

        let mut fields_amount = 0;

        for (k, v) in &self.object_fields {
            object_fields.insert(k.get_field_name(), v.clone());
        }

        for (k, v) in &self.unit_fields {
            unit_fields.insert(k.get_field_name(), v.clone());
        }

        for (k, v) in &self.player_fields {
            player_fields.insert(k.get_field_name(), v.clone());
        }

        if !object_fields.is_empty() {
            fields_amount += 1;
        }

        if !unit_fields.is_empty() {
            fields_amount += 1;
        }

        if !player_fields.is_empty() {
            fields_amount += 1;
        }

        let mut state = serializer.serialize_struct("UpdateData", fields_amount)?;
        if !object_fields.is_empty() {
            state.serialize_field("object_fields", &object_fields)?;
        }
        if !unit_fields.is_empty() {
            state.serialize_field("unit_fields", &unit_fields)?;
        }
        if !player_fields.is_empty() {
            state.serialize_field("player_fields", &player_fields)?;
        }
        state.end()
    }
}

#[non_exhaustive]
pub struct ObjectTypeMask;
#[allow(dead_code)]
impl ObjectTypeMask {
    pub const OBJECT: i32 = 0x0001;
    pub const ITEM: i32 = 0x0002;
    pub const CONTAINER: i32 = 0x0004;
    pub const UNIT: i32 = 0x0008;
    pub const PLAYER: i32 = 0x0010;
    pub const GAMEOBJECT: i32 = 0x0020;
    pub const DYNAMICOBJECT: i32 = 0x0040;
    pub const CORPSE: i32 = 0x0080;

    pub const IS_UNIT: i32 = ObjectTypeMask::OBJECT | ObjectTypeMask::UNIT;
    pub const IS_PLAYER: i32 = ObjectTypeMask::IS_UNIT | ObjectTypeMask::PLAYER;
}

#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct BlockType(pub u8);
#[allow(dead_code)]
impl BlockType {
    pub const VALUES: u8 = 0;
    pub const MOVEMENT: u8 = 1;
    pub const CREATE_OBJECT: u8 = 2;
    pub const CREATE_OBJECT2: u8 = 3;
    pub const OUT_OF_RANGE_OBJECTS: u8 = 4;
    pub const NEAR_OBJECTS: u8 = 5;
}

impl BinaryConverter for BlockType {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        u8::write_into(&mut self.0, buffer)?;

        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, dependencies: &mut Vec<u8>) -> AnyResult<Self> {
        let value = u8::read_from(reader, dependencies)?;

        Ok(Self(value))
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.0]
    }
}

impl Serialize for BlockType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let field_name = match self.0 {
            Self::VALUES => "VALUES",
            Self::MOVEMENT => "MOVEMENT",
            Self::CREATE_OBJECT => "CREATE_OBJECT",
            Self::CREATE_OBJECT2 => "CREATE_OBJECT2",
            Self::OUT_OF_RANGE_OBJECTS => "OUT_OF_RANGE_OBJECTS",
            Self::NEAR_OBJECTS => "NEAR_OBJECTS",
            _ => "NONE",
        };

        serializer.serialize_str(field_name)
    }
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct ObjectTypeID(pub i8);
#[allow(dead_code)]
impl ObjectTypeID {
    pub const NONE: i8 = -1;
    pub const OBJECT: i8 = 0;
    pub const ITEM: i8 = 1;
    pub const CONTAINER: i8 = 2;
    pub const UNIT: i8 = 3;
    pub const PLAYER: i8 = 4;
    pub const GAMEOBJECT: i8 = 5;
    pub const DYNAMICOBJECT: i8 = 6;
    pub const CORPSE: i8 = 7;

    pub fn is_none(instance: &Self) -> bool {
        instance.0 == Self::NONE
    }
}

impl Default for ObjectTypeID {
    fn default() -> Self {
        Self(ObjectTypeID::NONE)
    }
}

impl BinaryConverter for ObjectTypeID {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        i8::write_into(&mut self.0, buffer)?;

        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, dependencies: &mut Vec<u8>) -> AnyResult<Self> {
        let value = i8::read_from(reader, dependencies)?;

        Ok(Self(value))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.to_le_bytes().to_vec()
    }
}

impl Serialize for ObjectTypeID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let field_name = match self.0 {
            Self::OBJECT => "OBJECT",
            Self::ITEM => "ITEM",
            Self::CONTAINER => "CONTAINER",
            Self::UNIT => "UNIT",
            Self::PLAYER => "PLAYER",
            Self::GAMEOBJECT => "GAMEOBJECT",
            Self::DYNAMICOBJECT => "DYNAMICOBJECT",
            Self::CORPSE => "CORPSE",
            _ => "NONE",
        };

        serializer.serialize_str(field_name)
    }
}