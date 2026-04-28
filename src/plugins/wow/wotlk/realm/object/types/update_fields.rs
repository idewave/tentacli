use anyhow::anyhow;
use serde::Serialize;
use std::collections::BTreeMap;
use std::ops::Range;

pub trait FieldEnum: Ord + Clone + Sized {
    fn get_field_name(&self) -> String;
    fn get_limit() -> u32;
    fn get_layout_slots(variant: &Self) -> u32;

    fn read_from(blocks: BTreeMap<u32, u32>) -> anyhow::Result<BTreeMap<Self, FieldValue>>;

    fn get_variant_by_index(index: u32) -> Option<Self>;
    fn get_index(variant: &Self) -> u32;

    fn read_value(
        field_type: &str,
        types: Vec<String>,
        blocks: &BTreeMap<u32, u32>,
        range: Range<u32>,
    ) -> anyhow::Result<FieldValue> {
        let value = match field_type {
            "Long" => {
                let mut values_u64: Vec<Option<u64>> = Vec::new();

                let mut i = range.start;
                while i < range.end {
                    let lo_opt = blocks.get(&i).copied();
                    let hi_opt = blocks.get(&(i + 1)).copied();

                    // Only None if BOTH parts are missing
                    let v = match (lo_opt, hi_opt) {
                        (None, None) => None,
                        (lo, hi) => {
                            let lo = lo.unwrap_or(0) as u64;
                            let hi = hi.unwrap_or(0) as u64;
                            Some(lo | (hi << 32))
                        }
                    };

                    values_u64.push(v);
                    i += 2;
                }

                if values_u64.len() > 1 {
                    FieldValue::LongArray(values_u64)
                } else {
                    match values_u64.pop().unwrap() {
                        Some(v) => FieldValue::Long(v),
                        None => FieldValue::None,
                    }
                }
            }

            "Integer" => {
                let mut values: Vec<Option<i32>> = Vec::new();

                for i in range {
                    if let Some(raw) = blocks.get(&i) {
                        values.push(Some(*raw as i32));
                    } else {
                        values.push(None);
                    }
                }

                if values.len() > 1 {
                    FieldValue::IntegerArray(values)
                } else {
                    match values.pop().unwrap() {
                        Some(v) => FieldValue::Integer(v),
                        None => FieldValue::None,
                    }
                }
            }

            "Bytes" => {
                let mut values: Vec<Option<u32>> = Vec::new();

                for i in range {
                    if let Some(raw) = blocks.get(&i) {
                        values.push(Some(*raw));
                    } else {
                        values.push(None);
                    }
                }

                if values.len() > 1 {
                    FieldValue::BytesArray(values)
                } else {
                    match values.pop().unwrap() {
                        Some(v) => FieldValue::Bytes(v),
                        None => FieldValue::None,
                    }
                }
            }

            "Float" => {
                let mut values: Vec<Option<f32>> = Vec::new();

                for i in range {
                    if let Some(raw) = blocks.get(&i) {
                        values.push(Some(f32::from_bits(*raw)));
                    } else {
                        values.push(None);
                    }
                }

                if values.len() > 1 {
                    FieldValue::FloatArray(values)
                } else {
                    match values.pop().unwrap() {
                        Some(v) => FieldValue::Float(v),
                        None => FieldValue::None,
                    }
                }
            }

            "TwoShorts" => {
                let mut values: Vec<Option<(i16, i16)>> = Vec::new();

                for i in range {
                    if let Some(raw) = blocks.get(&i) {
                        let first = (*raw & 0xFFFF) as i16;
                        let second = (*raw >> 16) as i16;
                        values.push(Some((first, second)));
                    } else {
                        values.push(None);
                    }
                }

                if values.len() > 1 {
                    FieldValue::TwoShortsArray(values)
                } else {
                    match values.pop().unwrap() {
                        Some(v) => FieldValue::TwoShorts(v),
                        None => FieldValue::None,
                    }
                }
            }

            "Custom" => {
                let mut rows: Vec<Vec<Option<FieldValue>>> = Vec::new();
                let mut cycle_iter = types.iter().cycle();
                let mut sub_values: Vec<Option<FieldValue>> = Vec::new();

                for i in range {
                    let sub_type = cycle_iter
                        .next()
                        .ok_or(anyhow!("Cannot read field type for custom"))?;

                    if blocks.contains_key(&i) {
                        let v = Self::read_value(
                            sub_type,
                            vec![sub_type.to_string()],
                            blocks,
                            i..i + 1,
                        )?;
                        sub_values.push(Some(v));
                    } else {
                        sub_values.push(None);
                    }

                    if sub_values.len() == types.len() {
                        rows.push(sub_values.clone());
                        sub_values.clear();
                    }
                }

                if rows.len() > 1 {
                    FieldValue::CustomArray(rows)
                } else {
                    let row = rows.pop().unwrap_or_default();
                    let flat = row
                        .into_iter()
                        .map(|v| v.unwrap_or(FieldValue::None))
                        .collect();

                    FieldValue::Custom(flat)
                }
            }

            _ => FieldValue::None,
        };

        Ok(value)
    }
}

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
        #[derive(Serialize, Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
        #[allow(dead_code)]
        pub enum $enum_name {
            $(
                $(#[$variant_attr])*
                #[doc = concat!(
                    stringify!(FieldValue::$field_type ),
                    $(
                        "Array(", stringify!($len), ")"
                    )?
                )]
                $variant,
            )*
        }

        #[allow(dead_code)]
        impl FieldEnum for $enum_name {
            fn get_field_name(&self) -> String {
                match self {
                    $(
                        Self::$variant => stringify!($variant).to_string(),
                    )*
                }
            }

            fn get_layout_slots(variant: &Self) -> u32 {
                match variant {
                    $(
                        Self::$variant => {
                            #[allow(unused_mut, unused_assignments)]
                            let mut size: u32 = 1;
                            $(
                                size = $len;
                            )?

                            let field_type = stringify!($field_type);
                            let types: Vec<&str> = match field_type {
                                "Custom" => {
                                    vec![
                                        $(
                                            $(
                                                stringify!($custom_types),
                                            )*
                                        )?
                                    ]
                                }
                                _ => vec![field_type],
                            };


                            let slot_width: u32 = types.iter().map(|t| {
                                match *t {
                                    "Long" => 2,
                                    _ => 1,
                                }
                            }).sum();

                            size * slot_width
                        }
                    )*
                }
            }

            fn get_limit() -> u32 {
                let mut max_end: u32 = 0;

                $(
                    // how many logical elements (array length or 1)
                    #[allow(unused_mut, unused_assignments)]
                    let mut size: u32 = 1;
                    $(
                        size = $len;
                    )?

                    let field_type = stringify!($field_type);

                    // how many slots one element occupies
                   let types: Vec<&str> = match field_type {
                        "Custom" => {
                            vec![
                                $(
                                    $(
                                        stringify!($custom_types),
                                    )*
                                )?
                            ]
                        }
                        _ => vec![field_type],
                    };

                    let slot_width: u32 = types.iter().map(|t| {
                        match *t {
                            "Long" => 2,
                            _ => 1,
                        }
                    }).sum();

                    let field_end = $start_index + size * slot_width;
                    max_end = max_end.max(field_end);
                )*

                max_end
            }

            fn read_from(blocks: BTreeMap<u32, u32>) -> anyhow::Result<BTreeMap<Self, FieldValue>> {
                let mut fields = BTreeMap::new();

                $(
                    #[allow(unused_mut, unused_assignments)]
                    let mut size: u32 = 1;
                    $(
                        size = $len;
                    )?

                    let field_type = stringify!($field_type);

                    let types = match field_type {
                        "Custom" => {
                            vec![
                                $(
                                    $(
                                        stringify!($custom_types).to_string(),
                                    )*
                                )?
                            ]
                        }
                        _ => vec![field_type.to_string()],
                    };

                    let offset = types.iter().fold(0, |sum, field_type| {
                        sum + match field_type.as_str() {
                            "Long" => 2,
                            _ => 1
                        }
                    });

                    let range: Range<u32> = $start_index..($start_index + size * offset);
                    let has_any = range.clone().any(|i| blocks.contains_key(&i));

                    if has_any {
                        let field_value = Self::read_value(
                            field_type,
                            types,
                            &blocks,
                            range.clone()
                        )?;

                        if let Some(variant) = Self::get_variant_by_index($start_index) {
                            fields.insert(variant, field_value);
                        }
                    }
                )*

                Ok(fields)
            }

            fn get_variant_by_index(index: u32) -> Option<Self> {
                match index {
                    $(
                        $start_index => Some(Self::$variant),
                    )*
                    _ => None,
                }
            }

            fn get_index(variant: &$enum_name) -> u32 {
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
        Integer[25] CombatRating = 1231,
        Integer[21] ArenaTeamInfo1 = 1256,
        Integer HonorCurrency = 1277,
        Integer ArenaCurrency = 1278,
        Integer MaxLevel = 1279,
        Integer[25] DailyQuests = 1280,
        Float[4] RuneRegen = 1305,
        Integer[3] NoReagentCost = 1309,
        Integer[6] GlyphSlots = 1312,
        Integer[6] Glyphs = 1318,
        Integer GlyphsEnabled = 1324,
        Integer PetSpellPower = 1325,
    }
}

fields! {
    pub enum ItemField {
        Long Owner = 6,
        Long Contained = 8,
        Long Creator = 10,
        Long Giftcreator = 12,
        Integer StackCount = 14,
        Integer Duration = 15,
        Integer[5] SpellCharges = 16,
        Integer Flags = 21,
        Custom (Integer, Integer, TwoShorts)[12] Enchantment = 22,
        Integer PropertySeed = 58,
        Integer RandomPropertiesId = 59,
        Integer Durability = 60,
        Integer MaxDurability = 61,
        Integer CreatePlayedTime = 62,
    }
}

fields! {
    pub enum ContainerField {
        Integer NumSlots = 64,
        Bytes AlignPad = 65,
        Long[36] Slot = 66,
    }
}

fields! {
    pub enum GameObjectField {
        Long CreatedBy = 6,
        Integer DisplayId = 8,
        Integer Flags = 9,
        Float[4] ParentRotation = 10,
        TwoShorts Dynamic = 14,
        Integer Faction = 15,
        Integer Level = 16,
        Bytes Bytes1 = 17,
    }
}

fields! {
    pub enum DynamicObjectField {
        Long Caster = 6,
        Bytes Bytes = 8,
        Integer SpellId = 9,
        Float Radius = 10,
        Integer CastTime = 11,
    }
}

fields! {
    pub enum CorpseField {
        Long Owner = 6,
        Long Party = 8,
        Integer DisplayId = 10,
        Integer[19] Item = 11,
        Bytes Bytes1 = 36,
        Bytes Bytes2 = 37,
        Integer Guild = 38,
        Integer Flags = 39,
        Integer DynamicFlags = 40,
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum FieldValue {
    Integer(i32),
    IntegerArray(Vec<Option<i32>>),
    Long(u64),
    LongArray(Vec<Option<u64>>),
    Float(f32),
    FloatArray(Vec<Option<f32>>),
    Bytes(u32),
    BytesArray(Vec<Option<u32>>),
    TwoShorts((i16, i16)),
    TwoShortsArray(Vec<Option<(i16, i16)>>),
    Custom(Vec<FieldValue>),
    CustomArray(Vec<Vec<Option<FieldValue>>>),
    None,
}

impl FieldValue {
    /// Returns the exact byte size this field occupies in the WoW UpdateObject values stream.
    /// This MUST be structural, not based on Some/None presence.
    pub fn len(&self) -> usize {
        match self {
            // Scalar fields
            FieldValue::Integer(_) => 4,
            FieldValue::Float(_) => 4,
            FieldValue::Bytes(_) => 4,
            FieldValue::TwoShorts(_) => 4,
            FieldValue::Long(_) => 8,

            // Arrays: size = slots * element_size
            FieldValue::IntegerArray(v) => 4 * v.len(),
            FieldValue::FloatArray(v) => 4 * v.len(),
            FieldValue::BytesArray(v) => 4 * v.len(),
            FieldValue::TwoShortsArray(v) => 4 * v.len(),
            FieldValue::LongArray(v) => 8 * v.len(),

            // Custom = sum of children (structural, not "present-only")
            FieldValue::Custom(values) => values.iter().map(|v| v.len()).sum(),

            // CustomArray = rows * sum(slot_size per row)
            // Each slot always occupies space even if it's None
            FieldValue::CustomArray(rows) => {
                rows.iter()
                    .map(|row| {
                        row.iter()
                            .map(|slot| {
                                match slot {
                                    Some(v) => v.len(),
                                    None => 4, // minimum slot size in UpdateObject (u32)
                                }
                            })
                            .sum::<usize>()
                    })
                    .sum()
            }

            FieldValue::None => 0,
        }
    }

    #[inline]
    pub fn is_array_like(&self) -> bool {
        matches!(
            self,
            FieldValue::LongArray(_)
                | FieldValue::IntegerArray(_)
                | FieldValue::BytesArray(_)
                | FieldValue::FloatArray(_)
                | FieldValue::TwoShortsArray(_)
                | FieldValue::CustomArray(_)
        )
    }

    #[inline]
    pub fn element_stride_slots(&self) -> usize {
        match self {
            FieldValue::Long(_) | FieldValue::LongArray(_) => 2,
            _ => 1,
        }
    }
}
