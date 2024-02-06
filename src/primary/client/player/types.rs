use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeStruct;

use crate::primary::parsers::position_parser::types::Position;

#[derive(Clone, Default)]
pub struct Player {
    pub guid: u64,
    pub name: String,
    pub race: u8,
    pub class: u8,
    pub gender: u8,
    pub level: u8,
    pub fields: BTreeMap<u32, FieldValue>,
    pub movement_speed: BTreeMap<u8, f32>,
    pub position: Option<Position>,
}

impl Player {
    pub fn new(guid: u64, name: String, race: u8, class: u8, gender: u8, level: u8) -> Self {
        Self {
            guid,
            name,
            race,
            class,
            gender,
            level,
            fields: BTreeMap::new(),
            movement_speed: BTreeMap::new(),
            position: None,
        }
    }
}

impl Debug for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "guid: {:?}, name: {:?}, race: {:?}, class: {:?}\n \
            position: {:?}\nFields: \n{:?}\nMovement speed:\n{:?}",
            self.guid,
            self.name,
            self.race,
            self.class,
            self.position,
            self.fields,
            self.movement_speed,
        )
    }
}

impl<'de> Deserialize<'de> for Player {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        todo!()
    }
}

impl Serialize for Player {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        const FIELDS_AMOUNT: usize = 7;
        let mut state = serializer.serialize_struct("Character", FIELDS_AMOUNT)?;
        state.serialize_field("guid", &self.guid)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("race", &self.race)?;
        state.serialize_field("class", &self.class)?;
        state.serialize_field("gender", &self.gender)?;
        state.serialize_field("level", &self.level)?;
        state.serialize_field("position", &self.position)?;
        state.end()
    }
}

#[non_exhaustive]
pub struct Gender;

#[allow(dead_code)]
impl Gender {
    pub const GENDER_MALE: u8 = 0;
    pub const GENDER_FEMALE: u8 = 1;
    pub const GENDER_NONE: u8 = 2;
}

#[non_exhaustive]
pub struct Race;

#[allow(dead_code)]
impl Race {
    pub const HUMAN: u8 = 1;
    pub const ORC: u8 = 2;
    pub const DWARF: u8 = 3;
    pub const NIGHTELF: u8 = 4;
    pub const UNDEAD: u8 = 5;
    pub const TAUREN: u8 = 6;
    pub const GNOME: u8 = 7;
    pub const TROLL: u8 = 8;
    pub const GOBLIN: u8 = 9;
    pub const BLOODELF: u8 = 10;
    pub const DRAENEI: u8 = 11;
    pub const FEL_ORC: u8 = 12;
    pub const NAGA: u8 = 13;
    pub const BROKEN: u8 = 14;
    pub const SKELETON: u8 = 15;
    pub const VRYKUL: u8 = 16;
    pub const TUSKARR: u8 = 17;
    pub const FOREST_TROLL: u8 = 18;
    pub const TAUNKA: u8 = 19;
    pub const NORTHREND_SKELETON: u8 = 20;
    pub const ICE_TROLL: u8 = 21;
}

#[non_exhaustive]
pub struct Class;

#[allow(dead_code)]
impl Class {
    pub const WARRIOR: u8 = 1;
    pub const PALADIN: u8 = 2;
    pub const HUNTER: u8 = 3;
    pub const ROGUE: u8 = 4;
    pub const PRIEST: u8 = 5;
    pub const DEATH_KNIGHT: u8 = 6;
    pub const SHAMAN: u8 = 7;
    pub const MAGE: u8 = 8;
    pub const WARLOCK: u8 = 9;
    pub const DRUID: u8 = 11;
}

#[derive(PartialEq)]
pub enum FieldType {
    INT,
    LONG,
    FLOAT,
    NONE,
}

#[derive(Debug, Clone)]
pub enum FieldValue {
    INT(u32),
    LONG(u64),
    FLOAT(f32),
}

impl Serialize for FieldValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match *self {
            FieldValue::INT(value) => serializer.serialize_u32(value),
            FieldValue::LONG(value) => serializer.serialize_u64(value),
            FieldValue::FLOAT(value) => serializer.serialize_f32((value * 100.0).round() / 100.0),
        }
    }
}

#[non_exhaustive]
pub struct ObjectField;

#[allow(dead_code)]
impl ObjectField {
    pub const GUID: u32 = 0;
    pub const TYPE: u32 = 2;
    pub const ENTRY: u32 = 3;
    pub const SCALE_X: u32 = 4;

    // for check
    pub const LIMIT: u32 = 5;

    pub fn get_field_type(field_index: u32) -> FieldType {
        match field_index {
            ObjectField::GUID => FieldType::LONG,
            ObjectField::TYPE => FieldType::INT,
            ObjectField::ENTRY => FieldType::INT,
            ObjectField::SCALE_X => FieldType::FLOAT,
            _ => FieldType::NONE,
        }
    }

    pub fn get_field_name(field_index: u32) -> String {
        match field_index {
            ObjectField::GUID => String::from("ObjectField::GUID"),
            ObjectField::TYPE => String::from("ObjectField::TYPE"),
            ObjectField::ENTRY => String::from("ObjectField::ENTRY"),
            ObjectField::SCALE_X => String::from("ObjectField::SCALE_X"),
            _ => String::new(),
        }
    }
}

pub struct UnitField;

#[allow(dead_code)]
impl UnitField {
    pub const CHARM: u32 = 6;
    pub const SUMMON: u32 = 8;
    pub const CRITTER: u32 = 10;
    pub const CHARMEDBY: u32 = 12;
    pub const SUMMONEDBY: u32 = 14;
    pub const CREATEDBY: u32 = 16;
    pub const TARGET: u32 = 18;
    pub const CHANNEL_OBJECT: u32 = 20;
    pub const CHANNEL_SPELL: u32 = 22;
    pub const BYTES_0: u32 = 23;
    pub const HEALTH: u32 = 24;
    pub const POWER1: u32 = 25;
    pub const POWER2: u32 = 26;
    pub const POWER3: u32 = 27;
    pub const POWER4: u32 = 28;
    pub const POWER5: u32 = 29;
    pub const POWER6: u32 = 30;
    pub const POWER7: u32 = 31;
    pub const MAXHEALTH: u32 = 32;
    pub const MAXPOWER1: u32 = 33;
    pub const MAXPOWER2: u32 = 34;
    pub const MAXPOWER3: u32 = 35;
    pub const MAXPOWER4: u32 = 36;
    pub const MAXPOWER5: u32 = 37;
    pub const MAXPOWER6: u32 = 38;
    pub const MAXPOWER7: u32 = 39;
    pub const POWER_REGEN_FLAT_MODIFIER: u32 = 40;
    pub const POWER_REGEN_INTERRUPTED_FLAT_MODIFIER: u32 = 47;
    pub const LEVEL: u32 = 54;
    pub const FACTIONTEMPLATE: u32 = 55;
    pub const VIRTUAL_ITEM_SLOT_ID1: u32 = 56;
    pub const VIRTUAL_ITEM_SLOT_ID2: u32 = 57;
    pub const VIRTUAL_ITEM_SLOT_ID3: u32 = 58;
    pub const FLAGS: u32 = 59;
    pub const FLAGS_2: u32 = 60;
    pub const AURASTATE: u32 = 61;
    pub const BASEATTACKTIME: u32 = 62;
    pub const UNK63: u32 = 63;
    pub const RANGEDATTACKTIME: u32 = 64;
    pub const BOUNDINGRADIUS: u32 = 65;
    pub const COMBATREACH: u32 = 66;
    pub const DISPLAYID: u32 = 67;
    pub const NATIVEDISPLAYID: u32 = 68;
    pub const MOUNTDISPLAYID: u32 = 69;
    pub const MINDAMAGE: u32 = 70;
    pub const MAXDAMAGE: u32 = 71;
    pub const MINOFFHANDDAMAGE: u32 = 72;
    pub const MAXOFFHANDDAMAGE: u32 = 73;
    pub const BYTES_1: u32 = 74;
    pub const PETNUMBER: u32 = 75;
    pub const PET_NAME_TIMESTAMP: u32 = 76;
    pub const PETEXPERIENCE: u32 = 77;
    pub const PETNEXTLEVELEXP: u32 = 78;
    pub const DYNAMIC_FLAGS: u32 = 79;
    pub const MOD_CAST_SPEED: u32 = 80;
    pub const CREATED_BY_SPELL: u32 = 81;
    pub const NPC_FLAGS: u32 = 82;
    pub const NPC_EMOTESTATE: u32 = 83;
    pub const STAT0: u32 = 84;
    pub const STAT1: u32 = 85;
    pub const STAT2: u32 = 86;
    pub const STAT3: u32 = 87;
    pub const STAT4: u32 = 88;
    pub const POSSTAT0: u32 = 89;
    pub const POSSTAT1: u32 = 90;
    pub const POSSTAT2: u32 = 91;
    pub const POSSTAT3: u32 = 92;
    pub const POSSTAT4: u32 = 93;
    pub const NEGSTAT0: u32 = 94;
    pub const NEGSTAT1: u32 = 95;
    pub const NEGSTAT2: u32 = 96;
    pub const NEGSTAT3: u32 = 97;
    pub const NEGSTAT4: u32 = 98;
    pub const RESISTANCES_ARMOR: u32 = 99;
    pub const RESISTANCES_HOLY: u32 = 100;
    pub const RESISTANCES_FIRE: u32 = 101;
    pub const RESISTANCES_NATURE: u32 = 102;
    pub const RESISTANCES_FROST: u32 = 103;
    pub const RESISTANCES_SHADOW: u32 = 104;
    pub const RESISTANCES_ARCANE: u32 = 105;
    pub const RESISTANCEBUFFMODSPOSITIVE_ARMOR: u32 = 106;
    pub const RESISTANCEBUFFMODSPOSITIVE_HOLY: u32 = 107;
    pub const RESISTANCEBUFFMODSPOSITIVE_FIRE: u32 = 108;
    pub const RESISTANCEBUFFMODSPOSITIVE_NATURE: u32 = 109;
    pub const RESISTANCEBUFFMODSPOSITIVE_FROST: u32 = 110;
    pub const RESISTANCEBUFFMODSPOSITIVE_SHADOW: u32 = 111;
    pub const RESISTANCEBUFFMODSPOSITIVE_ARCANE: u32 = 112;
    pub const RESISTANCEBUFFMODSNEGATIVE_ARMOR: u32 = 113;
    pub const RESISTANCEBUFFMODSNEGATIVE_HOLY: u32 = 114;
    pub const RESISTANCEBUFFMODSNEGATIVE_FIRE: u32 = 115;
    pub const RESISTANCEBUFFMODSNEGATIVE_NATURE: u32 = 116;
    pub const RESISTANCEBUFFMODSNEGATIVE_FROST: u32 = 117;
    pub const RESISTANCEBUFFMODSNEGATIVE_SHADOW: u32 = 118;
    pub const RESISTANCEBUFFMODSNEGATIVE_ARCANE: u32 = 119;
    pub const BASE_MANA: u32 = 120;
    pub const BASE_HEALTH: u32 = 121;
    pub const BYTES_2: u32 = 122;
    pub const ATTACK_POWER: u32 = 123;
    pub const ATTACK_POWER_MODS: u32 = 124;
    pub const ATTACK_POWER_MULTIPLIER: u32 = 125;
    pub const RANGED_ATTACK_POWER: u32 = 126;
    pub const RANGED_ATTACK_POWER_MODS: u32 = 127;
    pub const RANGED_ATTACK_POWER_MULTIPLIER: u32 = 128;
    pub const MINRANGEDDAMAGE: u32 = 129;
    pub const MAXRANGEDDAMAGE: u32 = 130;
    pub const POWER_COST_MODIFIER: u32 = 131;
    pub const POWER_COST_MULTIPLIER1: u32 = 138;
    pub const POWER_COST_MULTIPLIER2: u32 = 139;
    pub const POWER_COST_MULTIPLIER3: u32 = 140;
    pub const POWER_COST_MULTIPLIER4: u32 = 141;
    pub const POWER_COST_MULTIPLIER5: u32 = 142;
    pub const POWER_COST_MULTIPLIER6: u32 = 143;
    pub const POWER_COST_MULTIPLIER7: u32 = 144;
    pub const MAXHEALTHMODIFIER: u32 = 145;
    pub const HOVERHEIGHT: u32 = 146;

    // for check
    pub const LIMIT: u32 = 147;

    pub fn get_field_type(field_index: u32) -> FieldType {
        match field_index {
            UnitField::CHARM => FieldType::LONG,
            UnitField::SUMMON => FieldType::LONG,
            UnitField::CRITTER => FieldType::LONG,
            UnitField::CHARMEDBY => FieldType::LONG,
            UnitField::SUMMONEDBY => FieldType::LONG,
            UnitField::CREATEDBY => FieldType::LONG,
            UnitField::TARGET => FieldType::LONG,
            UnitField::CHANNEL_OBJECT => FieldType::LONG,
            UnitField::CHANNEL_SPELL => FieldType::INT,
            UnitField::BYTES_0 => FieldType::INT,
            UnitField::HEALTH => FieldType::INT,
            UnitField::POWER1 => FieldType::INT,
            UnitField::POWER2 => FieldType::INT,
            UnitField::POWER3 => FieldType::INT,
            UnitField::POWER4 => FieldType::INT,
            UnitField::POWER5 => FieldType::INT,
            UnitField::POWER6 => FieldType::INT,
            UnitField::POWER7 => FieldType::INT,
            UnitField::MAXHEALTH => FieldType::INT,
            UnitField::MAXPOWER1 => FieldType::INT,
            UnitField::MAXPOWER2 => FieldType::INT,
            UnitField::MAXPOWER3 => FieldType::INT,
            UnitField::MAXPOWER4 => FieldType::INT,
            UnitField::MAXPOWER5 => FieldType::INT,
            UnitField::MAXPOWER6 => FieldType::INT,
            UnitField::MAXPOWER7 => FieldType::INT,
            UnitField::POWER_REGEN_FLAT_MODIFIER => FieldType::FLOAT,
            UnitField::POWER_REGEN_INTERRUPTED_FLAT_MODIFIER => FieldType::FLOAT,
            UnitField::LEVEL => FieldType::INT,
            UnitField::FACTIONTEMPLATE => FieldType::INT,
            UnitField::VIRTUAL_ITEM_SLOT_ID1 => FieldType::INT,
            UnitField::VIRTUAL_ITEM_SLOT_ID2 => FieldType::INT,
            UnitField::VIRTUAL_ITEM_SLOT_ID3 => FieldType::INT,
            UnitField::FLAGS => FieldType::INT,
            UnitField::FLAGS_2 => FieldType::INT,
            UnitField::AURASTATE => FieldType::INT,
            UnitField::BASEATTACKTIME => FieldType::INT,
            UnitField::UNK63 => FieldType::INT,
            UnitField::RANGEDATTACKTIME => FieldType::INT,
            UnitField::BOUNDINGRADIUS => FieldType::FLOAT,
            UnitField::COMBATREACH => FieldType::FLOAT,
            UnitField::DISPLAYID => FieldType::INT,
            UnitField::NATIVEDISPLAYID => FieldType::INT,
            UnitField::MOUNTDISPLAYID => FieldType::INT,
            UnitField::MINDAMAGE => FieldType::FLOAT,
            UnitField::MAXDAMAGE => FieldType::FLOAT,
            UnitField::MINOFFHANDDAMAGE => FieldType::FLOAT,
            UnitField::MAXOFFHANDDAMAGE => FieldType::FLOAT,
            UnitField::BYTES_1 => FieldType::INT,
            UnitField::PETNUMBER => FieldType::INT,
            UnitField::PET_NAME_TIMESTAMP => FieldType::INT,
            UnitField::PETEXPERIENCE => FieldType::INT,
            UnitField::PETNEXTLEVELEXP => FieldType::INT,
            UnitField::DYNAMIC_FLAGS => FieldType::INT,
            UnitField::MOD_CAST_SPEED => FieldType::FLOAT,
            UnitField::CREATED_BY_SPELL => FieldType::INT,
            UnitField::NPC_FLAGS => FieldType::INT,
            UnitField::NPC_EMOTESTATE => FieldType::INT,
            UnitField::STAT0 => FieldType::INT,
            UnitField::STAT1 => FieldType::INT,
            UnitField::STAT2 => FieldType::INT,
            UnitField::STAT3 => FieldType::INT,
            UnitField::STAT4 => FieldType::INT,
            UnitField::POSSTAT0 => FieldType::INT,
            UnitField::POSSTAT1 => FieldType::INT,
            UnitField::POSSTAT2 => FieldType::INT,
            UnitField::POSSTAT3 => FieldType::INT,
            UnitField::POSSTAT4 => FieldType::INT,
            UnitField::NEGSTAT0 => FieldType::INT,
            UnitField::NEGSTAT1 => FieldType::INT,
            UnitField::NEGSTAT2 => FieldType::INT,
            UnitField::NEGSTAT3 => FieldType::INT,
            UnitField::NEGSTAT4 => FieldType::INT,
            UnitField::RESISTANCES_ARMOR => FieldType::INT,
            UnitField::RESISTANCES_HOLY => FieldType::INT,
            UnitField::RESISTANCES_FIRE => FieldType::INT,
            UnitField::RESISTANCES_NATURE => FieldType::INT,
            UnitField::RESISTANCES_FROST => FieldType::INT,
            UnitField::RESISTANCES_SHADOW => FieldType::INT,
            UnitField::RESISTANCES_ARCANE => FieldType::INT,
            UnitField::RESISTANCEBUFFMODSPOSITIVE_ARMOR => FieldType::INT,
            UnitField::RESISTANCEBUFFMODSPOSITIVE_HOLY => FieldType::INT,
            UnitField::RESISTANCEBUFFMODSPOSITIVE_FIRE => FieldType::INT,
            UnitField::RESISTANCEBUFFMODSPOSITIVE_NATURE => FieldType::INT,
            UnitField::RESISTANCEBUFFMODSPOSITIVE_FROST => FieldType::INT,
            UnitField::RESISTANCEBUFFMODSPOSITIVE_SHADOW => FieldType::INT,
            UnitField::RESISTANCEBUFFMODSPOSITIVE_ARCANE => FieldType::INT,
            UnitField::RESISTANCEBUFFMODSNEGATIVE_ARMOR => FieldType::INT,
            UnitField::RESISTANCEBUFFMODSNEGATIVE_HOLY => FieldType::INT,
            UnitField::RESISTANCEBUFFMODSNEGATIVE_FIRE => FieldType::INT,
            UnitField::RESISTANCEBUFFMODSNEGATIVE_NATURE => FieldType::INT,
            UnitField::RESISTANCEBUFFMODSNEGATIVE_FROST => FieldType::INT,
            UnitField::RESISTANCEBUFFMODSNEGATIVE_SHADOW => FieldType::INT,
            UnitField::RESISTANCEBUFFMODSNEGATIVE_ARCANE => FieldType::INT,
            UnitField::BASE_MANA => FieldType::INT,
            UnitField::BASE_HEALTH => FieldType::INT,
            UnitField::BYTES_2 => FieldType::INT,
            UnitField::ATTACK_POWER => FieldType::INT,
            UnitField::ATTACK_POWER_MODS => FieldType::INT,
            UnitField::ATTACK_POWER_MULTIPLIER => FieldType::FLOAT,
            UnitField::RANGED_ATTACK_POWER => FieldType::INT,
            UnitField::RANGED_ATTACK_POWER_MODS => FieldType::INT,
            UnitField::RANGED_ATTACK_POWER_MULTIPLIER => FieldType::FLOAT,
            UnitField::MINRANGEDDAMAGE => FieldType::FLOAT,
            UnitField::MAXRANGEDDAMAGE => FieldType::FLOAT,
            UnitField::POWER_COST_MODIFIER => FieldType::INT,
            UnitField::POWER_COST_MULTIPLIER1 => FieldType::FLOAT,
            UnitField::POWER_COST_MULTIPLIER2 => FieldType::FLOAT,
            UnitField::POWER_COST_MULTIPLIER3 => FieldType::FLOAT,
            UnitField::POWER_COST_MULTIPLIER4 => FieldType::FLOAT,
            UnitField::POWER_COST_MULTIPLIER5 => FieldType::FLOAT,
            UnitField::POWER_COST_MULTIPLIER6 => FieldType::FLOAT,
            UnitField::POWER_COST_MULTIPLIER7 => FieldType::FLOAT,
            UnitField::MAXHEALTHMODIFIER => FieldType::FLOAT,
            UnitField::HOVERHEIGHT => FieldType::FLOAT,
            _ => FieldType::NONE,
        }
    }

    pub fn get_field_name(field_index: u32) -> String {
        match field_index {
            UnitField::CHARM => String::from("UnitField::CHARM"),
            UnitField::SUMMON => String::from("UnitField::SUMMON"),
            UnitField::CRITTER => String::from("UnitField::CRITTER"),
            UnitField::CHARMEDBY => String::from("UnitField::CHARMEDBY"),
            UnitField::SUMMONEDBY => String::from("UnitField::SUMMONEDBY"),
            UnitField::CREATEDBY => String::from("UnitField::CREATEDBY"),
            UnitField::TARGET => String::from("UnitField::TARGET"),
            UnitField::CHANNEL_OBJECT => String::from("UnitField::CHANNEL_OBJECT"),
            UnitField::CHANNEL_SPELL => String::from("UnitField::CHANNEL_SPELL"),
            UnitField::BYTES_0 => String::from("UnitField::BYTES_0"),
            UnitField::HEALTH => String::from("UnitField::HEALTH"),
            UnitField::POWER1 => String::from("UnitField::POWER1"),
            UnitField::POWER2 => String::from("UnitField::POWER2"),
            UnitField::POWER3 => String::from("UnitField::POWER3"),
            UnitField::POWER4 => String::from("UnitField::POWER4"),
            UnitField::POWER5 => String::from("UnitField::POWER5"),
            UnitField::POWER6 => String::from("UnitField::POWER6"),
            UnitField::POWER7 => String::from("UnitField::POWER7"),
            UnitField::MAXHEALTH => String::from("UnitField::MAXHEALTH"),
            UnitField::MAXPOWER1 => String::from("UnitField::MAXPOWER1"),
            UnitField::MAXPOWER2 => String::from("UnitField::MAXPOWER2"),
            UnitField::MAXPOWER3 => String::from("UnitField::MAXPOWER3"),
            UnitField::MAXPOWER4 => String::from("UnitField::MAXPOWER4"),
            UnitField::MAXPOWER5 => String::from("UnitField::MAXPOWER5"),
            UnitField::MAXPOWER6 => String::from("UnitField::MAXPOWER6"),
            UnitField::MAXPOWER7 => String::from("UnitField::MAXPOWER7"),
            UnitField::POWER_REGEN_FLAT_MODIFIER => String::from("UnitField::POWER_REGEN_FLAT_MODIFIER"),
            UnitField::POWER_REGEN_INTERRUPTED_FLAT_MODIFIER => String::from("UnitField::POWER_REGEN_INTERRUPTED_FLAT_MODIFIER"),
            UnitField::LEVEL => String::from("UnitField::LEVEL"),
            UnitField::FACTIONTEMPLATE => String::from("UnitField::FACTIONTEMPLATE"),
            UnitField::VIRTUAL_ITEM_SLOT_ID1 => String::from("UnitField::VIRTUAL_ITEM_SLOT_ID1"),
            UnitField::VIRTUAL_ITEM_SLOT_ID2 => String::from("UnitField::VIRTUAL_ITEM_SLOT_ID2"),
            UnitField::VIRTUAL_ITEM_SLOT_ID3 => String::from("UnitField::VIRTUAL_ITEM_SLOT_ID3"),
            UnitField::FLAGS => String::from("UnitField::FLAGS"),
            UnitField::FLAGS_2 => String::from("UnitField::FLAGS_2"),
            UnitField::AURASTATE => String::from("UnitField::AURASTATE"),
            UnitField::BASEATTACKTIME => String::from("UnitField::BASEATTACKTIME"),
            UnitField::UNK63 => String::from("UnitField::UNK63"),
            UnitField::RANGEDATTACKTIME => String::from("UnitField::RANGEDATTACKTIME"),
            UnitField::BOUNDINGRADIUS => String::from("UnitField::BOUNDINGRADIUS"),
            UnitField::COMBATREACH => String::from("UnitField::COMBATREACH"),
            UnitField::DISPLAYID => String::from("UnitField::DISPLAYID"),
            UnitField::NATIVEDISPLAYID => String::from("UnitField::NATIVEDISPLAYID"),
            UnitField::MOUNTDISPLAYID => String::from("UnitField::MOUNTDISPLAYID"),
            UnitField::MINDAMAGE => String::from("UnitField::MINDAMAGE"),
            UnitField::MAXDAMAGE => String::from("UnitField::MAXDAMAGE"),
            UnitField::MINOFFHANDDAMAGE => String::from("UnitField::MINOFFHANDDAMAGE"),
            UnitField::MAXOFFHANDDAMAGE => String::from("UnitField::MAXOFFHANDDAMAGE"),
            UnitField::BYTES_1 => String::from("UnitField::BYTES_1"),
            UnitField::PETNUMBER => String::from("UnitField::PETNUMBER"),
            UnitField::PET_NAME_TIMESTAMP => String::from("UnitField::PET_NAME_TIMESTAMP"),
            UnitField::PETEXPERIENCE => String::from("UnitField::PETEXPERIENCE"),
            UnitField::PETNEXTLEVELEXP => String::from("UnitField::PETNEXTLEVELEXP"),
            UnitField::DYNAMIC_FLAGS => String::from("UnitField::DYNAMIC_FLAGS"),
            UnitField::MOD_CAST_SPEED => String::from("UnitField::MOD_CAST_SPEED"),
            UnitField::CREATED_BY_SPELL => String::from("UnitField::CREATED_BY_SPELL"),
            UnitField::NPC_FLAGS => String::from("UnitField::NPC_FLAGS"),
            UnitField::NPC_EMOTESTATE => String::from("UnitField::NPC_EMOTESTATE"),
            UnitField::STAT0 => String::from("UnitField::STAT0"),
            UnitField::STAT1 => String::from("UnitField::STAT1"),
            UnitField::STAT2 => String::from("UnitField::STAT2"),
            UnitField::STAT3 => String::from("UnitField::STAT3"),
            UnitField::STAT4 => String::from("UnitField::STAT4"),
            UnitField::POSSTAT0 => String::from("UnitField::POSSTAT0"),
            UnitField::POSSTAT1 => String::from("UnitField::POSSTAT1"),
            UnitField::POSSTAT2 => String::from("UnitField::POSSTAT2"),
            UnitField::POSSTAT3 => String::from("UnitField::POSSTAT3"),
            UnitField::POSSTAT4 => String::from("UnitField::POSSTAT4"),
            UnitField::NEGSTAT0 => String::from("UnitField::NEGSTAT0"),
            UnitField::NEGSTAT1 => String::from("UnitField::NEGSTAT1"),
            UnitField::NEGSTAT2 => String::from("UnitField::NEGSTAT2"),
            UnitField::NEGSTAT3 => String::from("UnitField::NEGSTAT3"),
            UnitField::NEGSTAT4 => String::from("UnitField::NEGSTAT4"),
            UnitField::RESISTANCES_ARMOR => String::from("UnitField::RESISTANCES_ARMOR"),
            UnitField::RESISTANCES_HOLY => String::from("UnitField::RESISTANCES_HOLY"),
            UnitField::RESISTANCES_FIRE => String::from("UnitField::RESISTANCES_FIRE"),
            UnitField::RESISTANCES_NATURE => String::from("UnitField::RESISTANCES_NATURE"),
            UnitField::RESISTANCES_FROST => String::from("UnitField::RESISTANCES_FROST"),
            UnitField::RESISTANCES_SHADOW => String::from("UnitField::RESISTANCES_SHADOW"),
            UnitField::RESISTANCES_ARCANE => String::from("UnitField::RESISTANCES_ARCANE"),
            UnitField::RESISTANCEBUFFMODSPOSITIVE_ARMOR => String::from("UnitField::RESISTANCEBUFFMODSPOSITIVE_ARMOR"),
            UnitField::RESISTANCEBUFFMODSPOSITIVE_HOLY => String::from("UnitField::RESISTANCEBUFFMODSPOSITIVE_HOLY"),
            UnitField::RESISTANCEBUFFMODSPOSITIVE_FIRE => String::from("UnitField::RESISTANCEBUFFMODSPOSITIVE_FIRE"),
            UnitField::RESISTANCEBUFFMODSPOSITIVE_NATURE => String::from("UnitField::RESISTANCEBUFFMODSPOSITIVE_NATURE"),
            UnitField::RESISTANCEBUFFMODSPOSITIVE_FROST => String::from("UnitField::RESISTANCEBUFFMODSPOSITIVE_FROST"),
            UnitField::RESISTANCEBUFFMODSPOSITIVE_SHADOW => String::from("UnitField::RESISTANCEBUFFMODSPOSITIVE_SHADOW"),
            UnitField::RESISTANCEBUFFMODSPOSITIVE_ARCANE => String::from("UnitField::RESISTANCEBUFFMODSPOSITIVE_ARCANE"),
            UnitField::RESISTANCEBUFFMODSNEGATIVE_ARMOR => String::from("UnitField::RESISTANCEBUFFMODSNEGATIVE_ARMOR"),
            UnitField::RESISTANCEBUFFMODSNEGATIVE_HOLY => String::from("UnitField::RESISTANCEBUFFMODSNEGATIVE_HOLY"),
            UnitField::RESISTANCEBUFFMODSNEGATIVE_FIRE => String::from("UnitField::RESISTANCEBUFFMODSNEGATIVE_FIRE"),
            UnitField::RESISTANCEBUFFMODSNEGATIVE_NATURE => String::from("UnitField::RESISTANCEBUFFMODSNEGATIVE_NATURE"),
            UnitField::RESISTANCEBUFFMODSNEGATIVE_FROST => String::from("UnitField::RESISTANCEBUFFMODSNEGATIVE_FROST"),
            UnitField::RESISTANCEBUFFMODSNEGATIVE_SHADOW => String::from("UnitField::RESISTANCEBUFFMODSNEGATIVE_SHADOW"),
            UnitField::RESISTANCEBUFFMODSNEGATIVE_ARCANE => String::from("UnitField::RESISTANCEBUFFMODSNEGATIVE_ARCANE"),
            UnitField::BASE_MANA => String::from("UnitField::BASE_MANA"),
            UnitField::BASE_HEALTH => String::from("UnitField::BASE_HEALTH"),
            UnitField::BYTES_2 => String::from("UnitField::BYTES_2"),
            UnitField::ATTACK_POWER => String::from("UnitField::ATTACK_POWER"),
            UnitField::ATTACK_POWER_MODS => String::from("UnitField::ATTACK_POWER_MODS"),
            UnitField::ATTACK_POWER_MULTIPLIER => String::from("UnitField::ATTACK_POWER_MULTIPLIER"),
            UnitField::RANGED_ATTACK_POWER => String::from("UnitField::RANGED_ATTACK_POWER"),
            UnitField::RANGED_ATTACK_POWER_MODS => String::from("UnitField::RANGED_ATTACK_POWER_MODS"),
            UnitField::RANGED_ATTACK_POWER_MULTIPLIER => String::from("UnitField::RANGED_ATTACK_POWER_MULTIPLIER"),
            UnitField::MINRANGEDDAMAGE => String::from("UnitField::MINRANGEDDAMAGE"),
            UnitField::MAXRANGEDDAMAGE => String::from("UnitField::MAXRANGEDDAMAGE"),
            UnitField::POWER_COST_MODIFIER => String::from("UnitField::POWER_COST_MODIFIER"),
            UnitField::POWER_COST_MULTIPLIER1 => String::from("UnitField::POWER_COST_MULTIPLIER1"),
            UnitField::POWER_COST_MULTIPLIER2 => String::from("UnitField::POWER_COST_MULTIPLIER2"),
            UnitField::POWER_COST_MULTIPLIER3 => String::from("UnitField::POWER_COST_MULTIPLIER3"),
            UnitField::POWER_COST_MULTIPLIER4 => String::from("UnitField::POWER_COST_MULTIPLIER4"),
            UnitField::POWER_COST_MULTIPLIER5 => String::from("UnitField::POWER_COST_MULTIPLIER5"),
            UnitField::POWER_COST_MULTIPLIER6 => String::from("UnitField::POWER_COST_MULTIPLIER6"),
            UnitField::POWER_COST_MULTIPLIER7 => String::from("UnitField::POWER_COST_MULTIPLIER7"),
            UnitField::MAXHEALTHMODIFIER => String::from("UnitField::MAXHEALTHMODIFIER"),
            UnitField::HOVERHEIGHT => String::from("UnitField::HOVERHEIGHT"),
            _ => String::new(),
        }
    }
}

pub struct PlayerField;

#[allow(dead_code)]
impl PlayerField {
    pub const DUEL_ARBITER: u32 = 148;
    pub const FLAGS: u32 = 150;
    pub const GUILDID: u32 = 151;
    pub const GUILDRANK: u32 = 152;
    pub const BYTES: u32 = 153;
    pub const BYTES_2: u32 = 154;
    pub const BYTES_3: u32 = 155;
    pub const DUEL_TEAM: u32 = 156;
    pub const GUILD_TIMESTAMP: u32 = 157;
    pub const QUEST_LOG_1_1: u32 = 158;
    pub const QUEST_LOG_1_2: u32 = 159;
    pub const QUEST_LOG_1_3: u32 = 160;
    pub const QUEST_LOG_1_4: u32 = 162;
    pub const QUEST_LOG_2_1: u32 = 163;
    pub const QUEST_LOG_2_2: u32 = 164;
    pub const QUEST_LOG_2_3: u32 = 165;
    pub const QUEST_LOG_2_5: u32 = 167;
    pub const QUEST_LOG_3_1: u32 = 168;
    pub const QUEST_LOG_3_2: u32 = 169;
    pub const QUEST_LOG_3_3: u32 = 170;
    pub const QUEST_LOG_3_5: u32 = 172;
    pub const QUEST_LOG_4_1: u32 = 173;
    pub const QUEST_LOG_4_2: u32 = 174;
    pub const QUEST_LOG_4_3: u32 = 175;
    pub const QUEST_LOG_4_5: u32 = 177;
    pub const QUEST_LOG_5_1: u32 = 178;
    pub const QUEST_LOG_5_2: u32 = 179;
    pub const QUEST_LOG_5_3: u32 = 180;
    pub const QUEST_LOG_5_5: u32 = 182;
    pub const QUEST_LOG_6_1: u32 = 183;
    pub const QUEST_LOG_6_2: u32 = 184;
    pub const QUEST_LOG_6_3: u32 = 185;
    pub const QUEST_LOG_6_5: u32 = 187;
    pub const QUEST_LOG_7_1: u32 = 188;
    pub const QUEST_LOG_7_2: u32 = 189;
    pub const QUEST_LOG_7_3: u32 = 190;
    pub const QUEST_LOG_7_5: u32 = 192;
    pub const QUEST_LOG_8_1: u32 = 193;
    pub const QUEST_LOG_8_2: u32 = 194;
    pub const QUEST_LOG_8_3: u32 = 195;
    pub const QUEST_LOG_8_5: u32 = 197;
    pub const QUEST_LOG_9_1: u32 = 198;
    pub const QUEST_LOG_9_2: u32 = 199;
    pub const QUEST_LOG_9_3: u32 = 200;
    pub const QUEST_LOG_9_5: u32 = 202;
    pub const QUEST_LOG_10_1: u32 = 203;
    pub const QUEST_LOG_10_2: u32 = 204;
    pub const QUEST_LOG_10_3: u32 = 205;
    pub const QUEST_LOG_10_5: u32 = 207;
    pub const QUEST_LOG_11_1: u32 = 208;
    pub const QUEST_LOG_11_2: u32 = 209;
    pub const QUEST_LOG_11_3: u32 = 210;
    pub const QUEST_LOG_11_5: u32 = 212;
    pub const QUEST_LOG_12_1: u32 = 213;
    pub const QUEST_LOG_12_2: u32 = 214;
    pub const QUEST_LOG_12_3: u32 = 215;
    pub const QUEST_LOG_12_5: u32 = 217;
    pub const QUEST_LOG_13_1: u32 = 218;
    pub const QUEST_LOG_13_2: u32 = 219;
    pub const QUEST_LOG_13_3: u32 = 220;
    pub const QUEST_LOG_13_5: u32 = 222;
    pub const QUEST_LOG_14_1: u32 = 223;
    pub const QUEST_LOG_14_2: u32 = 224;
    pub const QUEST_LOG_14_3: u32 = 225;
    pub const QUEST_LOG_14_5: u32 = 227;
    pub const QUEST_LOG_15_1: u32 = 228;
    pub const QUEST_LOG_15_2: u32 = 229;
    pub const QUEST_LOG_15_3: u32 = 230;
    pub const QUEST_LOG_15_5: u32 = 232;
    pub const QUEST_LOG_16_1: u32 = 233;
    pub const QUEST_LOG_16_2: u32 = 234;
    pub const QUEST_LOG_16_3: u32 = 235;
    pub const QUEST_LOG_16_5: u32 = 237;
    pub const QUEST_LOG_17_1: u32 = 238;
    pub const QUEST_LOG_17_2: u32 = 239;
    pub const QUEST_LOG_17_3: u32 = 240;
    pub const QUEST_LOG_17_5: u32 = 242;
    pub const QUEST_LOG_18_1: u32 = 243;
    pub const QUEST_LOG_18_2: u32 = 244;
    pub const QUEST_LOG_18_3: u32 = 245;
    pub const QUEST_LOG_18_5: u32 = 247;
    pub const QUEST_LOG_19_1: u32 = 248;
    pub const QUEST_LOG_19_2: u32 = 249;
    pub const QUEST_LOG_19_3: u32 = 250;
    pub const QUEST_LOG_19_5: u32 = 252;
    pub const QUEST_LOG_20_1: u32 = 253;
    pub const QUEST_LOG_20_2: u32 = 254;
    pub const QUEST_LOG_20_3: u32 = 255;
    pub const QUEST_LOG_20_5: u32 = 257;
    pub const QUEST_LOG_21_1: u32 = 258;
    pub const QUEST_LOG_21_2: u32 = 259;
    pub const QUEST_LOG_21_3: u32 = 260;
    pub const QUEST_LOG_21_5: u32 = 262;
    pub const QUEST_LOG_22_1: u32 = 263;
    pub const QUEST_LOG_22_2: u32 = 264;
    pub const QUEST_LOG_22_3: u32 = 265;
    pub const QUEST_LOG_22_5: u32 = 267;
    pub const QUEST_LOG_23_1: u32 = 268;
    pub const QUEST_LOG_23_2: u32 = 269;
    pub const QUEST_LOG_23_3: u32 = 270;
    pub const QUEST_LOG_23_5: u32 = 272;
    pub const QUEST_LOG_24_1: u32 = 273;
    pub const QUEST_LOG_24_2: u32 = 274;
    pub const QUEST_LOG_24_3: u32 = 275;
    pub const QUEST_LOG_24_5: u32 = 277;
    pub const QUEST_LOG_25_1: u32 = 278;
    pub const QUEST_LOG_25_2: u32 = 279;
    pub const QUEST_LOG_25_3: u32 = 280;
    pub const QUEST_LOG_25_5: u32 = 282;
    pub const VISIBLE_ITEM_1_ENTRYID: u32 = 283;
    pub const VISIBLE_ITEM_1_ENCHANTMENT: u32 = 284;
    pub const VISIBLE_ITEM_2_ENTRYID: u32 = 285;
    pub const VISIBLE_ITEM_2_ENCHANTMENT: u32 = 286;
    pub const VISIBLE_ITEM_3_ENTRYID: u32 = 287;
    pub const VISIBLE_ITEM_3_ENCHANTMENT: u32 = 288;
    pub const VISIBLE_ITEM_4_ENTRYID: u32 = 289;
    pub const VISIBLE_ITEM_4_ENCHANTMENT: u32 = 290;
    pub const VISIBLE_ITEM_5_ENTRYID: u32 = 291;
    pub const VISIBLE_ITEM_5_ENCHANTMENT: u32 = 292;
    pub const VISIBLE_ITEM_6_ENTRYID: u32 = 293;
    pub const VISIBLE_ITEM_6_ENCHANTMENT: u32 = 294;
    pub const VISIBLE_ITEM_7_ENTRYID: u32 = 295;
    pub const VISIBLE_ITEM_7_ENCHANTMENT: u32 = 296;
    pub const VISIBLE_ITEM_8_ENTRYID: u32 = 297;
    pub const VISIBLE_ITEM_8_ENCHANTMENT: u32 = 298;
    pub const VISIBLE_ITEM_9_ENTRYID: u32 = 299;
    pub const VISIBLE_ITEM_9_ENCHANTMENT: u32 = 300;
    pub const VISIBLE_ITEM_10_ENTRYID: u32 = 301;
    pub const VISIBLE_ITEM_10_ENCHANTMENT: u32 = 302;
    pub const VISIBLE_ITEM_11_ENTRYID: u32 = 303;
    pub const VISIBLE_ITEM_11_ENCHANTMENT: u32 = 304;
    pub const VISIBLE_ITEM_12_ENTRYID: u32 = 305;
    pub const VISIBLE_ITEM_12_ENCHANTMENT: u32 = 306;
    pub const VISIBLE_ITEM_13_ENTRYID: u32 = 307;
    pub const VISIBLE_ITEM_13_ENCHANTMENT: u32 = 308;
    pub const VISIBLE_ITEM_14_ENTRYID: u32 = 309;
    pub const VISIBLE_ITEM_14_ENCHANTMENT: u32 = 310;
    pub const VISIBLE_ITEM_15_ENTRYID: u32 = 311;
    pub const VISIBLE_ITEM_15_ENCHANTMENT: u32 = 312;
    pub const VISIBLE_ITEM_16_ENTRYID: u32 = 313;
    pub const VISIBLE_ITEM_16_ENCHANTMENT: u32 = 314;
    pub const VISIBLE_ITEM_17_ENTRYID: u32 = 315;
    pub const VISIBLE_ITEM_17_ENCHANTMENT: u32 = 316;
    pub const VISIBLE_ITEM_18_ENTRYID: u32 = 317;
    pub const VISIBLE_ITEM_18_ENCHANTMENT: u32 = 318;
    pub const VISIBLE_ITEM_19_ENTRYID: u32 = 319;
    pub const VISIBLE_ITEM_19_ENCHANTMENT: u32 = 320;
    pub const CHOSEN_TITLE: u32 = 321;
    pub const FAKE_INEBRIATION: u32 = 322;
    pub const FIELD_PAD_0: u32 = 323;
    pub const FIELD_INV_SLOT_HEAD: u32 = 324;
    pub const FIELD_INV_SLOT_FIXME1: u32 = 326;
    pub const FIELD_INV_SLOT_FIXME2: u32 = 328;
    pub const FIELD_INV_SLOT_FIXME3: u32 = 330;
    pub const FIELD_INV_SLOT_FIXME4: u32 = 332;
    pub const FIELD_INV_SLOT_FIXME5: u32 = 334;
    pub const FIELD_INV_SLOT_FIXME6: u32 = 336;
    pub const FIELD_INV_SLOT_FIXME7: u32 = 338;
    pub const FIELD_INV_SLOT_FIXME8: u32 = 340;
    pub const FIELD_INV_SLOT_FIXME9: u32 = 342;
    pub const FIELD_INV_SLOT_FIXME10: u32 = 344;
    pub const FIELD_INV_SLOT_FIXME11: u32 = 346;
    pub const FIELD_INV_SLOT_FIXME12: u32 = 348;
    pub const FIELD_INV_SLOT_FIXME13: u32 = 350;
    pub const FIELD_INV_SLOT_FIXME14: u32 = 352;
    pub const FIELD_INV_SLOT_FIXME15: u32 = 354;
    pub const FIELD_INV_SLOT_FIXME16: u32 = 356;
    pub const FIELD_INV_SLOT_FIXME17: u32 = 358;
    pub const FIELD_INV_SLOT_FIXME18: u32 = 360;
    pub const FIELD_INV_SLOT_FIXME19: u32 = 362;
    pub const FIELD_INV_SLOT_FIXME20: u32 = 364;
    pub const FIELD_INV_SLOT_FIXME21: u32 = 366;
    pub const FIELD_INV_SLOT_FIXME22: u32 = 368;
    pub const FIELD_PACK_SLOT_1: u32 = 370;
    pub const FIELD_BANK_SLOT_1: u32 = 402;
    pub const FIELD_BANKBAG_SLOT_1: u32 = 458;
    pub const FIELD_VENDORBUYBACK_SLOT_1: u32 = 472;
    pub const FIELD_KEYRING_SLOT_1: u32 = 496;
    pub const FIELD_CURRENCYTOKEN_SLOT_1: u32 = 560;
    pub const FARSIGHT: u32 = 624;
    pub const FIELD_KNOWN_TITLES: u32 = 626;
    pub const FIELD_KNOWN_TITLES1: u32 = 628;
    pub const FIELD_KNOWN_TITLES2: u32 = 630;
    pub const FIELD_KNOWN_CURRENCIES: u32 = 632;
    pub const XP: u32 = 634;
    pub const NEXT_LEVEL_XP: u32 = 635;
    pub const SKILL_INFO_1_1: u32 = 636;
    pub const CHARACTER_POINTS1: u32 = 1020;
    pub const CHARACTER_POINTS2: u32 = 1021;
    pub const TRACK_CREATURES: u32 = 1022;
    pub const TRACK_RESOURCES: u32 = 1023;
    pub const BLOCK_PERCENTAGE: u32 = 1024;
    pub const DODGE_PERCENTAGE: u32 = 1025;
    pub const PARRY_PERCENTAGE: u32 = 1026;
    pub const EXPERTISE: u32 = 1027;
    pub const OFFHAND_EXPERTISE: u32 = 1028;
    pub const CRIT_PERCENTAGE: u32 = 1029;
    pub const RANGED_CRIT_PERCENTAGE: u32 = 1030;
    pub const OFFHAND_CRIT_PERCENTAGE: u32 = 1031;
    pub const SPELL_CRIT_PERCENTAGE1: u32 = 1032;
    pub const SPELL_CRIT_PERCENTAGE2: u32 = 1033;
    pub const SPELL_CRIT_PERCENTAGE3: u32 = 1034;
    pub const SPELL_CRIT_PERCENTAGE4: u32 = 1035;
    pub const SPELL_CRIT_PERCENTAGE5: u32 = 1036;
    pub const SPELL_CRIT_PERCENTAGE6: u32 = 1037;
    pub const SPELL_CRIT_PERCENTAGE7: u32 = 1038;
    pub const SHIELD_BLOCK: u32 = 1039;
    pub const SHIELD_BLOCK_CRIT_PERCENTAGE: u32 = 1040;
    pub const EXPLORED_ZONES_1: u32 = 1041;
    pub const REST_STATE_EXPERIENCE: u32 = 1169;
    pub const FIELD_COINAGE: u32 = 1170;
    pub const FIELD_MOD_DAMAGE_DONE_POS: u32 = 1171;
    pub const FIELD_MOD_DAMAGE_DONE_NEG: u32 = 1178;
    pub const FIELD_MOD_DAMAGE_DONE_PCT1: u32 = 1185;
    pub const FIELD_MOD_DAMAGE_DONE_PCT2: u32 = 1186;
    pub const FIELD_MOD_DAMAGE_DONE_PCT3: u32 = 1187;
    pub const FIELD_MOD_DAMAGE_DONE_PCT4: u32 = 1188;
    pub const FIELD_MOD_DAMAGE_DONE_PCT5: u32 = 1189;
    pub const FIELD_MOD_DAMAGE_DONE_PCT6: u32 = 1190;
    pub const FIELD_MOD_DAMAGE_DONE_PCT7: u32 = 1191;
    pub const FIELD_MOD_HEALING_DONE_POS: u32 = 1192;
    pub const FIELD_MOD_HEALING_PCT: u32 = 1193;
    pub const FIELD_MOD_HEALING_DONE_PCT: u32 = 1194;
    pub const FIELD_MOD_TARGET_RESISTANCE: u32 = 1195;
    pub const FIELD_MOD_TARGET_PHYSICAL_RESISTANCE: u32 = 1196;
    pub const FIELD_BYTES: u32 = 1197;
    pub const AMMO_ID: u32 = 1198;
    pub const SELF_RES_SPELL: u32 = 1199;
    pub const FIELD_PVP_MEDALS: u32 = 1200;
    pub const FIELD_BUYBACK_PRICE_1: u32 = 1201;
    pub const FIELD_BUYBACK_PRICE_2: u32 = 1202;
    pub const FIELD_BUYBACK_PRICE_3: u32 = 1203;
    pub const FIELD_BUYBACK_PRICE_4: u32 = 1204;
    pub const FIELD_BUYBACK_PRICE_5: u32 = 1205;
    pub const FIELD_BUYBACK_PRICE_6: u32 = 1206;
    pub const FIELD_BUYBACK_PRICE_7: u32 = 1207;
    pub const FIELD_BUYBACK_PRICE_8: u32 = 1208;
    pub const FIELD_BUYBACK_PRICE_9: u32 = 1209;
    pub const FIELD_BUYBACK_PRICE_10: u32 = 1210;
    pub const FIELD_BUYBACK_PRICE_11: u32 = 1211;
    pub const FIELD_BUYBACK_PRICE_12: u32 = 1212;
    pub const FIELD_BUYBACK_TIMESTAMP_1: u32 = 1213;
    pub const FIELD_BUYBACK_TIMESTAMP_2: u32 = 1214;
    pub const FIELD_BUYBACK_TIMESTAMP_3: u32 = 1215;
    pub const FIELD_BUYBACK_TIMESTAMP_4: u32 = 1216;
    pub const FIELD_BUYBACK_TIMESTAMP_5: u32 = 1217;
    pub const FIELD_BUYBACK_TIMESTAMP_6: u32 = 1218;
    pub const FIELD_BUYBACK_TIMESTAMP_7: u32 = 1219;
    pub const FIELD_BUYBACK_TIMESTAMP_8: u32 = 1220;
    pub const FIELD_BUYBACK_TIMESTAMP_9: u32 = 1221;
    pub const FIELD_BUYBACK_TIMESTAMP_10: u32 = 1222;
    pub const FIELD_BUYBACK_TIMESTAMP_11: u32 = 1223;
    pub const FIELD_BUYBACK_TIMESTAMP_12: u32 = 1224;
    pub const FIELD_KILLS: u32 = 1225;
    pub const FIELD_TODAY_CONTRIBUTION: u32 = 1226;
    pub const FIELD_YESTERDAY_CONTRIBUTION: u32 = 1227;
    pub const FIELD_LIFETIME_HONORABLE_KILLS: u32 = 1228;
    pub const FIELD_BYTES2: u32 = 1229;
    pub const FIELD_WATCHED_FACTION_INDEX: u32 = 1230;
    pub const FIELD_COMBAT_RATING_1: u32 = 1231;
    pub const FIELD_ARENA_TEAM_INFO_1_1: u32 = 1256;
    pub const FIELD_HONOR_CURRENCY: u32 = 1277;
    pub const FIELD_ARENA_CURRENCY: u32 = 1278;
    pub const FIELD_MAX_LEVEL: u32 = 1279;
    pub const FIELD_DAILY_QUESTS_1: u32 = 1280;
    pub const RUNE_REGEN_1: u32 = 1305;
    pub const RUNE_REGEN_2: u32 = 1306;
    pub const RUNE_REGEN_3: u32 = 1307;
    pub const RUNE_REGEN_4: u32 = 1308;
    pub const NO_REAGENT_COST_1: u32 = 1309;
    pub const FIELD_GLYPH_SLOTS_1: u32 = 1312;
    pub const FIELD_GLYPH_SLOTS_2: u32 = 1313;
    pub const FIELD_GLYPH_SLOTS_3: u32 = 1314;
    pub const FIELD_GLYPH_SLOTS_4: u32 = 1315;
    pub const FIELD_GLYPH_SLOTS_5: u32 = 1316;
    pub const FIELD_GLYPH_SLOTS_6: u32 = 1317;
    pub const FIELD_GLYPHS_1: u32 = 1318;
    pub const FIELD_GLYPHS_2: u32 = 1319;
    pub const FIELD_GLYPHS_3: u32 = 1320;
    pub const FIELD_GLYPHS_4: u32 = 1321;
    pub const FIELD_GLYPHS_5: u32 = 1322;
    pub const FIELD_GLYPHS_6: u32 = 1323;
    pub const GLYPHS_ENABLED: u32 = 1324;
    pub const PET_SPELL_POWER: u32 = 1325;

    pub fn get_field_type(field_index: u32) -> FieldType {
        match field_index {
            PlayerField::DUEL_ARBITER => FieldType::LONG,
            PlayerField::FLAGS => FieldType::INT,
            PlayerField::GUILDID => FieldType::INT,
            PlayerField::GUILDRANK => FieldType::INT,
            PlayerField::BYTES => FieldType::INT,
            PlayerField::BYTES_2 => FieldType::INT,
            PlayerField::BYTES_3 => FieldType::INT,
            PlayerField::DUEL_TEAM => FieldType::INT,
            PlayerField::GUILD_TIMESTAMP => FieldType::INT,
            PlayerField::QUEST_LOG_1_1 => FieldType::INT,
            PlayerField::QUEST_LOG_1_2 => FieldType::INT,
            PlayerField::QUEST_LOG_1_3 => FieldType::INT,
            PlayerField::QUEST_LOG_1_4 => FieldType::INT,
            PlayerField::QUEST_LOG_2_1 => FieldType::INT,
            PlayerField::QUEST_LOG_2_2 => FieldType::INT,
            PlayerField::QUEST_LOG_2_3 => FieldType::INT,
            PlayerField::QUEST_LOG_2_5 => FieldType::INT,
            PlayerField::QUEST_LOG_3_1 => FieldType::INT,
            PlayerField::QUEST_LOG_3_2 => FieldType::INT,
            PlayerField::QUEST_LOG_3_3 => FieldType::INT,
            PlayerField::QUEST_LOG_3_5 => FieldType::INT,
            PlayerField::QUEST_LOG_4_1 => FieldType::INT,
            PlayerField::QUEST_LOG_4_2 => FieldType::INT,
            PlayerField::QUEST_LOG_4_3 => FieldType::INT,
            PlayerField::QUEST_LOG_4_5 => FieldType::INT,
            PlayerField::QUEST_LOG_5_1 => FieldType::INT,
            PlayerField::QUEST_LOG_5_2 => FieldType::INT,
            PlayerField::QUEST_LOG_5_3 => FieldType::INT,
            PlayerField::QUEST_LOG_5_5 => FieldType::INT,
            PlayerField::QUEST_LOG_6_1 => FieldType::INT,
            PlayerField::QUEST_LOG_6_2 => FieldType::INT,
            PlayerField::QUEST_LOG_6_3 => FieldType::INT,
            PlayerField::QUEST_LOG_6_5 => FieldType::INT,
            PlayerField::QUEST_LOG_7_1 => FieldType::INT,
            PlayerField::QUEST_LOG_7_2 => FieldType::INT,
            PlayerField::QUEST_LOG_7_3 => FieldType::INT,
            PlayerField::QUEST_LOG_7_5 => FieldType::INT,
            PlayerField::QUEST_LOG_8_1 => FieldType::INT,
            PlayerField::QUEST_LOG_8_2 => FieldType::INT,
            PlayerField::QUEST_LOG_8_3 => FieldType::INT,
            PlayerField::QUEST_LOG_8_5 => FieldType::INT,
            PlayerField::QUEST_LOG_9_1 => FieldType::INT,
            PlayerField::QUEST_LOG_9_2 => FieldType::INT,
            PlayerField::QUEST_LOG_9_3 => FieldType::INT,
            PlayerField::QUEST_LOG_9_5 => FieldType::INT,
            PlayerField::QUEST_LOG_10_1 => FieldType::INT,
            PlayerField::QUEST_LOG_10_2 => FieldType::INT,
            PlayerField::QUEST_LOG_10_3 => FieldType::INT,
            PlayerField::QUEST_LOG_10_5 => FieldType::INT,
            PlayerField::QUEST_LOG_11_1 => FieldType::INT,
            PlayerField::QUEST_LOG_11_2 => FieldType::INT,
            PlayerField::QUEST_LOG_11_3 => FieldType::INT,
            PlayerField::QUEST_LOG_11_5 => FieldType::INT,
            PlayerField::QUEST_LOG_12_1 => FieldType::INT,
            PlayerField::QUEST_LOG_12_2 => FieldType::INT,
            PlayerField::QUEST_LOG_12_3 => FieldType::INT,
            PlayerField::QUEST_LOG_12_5 => FieldType::INT,
            PlayerField::QUEST_LOG_13_1 => FieldType::INT,
            PlayerField::QUEST_LOG_13_2 => FieldType::INT,
            PlayerField::QUEST_LOG_13_3 => FieldType::INT,
            PlayerField::QUEST_LOG_13_5 => FieldType::INT,
            PlayerField::QUEST_LOG_14_1 => FieldType::INT,
            PlayerField::QUEST_LOG_14_2 => FieldType::INT,
            PlayerField::QUEST_LOG_14_3 => FieldType::INT,
            PlayerField::QUEST_LOG_14_5 => FieldType::INT,
            PlayerField::QUEST_LOG_15_1 => FieldType::INT,
            PlayerField::QUEST_LOG_15_2 => FieldType::INT,
            PlayerField::QUEST_LOG_15_3 => FieldType::INT,
            PlayerField::QUEST_LOG_15_5 => FieldType::INT,
            PlayerField::QUEST_LOG_16_1 => FieldType::INT,
            PlayerField::QUEST_LOG_16_2 => FieldType::INT,
            PlayerField::QUEST_LOG_16_3 => FieldType::INT,
            PlayerField::QUEST_LOG_16_5 => FieldType::INT,
            PlayerField::QUEST_LOG_17_1 => FieldType::INT,
            PlayerField::QUEST_LOG_17_2 => FieldType::INT,
            PlayerField::QUEST_LOG_17_3 => FieldType::INT,
            PlayerField::QUEST_LOG_17_5 => FieldType::INT,
            PlayerField::QUEST_LOG_18_1 => FieldType::INT,
            PlayerField::QUEST_LOG_18_2 => FieldType::INT,
            PlayerField::QUEST_LOG_18_3 => FieldType::INT,
            PlayerField::QUEST_LOG_18_5 => FieldType::INT,
            PlayerField::QUEST_LOG_19_1 => FieldType::INT,
            PlayerField::QUEST_LOG_19_2 => FieldType::INT,
            PlayerField::QUEST_LOG_19_3 => FieldType::INT,
            PlayerField::QUEST_LOG_19_5 => FieldType::INT,
            PlayerField::QUEST_LOG_20_1 => FieldType::INT,
            PlayerField::QUEST_LOG_20_2 => FieldType::INT,
            PlayerField::QUEST_LOG_20_3 => FieldType::INT,
            PlayerField::QUEST_LOG_20_5 => FieldType::INT,
            PlayerField::QUEST_LOG_21_1 => FieldType::INT,
            PlayerField::QUEST_LOG_21_2 => FieldType::INT,
            PlayerField::QUEST_LOG_21_3 => FieldType::INT,
            PlayerField::QUEST_LOG_21_5 => FieldType::INT,
            PlayerField::QUEST_LOG_22_1 => FieldType::INT,
            PlayerField::QUEST_LOG_22_2 => FieldType::INT,
            PlayerField::QUEST_LOG_22_3 => FieldType::INT,
            PlayerField::QUEST_LOG_22_5 => FieldType::INT,
            PlayerField::QUEST_LOG_23_1 => FieldType::INT,
            PlayerField::QUEST_LOG_23_2 => FieldType::INT,
            PlayerField::QUEST_LOG_23_3 => FieldType::INT,
            PlayerField::QUEST_LOG_23_5 => FieldType::INT,
            PlayerField::QUEST_LOG_24_1 => FieldType::INT,
            PlayerField::QUEST_LOG_24_2 => FieldType::INT,
            PlayerField::QUEST_LOG_24_3 => FieldType::INT,
            PlayerField::QUEST_LOG_24_5 => FieldType::INT,
            PlayerField::QUEST_LOG_25_1 => FieldType::INT,
            PlayerField::QUEST_LOG_25_2 => FieldType::INT,
            PlayerField::QUEST_LOG_25_3 => FieldType::INT,
            PlayerField::QUEST_LOG_25_5 => FieldType::INT,
            PlayerField::VISIBLE_ITEM_1_ENTRYID => FieldType::INT,
            PlayerField::VISIBLE_ITEM_1_ENCHANTMENT => FieldType::INT,
            PlayerField::VISIBLE_ITEM_2_ENTRYID => FieldType::INT,
            PlayerField::VISIBLE_ITEM_2_ENCHANTMENT => FieldType::INT,
            PlayerField::VISIBLE_ITEM_3_ENTRYID => FieldType::INT,
            PlayerField::VISIBLE_ITEM_3_ENCHANTMENT => FieldType::INT,
            PlayerField::VISIBLE_ITEM_4_ENTRYID => FieldType::INT,
            PlayerField::VISIBLE_ITEM_4_ENCHANTMENT => FieldType::INT,
            PlayerField::VISIBLE_ITEM_5_ENTRYID => FieldType::INT,
            PlayerField::VISIBLE_ITEM_5_ENCHANTMENT => FieldType::INT,
            PlayerField::VISIBLE_ITEM_6_ENTRYID => FieldType::INT,
            PlayerField::VISIBLE_ITEM_6_ENCHANTMENT => FieldType::INT,
            PlayerField::VISIBLE_ITEM_7_ENTRYID => FieldType::INT,
            PlayerField::VISIBLE_ITEM_7_ENCHANTMENT => FieldType::INT,
            PlayerField::VISIBLE_ITEM_8_ENTRYID => FieldType::INT,
            PlayerField::VISIBLE_ITEM_8_ENCHANTMENT => FieldType::INT,
            PlayerField::VISIBLE_ITEM_9_ENTRYID => FieldType::INT,
            PlayerField::VISIBLE_ITEM_9_ENCHANTMENT => FieldType::INT,
            PlayerField::VISIBLE_ITEM_10_ENTRYID => FieldType::INT,
            PlayerField::VISIBLE_ITEM_10_ENCHANTMENT => FieldType::INT,
            PlayerField::VISIBLE_ITEM_11_ENTRYID => FieldType::INT,
            PlayerField::VISIBLE_ITEM_11_ENCHANTMENT => FieldType::INT,
            PlayerField::VISIBLE_ITEM_12_ENTRYID => FieldType::INT,
            PlayerField::VISIBLE_ITEM_12_ENCHANTMENT => FieldType::INT,
            PlayerField::VISIBLE_ITEM_13_ENTRYID => FieldType::INT,
            PlayerField::VISIBLE_ITEM_13_ENCHANTMENT => FieldType::INT,
            PlayerField::VISIBLE_ITEM_14_ENTRYID => FieldType::INT,
            PlayerField::VISIBLE_ITEM_14_ENCHANTMENT => FieldType::INT,
            PlayerField::VISIBLE_ITEM_15_ENTRYID => FieldType::INT,
            PlayerField::VISIBLE_ITEM_15_ENCHANTMENT => FieldType::INT,
            PlayerField::VISIBLE_ITEM_16_ENTRYID => FieldType::INT,
            PlayerField::VISIBLE_ITEM_16_ENCHANTMENT => FieldType::INT,
            PlayerField::VISIBLE_ITEM_17_ENTRYID => FieldType::INT,
            PlayerField::VISIBLE_ITEM_17_ENCHANTMENT => FieldType::INT,
            PlayerField::VISIBLE_ITEM_18_ENTRYID => FieldType::INT,
            PlayerField::VISIBLE_ITEM_18_ENCHANTMENT => FieldType::INT,
            PlayerField::VISIBLE_ITEM_19_ENTRYID => FieldType::INT,
            PlayerField::VISIBLE_ITEM_19_ENCHANTMENT => FieldType::INT,
            PlayerField::CHOSEN_TITLE => FieldType::INT,
            PlayerField::FAKE_INEBRIATION => FieldType::INT,
            PlayerField::FIELD_PAD_0 => FieldType::INT,
            PlayerField::FIELD_INV_SLOT_HEAD => FieldType::LONG,
            PlayerField::FIELD_INV_SLOT_FIXME1 => FieldType::LONG,
            PlayerField::FIELD_INV_SLOT_FIXME2 => FieldType::LONG,
            PlayerField::FIELD_INV_SLOT_FIXME3 => FieldType::LONG,
            PlayerField::FIELD_INV_SLOT_FIXME4 => FieldType::LONG,
            PlayerField::FIELD_INV_SLOT_FIXME5 => FieldType::LONG,
            PlayerField::FIELD_INV_SLOT_FIXME6 => FieldType::LONG,
            PlayerField::FIELD_INV_SLOT_FIXME7 => FieldType::LONG,
            PlayerField::FIELD_INV_SLOT_FIXME8 => FieldType::LONG,
            PlayerField::FIELD_INV_SLOT_FIXME9 => FieldType::LONG,
            PlayerField::FIELD_INV_SLOT_FIXME10 => FieldType::LONG,
            PlayerField::FIELD_INV_SLOT_FIXME11 => FieldType::LONG,
            PlayerField::FIELD_INV_SLOT_FIXME12 => FieldType::LONG,
            PlayerField::FIELD_INV_SLOT_FIXME13 => FieldType::LONG,
            PlayerField::FIELD_INV_SLOT_FIXME14 => FieldType::LONG,
            PlayerField::FIELD_INV_SLOT_FIXME15 => FieldType::LONG,
            PlayerField::FIELD_INV_SLOT_FIXME16 => FieldType::LONG,
            PlayerField::FIELD_INV_SLOT_FIXME17 => FieldType::LONG,
            PlayerField::FIELD_INV_SLOT_FIXME18 => FieldType::LONG,
            PlayerField::FIELD_INV_SLOT_FIXME19 => FieldType::LONG,
            PlayerField::FIELD_INV_SLOT_FIXME20 => FieldType::LONG,
            PlayerField::FIELD_INV_SLOT_FIXME21 => FieldType::LONG,
            PlayerField::FIELD_INV_SLOT_FIXME22 => FieldType::LONG,
            PlayerField::FIELD_PACK_SLOT_1 => FieldType::LONG,
            PlayerField::FIELD_BANK_SLOT_1 => FieldType::LONG,
            PlayerField::FIELD_BANKBAG_SLOT_1 => FieldType::LONG,
            PlayerField::FIELD_VENDORBUYBACK_SLOT_1 => FieldType::LONG,
            PlayerField::FIELD_KEYRING_SLOT_1 => FieldType::LONG,
            PlayerField::FIELD_CURRENCYTOKEN_SLOT_1 => FieldType::LONG,
            PlayerField::FARSIGHT => FieldType::LONG,
            PlayerField::FIELD_KNOWN_TITLES => FieldType::LONG,
            PlayerField::FIELD_KNOWN_TITLES1 => FieldType::LONG,
            PlayerField::FIELD_KNOWN_TITLES2 => FieldType::LONG,
            PlayerField::FIELD_KNOWN_CURRENCIES => FieldType::LONG,
            PlayerField::XP => FieldType::INT,
            PlayerField::NEXT_LEVEL_XP => FieldType::INT,
            PlayerField::SKILL_INFO_1_1 => FieldType::INT,
            PlayerField::CHARACTER_POINTS1 => FieldType::INT,
            PlayerField::CHARACTER_POINTS2 => FieldType::INT,
            PlayerField::TRACK_CREATURES => FieldType::INT,
            PlayerField::TRACK_RESOURCES => FieldType::INT,
            PlayerField::BLOCK_PERCENTAGE => FieldType::FLOAT,
            PlayerField::DODGE_PERCENTAGE => FieldType::FLOAT,
            PlayerField::PARRY_PERCENTAGE => FieldType::FLOAT,
            PlayerField::EXPERTISE => FieldType::INT,
            PlayerField::OFFHAND_EXPERTISE => FieldType::INT,
            PlayerField::CRIT_PERCENTAGE => FieldType::FLOAT,
            PlayerField::RANGED_CRIT_PERCENTAGE => FieldType::FLOAT,
            PlayerField::OFFHAND_CRIT_PERCENTAGE => FieldType::FLOAT,
            PlayerField::SPELL_CRIT_PERCENTAGE1 => FieldType::FLOAT,
            PlayerField::SPELL_CRIT_PERCENTAGE2 => FieldType::FLOAT,
            PlayerField::SPELL_CRIT_PERCENTAGE3 => FieldType::FLOAT,
            PlayerField::SPELL_CRIT_PERCENTAGE4 => FieldType::FLOAT,
            PlayerField::SPELL_CRIT_PERCENTAGE5 => FieldType::FLOAT,
            PlayerField::SPELL_CRIT_PERCENTAGE6 => FieldType::FLOAT,
            PlayerField::SPELL_CRIT_PERCENTAGE7 => FieldType::FLOAT,
            PlayerField::SHIELD_BLOCK => FieldType::INT,
            PlayerField::SHIELD_BLOCK_CRIT_PERCENTAGE => FieldType::FLOAT,
            PlayerField::EXPLORED_ZONES_1 => FieldType::INT,
            PlayerField::REST_STATE_EXPERIENCE => FieldType::INT,
            PlayerField::FIELD_COINAGE => FieldType::INT,
            PlayerField::FIELD_MOD_DAMAGE_DONE_POS => FieldType::INT,
            PlayerField::FIELD_MOD_DAMAGE_DONE_NEG => FieldType::INT,
            PlayerField::FIELD_MOD_DAMAGE_DONE_PCT1 => FieldType::INT,
            PlayerField::FIELD_MOD_DAMAGE_DONE_PCT2 => FieldType::INT,
            PlayerField::FIELD_MOD_DAMAGE_DONE_PCT3 => FieldType::INT,
            PlayerField::FIELD_MOD_DAMAGE_DONE_PCT4 => FieldType::INT,
            PlayerField::FIELD_MOD_DAMAGE_DONE_PCT5 => FieldType::INT,
            PlayerField::FIELD_MOD_DAMAGE_DONE_PCT6 => FieldType::INT,
            PlayerField::FIELD_MOD_DAMAGE_DONE_PCT7 => FieldType::INT,
            PlayerField::FIELD_MOD_HEALING_DONE_POS => FieldType::INT,
            PlayerField::FIELD_MOD_HEALING_PCT => FieldType::FLOAT,
            PlayerField::FIELD_MOD_HEALING_DONE_PCT => FieldType::FLOAT,
            PlayerField::FIELD_MOD_TARGET_RESISTANCE => FieldType::INT,
            PlayerField::FIELD_MOD_TARGET_PHYSICAL_RESISTANCE => FieldType::INT,
            PlayerField::FIELD_BYTES => FieldType::INT,
            PlayerField::AMMO_ID => FieldType::LONG,
            PlayerField::SELF_RES_SPELL => FieldType::INT,
            PlayerField::FIELD_PVP_MEDALS => FieldType::INT,
            PlayerField::FIELD_BUYBACK_PRICE_1 => FieldType::INT,
            PlayerField::FIELD_BUYBACK_PRICE_2 => FieldType::INT,
            PlayerField::FIELD_BUYBACK_PRICE_3 => FieldType::INT,
            PlayerField::FIELD_BUYBACK_PRICE_4 => FieldType::INT,
            PlayerField::FIELD_BUYBACK_PRICE_5 => FieldType::INT,
            PlayerField::FIELD_BUYBACK_PRICE_6 => FieldType::INT,
            PlayerField::FIELD_BUYBACK_PRICE_7 => FieldType::INT,
            PlayerField::FIELD_BUYBACK_PRICE_8 => FieldType::INT,
            PlayerField::FIELD_BUYBACK_PRICE_9 => FieldType::INT,
            PlayerField::FIELD_BUYBACK_PRICE_10 => FieldType::INT,
            PlayerField::FIELD_BUYBACK_PRICE_11 => FieldType::INT,
            PlayerField::FIELD_BUYBACK_PRICE_12 => FieldType::INT,
            PlayerField::FIELD_BUYBACK_TIMESTAMP_1 => FieldType::INT,
            PlayerField::FIELD_BUYBACK_TIMESTAMP_2 => FieldType::INT,
            PlayerField::FIELD_BUYBACK_TIMESTAMP_3 => FieldType::INT,
            PlayerField::FIELD_BUYBACK_TIMESTAMP_4 => FieldType::INT,
            PlayerField::FIELD_BUYBACK_TIMESTAMP_5 => FieldType::INT,
            PlayerField::FIELD_BUYBACK_TIMESTAMP_6 => FieldType::INT,
            PlayerField::FIELD_BUYBACK_TIMESTAMP_7 => FieldType::INT,
            PlayerField::FIELD_BUYBACK_TIMESTAMP_8 => FieldType::INT,
            PlayerField::FIELD_BUYBACK_TIMESTAMP_9 => FieldType::INT,
            PlayerField::FIELD_BUYBACK_TIMESTAMP_10 => FieldType::INT,
            PlayerField::FIELD_BUYBACK_TIMESTAMP_11 => FieldType::INT,
            PlayerField::FIELD_BUYBACK_TIMESTAMP_12 => FieldType::INT,
            PlayerField::FIELD_KILLS => FieldType::INT,
            PlayerField::FIELD_TODAY_CONTRIBUTION => FieldType::INT,
            PlayerField::FIELD_YESTERDAY_CONTRIBUTION => FieldType::INT,
            PlayerField::FIELD_LIFETIME_HONORABLE_KILLS => FieldType::INT,
            PlayerField::FIELD_BYTES2 => FieldType::INT,
            PlayerField::FIELD_WATCHED_FACTION_INDEX => FieldType::INT,
            PlayerField::FIELD_COMBAT_RATING_1 => FieldType::INT,
            PlayerField::FIELD_ARENA_TEAM_INFO_1_1 => FieldType::INT,
            PlayerField::FIELD_HONOR_CURRENCY => FieldType::INT,
            PlayerField::FIELD_ARENA_CURRENCY => FieldType::INT,
            PlayerField::FIELD_MAX_LEVEL => FieldType::INT,
            PlayerField::FIELD_DAILY_QUESTS_1 => FieldType::INT,
            PlayerField::RUNE_REGEN_1 => FieldType::FLOAT,
            PlayerField::RUNE_REGEN_2 => FieldType::FLOAT,
            PlayerField::RUNE_REGEN_3 => FieldType::FLOAT,
            PlayerField::RUNE_REGEN_4 => FieldType::FLOAT,
            PlayerField::NO_REAGENT_COST_1 => FieldType::INT,
            PlayerField::FIELD_GLYPH_SLOTS_1 => FieldType::INT,
            PlayerField::FIELD_GLYPH_SLOTS_2 => FieldType::INT,
            PlayerField::FIELD_GLYPH_SLOTS_3 => FieldType::INT,
            PlayerField::FIELD_GLYPH_SLOTS_4 => FieldType::INT,
            PlayerField::FIELD_GLYPH_SLOTS_5 => FieldType::INT,
            PlayerField::FIELD_GLYPH_SLOTS_6 => FieldType::INT,
            PlayerField::FIELD_GLYPHS_1 => FieldType::INT,
            PlayerField::FIELD_GLYPHS_2 => FieldType::INT,
            PlayerField::FIELD_GLYPHS_3 => FieldType::INT,
            PlayerField::FIELD_GLYPHS_4 => FieldType::INT,
            PlayerField::FIELD_GLYPHS_5 => FieldType::INT,
            PlayerField::FIELD_GLYPHS_6 => FieldType::INT,
            PlayerField::GLYPHS_ENABLED => FieldType::INT,
            PlayerField::PET_SPELL_POWER => FieldType::INT,
            _ => FieldType::NONE,
        }
    }

    pub fn get_field_name(field_index: u32) -> String {
        match field_index {
            PlayerField::DUEL_ARBITER => String::from("PlayerField::DUEL_ARBITER"),
            PlayerField::FLAGS => String::from("PlayerField::FLAGS"),
            PlayerField::GUILDID => String::from("PlayerField::GUILDID"),
            PlayerField::GUILDRANK => String::from("PlayerField::GUILDRANK"),
            PlayerField::BYTES => String::from("PlayerField::BYTES"),
            PlayerField::BYTES_2 => String::from("PlayerField::BYTES_2"),
            PlayerField::BYTES_3 => String::from("PlayerField::BYTES_3"),
            PlayerField::DUEL_TEAM => String::from("PlayerField::DUEL_TEAM"),
            PlayerField::GUILD_TIMESTAMP => String::from("PlayerField::GUILD_TIMESTAMP"),
            PlayerField::QUEST_LOG_1_1 => String::from("PlayerField::QUEST_LOG_1_1"),
            PlayerField::QUEST_LOG_1_2 => String::from("PlayerField::QUEST_LOG_1_2"),
            PlayerField::QUEST_LOG_1_3 => String::from("PlayerField::QUEST_LOG_1_3"),
            PlayerField::QUEST_LOG_1_4 => String::from("PlayerField::QUEST_LOG_1_4"),
            PlayerField::QUEST_LOG_2_1 => String::from("PlayerField::QUEST_LOG_2_1"),
            PlayerField::QUEST_LOG_2_2 => String::from("PlayerField::QUEST_LOG_2_2"),
            PlayerField::QUEST_LOG_2_3 => String::from("PlayerField::QUEST_LOG_2_3"),
            PlayerField::QUEST_LOG_2_5 => String::from("PlayerField::QUEST_LOG_2_5"),
            PlayerField::QUEST_LOG_3_1 => String::from("PlayerField::QUEST_LOG_3_1"),
            PlayerField::QUEST_LOG_3_2 => String::from("PlayerField::QUEST_LOG_3_2"),
            PlayerField::QUEST_LOG_3_3 => String::from("PlayerField::QUEST_LOG_3_3"),
            PlayerField::QUEST_LOG_3_5 => String::from("PlayerField::QUEST_LOG_3_5"),
            PlayerField::QUEST_LOG_4_1 => String::from("PlayerField::QUEST_LOG_4_1"),
            PlayerField::QUEST_LOG_4_2 => String::from("PlayerField::QUEST_LOG_4_2"),
            PlayerField::QUEST_LOG_4_3 => String::from("PlayerField::QUEST_LOG_4_3"),
            PlayerField::QUEST_LOG_4_5 => String::from("PlayerField::QUEST_LOG_4_5"),
            PlayerField::QUEST_LOG_5_1 => String::from("PlayerField::QUEST_LOG_5_1"),
            PlayerField::QUEST_LOG_5_2 => String::from("PlayerField::QUEST_LOG_5_2"),
            PlayerField::QUEST_LOG_5_3 => String::from("PlayerField::QUEST_LOG_5_3"),
            PlayerField::QUEST_LOG_5_5 => String::from("PlayerField::QUEST_LOG_5_5"),
            PlayerField::QUEST_LOG_6_1 => String::from("PlayerField::QUEST_LOG_6_1"),
            PlayerField::QUEST_LOG_6_2 => String::from("PlayerField::QUEST_LOG_6_2"),
            PlayerField::QUEST_LOG_6_3 => String::from("PlayerField::QUEST_LOG_6_3"),
            PlayerField::QUEST_LOG_6_5 => String::from("PlayerField::QUEST_LOG_6_5"),
            PlayerField::QUEST_LOG_7_1 => String::from("PlayerField::QUEST_LOG_7_1"),
            PlayerField::QUEST_LOG_7_2 => String::from("PlayerField::QUEST_LOG_7_2"),
            PlayerField::QUEST_LOG_7_3 => String::from("PlayerField::QUEST_LOG_7_3"),
            PlayerField::QUEST_LOG_7_5 => String::from("PlayerField::QUEST_LOG_7_5"),
            PlayerField::QUEST_LOG_8_1 => String::from("PlayerField::QUEST_LOG_8_1"),
            PlayerField::QUEST_LOG_8_2 => String::from("PlayerField::QUEST_LOG_8_2"),
            PlayerField::QUEST_LOG_8_3 => String::from("PlayerField::QUEST_LOG_8_3"),
            PlayerField::QUEST_LOG_8_5 => String::from("PlayerField::QUEST_LOG_8_5"),
            PlayerField::QUEST_LOG_9_1 => String::from("PlayerField::QUEST_LOG_9_1"),
            PlayerField::QUEST_LOG_9_2 => String::from("PlayerField::QUEST_LOG_9_2"),
            PlayerField::QUEST_LOG_9_3 => String::from("PlayerField::QUEST_LOG_9_3"),
            PlayerField::QUEST_LOG_9_5 => String::from("PlayerField::QUEST_LOG_9_5"),
            PlayerField::QUEST_LOG_10_1 => String::from("PlayerField::QUEST_LOG_10_1"),
            PlayerField::QUEST_LOG_10_2 => String::from("PlayerField::QUEST_LOG_10_2"),
            PlayerField::QUEST_LOG_10_3 => String::from("PlayerField::QUEST_LOG_10_3"),
            PlayerField::QUEST_LOG_10_5 => String::from("PlayerField::QUEST_LOG_10_5"),
            PlayerField::QUEST_LOG_11_1 => String::from("PlayerField::QUEST_LOG_11_1"),
            PlayerField::QUEST_LOG_11_2 => String::from("PlayerField::QUEST_LOG_11_2"),
            PlayerField::QUEST_LOG_11_3 => String::from("PlayerField::QUEST_LOG_11_3"),
            PlayerField::QUEST_LOG_11_5 => String::from("PlayerField::QUEST_LOG_11_5"),
            PlayerField::QUEST_LOG_12_1 => String::from("PlayerField::QUEST_LOG_12_1"),
            PlayerField::QUEST_LOG_12_2 => String::from("PlayerField::QUEST_LOG_12_2"),
            PlayerField::QUEST_LOG_12_3 => String::from("PlayerField::QUEST_LOG_12_3"),
            PlayerField::QUEST_LOG_12_5 => String::from("PlayerField::QUEST_LOG_12_5"),
            PlayerField::QUEST_LOG_13_1 => String::from("PlayerField::QUEST_LOG_13_1"),
            PlayerField::QUEST_LOG_13_2 => String::from("PlayerField::QUEST_LOG_13_2"),
            PlayerField::QUEST_LOG_13_3 => String::from("PlayerField::QUEST_LOG_13_3"),
            PlayerField::QUEST_LOG_13_5 => String::from("PlayerField::QUEST_LOG_13_5"),
            PlayerField::QUEST_LOG_14_1 => String::from("PlayerField::QUEST_LOG_14_1"),
            PlayerField::QUEST_LOG_14_2 => String::from("PlayerField::QUEST_LOG_14_2"),
            PlayerField::QUEST_LOG_14_3 => String::from("PlayerField::QUEST_LOG_14_3"),
            PlayerField::QUEST_LOG_14_5 => String::from("PlayerField::QUEST_LOG_14_5"),
            PlayerField::QUEST_LOG_15_1 => String::from("PlayerField::QUEST_LOG_15_1"),
            PlayerField::QUEST_LOG_15_2 => String::from("PlayerField::QUEST_LOG_15_2"),
            PlayerField::QUEST_LOG_15_3 => String::from("PlayerField::QUEST_LOG_15_3"),
            PlayerField::QUEST_LOG_15_5 => String::from("PlayerField::QUEST_LOG_15_5"),
            PlayerField::QUEST_LOG_16_1 => String::from("PlayerField::QUEST_LOG_16_1"),
            PlayerField::QUEST_LOG_16_2 => String::from("PlayerField::QUEST_LOG_16_2"),
            PlayerField::QUEST_LOG_16_3 => String::from("PlayerField::QUEST_LOG_16_3"),
            PlayerField::QUEST_LOG_16_5 => String::from("PlayerField::QUEST_LOG_16_5"),
            PlayerField::QUEST_LOG_17_1 => String::from("PlayerField::QUEST_LOG_17_1"),
            PlayerField::QUEST_LOG_17_2 => String::from("PlayerField::QUEST_LOG_17_2"),
            PlayerField::QUEST_LOG_17_3 => String::from("PlayerField::QUEST_LOG_17_3"),
            PlayerField::QUEST_LOG_17_5 => String::from("PlayerField::QUEST_LOG_17_5"),
            PlayerField::QUEST_LOG_18_1 => String::from("PlayerField::QUEST_LOG_18_1"),
            PlayerField::QUEST_LOG_18_2 => String::from("PlayerField::QUEST_LOG_18_2"),
            PlayerField::QUEST_LOG_18_3 => String::from("PlayerField::QUEST_LOG_18_3"),
            PlayerField::QUEST_LOG_18_5 => String::from("PlayerField::QUEST_LOG_18_5"),
            PlayerField::QUEST_LOG_19_1 => String::from("PlayerField::QUEST_LOG_19_1"),
            PlayerField::QUEST_LOG_19_2 => String::from("PlayerField::QUEST_LOG_19_2"),
            PlayerField::QUEST_LOG_19_3 => String::from("PlayerField::QUEST_LOG_19_3"),
            PlayerField::QUEST_LOG_19_5 => String::from("PlayerField::QUEST_LOG_19_5"),
            PlayerField::QUEST_LOG_20_1 => String::from("PlayerField::QUEST_LOG_20_1"),
            PlayerField::QUEST_LOG_20_2 => String::from("PlayerField::QUEST_LOG_20_2"),
            PlayerField::QUEST_LOG_20_3 => String::from("PlayerField::QUEST_LOG_20_3"),
            PlayerField::QUEST_LOG_20_5 => String::from("PlayerField::QUEST_LOG_20_5"),
            PlayerField::QUEST_LOG_21_1 => String::from("PlayerField::QUEST_LOG_21_1"),
            PlayerField::QUEST_LOG_21_2 => String::from("PlayerField::QUEST_LOG_21_2"),
            PlayerField::QUEST_LOG_21_3 => String::from("PlayerField::QUEST_LOG_21_3"),
            PlayerField::QUEST_LOG_21_5 => String::from("PlayerField::QUEST_LOG_21_5"),
            PlayerField::QUEST_LOG_22_1 => String::from("PlayerField::QUEST_LOG_22_1"),
            PlayerField::QUEST_LOG_22_2 => String::from("PlayerField::QUEST_LOG_22_2"),
            PlayerField::QUEST_LOG_22_3 => String::from("PlayerField::QUEST_LOG_22_3"),
            PlayerField::QUEST_LOG_22_5 => String::from("PlayerField::QUEST_LOG_22_5"),
            PlayerField::QUEST_LOG_23_1 => String::from("PlayerField::QUEST_LOG_23_1"),
            PlayerField::QUEST_LOG_23_2 => String::from("PlayerField::QUEST_LOG_23_2"),
            PlayerField::QUEST_LOG_23_3 => String::from("PlayerField::QUEST_LOG_23_3"),
            PlayerField::QUEST_LOG_23_5 => String::from("PlayerField::QUEST_LOG_23_5"),
            PlayerField::QUEST_LOG_24_1 => String::from("PlayerField::QUEST_LOG_24_1"),
            PlayerField::QUEST_LOG_24_2 => String::from("PlayerField::QUEST_LOG_24_2"),
            PlayerField::QUEST_LOG_24_3 => String::from("PlayerField::QUEST_LOG_24_3"),
            PlayerField::QUEST_LOG_24_5 => String::from("PlayerField::QUEST_LOG_24_5"),
            PlayerField::QUEST_LOG_25_1 => String::from("PlayerField::QUEST_LOG_25_1"),
            PlayerField::QUEST_LOG_25_2 => String::from("PlayerField::QUEST_LOG_25_2"),
            PlayerField::QUEST_LOG_25_3 => String::from("PlayerField::QUEST_LOG_25_3"),
            PlayerField::QUEST_LOG_25_5 => String::from("PlayerField::QUEST_LOG_25_5"),
            PlayerField::VISIBLE_ITEM_1_ENTRYID => String::from("PlayerField::VISIBLE_ITEM_1_ENTRYID"),
            PlayerField::VISIBLE_ITEM_1_ENCHANTMENT => String::from("PlayerField::VISIBLE_ITEM_1_ENCHANTMENT"),
            PlayerField::VISIBLE_ITEM_2_ENTRYID => String::from("PlayerField::VISIBLE_ITEM_2_ENTRYID"),
            PlayerField::VISIBLE_ITEM_2_ENCHANTMENT => String::from("PlayerField::VISIBLE_ITEM_2_ENCHANTMENT"),
            PlayerField::VISIBLE_ITEM_3_ENTRYID => String::from("PlayerField::VISIBLE_ITEM_3_ENTRYID"),
            PlayerField::VISIBLE_ITEM_3_ENCHANTMENT => String::from("PlayerField::VISIBLE_ITEM_3_ENCHANTMENT"),
            PlayerField::VISIBLE_ITEM_4_ENTRYID => String::from("PlayerField::VISIBLE_ITEM_4_ENTRYID"),
            PlayerField::VISIBLE_ITEM_4_ENCHANTMENT => String::from("PlayerField::VISIBLE_ITEM_4_ENCHANTMENT"),
            PlayerField::VISIBLE_ITEM_5_ENTRYID => String::from("PlayerField::VISIBLE_ITEM_5_ENTRYID"),
            PlayerField::VISIBLE_ITEM_5_ENCHANTMENT => String::from("PlayerField::VISIBLE_ITEM_5_ENCHANTMENT"),
            PlayerField::VISIBLE_ITEM_6_ENTRYID => String::from("PlayerField::VISIBLE_ITEM_6_ENTRYID"),
            PlayerField::VISIBLE_ITEM_6_ENCHANTMENT => String::from("PlayerField::VISIBLE_ITEM_6_ENCHANTMENT"),
            PlayerField::VISIBLE_ITEM_7_ENTRYID => String::from("PlayerField::VISIBLE_ITEM_7_ENTRYID"),
            PlayerField::VISIBLE_ITEM_7_ENCHANTMENT => String::from("PlayerField::VISIBLE_ITEM_7_ENCHANTMENT"),
            PlayerField::VISIBLE_ITEM_8_ENTRYID => String::from("PlayerField::VISIBLE_ITEM_8_ENTRYID"),
            PlayerField::VISIBLE_ITEM_8_ENCHANTMENT => String::from("PlayerField::VISIBLE_ITEM_8_ENCHANTMENT"),
            PlayerField::VISIBLE_ITEM_9_ENTRYID => String::from("PlayerField::VISIBLE_ITEM_9_ENTRYID"),
            PlayerField::VISIBLE_ITEM_9_ENCHANTMENT => String::from("PlayerField::VISIBLE_ITEM_9_ENCHANTMENT"),
            PlayerField::VISIBLE_ITEM_10_ENTRYID => String::from("PlayerField::VISIBLE_ITEM_10_ENTRYID"),
            PlayerField::VISIBLE_ITEM_10_ENCHANTMENT => String::from("PlayerField::VISIBLE_ITEM_10_ENCHANTMENT"),
            PlayerField::VISIBLE_ITEM_11_ENTRYID => String::from("PlayerField::VISIBLE_ITEM_11_ENTRYID"),
            PlayerField::VISIBLE_ITEM_11_ENCHANTMENT => String::from("PlayerField::VISIBLE_ITEM_11_ENCHANTMENT"),
            PlayerField::VISIBLE_ITEM_12_ENTRYID => String::from("PlayerField::VISIBLE_ITEM_12_ENTRYID"),
            PlayerField::VISIBLE_ITEM_12_ENCHANTMENT => String::from("PlayerField::VISIBLE_ITEM_12_ENCHANTMENT"),
            PlayerField::VISIBLE_ITEM_13_ENTRYID => String::from("PlayerField::VISIBLE_ITEM_13_ENTRYID"),
            PlayerField::VISIBLE_ITEM_13_ENCHANTMENT => String::from("PlayerField::VISIBLE_ITEM_13_ENCHANTMENT"),
            PlayerField::VISIBLE_ITEM_14_ENTRYID => String::from("PlayerField::VISIBLE_ITEM_14_ENTRYID"),
            PlayerField::VISIBLE_ITEM_14_ENCHANTMENT => String::from("PlayerField::VISIBLE_ITEM_14_ENCHANTMENT"),
            PlayerField::VISIBLE_ITEM_15_ENTRYID => String::from("PlayerField::VISIBLE_ITEM_15_ENTRYID"),
            PlayerField::VISIBLE_ITEM_15_ENCHANTMENT => String::from("PlayerField::VISIBLE_ITEM_15_ENCHANTMENT"),
            PlayerField::VISIBLE_ITEM_16_ENTRYID => String::from("PlayerField::VISIBLE_ITEM_16_ENTRYID"),
            PlayerField::VISIBLE_ITEM_16_ENCHANTMENT => String::from("PlayerField::VISIBLE_ITEM_16_ENCHANTMENT"),
            PlayerField::VISIBLE_ITEM_17_ENTRYID => String::from("PlayerField::VISIBLE_ITEM_17_ENTRYID"),
            PlayerField::VISIBLE_ITEM_17_ENCHANTMENT => String::from("PlayerField::VISIBLE_ITEM_17_ENCHANTMENT"),
            PlayerField::VISIBLE_ITEM_18_ENTRYID => String::from("PlayerField::VISIBLE_ITEM_18_ENTRYID"),
            PlayerField::VISIBLE_ITEM_18_ENCHANTMENT => String::from("PlayerField::VISIBLE_ITEM_18_ENCHANTMENT"),
            PlayerField::VISIBLE_ITEM_19_ENTRYID => String::from("PlayerField::VISIBLE_ITEM_19_ENTRYID"),
            PlayerField::VISIBLE_ITEM_19_ENCHANTMENT => String::from("PlayerField::VISIBLE_ITEM_19_ENCHANTMENT"),
            PlayerField::CHOSEN_TITLE => String::from("PlayerField::CHOSEN_TITLE"),
            PlayerField::FAKE_INEBRIATION => String::from("PlayerField::FAKE_INEBRIATION"),
            PlayerField::FIELD_PAD_0 => String::from("PlayerField::FIELD_PAD_0"),
            PlayerField::FIELD_INV_SLOT_HEAD => String::from("PlayerField::FIELD_INV_SLOT_HEAD"),
            PlayerField::FIELD_INV_SLOT_FIXME1 => String::from("PlayerField::FIELD_INV_SLOT_FIXME1"),
            PlayerField::FIELD_INV_SLOT_FIXME2 => String::from("PlayerField::FIELD_INV_SLOT_FIXME2"),
            PlayerField::FIELD_INV_SLOT_FIXME3 => String::from("PlayerField::FIELD_INV_SLOT_FIXME3"),
            PlayerField::FIELD_INV_SLOT_FIXME4 => String::from("PlayerField::FIELD_INV_SLOT_FIXME4"),
            PlayerField::FIELD_INV_SLOT_FIXME5 => String::from("PlayerField::FIELD_INV_SLOT_FIXME5"),
            PlayerField::FIELD_INV_SLOT_FIXME6 => String::from("PlayerField::FIELD_INV_SLOT_FIXME6"),
            PlayerField::FIELD_INV_SLOT_FIXME7 => String::from("PlayerField::FIELD_INV_SLOT_FIXME7"),
            PlayerField::FIELD_INV_SLOT_FIXME8 => String::from("PlayerField::FIELD_INV_SLOT_FIXME8"),
            PlayerField::FIELD_INV_SLOT_FIXME9 => String::from("PlayerField::FIELD_INV_SLOT_FIXME9"),
            PlayerField::FIELD_INV_SLOT_FIXME10 => String::from("PlayerField::FIELD_INV_SLOT_FIXME10"),
            PlayerField::FIELD_INV_SLOT_FIXME11 => String::from("PlayerField::FIELD_INV_SLOT_FIXME11"),
            PlayerField::FIELD_INV_SLOT_FIXME12 => String::from("PlayerField::FIELD_INV_SLOT_FIXME12"),
            PlayerField::FIELD_INV_SLOT_FIXME13 => String::from("PlayerField::FIELD_INV_SLOT_FIXME13"),
            PlayerField::FIELD_INV_SLOT_FIXME14 => String::from("PlayerField::FIELD_INV_SLOT_FIXME14"),
            PlayerField::FIELD_INV_SLOT_FIXME15 => String::from("PlayerField::FIELD_INV_SLOT_FIXME15"),
            PlayerField::FIELD_INV_SLOT_FIXME16 => String::from("PlayerField::FIELD_INV_SLOT_FIXME16"),
            PlayerField::FIELD_INV_SLOT_FIXME17 => String::from("PlayerField::FIELD_INV_SLOT_FIXME17"),
            PlayerField::FIELD_INV_SLOT_FIXME18 => String::from("PlayerField::FIELD_INV_SLOT_FIXME18"),
            PlayerField::FIELD_INV_SLOT_FIXME19 => String::from("PlayerField::FIELD_INV_SLOT_FIXME19"),
            PlayerField::FIELD_INV_SLOT_FIXME20 => String::from("PlayerField::FIELD_INV_SLOT_FIXME20"),
            PlayerField::FIELD_INV_SLOT_FIXME21 => String::from("PlayerField::FIELD_INV_SLOT_FIXME21"),
            PlayerField::FIELD_INV_SLOT_FIXME22 => String::from("PlayerField::FIELD_INV_SLOT_FIXME22"),
            PlayerField::FIELD_PACK_SLOT_1 => String::from("PlayerField::FIELD_PACK_SLOT_1"),
            PlayerField::FIELD_BANK_SLOT_1 => String::from("PlayerField::FIELD_BANK_SLOT_1"),
            PlayerField::FIELD_BANKBAG_SLOT_1 => String::from("PlayerField::FIELD_BANKBAG_SLOT_1"),
            PlayerField::FIELD_VENDORBUYBACK_SLOT_1 => String::from("PlayerField::FIELD_VENDORBUYBACK_SLOT_1"),
            PlayerField::FIELD_KEYRING_SLOT_1 => String::from("PlayerField::FIELD_KEYRING_SLOT_1"),
            PlayerField::FIELD_CURRENCYTOKEN_SLOT_1 => String::from("PlayerField::FIELD_CURRENCYTOKEN_SLOT_1"),
            PlayerField::FARSIGHT => String::from("PlayerField::FARSIGHT"),
            PlayerField::FIELD_KNOWN_TITLES => String::from("PlayerField::FIELD_KNOWN_TITLES"),
            PlayerField::FIELD_KNOWN_TITLES1 => String::from("PlayerField::FIELD_KNOWN_TITLES1"),
            PlayerField::FIELD_KNOWN_TITLES2 => String::from("PlayerField::FIELD_KNOWN_TITLES2"),
            PlayerField::FIELD_KNOWN_CURRENCIES => String::from("PlayerField::FIELD_KNOWN_CURRENCIES"),
            PlayerField::XP => String::from("PlayerField::XP"),
            PlayerField::NEXT_LEVEL_XP => String::from("PlayerField::NEXT_LEVEL_XP"),
            PlayerField::SKILL_INFO_1_1 => String::from("PlayerField::SKILL_INFO_1_1"),
            PlayerField::CHARACTER_POINTS1 => String::from("PlayerField::CHARACTER_POINTS1"),
            PlayerField::CHARACTER_POINTS2 => String::from("PlayerField::CHARACTER_POINTS2"),
            PlayerField::TRACK_CREATURES => String::from("PlayerField::TRACK_CREATURES"),
            PlayerField::TRACK_RESOURCES => String::from("PlayerField::TRACK_RESOURCES"),
            PlayerField::BLOCK_PERCENTAGE => String::from("PlayerField::BLOCK_PERCENTAGE"),
            PlayerField::DODGE_PERCENTAGE => String::from("PlayerField::DODGE_PERCENTAGE"),
            PlayerField::PARRY_PERCENTAGE => String::from("PlayerField::PARRY_PERCENTAGE"),
            PlayerField::EXPERTISE => String::from("PlayerField::EXPERTISE"),
            PlayerField::OFFHAND_EXPERTISE => String::from("PlayerField::OFFHAND_EXPERTISE"),
            PlayerField::CRIT_PERCENTAGE => String::from("PlayerField::CRIT_PERCENTAGE"),
            PlayerField::RANGED_CRIT_PERCENTAGE => String::from("PlayerField::RANGED_CRIT_PERCENTAGE"),
            PlayerField::OFFHAND_CRIT_PERCENTAGE => String::from("PlayerField::OFFHAND_CRIT_PERCENTAGE"),
            PlayerField::SPELL_CRIT_PERCENTAGE1 => String::from("PlayerField::SPELL_CRIT_PERCENTAGE1"),
            PlayerField::SPELL_CRIT_PERCENTAGE2 => String::from("PlayerField::SPELL_CRIT_PERCENTAGE2"),
            PlayerField::SPELL_CRIT_PERCENTAGE3 => String::from("PlayerField::SPELL_CRIT_PERCENTAGE3"),
            PlayerField::SPELL_CRIT_PERCENTAGE4 => String::from("PlayerField::SPELL_CRIT_PERCENTAGE4"),
            PlayerField::SPELL_CRIT_PERCENTAGE5 => String::from("PlayerField::SPELL_CRIT_PERCENTAGE5"),
            PlayerField::SPELL_CRIT_PERCENTAGE6 => String::from("PlayerField::SPELL_CRIT_PERCENTAGE6"),
            PlayerField::SPELL_CRIT_PERCENTAGE7 => String::from("PlayerField::SPELL_CRIT_PERCENTAGE7"),
            PlayerField::SHIELD_BLOCK => String::from("PlayerField::SHIELD_BLOCK"),
            PlayerField::SHIELD_BLOCK_CRIT_PERCENTAGE => String::from("PlayerField::SHIELD_BLOCK_CRIT_PERCENTAGE"),
            PlayerField::EXPLORED_ZONES_1 => String::from("PlayerField::EXPLORED_ZONES_1"),
            PlayerField::REST_STATE_EXPERIENCE => String::from("PlayerField::REST_STATE_EXPERIENCE"),
            PlayerField::FIELD_COINAGE => String::from("PlayerField::FIELD_COINAGE"),
            PlayerField::FIELD_MOD_DAMAGE_DONE_POS => String::from("PlayerField::FIELD_MOD_DAMAGE_DONE_POS"),
            PlayerField::FIELD_MOD_DAMAGE_DONE_NEG => String::from("PlayerField::FIELD_MOD_DAMAGE_DONE_NEG"),
            PlayerField::FIELD_MOD_DAMAGE_DONE_PCT1 => String::from("PlayerField::FIELD_MOD_DAMAGE_DONE_PCT1"),
            PlayerField::FIELD_MOD_DAMAGE_DONE_PCT2 => String::from("PlayerField::FIELD_MOD_DAMAGE_DONE_PCT2"),
            PlayerField::FIELD_MOD_DAMAGE_DONE_PCT3 => String::from("PlayerField::FIELD_MOD_DAMAGE_DONE_PCT3"),
            PlayerField::FIELD_MOD_DAMAGE_DONE_PCT4 => String::from("PlayerField::FIELD_MOD_DAMAGE_DONE_PCT4"),
            PlayerField::FIELD_MOD_DAMAGE_DONE_PCT5 => String::from("PlayerField::FIELD_MOD_DAMAGE_DONE_PCT5"),
            PlayerField::FIELD_MOD_DAMAGE_DONE_PCT6 => String::from("PlayerField::FIELD_MOD_DAMAGE_DONE_PCT6"),
            PlayerField::FIELD_MOD_DAMAGE_DONE_PCT7 => String::from("PlayerField::FIELD_MOD_DAMAGE_DONE_PCT7"),
            PlayerField::FIELD_MOD_HEALING_DONE_POS => String::from("PlayerField::FIELD_MOD_HEALING_DONE_POS"),
            PlayerField::FIELD_MOD_HEALING_PCT => String::from("PlayerField::FIELD_MOD_HEALING_PCT"),
            PlayerField::FIELD_MOD_HEALING_DONE_PCT => String::from("PlayerField::FIELD_MOD_HEALING_DONE_PCT"),
            PlayerField::FIELD_MOD_TARGET_RESISTANCE => String::from("PlayerField::FIELD_MOD_TARGET_RESISTANCE"),
            PlayerField::FIELD_MOD_TARGET_PHYSICAL_RESISTANCE => String::from("PlayerField::FIELD_MOD_TARGET_PHYSICAL_RESISTANCE"),
            PlayerField::FIELD_BYTES => String::from("PlayerField::FIELD_BYTES"),
            PlayerField::AMMO_ID => String::from("PlayerField::AMMO_ID"),
            PlayerField::SELF_RES_SPELL => String::from("PlayerField::SELF_RES_SPELL"),
            PlayerField::FIELD_PVP_MEDALS => String::from("PlayerField::FIELD_PVP_MEDALS"),
            PlayerField::FIELD_BUYBACK_PRICE_1 => String::from("PlayerField::FIELD_BUYBACK_PRICE_1"),
            PlayerField::FIELD_BUYBACK_PRICE_2 => String::from("PlayerField::FIELD_BUYBACK_PRICE_2"),
            PlayerField::FIELD_BUYBACK_PRICE_3 => String::from("PlayerField::FIELD_BUYBACK_PRICE_3"),
            PlayerField::FIELD_BUYBACK_PRICE_4 => String::from("PlayerField::FIELD_BUYBACK_PRICE_4"),
            PlayerField::FIELD_BUYBACK_PRICE_5 => String::from("PlayerField::FIELD_BUYBACK_PRICE_5"),
            PlayerField::FIELD_BUYBACK_PRICE_6 => String::from("PlayerField::FIELD_BUYBACK_PRICE_6"),
            PlayerField::FIELD_BUYBACK_PRICE_7 => String::from("PlayerField::FIELD_BUYBACK_PRICE_7"),
            PlayerField::FIELD_BUYBACK_PRICE_8 => String::from("PlayerField::FIELD_BUYBACK_PRICE_8"),
            PlayerField::FIELD_BUYBACK_PRICE_9 => String::from("PlayerField::FIELD_BUYBACK_PRICE_9"),
            PlayerField::FIELD_BUYBACK_PRICE_10 => String::from("PlayerField::FIELD_BUYBACK_PRICE_10"),
            PlayerField::FIELD_BUYBACK_PRICE_11 => String::from("PlayerField::FIELD_BUYBACK_PRICE_11"),
            PlayerField::FIELD_BUYBACK_PRICE_12 => String::from("PlayerField::FIELD_BUYBACK_PRICE_12"),
            PlayerField::FIELD_BUYBACK_TIMESTAMP_1 => String::from("PlayerField::FIELD_BUYBACK_TIMESTAMP_1"),
            PlayerField::FIELD_BUYBACK_TIMESTAMP_2 => String::from("PlayerField::FIELD_BUYBACK_TIMESTAMP_2"),
            PlayerField::FIELD_BUYBACK_TIMESTAMP_3 => String::from("PlayerField::FIELD_BUYBACK_TIMESTAMP_3"),
            PlayerField::FIELD_BUYBACK_TIMESTAMP_4 => String::from("PlayerField::FIELD_BUYBACK_TIMESTAMP_4"),
            PlayerField::FIELD_BUYBACK_TIMESTAMP_5 => String::from("PlayerField::FIELD_BUYBACK_TIMESTAMP_5"),
            PlayerField::FIELD_BUYBACK_TIMESTAMP_6 => String::from("PlayerField::FIELD_BUYBACK_TIMESTAMP_6"),
            PlayerField::FIELD_BUYBACK_TIMESTAMP_7 => String::from("PlayerField::FIELD_BUYBACK_TIMESTAMP_7"),
            PlayerField::FIELD_BUYBACK_TIMESTAMP_8 => String::from("PlayerField::FIELD_BUYBACK_TIMESTAMP_8"),
            PlayerField::FIELD_BUYBACK_TIMESTAMP_9 => String::from("PlayerField::FIELD_BUYBACK_TIMESTAMP_9"),
            PlayerField::FIELD_BUYBACK_TIMESTAMP_10 => String::from("PlayerField::FIELD_BUYBACK_TIMESTAMP_10"),
            PlayerField::FIELD_BUYBACK_TIMESTAMP_11 => String::from("PlayerField::FIELD_BUYBACK_TIMESTAMP_11"),
            PlayerField::FIELD_BUYBACK_TIMESTAMP_12 => String::from("PlayerField::FIELD_BUYBACK_TIMESTAMP_12"),
            PlayerField::FIELD_KILLS => String::from("PlayerField::FIELD_KILLS"),
            PlayerField::FIELD_TODAY_CONTRIBUTION => String::from("PlayerField::FIELD_TODAY_CONTRIBUTION"),
            PlayerField::FIELD_YESTERDAY_CONTRIBUTION => String::from("PlayerField::FIELD_YESTERDAY_CONTRIBUTION"),
            PlayerField::FIELD_LIFETIME_HONORABLE_KILLS => String::from("PlayerField::FIELD_LIFETIME_HONORABLE_KILLS"),
            PlayerField::FIELD_BYTES2 => String::from("PlayerField::FIELD_BYTES2"),
            PlayerField::FIELD_WATCHED_FACTION_INDEX => String::from("PlayerField::FIELD_WATCHED_FACTION_INDEX"),
            PlayerField::FIELD_COMBAT_RATING_1 => String::from("PlayerField::FIELD_COMBAT_RATING_1"),
            PlayerField::FIELD_ARENA_TEAM_INFO_1_1 => String::from("PlayerField::FIELD_ARENA_TEAM_INFO_1_1"),
            PlayerField::FIELD_HONOR_CURRENCY => String::from("PlayerField::FIELD_HONOR_CURRENCY"),
            PlayerField::FIELD_ARENA_CURRENCY => String::from("PlayerField::FIELD_ARENA_CURRENCY"),
            PlayerField::FIELD_MAX_LEVEL => String::from("PlayerField::FIELD_MAX_LEVEL"),
            PlayerField::FIELD_DAILY_QUESTS_1 => String::from("PlayerField::FIELD_DAILY_QUESTS_1"),
            PlayerField::RUNE_REGEN_1 => String::from("PlayerField::RUNE_REGEN_1"),
            PlayerField::RUNE_REGEN_2 => String::from("PlayerField::RUNE_REGEN_2"),
            PlayerField::RUNE_REGEN_3 => String::from("PlayerField::RUNE_REGEN_3"),
            PlayerField::RUNE_REGEN_4 => String::from("PlayerField::RUNE_REGEN_4"),
            PlayerField::NO_REAGENT_COST_1 => String::from("PlayerField::NO_REAGENT_COST_1"),
            PlayerField::FIELD_GLYPH_SLOTS_1 => String::from("PlayerField::FIELD_GLYPH_SLOTS_1"),
            PlayerField::FIELD_GLYPH_SLOTS_2 => String::from("PlayerField::FIELD_GLYPH_SLOTS_2"),
            PlayerField::FIELD_GLYPH_SLOTS_3 => String::from("PlayerField::FIELD_GLYPH_SLOTS_3"),
            PlayerField::FIELD_GLYPH_SLOTS_4 => String::from("PlayerField::FIELD_GLYPH_SLOTS_4"),
            PlayerField::FIELD_GLYPH_SLOTS_5 => String::from("PlayerField::FIELD_GLYPH_SLOTS_5"),
            PlayerField::FIELD_GLYPH_SLOTS_6 => String::from("PlayerField::FIELD_GLYPH_SLOTS_6"),
            PlayerField::FIELD_GLYPHS_1 => String::from("PlayerField::FIELD_GLYPHS_1"),
            PlayerField::FIELD_GLYPHS_2 => String::from("PlayerField::FIELD_GLYPHS_2"),
            PlayerField::FIELD_GLYPHS_3 => String::from("PlayerField::FIELD_GLYPHS_3"),
            PlayerField::FIELD_GLYPHS_4 => String::from("PlayerField::FIELD_GLYPHS_4"),
            PlayerField::FIELD_GLYPHS_5 => String::from("PlayerField::FIELD_GLYPHS_5"),
            PlayerField::FIELD_GLYPHS_6 => String::from("PlayerField::FIELD_GLYPHS_6"),
            PlayerField::GLYPHS_ENABLED => String::from("PlayerField::GLYPHS_ENABLED"),
            PlayerField::PET_SPELL_POWER => String::from("PlayerField::PET_SPELL_POWER"),
            _ => String::new(),
        }
    }
}

bitflags! {
    #[derive(Default, Clone, Debug, PartialEq)]
    pub struct UnitFlags: u32 {
        // Movement checks disabled; likely paired with loss of client control packet.
        // We use it to add custom cliffwalking to GM mode until actual usecases will be known.
        const UNK_0 = 0x00000001;
        // not attackable
        const SPAWNING = 0x00000002;
        // Generic unspecified loss of control initiated by server script,
        // movement checks disabled; paired with loss of client control packet.
        const CLIENT_CONTROL_LOST = 0x00000004;
        // players, pets, totems, guardians, companions, charms; any units associated with players
        const PLAYER_CONTROLLED = 0x00000008;
        const RENAME = 0x00000010;
        // don't take reagents for spells with SPELL_ATTR_EX5_NO_REAGENT_WHILE_PREP
        const PREPARATION = 0x00000020;
        const UNK_6 = 0x00000040;
        // ?? (UNIT_FLAG_PVP_ATTACKABLE | UNIT_FLAG_NOT_ATTACKABLE_1) is NON_PVP_ATTACKABLE
        const NOT_ATTACKABLE_1 = 0x00000080;
        // Target is immune to players
        const IMMUNE_TO_PLAYER = 0x00000100;
        // Target is immune to Non-Player Characters
        const IMMUNE_TO_NPC = 0x00000200;
        // loot animation
        const LOOTING = 0x00000400;
        // in combat?; 2.0.8
        const PET_IN_COMBAT = 0x00000800;
        // changed in 3.0.3
        const PVP_DEPRECATED = 0x00001000;
        // silenced; 2.1.1
        const SILENCED = 0x00002000;
        // 2.0.8
        const UNK_14 = 0x00004000;
        // related to jerky movement in water?
        const SWIMMING = 0x00008000;
        // is not targetable by attack or spell
        const UNTARGETABLE = 0x00010000;
        // 3.0.3 ok
        const PACIFIED = 0x00020000;
        // Unit is a subject to stun; turn and strafe movement disabled
        const STUNNED = 0x00040000;
        const IN_COMBAT = 0x00080000;
        // Unit is on taxi; paired with a duplicate loss of client control packet (likely a legacy serverside hack).
        // Disables any spellcasts not allowed in taxi flight client-side.
        const TAXI_FLIGHT = 0x00100000;
        // 3.0.3, disable melee spells casting...; "Required melee weapon" added to melee spells tooltip.
        const DISARMED = 0x00200000;
        // Unit is a subject to confused movement, movement checks disabled; paired with loss of client control packet.
        const CONFUSED = 0x00400000;
        // Unit is a subject to fleeing movement, movement checks disabled; paired with loss of client control packet.
        const FLEEING = 0x00800000;
        // Unit is under remote control by another unit, movement checks disabled;
        // paired with loss of client control packet. New master is allowed to use melee attack
        // and can't select this unit via mouse in the world (as if it was own character).
        const POSSESSED = 0x01000000;
        const UNINTERACTIBLE = 0x02000000;
        const SKINNABLE = 0x04000000;
        const MOUNT = 0x08000000;
        const UNK_28 = 0x10000000;
        // used in Feing Death spell
        const PREVENT_ANIM = 0x20000000;
        const SHEATHE = 0x40000000;
        const IMMUNE = 0x80000000;
    }
}

bitflags! {
    #[derive(Default, Clone, Debug, PartialEq)]
    pub struct UnitFlags2: u32 {
        const FEIGN_DEATH = 0x00000001;
        // Hides body and body armor. Weapons and shoulder and head armor still visible
        const HIDE_BODY = 0x00000002;
        const IGNORE_REPUTATION = 0x00000004;
        const COMPREHEND_LANG = 0x00000008;
         // Used in SPELL_AURA_MIRROR_IMAGE
        const CLONED = 0x00000010;
        const DO_NOT_FADE_IN = 0x00000020;
        const FORCE_MOVE = 0x00000040;
        // also shield case
        const DISARM_OFFHAND = 0x00000080;
        const UNK8 = 0x00000100;
        const UNK9 = 0x00000200;
        const DISARM_RANGED = 0x00000400;
        const REGENERATE_POWER = 0x00000800;
        const SPELL_CLICK_IN_GROUP = 0x00001000;
        const SPELL_CLICK_DISABLED = 0x00002000;
        const INTERACT_ANY_REACTION = 0x00004000;
        const UNK15 = 0x00008000;
        const UNK16 = 0x00010000;
        const ALLOW_CHEAT_SPELLS = 0x00040000;
    }
}

bitflags! {
    #[derive(Default, Clone, Debug, PartialEq)]
    pub struct PlayerFlags: u32 {
        const NONE = 0x00000000;
        const GROUP_LEADER = 0x00000001;
        const AFK = 0x00000002;
        const DND = 0x00000004;
        const GM = 0x00000008;
        const GHOST = 0x00000010;
        const RESTING = 0x00000020;
        const UNK7 = 0x00000040;
        // pre-3.0.3 PLAYER_FLAGS_FFA_PVP flag for FFA PVP state
        const UNK8 = 0x00000080;
        // Player has been involved in a PvP combat and will be attacked by contested guards
        const CONTESTED_PVP = 0x00000100;
        // Stores player's permanent PvP flag preference
        const PVP_DESIRED = 0x00000200;
        const HIDE_HELM = 0x00000400;
        const HIDE_CLOAK = 0x00000800;
        // played long time
        const PARTIAL_PLAY_TIME = 0x00001000;
        // played too long time
        const NO_PLAY_TIME = 0x00002000;
        // Lua_IsOutOfBounds
        const IS_OUT_OF_BOUNDS = 0x00004000;
        // <Dev> chat tag; name prefix
        const DEVELOPER = 0x00008000;
        // triggers lua event EVENT_ENABLE_LOW_LEVEL_RAID
        const ENABLE_LOW_LEVEL_RAID = 0x00010000;
        // taxi benchmark mode (on/off) (2.0.1)
        const TAXI_BENCHMARK = 0x00020000;
        // 3.0.2; pvp timer active (after you disable pvp manually or leave pvp zones)
        const PVP_TIMER = 0x00040000;
        // first appeared in TBC
        const COMMENTATOR = 0x00080000;
        const UNK21 = 0x00100000;
        const UNK22 = 0x00200000;
        // something like COMMENTATOR_CAN_USE_INSTANCE_COMMAND
        const COMMENTATOR_UBER = 0x00400000;
        // EVENT_SPELL_UPDATE_USABLE and EVENT_UPDATE_SHAPESHIFT_USABLE; disabled all abilitys on tab except autoattack
        const ALLOW_ONLY_ABILITY = 0x00800000;
        // EVENT_SPELL_UPDATE_USABLE and EVENT_UPDATE_SHAPESHIFT_USABLE;
        // disabled all melee ability on tab include autoattack
        const UNK25 = 0x01000000;
        const XP_USER_DISABLED = 0x02000000;
    }
}