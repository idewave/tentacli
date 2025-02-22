use std::collections::BTreeMap;
use std::io::BufRead;

use anyhow::Context;
use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde::{Serialize, Serializer};

use crate::BinaryConverter;
use crate::types::update_fields::{
    ContainerField,
    CorpseField,
    DynamicObjectField,
    FieldValue,
    GameObjectField,
    ItemField,
    ObjectField,
    PlayerField,
    UnitField,
};

#[derive(Serialize, Clone, Default, Debug, PartialEq)]
pub struct UpdateData {
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub object_fields: BTreeMap<ObjectField, FieldValue>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub unit_fields: BTreeMap<UnitField, FieldValue>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub player_fields: BTreeMap<PlayerField, FieldValue>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub item_fields: BTreeMap<ItemField, FieldValue>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub container_fields: BTreeMap<ContainerField, FieldValue>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub game_object_fields: BTreeMap<GameObjectField, FieldValue>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub dynamic_object_fields: BTreeMap<DynamicObjectField, FieldValue>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub corpse_fields: BTreeMap<CorpseField, FieldValue>,
}

impl UpdateData {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }

    pub fn update_fields(&mut self, source: &mut UpdateData) {
        self.player_fields.append(&mut source.player_fields);
        self.unit_fields.append(&mut source.unit_fields);
        self.object_fields.append(&mut source.object_fields);
        self.item_fields.append(&mut source.item_fields);
        self.game_object_fields.append(&mut source.game_object_fields);
        self.dynamic_object_fields.append(&mut source.dynamic_object_fields);
        self.container_fields.append(&mut source.container_fields);
        self.corpse_fields.append(&mut source.corpse_fields);
    }

    pub fn parse_value(option: &FieldValue) -> Vec<u32> {
        match option {
            FieldValue::Long(value) => {
                let high = (value >> 32) as u32;
                let low = (value & 0xFFFFFFFF) as u32;
                vec![low, high]
            }
            FieldValue::LongArray(values) => {
                let mut result: Vec<u32> = vec![];
                for value in values {
                    let high = (value >> 32) as u32;
                    let low = (value & 0xFFFFFFFF) as u32;
                    result.push(low);
                    result.push(high);
                }

                result
            }
            FieldValue::Integer(value) => {
                vec![*value as u32]
            }
            FieldValue::IntegerArray(values) => {
                values.iter().map(|&value| value as u32).collect()
            }
            FieldValue::Bytes(value) => vec![*value],
            FieldValue::BytesArray(values) => values.clone(),
            FieldValue::Float(value) => vec![value.to_bits()],
            FieldValue::FloatArray(values) => values.iter().map(|value| value.to_bits()).collect(),
            FieldValue::TwoShorts((first, second)) => {
                vec![(*first as u32) << 16 | *second as u32]
            }
            FieldValue::TwoShortsArray(values) => {
                values.iter()
                    .map(|&(first, second)| (first as u32) << 16 | second as u32)
                    .collect()
            }
            _ => { vec![] }
        }
    }

    fn build_blocks(
        update_blocks: &BTreeMap<u32, u32>,
        start: u32,
        end: u32,
    ) -> BTreeMap<u32, u32> {
        update_blocks.range(start..=end).map(|(&key, &value)| (key, value)).collect()
        // update_blocks.range(start..=end).copied().collect()
    }
}

impl BinaryConverter for UpdateData {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        let mut update_fields: BTreeMap<u32, u32> = BTreeMap::new();

        for (key, option) in self.object_fields.iter() {
            let values = Self::parse_value(option);
            let start_index = ObjectField::get_index(key);

            for (i, &value) in values.iter().enumerate() {
                // zero values exists in arrays (when some items omitted)
                if value != 0 {
                    update_fields.insert(start_index + i as u32, value);
                }
            }
        }

        for (key, option) in self.unit_fields.iter() {
            let values = Self::parse_value(option);
            let start_index = UnitField::get_index(key);

            for (i, &value) in values.iter().enumerate() {
                if value != 0 {
                    update_fields.insert(start_index + i as u32, value);
                }
            }
        }

        for (key, option) in self.player_fields.iter() {
            let values = Self::parse_value(option);
            let start_index = PlayerField::get_index(key);

            for (i, &value) in values.iter().enumerate() {
                if value != 0 {
                    update_fields.insert(start_index + i as u32, value);
                }
            }
        }

        let values_limit = if !self.player_fields.is_empty() {
            PlayerField::get_limit()
        } else if !self.unit_fields.is_empty() {
            UnitField::get_limit()
        } else {
            ObjectField::get_limit()
        };

        let blocks_amount = ((values_limit + 31) / 32) as u8;

        buffer.write_u8(blocks_amount)?;

        let mut update_mask: Vec<u32> = vec![0; blocks_amount as usize];
        for index in update_fields.keys() {
            let block_index = index / 32;
            let bit_index = index % 32;
            update_mask[block_index as usize] |= 1 << bit_index;
        }

        for value in update_mask {
            buffer.extend(value.to_le_bytes());
        }

        for value in update_fields.values() {
            buffer.extend(value.to_le_bytes());
        }

        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> anyhow::Result<Self> {
        let blocks_amount = u8::read_from(reader, &mut vec![])?;

        let mut instance = Self::default();

        if blocks_amount > 0 {
            let mut update_mask = (0..blocks_amount)
                .map(|_| reader
                    .read_u32::<LittleEndian>()
                    .context(format!("Failed to read MASK VAL from reader with {}", blocks_amount)))
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

            let update_indices: Vec<u32> = update_mask.iter()
                .enumerate()
                .filter_map(|(index, &value)| if value { Some(index as u32) } else { None })
                .collect();

            let update_blocks: BTreeMap<u32, u32> = update_indices
                .into_iter()
                .map(|index| (index, reader.read_u32::<LittleEndian>().unwrap()))
                .collect();

            let object_fields: BTreeMap<ObjectField, FieldValue> = {
                let blocks: BTreeMap<u32, u32> = update_blocks
                    .range(0..=ObjectField::get_limit())
                    .map(|(&key, &value)| (key, value))
                    .collect();

                ObjectField::read_from(
                    blocks.values().copied().collect::<Vec<u32>>(),
                    &mut update_mask,
                ).unwrap_or_default()
            };

            let mask = object_fields
                .get(&ObjectField::Type)
                .map_or(
                    ObjectTypeMask::NONE,
                    |field| if let FieldValue::Integer(mask) = field {
                        ObjectTypeMask::from_bits(*mask).unwrap()
                    } else {
                        ObjectTypeMask::NONE
                    },
                );

            if mask.contains(ObjectTypeMask::UNIT) || mask.is_empty() {
                instance.unit_fields = {
                    let blocks = Self::build_blocks(
                        &update_blocks,
                        ObjectField::get_limit() + 1,
                        UnitField::get_limit(),
                    );

                    UnitField::read_from(
                        blocks.values().copied().collect::<Vec<u32>>(),
                        &mut update_mask,
                    ).unwrap_or_default()
                };
            }

            if mask.contains(ObjectTypeMask::PLAYER) || mask.is_empty() {
                instance.player_fields = {
                    let blocks = Self::build_blocks(
                        &update_blocks,
                        UnitField::get_limit() + 1,
                        PlayerField::get_limit(),
                    );

                    PlayerField::read_from(
                        blocks.values().copied().collect::<Vec<u32>>(),
                        &mut update_mask,
                    ).unwrap_or_default()
                };
            }

            if mask.contains(ObjectTypeMask::ITEM) || mask.is_empty() {
                instance.item_fields = {
                    let blocks = Self::build_blocks(
                        &update_blocks,
                        ObjectField::get_limit() + 1,
                        ItemField::get_limit(),
                    );

                    ItemField::read_from(
                        blocks.values().copied().collect::<Vec<u32>>(),
                        &mut update_mask,
                    ).unwrap_or_default()
                };
            }

            if mask.contains(ObjectTypeMask::GAMEOBJECT) || mask.is_empty() {
                instance.game_object_fields = {
                    let blocks = Self::build_blocks(
                        &update_blocks,
                        ObjectField::get_limit() + 1,
                        GameObjectField::get_limit(),
                    );

                    GameObjectField::read_from(
                        blocks.values().copied().collect::<Vec<u32>>(),
                        &mut update_mask,
                    ).unwrap_or_default()
                };
            }

            if mask.contains(ObjectTypeMask::DYNAMICOBJECT) || mask.is_empty() {
                instance.dynamic_object_fields = {
                    let blocks = Self::build_blocks(
                        &update_blocks,
                        ObjectField::get_limit() + 1,
                        DynamicObjectField::get_limit(),
                    );

                    DynamicObjectField::read_from(
                        blocks.values().copied().collect::<Vec<u32>>(),
                        &mut update_mask,
                    ).unwrap_or_default()
                };
            }

            if mask.contains(ObjectTypeMask::CONTAINER) || mask.is_empty() {
                instance.container_fields = {
                    let blocks = Self::build_blocks(
                        &update_blocks,
                        ItemField::get_limit() + 1,
                        ContainerField::get_limit(),
                    );

                    ContainerField::read_from(
                        blocks.values().copied().collect::<Vec<u32>>(),
                        &mut update_mask,
                    ).unwrap_or_default()
                };
            }

            if mask.contains(ObjectTypeMask::CORPSE) || mask.is_empty() {
                instance.corpse_fields = {
                    let blocks = Self::build_blocks(
                        &update_blocks,
                        ObjectField::get_limit() + 1,
                        CorpseField::get_limit(),
                    );

                    CorpseField::read_from(
                        blocks.values().copied().collect::<Vec<u32>>(),
                        &mut update_mask,
                    ).unwrap_or_default()
                };
            }

            instance.object_fields = object_fields;
        }

        Ok(instance)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::BinaryConverter;
    use crate::types::update_data::UpdateData;
    use crate::types::update_fields::{FieldValue, ObjectField, PlayerField, UnitField};

    #[test]
    fn test_generated_update_mask_is_correct() {
        const GUID: u64 = 1;
        const TYPE: i32 = 2;
        const SCALE_X: f32 = 3.;
        const AURA_STATE: i32 = 35;
        const HEALTH: i32 = 52;
        const BASE_ATTACK_TIME_0: i32 = 100;
        const BASE_ATTACK_TIME_1: i32 = 200;
        const POWER_MANA: i32 = 10;
        const POWER_RAGE: i32 = 11;
        const POWER_FOCUS: i32 = 12;
        const POWER_ENERGY: i32 = 13;
        const POWER_HAPPINESS: i32 = 14;
        const POWER_RUNES: i32 = 15;
        const POWER_RUNIC_POWER: i32 = 16;
        const BYTES: u32 = 213;
        const XP: i32 = 152;
        const SLOT_0: u64 = 5116089176692927345;
        const SLOT_10: u64 = 2156389176699927345;
        const CHAR_POINT_0: i32 = 2;
        const CHAR_POINT_1: i32 = 4;
        const KNOWN_TITLE_0: u64 = 13423323;
        const KNOWN_TITLE_1: u64 = 21231324;
        const KNOWN_TITLE_2: u64 = 33213225;

        let inv_slots = {
            let mut slots = vec![0u64; 23];
            slots[0] = SLOT_0;
            slots[10] = SLOT_10;

            slots
        };

        let keyring_slots = {
            let mut slots = vec![0u64; 32];
            slots[0] = SLOT_0;
            slots[10] = SLOT_10;

            slots
        };

        let cur_token_slots = {
            let mut slots = vec![0u64; 32];
            slots[0] = SLOT_0;
            slots[10] = SLOT_10;

            slots
        };

        let character_points = vec![CHAR_POINT_0, CHAR_POINT_1];

        let known_titles = vec![KNOWN_TITLE_0, KNOWN_TITLE_1, KNOWN_TITLE_2];

        let mut update_data = UpdateData {
            object_fields: {
                let mut map: BTreeMap<ObjectField, FieldValue> = BTreeMap::new();
                map.insert(ObjectField::Guid, FieldValue::Long(GUID));
                map.insert(ObjectField::Type, FieldValue::Integer(TYPE));
                map.insert(ObjectField::ScaleX, FieldValue::Float(SCALE_X));

                map
            },
            unit_fields: {
                let mut map: BTreeMap<UnitField, FieldValue> = BTreeMap::new();
                map.insert(UnitField::AuraState, FieldValue::Integer(AURA_STATE));
                map.insert(UnitField::Charm, FieldValue::Long(GUID));
                map.insert(UnitField::Health, FieldValue::Integer(HEALTH));
                map.insert(
                    UnitField::BaseAttackTime,
                    FieldValue::IntegerArray(vec![BASE_ATTACK_TIME_0, BASE_ATTACK_TIME_1]),
                );
                map.insert(
                    UnitField::Powers,
                    FieldValue::IntegerArray(
                        vec![POWER_MANA, POWER_RAGE, POWER_FOCUS, POWER_ENERGY,
                             POWER_HAPPINESS, POWER_RUNES, POWER_RUNIC_POWER]
                    ),
                );

                map
            },
            player_fields: {
                let mut map: BTreeMap<PlayerField, FieldValue> = BTreeMap::new();
                map.insert(PlayerField::Bytes, FieldValue::BytesArray(vec![BYTES, BYTES, BYTES]));
                map.insert(PlayerField::Xp, FieldValue::Integer(XP));
                map.insert(PlayerField::InvSlot, FieldValue::LongArray(inv_slots.clone()));
                map.insert(PlayerField::KeyringSlot, FieldValue::LongArray(keyring_slots.clone()));
                map.insert(
                    PlayerField::CurrencyTokenSlot,
                    FieldValue::LongArray(cur_token_slots.clone()),
                );
                map.insert(
                    PlayerField::CharacterPoints,
                    FieldValue::IntegerArray(character_points.clone()),
                );
                map.insert(
                    PlayerField::KnownTitles,
                    FieldValue::LongArray(known_titles.clone()),
                );

                map
            },
            ..UpdateData::default()
        };

        let mut buffer: Vec<u8> = vec![];
        update_data.write_into(&mut buffer).unwrap();

        let mut reader = std::io::Cursor::new(&buffer);

        let result = UpdateData::read_from(&mut reader, &mut vec![]).unwrap();
        assert_eq!(result.object_fields.get(&ObjectField::Guid), Some(&FieldValue::Long(GUID)));
        assert_eq!(result.object_fields.get(&ObjectField::Type), Some(&FieldValue::Integer(TYPE)));
        assert_eq!(
            result.object_fields.get(&ObjectField::ScaleX), Some(&FieldValue::Float(SCALE_X))
        );
        assert_eq!(
            result.unit_fields.get(&UnitField::AuraState), Some(&FieldValue::Integer(AURA_STATE))
        );
        assert_eq!(
            result.unit_fields.get(&UnitField::Charm), Some(&FieldValue::Long(GUID))
        );
        assert_eq!(
            result.unit_fields.get(&UnitField::Health), Some(&FieldValue::Integer(HEALTH))
        );
        assert_eq!(
            result.unit_fields.get(&UnitField::BaseAttackTime),
            Some(&FieldValue::IntegerArray(vec![BASE_ATTACK_TIME_0, BASE_ATTACK_TIME_1]))
        );
        assert_eq!(
            result.unit_fields.get(&UnitField::Powers),
            Some(&FieldValue::IntegerArray(vec![
                POWER_MANA, POWER_RAGE, POWER_FOCUS, POWER_ENERGY,
                POWER_HAPPINESS, POWER_RUNES, POWER_RUNIC_POWER,
            ]))
        );
        assert_eq!(
            result.player_fields.get(&PlayerField::Bytes),
            Some(&FieldValue::BytesArray(vec![BYTES, BYTES, BYTES]))
        );
        assert_eq!(
            result.player_fields.get(&PlayerField::Xp), Some(&FieldValue::Integer(XP))
        );
        assert_eq!(
            result.player_fields.get(&PlayerField::InvSlot),
            Some(&FieldValue::LongArray(inv_slots))
        );
        assert_eq!(
            result.player_fields.get(&PlayerField::KeyringSlot),
            Some(&FieldValue::LongArray(keyring_slots))
        );
        assert_eq!(
            result.player_fields.get(&PlayerField::CurrencyTokenSlot),
            Some(&FieldValue::LongArray(cur_token_slots))
        );
        assert_eq!(
            result.player_fields.get(&PlayerField::CharacterPoints),
            Some(&FieldValue::IntegerArray(character_points))
        );
        assert_eq!(
            result.player_fields.get(&PlayerField::KnownTitles),
            Some(&FieldValue::LongArray(known_titles))
        );
    }
}

bitflags! {
    #[derive(Default, Debug)]
    pub struct ObjectTypeMask: i32 {
        const NONE = 0x0000;
        const OBJECT = 0x0001;
        const ITEM = 0x0002;
        const CONTAINER = 0x0004;
        const UNIT = 0x0008;
        const PLAYER = 0x0010;
        const GAMEOBJECT = 0x0020;
        const DYNAMICOBJECT = 0x0040;
        const CORPSE = 0x0080;
    }
}

#[non_exhaustive]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BlockType(pub u8);

#[allow(dead_code)]
impl BlockType {
    pub const VALUES: u8 = 0;
    pub const MOVEMENT: u8 = 1;
    pub const CREATE_OBJECT: u8 = 2;
    pub const CREATE_OBJECT2: u8 = 3;
    pub const OUT_OF_RANGE_OBJECTS: u8 = 4;
    pub const NEAR_OBJECTS: u8 = 5;

    pub fn new(value: u8) -> Self {
        Self(value)
    }
}

impl BinaryConverter for BlockType {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        u8::write_into(&mut self.0, buffer)?;

        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, dependencies: &mut Vec<u8>) -> anyhow::Result<Self> {
        let value = u8::read_from(reader, dependencies)?;

        Ok(Self(value))
    }
}

impl Serialize for BlockType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
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
#[derive(Debug, Clone, Copy, PartialEq)]
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

    pub fn new(value: i8) -> Self {
        Self(value)
    }

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
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        i8::write_into(&mut self.0, buffer)?;

        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, dependencies: &mut Vec<u8>) -> anyhow::Result<Self> {
        let value = i8::read_from(reader, dependencies)?;

        Ok(Self(value))
    }
}

impl Serialize for ObjectTypeID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
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