use anyhow::{anyhow, Result as AnyResult};
use std::collections::{BTreeMap};
use std::ops::Range;
use core::slice::Iter;
use serde::{Serialize, Serializer};
use serde::ser::{SerializeSeq, SerializeTuple};

#[macro_export]
macro_rules! fields {
    (
        $(#[$enum_attr:meta])*
        pub enum $enum_name:ident {
            $(
                $(#[$variant_attr:meta])*
                $field_type:ident $(( $($custom_types:ident),+ ))?
                $([$len:expr])? $variant:ident = $start_index:expr$(,)?
            )*
        }
    ) => {
        $(#[$enum_attr])*
        #[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
        #[allow(dead_code)]
        pub enum $enum_name {
            $(
                $(#[$variant_attr])*
                $variant,
            )*
        }

        #[allow(dead_code)]
        impl $enum_name {
            pub fn get_field_name(&self) -> String {
                match self {
                    $(
                        Self::$variant => format!("{}::{}",
                            stringify!($enum_name),
                            tentacli_utils::camel_to_upper_snake_case(
                                stringify!($variant)
                            ).to_uppercase()
                        ),
                    )*
                }
            }

            pub fn get_limit() -> u32 {
                let mut max_value: u32 = 0;
                $(
                    max_value = max_value.max($start_index);
                )*
                max_value.wrapping_add(1)
            }

            pub fn read_from(
                buffer: Vec<u32>,
                update_mask: &mut Vec<bool>,
            ) -> AnyResult<BTreeMap<Self, FieldValue>> {
                let mut fields = BTreeMap::new();

                let indices_to_update: Vec<u32> = update_mask.iter()
                    .enumerate()
                    .filter_map(|(index, &value)| if value { Some(index as u32) } else { None })
                    .collect();

                let mut buffer_iter = buffer.iter();

                $(
                    #[allow(unused_assignments)]
                    #[allow(unused_mut)]
                    let mut size: u32 = 1;
                    $(
                        size = $len;
                    )?

                    let field_type = stringify!($field_type);

                    let types = match field_type {
                        "Custom" => {
                            #[allow(unused_mut)]
                            let mut types = Vec::new();
                            $(
                                $(
                                    types.push(stringify!($custom_types).to_string());
                                )*
                            )?
                            types
                        }
                        _ => vec![field_type.to_string()],
                    };

                    // TODO: rename this
                    let offset = types.iter().fold(0, |sum, field_type| {
                        sum + match field_type.as_str() {
                            "Long" => 2,
                            _ => 1
                        }
                    });

                    let range: Range<u32> = $start_index..($start_index + size * offset);

                    let field_indices = indices_to_update.iter()
                        .filter(|&index| range.contains(index))
                        .cloned()
                        .collect::<Vec<u32>>();

                    if !field_indices.is_empty() {
                        let field_value = Self::read_value(field_type, types, &mut buffer_iter, field_indices, range)?;

                        if let Some(variant) = Self::get_variant_by_index($start_index) {
                            fields.insert(variant, field_value);
                        }
                    }
                )*

                Ok(fields)
            }

            fn read_value(
                field_type: &str,
                types: Vec<String>,
                buffer_iter: &mut Iter<u32>,
                field_indices: Vec<u32>,
                range: Range<u32>
            ) -> AnyResult<FieldValue> {
                let value = match field_type {
                    "Long" => {
                        let mut values: Vec<u32> = vec![];

                        for i in range {
                            if !field_indices.contains(&i) {
                                values.push(0);
                                continue;
                            }

                            values.push(*buffer_iter.next()
                                .ok_or(anyhow!("Cannot read item(u32 of u64)"))? as u32);
                        }

                        let mut values_u64 = vec![];
                        for i in (0..values.len()).step_by(2) {
                            let low = values[i] as u64;
                            let high = values[i + 1] as u64;
                            values_u64.push(low | (high << 32))
                        }

                        if values.len() > 2 {
                            FieldValue::LongArray(values_u64)
                        } else {
                            FieldValue::Long(values_u64[0])
                        }
                    }
                    "Integer" => {
                        let mut values = vec![];
                        for i in range {
                            if !field_indices.contains(&i) {
                                values.push(0);
                                continue;
                            }

                            values.push(*buffer_iter.next()
                                .ok_or(anyhow!("Cannot read item(i32)"))? as i32);
                        }

                        if values.len() > 1 {
                            FieldValue::IntegerArray(values)
                        } else {
                            FieldValue::Integer(values[0])
                        }
                    }
                    "Bytes" => {
                        let mut values = vec![];

                        for i in range {
                            if !field_indices.contains(&i) {
                                values.push(0);
                                continue;
                            }

                            values.push(*buffer_iter.next()
                                .ok_or(anyhow!("Cannot read item(bytes)"))?);
                        }

                        if values.len() > 1 {
                            FieldValue::BytesArray(values)
                        } else {
                            FieldValue::Bytes(values[0])
                        }
                    }
                    "Float" => {
                        let mut values = vec![];

                        for i in range {
                            if !field_indices.contains(&i) {
                                values.push(0.);
                                continue;
                            }

                            let value = *buffer_iter.next()
                                .ok_or(anyhow!("Cannot read item(f32)"))?;
                            values.push(f32::from_bits(value));
                        }

                        if values.len() > 1 {
                            FieldValue::FloatArray(values)
                        } else {
                            FieldValue::Float(values[0])
                        }
                    }
                    "TwoShorts" => {
                        let mut values = vec![];

                        for i in range {
                            if !field_indices.contains(&i) {
                                values.push((0, 0));
                                continue;
                            }

                            let value = *buffer_iter.next()
                                .ok_or(anyhow!("Cannot read item(two shorts)"))?;
                            let first = (value & 0xFFFF) as i16;
                            let second = (value >> 16) as i16;

                            values.push((first, second));
                        }

                        if values.len() > 1 {
                            FieldValue::TwoShortsArray(values)
                        } else {
                            FieldValue::TwoShorts(values[0])
                        }
                    }
                    "Custom" => {
                        let mut values = vec![];
                        let mut cycle_iter = types.iter().cycle();
                        let mut sub_values = vec![];

                        for i in range {
                            let field_type = cycle_iter.next()
                                .ok_or(anyhow!("Cannot read field type for custom"))?;

                            if !field_indices.contains(&i) {
                                sub_values.push(FieldValue::None);
                            } else {
                                let value = Self::read_value(
                                    field_type, vec![field_type.to_string()], buffer_iter, vec![i], i..i+1,
                                )?;
                                sub_values.push(value);
                            }

                            if sub_values.len() == types.len() {
                                values.push(sub_values.clone());
                                sub_values.clear();
                            }
                        }

                        if values.len() > 1 {
                            FieldValue::CustomArray(values)
                        } else {
                            FieldValue::Custom(values[0].clone())
                        }
                    },
                    _ => FieldValue::None
                };

                Ok(value)
            }

            fn get_name_by_index(index: u32) -> Option<String> {
                match index {
                    $(
                        $start_index => Some(stringify!($variant).to_string()),
                    )*
                    _ => None,
                }
            }

            fn get_variant_by_index(index: u32) -> Option<Self> {
                match index {
                    $(
                        $start_index => Some(Self::$variant),
                    )*
                    _ => None,
                }
            }

            pub fn get_index(variant: &$enum_name) -> u32 {
                match variant {
                    $(
                        Self::$variant => $start_index,
                    )*
                }
            }
        }
    };
}

fields! {
    pub enum ObjectField {
        Long Guid = 0,
        Integer Type = 2,
        Integer Entry = 3,
        Float ScaleX = 4
    }
}

fields! {
    pub enum UnitField {
        Long Charm = 6,
        Long Summon = 8,
        Long Critter = 10,
        Long CharmedBy = 12,
        Long SummonedBy = 14,
        Long CreatedBy = 16,
        Long Target = 18,
        Long ChannelObject = 20,
        Integer ChannelSpell = 22,
        Bytes Bytes0 = 23,
        Integer Health = 24,
        Integer[7] Powers = 25,
        Integer MaxHealth = 32,
        Integer[7] MaxPowers = 33,
        Float[7] PowerRegenFlatModifier = 40,
        Float[7] PowerRegenInterruptedFlatModifier = 47,
        Integer Level = 54,
        Integer FactionTemplate = 55,
        Integer[3] VirtualItemSlotId = 56,
        Integer Flags = 59,
        Integer Flags2 = 60,
        Integer AuraState = 61,
        Integer[2] BaseAttackTime = 62,
        Integer RangedAttackTime = 64,
        Float BoundingRadius = 65,
        Float CombatReach = 66,
        Integer DisplayId = 67,
        Integer NativeDisplayId = 68,
        Integer MountDisplayId = 69,
        Float MinDamage = 70,
        Float MaxDamage = 71,
        Float MinOffhandDamage = 72,
        Float MaxOffhandDamage = 73,
        Bytes Bytes1 = 74,
        Integer PetNumber = 75,
        Integer PetNameTimestamp = 76,
        Integer PetExperience = 77,
        Integer PetNextLevelExp = 78,
        Integer DynamicFlags = 79,
        Float ModCastSpeed = 80,
        Integer CreatedBySpell = 81,
        Integer NpcFlags = 82,
        Integer NpcEmoteState = 83,
        Integer[5] Stats = 84,
        Integer[5] PosStats = 89,
        Integer[5] NegStats = 94,
        Integer[7] Resistances = 99,
        Integer[7] ResistanceBuffModsPositive = 106,
        Integer[7] ResistanceBuffModsNegative = 113,
        Integer BaseMana = 120,
        Integer BaseHealth = 121,
        Bytes Bytes2 = 122,
        Integer AttackPower = 123,
        TwoShorts AttackPowerMods = 124,
        Float AttackPowerMultiplier = 125,
        Integer RangedAttackPower = 126,
        TwoShorts RangedAttackPowerMods = 127,
        Float RangedAttackPowerMultiplier = 128,
        Float MinRangedDamage = 129,
        Float MaxRangedDamage = 130,
        Integer[7] PowerCostModifier = 131,
        Float[7] PowerCostMultiplier = 138,
        Float MaxHealthModifier = 145,
        Float HoverHeight = 146
    }
}

fields! {
    pub enum PlayerField {
        Long DuelArbiter = 148,
        Integer Flags = 150,
        Integer GuildId = 151,
        Integer GuildRank = 152,
        Bytes[3] Bytes = 153,
        Integer DuelTeam = 156,
        Integer GuildTimestamp = 157,
        Custom (Integer, Integer, TwoShorts, TwoShorts, Integer)[25] QuestLog = 158,
        Custom (Integer, TwoShorts)[12] VisibleItems = 283,
        Integer ChosenTitle = 321,
        Integer FakeInebriation = 322,
        Integer Pad0 = 323,
        Long[23] InvSlot = 324,
        Long[16] PackSlot = 370,
        Long[28] BankSlot = 402,
        Long[7] BankBagSlot = 458,
        Long[12] VendorBuybackSlot = 472,
        Long[32] KeyringSlot = 496,
        Long[32] CurrencyTokenSlot = 560,
        Long Farsight = 624,
        Long[3] KnownTitles = 626,
        Long KnownCurrencies = 632,
        Integer Xp = 634,
        Integer NextLevelXp = 635,
        TwoShorts[384] SkillInfo = 636,
        Integer[2] CharacterPoints = 1020,
        Integer TrackCreatures = 1022,
        Integer TrackResources = 1023,
        Float BlockPercentage = 1024,
        Float DodgePercentage = 1025,
        Float ParryPercentage = 1026,
        Integer Expertise = 1027,
        Integer OffhandExpertise = 1028,
        Float CritPercentage = 1029,
        Float RangedCritPercentage = 1030,
        Float OffhandCritPercentage = 1031,
        Float[7] SpellCritPercentage = 1032,
        Integer ShieldBlock = 1039,
        Float ShieldBlockCritPercentage = 1040,
        Bytes[128] ExploredZones = 1041,
        Integer RestStateExperience = 1169,
        Integer Coinage = 1170,
        Integer[7] ModDamageDonePos = 1171,
        Integer[7] ModDamageDoneNeg = 1178,
        Integer[7] ModDamageDonePct = 1185,
        Integer ModHealingDonePos = 1192,
        Float ModHealingPct = 1193,
        Float ModHealingDonePct = 1194,
        Integer ModTargetResistance = 1195,
        Integer ModTargetPhysicalResistance = 1196,
        Bytes FieldBytes = 1197,
        Long AmmoId = 1198,
        Integer SelfResSpell = 1199,
        Integer PvpMedals = 1200,
        Integer[12] BuybackPrice = 1201,
        Integer[12] BuybackTimestamp = 1213,
        TwoShorts Kills = 1225,
        Integer TodayContribution = 1226,
        Integer YesterdayContribution = 1227,
        Integer LifetimeHonorableKills = 1228,
        Integer FieldBytes2 = 1229,
        Integer WatchedFactionIndex = 1230,
        Integer CombatRating1 = 1231,
        Integer ArenaTeamInfo11 = 1256,
        Integer HonorCurrency = 1277,
        Integer ArenaCurrency = 1278,
        Integer MaxLevel = 1279,
        Integer DailyQuests1 = 1280,
        Float[4] RuneRegen = 1305,
        Integer NoReagentCost1 = 1309,
        Integer[6] GlyphSlots = 1312,
        Integer[6] Glyphs = 1318,
        Integer GlyphsEnabled = 1324,
        Integer PetSpellPower = 1325,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldValue {
    Integer(i32),
    IntegerArray(Vec<i32>),
    Long(u64),
    LongArray(Vec<u64>),
    Float(f32),
    FloatArray(Vec<f32>),
    Bytes(u32),
    BytesArray(Vec<u32>),
    TwoShorts((i16, i16)),
    TwoShortsArray(Vec<(i16, i16)>),
    Custom(Vec<FieldValue>),
    CustomArray(Vec<Vec<FieldValue>>),
    None,
}

impl Serialize for FieldValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self {
            FieldValue::Integer(value) => serializer.serialize_i32(*value),
            FieldValue::IntegerArray(array) => {
                let mut seq = serializer.serialize_seq(Some(array.len()))?;
                for item in array {
                    seq.serialize_element(&item)?;
                }
                seq.end()
            }
            FieldValue::Long(value) => serializer.serialize_u64(*value),
            FieldValue::LongArray(array) => {
                let mut seq = serializer.serialize_seq(Some(array.len()))?;
                for item in array {
                    seq.serialize_element(&item)?;
                }
                seq.end()
            },
            FieldValue::Float(value) => serializer.serialize_f32((value * 100.0).round() / 100.0),
            FieldValue::FloatArray(array) => {
                let mut seq = serializer.serialize_seq(Some(array.len()))?;
                for item in array {
                    let item = (item * 100.0).round() / 100.0;
                    seq.serialize_element(&item)?;
                }
                seq.end()
            },
            FieldValue::Bytes(value) => serializer.serialize_u32(*value),
            FieldValue::BytesArray(array) => {
                let mut seq = serializer.serialize_seq(Some(array.len()))?;
                for item in array {
                    seq.serialize_element(&item)?;
                }
                seq.end()
            },
            FieldValue::TwoShorts((value1, value2)) => {
                let mut tuple = serializer.serialize_tuple(2)?;
                tuple.serialize_element(&value1)?;
                tuple.serialize_element(&value2)?;
                tuple.end()
            },
            FieldValue::TwoShortsArray(array) => {
                let mut seq = serializer.serialize_seq(Some(array.len()))?;
                for item in array {
                    seq.serialize_element(&item)?;
                }
                seq.end()
            },
            FieldValue::Custom(array) => {
                serializer.serialize_none()
                // let bytes: Vec<u8> = array.iter().flat_map(|&x| x.to_le_bytes()).collect();
                // serializer.serialize_bytes(&bytes)
            },
            FieldValue::CustomArray(array) => {
                // let mut seq = serializer.serialize_seq(Some(array.len()))?;
                // for item in array {
                //     let bytes: Vec<u8> = item.iter().flat_map(|&x| x.to_le_bytes()).collect();
                //     seq.serialize_element(&bytes)?;
                // }
                // seq.end()
                serializer.serialize_none()
            },
            FieldValue::None => serializer.serialize_none(),
        }
    }
}