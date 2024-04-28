use anyhow::{Result as AnyResult};
use std::collections::BTreeMap;
use std::io::{BufRead};
use byteorder::{LittleEndian, ReadBytesExt};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use crate::{BinaryConverter};
use crate::types::player::{FieldType, FieldValue, ObjectField, PlayerField, UnitField};

#[derive(Clone, Default, Debug)]
pub struct UpdateData {
    pub update_mask: Vec<u32>,
    pub update_fields:  BTreeMap<u32, FieldValue>
}

impl BinaryConverter for UpdateData {
    fn write_into(&mut self, _: &mut Vec<u8>) -> AnyResult<()> {
        todo!()
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
        let blocks_amount = reader.read_u8()?;
        let mut update_blocks: BTreeMap<u32, u32> = BTreeMap::new();
        let mut update_fields:  BTreeMap<u32, FieldValue> = BTreeMap::new();

        let mut update_mask = vec![0u32; blocks_amount as usize];

        for i in 0..blocks_amount {
            update_mask[i as usize] = reader.read_u32::<LittleEndian>()?;
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

        Ok(Self {
            update_mask,
            update_fields,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

impl Serialize for UpdateData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut update_fields: BTreeMap<String, FieldValue> = BTreeMap::new();
        for (k, v) in &self.update_fields {
            let key = if k < &ObjectField::LIMIT {
                ObjectField::get_field_name(*k)
            } else if k < &UnitField::LIMIT {
                UnitField::get_field_name(*k)
            } else {
                PlayerField::get_field_name(*k)
            };

            update_fields.insert(key, v.clone());
        }

        const FIELDS_AMOUNT: usize = 1;
        let mut state = serializer.serialize_struct("UpdateData", FIELDS_AMOUNT)?;
        state.serialize_field("update_fields", &update_fields)?;
        state.end()
    }
}

#[non_exhaustive]
pub struct ObjectTypeMask;
#[allow(dead_code)]
impl ObjectTypeMask {
    pub const OBJECT: u32 = 0x0001;
    pub const ITEM: u32 = 0x0002;
    pub const CONTAINER: u32 = 0x0004;
    pub const UNIT: u32 = 0x0008;
    pub const PLAYER: u32 = 0x0010;
    pub const GAMEOBJECT: u32 = 0x0020;
    pub const DYNAMICOBJECT: u32 = 0x0040;
    pub const CORPSE: u32 = 0x0080;

    pub const IS_UNIT: u32 = ObjectTypeMask::OBJECT | ObjectTypeMask::UNIT;
    pub const IS_PLAYER: u32 = ObjectTypeMask::IS_UNIT | ObjectTypeMask::PLAYER;
}

#[non_exhaustive]
pub struct ObjectBlockType;
#[allow(dead_code)]
impl ObjectBlockType {
    pub const VALUES: u8 = 0;
    pub const MOVEMENT: u8 = 1;
    pub const CREATE_OBJECT: u8 = 2;
    pub const CREATE_OBJECT2: u8 = 3;
    pub const OUT_OF_RANGE_OBJECTS: u8 = 4;
    pub const NEAR_OBJECTS: u8 = 5;
}

#[non_exhaustive]
pub struct ObjectTypeID;
#[allow(dead_code)]
impl ObjectTypeID {
    pub const OBJECT: u8 = 0;
    pub const ITEM: u8 = 1;
    pub const CONTAINER: u8 = 2;
    pub const UNIT: u8 = 3;
    pub const PLAYER: u8 = 4;
    pub const GAMEOBJECT: u8 = 5;
    pub const DYNAMICOBJECT: u8 = 6;
    pub const CORPSE: u8 = 7;
}