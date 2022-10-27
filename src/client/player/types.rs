use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};

use crate::client::Character;
use crate::parsers::position_parser::types::Position;

#[derive(Clone, Default)]
pub struct Player {
    pub guid: u64,
    pub name: String,
    pub race: u8,
    pub class: u8,
    pub fields: BTreeMap<u32, u32>,
    pub movement_speed: BTreeMap<u8, f32>,
    pub position: Option<Position>,
}

impl Player {
    pub fn new(guid: u64, name: String, race: u8, class: u8) -> Self {
        Self {
            guid,
            name,
            race,
            class,
            fields: BTreeMap::new(),
            movement_speed: BTreeMap::new(),
            position: None,
        }
    }

    pub fn from(character: Character) -> Self {
        Self {
            guid: character.guid,
            name: character.name,
            race: character.race,
            class: character.class,
            fields: BTreeMap::new(),
            movement_speed: BTreeMap::new(),
            position: Some(character.position),
        }
    }

    #[allow(dead_code)]
    pub fn get_health(&mut self) -> u32 {
        *self.fields.get(&(UnitField::UNIT_FIELD_HEALTH as u32)).expect(
            "Health should be set on world enter (if me) or CREATE_OBJECT (another player). \
            Need to investigate why health not set."
        )
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

#[non_exhaustive]
pub struct ObjectField;

#[allow(dead_code)]
impl ObjectField {
    pub const OBJECT_FIELD_GUID: u32 = 0;
    pub const OBJECT_FIELD_TYPE: u32 = 2;
    pub const OBJECT_FIELD_ENTRY: u32 = 3;
    pub const OBJECT_FIELD_SCALE_X: u32 = 4;
    pub const OBJECT_FIELD_PADDING: u32 = 5;
}

pub struct UnitField;

#[allow(dead_code)]
impl UnitField {
    pub const UNIT_FIELD_CHARM: u32 = 6;
    pub const UNIT_FIELD_SUMMON: u32 = 8;
    pub const UNIT_FIELD_CRITTER: u32 = 10;
    pub const UNIT_FIELD_CHARMEDBY: u32 = 12;
    pub const UNIT_FIELD_SUMMONEDBY: u32 = 14;
    pub const UNIT_FIELD_CREATEDBY: u32 = 16;
    pub const UNIT_FIELD_TARGET: u32 = 18;
    pub const UNIT_FIELD_CHANNEL_OBJECT: u32 = 20;
    pub const UNIT_CHANNEL_SPELL: u32 = 22;
    pub const UNIT_FIELD_BYTES_0: u32 = 23;
    pub const UNIT_FIELD_HEALTH: u32 = 24;
    pub const UNIT_FIELD_POWER1: u32 = 25;
    pub const UNIT_FIELD_POWER2: u32 = 26;
    pub const UNIT_FIELD_POWER3: u32 = 27;
    pub const UNIT_FIELD_POWER4: u32 = 28;
    pub const UNIT_FIELD_POWER5: u32 = 29;
    pub const UNIT_FIELD_POWER6: u32 = 30;
    pub const UNIT_FIELD_POWER7: u32 = 31;
    pub const UNIT_FIELD_MAXHEALTH: u32 = 32;
    pub const UNIT_FIELD_MAXPOWER1: u32 = 33;
    pub const UNIT_FIELD_MAXPOWER2: u32 = 34;
    pub const UNIT_FIELD_MAXPOWER3: u32 = 35;
    pub const UNIT_FIELD_MAXPOWER4: u32 = 36;
    pub const UNIT_FIELD_MAXPOWER5: u32 = 37;
    pub const UNIT_FIELD_MAXPOWER6: u32 = 38;
    pub const UNIT_FIELD_MAXPOWER7: u32 = 39;
    pub const UNIT_FIELD_POWER_REGEN_FLAT_MODIFIER: u32 = 40;
    pub const UNIT_FIELD_POWER_REGEN_INTERRUPTED_FLAT_MODIFIER: u32 = 47;
    pub const UNIT_FIELD_LEVEL: u32 = 54;
    pub const UNIT_FIELD_FACTIONTEMPLATE: u32 = 55;
    pub const UNIT_VIRTUAL_ITEM_SLOT_ID1: u32 = 56;
    pub const UNIT_VIRTUAL_ITEM_SLOT_ID2: u32 = 57;
    pub const UNIT_VIRTUAL_ITEM_SLOT_ID3: u32 = 58;
    pub const UNIT_FIELD_FLAGS: u32 = 59;
    pub const UNIT_FIELD_FLAGS_2: u32 = 60;
    pub const UNIT_FIELD_AURASTATE: u32 = 61;
    pub const UNIT_FIELD_BASEATTACKTIME: u32 = 62;
    pub const UNIT_FIELD_UNK63: u32 = 63;
    pub const UNIT_FIELD_RANGEDATTACKTIME: u32 = 64;
    pub const UNIT_FIELD_BOUNDINGRADIUS: u32 = 65;
    pub const UNIT_FIELD_COMBATREACH: u32 = 66;
    pub const UNIT_FIELD_DISPLAYID: u32 = 67;
    pub const UNIT_FIELD_NATIVEDISPLAYID: u32 = 68;
    pub const UNIT_FIELD_MOUNTDISPLAYID: u32 = 69;
    pub const UNIT_FIELD_MINDAMAGE: u32 = 70;
    pub const UNIT_FIELD_MAXDAMAGE: u32 = 71;
    pub const UNIT_FIELD_MINOFFHANDDAMAGE: u32 = 72;
    pub const UNIT_FIELD_MAXOFFHANDDAMAGE: u32 = 73;
    pub const UNIT_FIELD_BYTES_1: u32 = 74;
    pub const UNIT_FIELD_PETNUMBER: u32 = 75;
    pub const UNIT_FIELD_PET_NAME_TIMESTAMP: u32 = 76;
    pub const UNIT_FIELD_PETEXPERIENCE: u32 = 77;
    pub const UNIT_FIELD_PETNEXTLEVELEXP: u32 = 78;
    pub const UNIT_DYNAMIC_FLAGS: u32 = 79;
    pub const UNIT_MOD_CAST_SPEED: u32 = 80;
    pub const UNIT_CREATED_BY_SPELL: u32 = 81;
    pub const UNIT_NPC_FLAGS: u32 = 82;
    pub const UNIT_NPC_EMOTESTATE: u32 = 83;
    pub const UNIT_FIELD_STAT0: u32 = 84;
    pub const UNIT_FIELD_STAT1: u32 = 85;
    pub const UNIT_FIELD_STAT2: u32 = 86;
    pub const UNIT_FIELD_STAT3: u32 = 87;
    pub const UNIT_FIELD_STAT4: u32 = 88;
    pub const UNIT_FIELD_POSSTAT0: u32 = 89;
    pub const UNIT_FIELD_POSSTAT1: u32 = 90;
    pub const UNIT_FIELD_POSSTAT2: u32 = 91;
    pub const UNIT_FIELD_POSSTAT3: u32 = 92;
    pub const UNIT_FIELD_POSSTAT4: u32 = 93;
    pub const UNIT_FIELD_NEGSTAT0: u32 = 94;
    pub const UNIT_FIELD_NEGSTAT1: u32 = 95;
    pub const UNIT_FIELD_NEGSTAT2: u32 = 96;
    pub const UNIT_FIELD_NEGSTAT3: u32 = 97;
    pub const UNIT_FIELD_NEGSTAT4: u32 = 98;
    pub const UNIT_FIELD_RESISTANCES_ARMOR: u32 = 99;
    pub const UNIT_FIELD_RESISTANCES_HOLY: u32 = 100;
    pub const UNIT_FIELD_RESISTANCES_FIRE: u32 = 101;
    pub const UNIT_FIELD_RESISTANCES_NATURE: u32 = 102;
    pub const UNIT_FIELD_RESISTANCES_FROST: u32 = 103;
    pub const UNIT_FIELD_RESISTANCES_SHADOW: u32 = 104;
    pub const UNIT_FIELD_RESISTANCES_ARCANE: u32 = 105;
    pub const UNIT_FIELD_RESISTANCEBUFFMODSPOSITIVE_ARMOR: u32 = 106;
    pub const UNIT_FIELD_RESISTANCEBUFFMODSPOSITIVE_HOLY: u32 = 107;
    pub const UNIT_FIELD_RESISTANCEBUFFMODSPOSITIVE_FIRE: u32 = 108;
    pub const UNIT_FIELD_RESISTANCEBUFFMODSPOSITIVE_NATURE: u32 = 109;
    pub const UNIT_FIELD_RESISTANCEBUFFMODSPOSITIVE_FROST: u32 = 110;
    pub const UNIT_FIELD_RESISTANCEBUFFMODSPOSITIVE_SHADOW: u32 = 111;
    pub const UNIT_FIELD_RESISTANCEBUFFMODSPOSITIVE_ARCANE: u32 = 112;
    pub const UNIT_FIELD_RESISTANCEBUFFMODSNEGATIVE_ARMOR: u32 = 113;
    pub const UNIT_FIELD_RESISTANCEBUFFMODSNEGATIVE_HOLY: u32 = 114;
    pub const UNIT_FIELD_RESISTANCEBUFFMODSNEGATIVE_FIRE: u32 = 115;
    pub const UNIT_FIELD_RESISTANCEBUFFMODSNEGATIVE_NATURE: u32 = 116;
    pub const UNIT_FIELD_RESISTANCEBUFFMODSNEGATIVE_FROST: u32 = 117;
    pub const UNIT_FIELD_RESISTANCEBUFFMODSNEGATIVE_SHADOW: u32 = 118;
    pub const UNIT_FIELD_RESISTANCEBUFFMODSNEGATIVE_ARCANE: u32 = 119;
    pub const UNIT_FIELD_BASE_MANA: u32 = 120;
    pub const UNIT_FIELD_BASE_HEALTH: u32 = 121;
    pub const UNIT_FIELD_BYTES_2: u32 = 122;
    pub const UNIT_FIELD_ATTACK_POWER: u32 = 123;
    pub const UNIT_FIELD_ATTACK_POWER_MODS: u32 = 124;
    pub const UNIT_FIELD_ATTACK_POWER_MULTIPLIER: u32 = 125;
    pub const UNIT_FIELD_RANGED_ATTACK_POWER: u32 = 126;
    pub const UNIT_FIELD_RANGED_ATTACK_POWER_MODS: u32 = 127;
    pub const UNIT_FIELD_RANGED_ATTACK_POWER_MULTIPLIER: u32 = 128;
    pub const UNIT_FIELD_MINRANGEDDAMAGE: u32 = 129;
    pub const UNIT_FIELD_MAXRANGEDDAMAGE: u32 = 130;
    pub const UNIT_FIELD_POWER_COST_MODIFIER: u32 = 131;
    pub const UNIT_FIELD_POWER_COST_MULTIPLIER1: u32 = 138;
    pub const UNIT_FIELD_POWER_COST_MULTIPLIER2: u32 = 139;
    pub const UNIT_FIELD_POWER_COST_MULTIPLIER3: u32 = 140;
    pub const UNIT_FIELD_POWER_COST_MULTIPLIER4: u32 = 141;
    pub const UNIT_FIELD_POWER_COST_MULTIPLIER5: u32 = 142;
    pub const UNIT_FIELD_POWER_COST_MULTIPLIER6: u32 = 143;
    pub const UNIT_FIELD_POWER_COST_MULTIPLIER7: u32 = 144;
    pub const UNIT_FIELD_MAXHEALTHMODIFIER: u32 = 145;
    pub const UNIT_FIELD_HOVERHEIGHT: u32 = 146;
    pub const UNIT_FIELD_PADDING: u32 = 147;
}

pub struct PlayerField;

#[allow(dead_code)]
impl PlayerField {
    pub const PLAYER_DUEL_ARBITER: u32 = 148;
    pub const PLAYER_FLAGS: u32 = 150;
    pub const PLAYER_GUILDID: u32 = 151;
    pub const PLAYER_GUILDRANK: u32 = 152;
    pub const PLAYER_BYTES: u32 = 153;
    pub const PLAYER_BYTES_2: u32 = 154;
    pub const PLAYER_BYTES_3: u32 = 155;
    pub const PLAYER_DUEL_TEAM: u32 = 156;
    pub const PLAYER_GUILD_TIMESTAMP: u32 = 157;
    pub const PLAYER_QUEST_LOG_1_1: u32 = 158;
    pub const PLAYER_QUEST_LOG_1_2: u32 = 159;
    pub const PLAYER_QUEST_LOG_1_3: u32 = 160;
    pub const PLAYER_QUEST_LOG_1_4: u32 = 162;
    pub const PLAYER_QUEST_LOG_2_1: u32 = 163;
    pub const PLAYER_QUEST_LOG_2_2: u32 = 164;
    pub const PLAYER_QUEST_LOG_2_3: u32 = 165;
    pub const PLAYER_QUEST_LOG_2_5: u32 = 167;
    pub const PLAYER_QUEST_LOG_3_1: u32 = 168;
    pub const PLAYER_QUEST_LOG_3_2: u32 = 169;
    pub const PLAYER_QUEST_LOG_3_3: u32 = 170;
    pub const PLAYER_QUEST_LOG_3_5: u32 = 172;
    pub const PLAYER_QUEST_LOG_4_1: u32 = 173;
    pub const PLAYER_QUEST_LOG_4_2: u32 = 174;
    pub const PLAYER_QUEST_LOG_4_3: u32 = 175;
    pub const PLAYER_QUEST_LOG_4_5: u32 = 177;
    pub const PLAYER_QUEST_LOG_5_1: u32 = 178;
    pub const PLAYER_QUEST_LOG_5_2: u32 = 179;
    pub const PLAYER_QUEST_LOG_5_3: u32 = 180;
    pub const PLAYER_QUEST_LOG_5_5: u32 = 182;
    pub const PLAYER_QUEST_LOG_6_1: u32 = 183;
    pub const PLAYER_QUEST_LOG_6_2: u32 = 184;
    pub const PLAYER_QUEST_LOG_6_3: u32 = 185;
    pub const PLAYER_QUEST_LOG_6_5: u32 = 187;
    pub const PLAYER_QUEST_LOG_7_1: u32 = 188;
    pub const PLAYER_QUEST_LOG_7_2: u32 = 189;
    pub const PLAYER_QUEST_LOG_7_3: u32 = 190;
    pub const PLAYER_QUEST_LOG_7_5: u32 = 192;
    pub const PLAYER_QUEST_LOG_8_1: u32 = 193;
    pub const PLAYER_QUEST_LOG_8_2: u32 = 194;
    pub const PLAYER_QUEST_LOG_8_3: u32 = 195;
    pub const PLAYER_QUEST_LOG_8_5: u32 = 197;
    pub const PLAYER_QUEST_LOG_9_1: u32 = 198;
    pub const PLAYER_QUEST_LOG_9_2: u32 = 199;
    pub const PLAYER_QUEST_LOG_9_3: u32 = 200;
    pub const PLAYER_QUEST_LOG_9_5: u32 = 202;
    pub const PLAYER_QUEST_LOG_10_1: u32 = 203;
    pub const PLAYER_QUEST_LOG_10_2: u32 = 204;
    pub const PLAYER_QUEST_LOG_10_3: u32 = 205;
    pub const PLAYER_QUEST_LOG_10_5: u32 = 207;
    pub const PLAYER_QUEST_LOG_11_1: u32 = 208;
    pub const PLAYER_QUEST_LOG_11_2: u32 = 209;
    pub const PLAYER_QUEST_LOG_11_3: u32 = 210;
    pub const PLAYER_QUEST_LOG_11_5: u32 = 212;
    pub const PLAYER_QUEST_LOG_12_1: u32 = 213;
    pub const PLAYER_QUEST_LOG_12_2: u32 = 214;
    pub const PLAYER_QUEST_LOG_12_3: u32 = 215;
    pub const PLAYER_QUEST_LOG_12_5: u32 = 217;
    pub const PLAYER_QUEST_LOG_13_1: u32 = 218;
    pub const PLAYER_QUEST_LOG_13_2: u32 = 219;
    pub const PLAYER_QUEST_LOG_13_3: u32 = 220;
    pub const PLAYER_QUEST_LOG_13_5: u32 = 222;
    pub const PLAYER_QUEST_LOG_14_1: u32 = 223;
    pub const PLAYER_QUEST_LOG_14_2: u32 = 224;
    pub const PLAYER_QUEST_LOG_14_3: u32 = 225;
    pub const PLAYER_QUEST_LOG_14_5: u32 = 227;
    pub const PLAYER_QUEST_LOG_15_1: u32 = 228;
    pub const PLAYER_QUEST_LOG_15_2: u32 = 229;
    pub const PLAYER_QUEST_LOG_15_3: u32 = 230;
    pub const PLAYER_QUEST_LOG_15_5: u32 = 232;
    pub const PLAYER_QUEST_LOG_16_1: u32 = 233;
    pub const PLAYER_QUEST_LOG_16_2: u32 = 234;
    pub const PLAYER_QUEST_LOG_16_3: u32 = 235;
    pub const PLAYER_QUEST_LOG_16_5: u32 = 237;
    pub const PLAYER_QUEST_LOG_17_1: u32 = 238;
    pub const PLAYER_QUEST_LOG_17_2: u32 = 239;
    pub const PLAYER_QUEST_LOG_17_3: u32 = 240;
    pub const PLAYER_QUEST_LOG_17_5: u32 = 242;
    pub const PLAYER_QUEST_LOG_18_1: u32 = 243;
    pub const PLAYER_QUEST_LOG_18_2: u32 = 244;
    pub const PLAYER_QUEST_LOG_18_3: u32 = 245;
    pub const PLAYER_QUEST_LOG_18_5: u32 = 247;
    pub const PLAYER_QUEST_LOG_19_1: u32 = 248;
    pub const PLAYER_QUEST_LOG_19_2: u32 = 249;
    pub const PLAYER_QUEST_LOG_19_3: u32 = 250;
    pub const PLAYER_QUEST_LOG_19_5: u32 = 252;
    pub const PLAYER_QUEST_LOG_20_1: u32 = 253;
    pub const PLAYER_QUEST_LOG_20_2: u32 = 254;
    pub const PLAYER_QUEST_LOG_20_3: u32 = 255;
    pub const PLAYER_QUEST_LOG_20_5: u32 = 257;
    pub const PLAYER_QUEST_LOG_21_1: u32 = 258;
    pub const PLAYER_QUEST_LOG_21_2: u32 = 259;
    pub const PLAYER_QUEST_LOG_21_3: u32 = 260;
    pub const PLAYER_QUEST_LOG_21_5: u32 = 262;
    pub const PLAYER_QUEST_LOG_22_1: u32 = 263;
    pub const PLAYER_QUEST_LOG_22_2: u32 = 264;
    pub const PLAYER_QUEST_LOG_22_3: u32 = 265;
    pub const PLAYER_QUEST_LOG_22_5: u32 = 267;
    pub const PLAYER_QUEST_LOG_23_1: u32 = 268;
    pub const PLAYER_QUEST_LOG_23_2: u32 = 269;
    pub const PLAYER_QUEST_LOG_23_3: u32 = 270;
    pub const PLAYER_QUEST_LOG_23_5: u32 = 272;
    pub const PLAYER_QUEST_LOG_24_1: u32 = 273;
    pub const PLAYER_QUEST_LOG_24_2: u32 = 274;
    pub const PLAYER_QUEST_LOG_24_3: u32 = 275;
    pub const PLAYER_QUEST_LOG_24_5: u32 = 277;
    pub const PLAYER_QUEST_LOG_25_1: u32 = 278;
    pub const PLAYER_QUEST_LOG_25_2: u32 = 279;
    pub const PLAYER_QUEST_LOG_25_3: u32 = 280;
    pub const PLAYER_QUEST_LOG_25_5: u32 = 282;
    pub const PLAYER_VISIBLE_ITEM_1_ENTRYID: u32 = 283;
    pub const PLAYER_VISIBLE_ITEM_1_ENCHANTMENT: u32 = 284;
    pub const PLAYER_VISIBLE_ITEM_2_ENTRYID: u32 = 285;
    pub const PLAYER_VISIBLE_ITEM_2_ENCHANTMENT: u32 = 286;
    pub const PLAYER_VISIBLE_ITEM_3_ENTRYID: u32 = 287;
    pub const PLAYER_VISIBLE_ITEM_3_ENCHANTMENT: u32 = 288;
    pub const PLAYER_VISIBLE_ITEM_4_ENTRYID: u32 = 289;
    pub const PLAYER_VISIBLE_ITEM_4_ENCHANTMENT: u32 = 290;
    pub const PLAYER_VISIBLE_ITEM_5_ENTRYID: u32 = 291;
    pub const PLAYER_VISIBLE_ITEM_5_ENCHANTMENT: u32 = 292;
    pub const PLAYER_VISIBLE_ITEM_6_ENTRYID: u32 = 293;
    pub const PLAYER_VISIBLE_ITEM_6_ENCHANTMENT: u32 = 294;
    pub const PLAYER_VISIBLE_ITEM_7_ENTRYID: u32 = 295;
    pub const PLAYER_VISIBLE_ITEM_7_ENCHANTMENT: u32 = 296;
    pub const PLAYER_VISIBLE_ITEM_8_ENTRYID: u32 = 297;
    pub const PLAYER_VISIBLE_ITEM_8_ENCHANTMENT: u32 = 298;
    pub const PLAYER_VISIBLE_ITEM_9_ENTRYID: u32 = 299;
    pub const PLAYER_VISIBLE_ITEM_9_ENCHANTMENT: u32 = 300;
    pub const PLAYER_VISIBLE_ITEM_10_ENTRYID: u32 = 301;
    pub const PLAYER_VISIBLE_ITEM_10_ENCHANTMENT: u32 = 302;
    pub const PLAYER_VISIBLE_ITEM_11_ENTRYID: u32 = 303;
    pub const PLAYER_VISIBLE_ITEM_11_ENCHANTMENT: u32 = 304;
    pub const PLAYER_VISIBLE_ITEM_12_ENTRYID: u32 = 305;
    pub const PLAYER_VISIBLE_ITEM_12_ENCHANTMENT: u32 = 306;
    pub const PLAYER_VISIBLE_ITEM_13_ENTRYID: u32 = 307;
    pub const PLAYER_VISIBLE_ITEM_13_ENCHANTMENT: u32 = 308;
    pub const PLAYER_VISIBLE_ITEM_14_ENTRYID: u32 = 309;
    pub const PLAYER_VISIBLE_ITEM_14_ENCHANTMENT: u32 = 310;
    pub const PLAYER_VISIBLE_ITEM_15_ENTRYID: u32 = 311;
    pub const PLAYER_VISIBLE_ITEM_15_ENCHANTMENT: u32 = 312;
    pub const PLAYER_VISIBLE_ITEM_16_ENTRYID: u32 = 313;
    pub const PLAYER_VISIBLE_ITEM_16_ENCHANTMENT: u32 = 314;
    pub const PLAYER_VISIBLE_ITEM_17_ENTRYID: u32 = 315;
    pub const PLAYER_VISIBLE_ITEM_17_ENCHANTMENT: u32 = 316;
    pub const PLAYER_VISIBLE_ITEM_18_ENTRYID: u32 = 317;
    pub const PLAYER_VISIBLE_ITEM_18_ENCHANTMENT: u32 = 318;
    pub const PLAYER_VISIBLE_ITEM_19_ENTRYID: u32 = 319;
    pub const PLAYER_VISIBLE_ITEM_19_ENCHANTMENT: u32 = 320;
    pub const PLAYER_CHOSEN_TITLE: u32 = 321;
    pub const PLAYER_FAKE_INEBRIATION: u32 = 322;
    pub const PLAYER_FIELD_PAD_0: u32 = 323;
    pub const PLAYER_FIELD_INV_SLOT_HEAD: u32 = 324;
    pub const PLAYER_FIELD_INV_SLOT_FIXME1: u32 = 326;
    pub const PLAYER_FIELD_INV_SLOT_FIXME2: u32 = 328;
    pub const PLAYER_FIELD_INV_SLOT_FIXME3: u32 = 330;
    pub const PLAYER_FIELD_INV_SLOT_FIXME4: u32 = 332;
    pub const PLAYER_FIELD_INV_SLOT_FIXME5: u32 = 334;
    pub const PLAYER_FIELD_INV_SLOT_FIXME6: u32 = 336;
    pub const PLAYER_FIELD_INV_SLOT_FIXME7: u32 = 338;
    pub const PLAYER_FIELD_INV_SLOT_FIXME8: u32 = 340;
    pub const PLAYER_FIELD_INV_SLOT_FIXME9: u32 = 342;
    pub const PLAYER_FIELD_INV_SLOT_FIXME10: u32 = 344;
    pub const PLAYER_FIELD_INV_SLOT_FIXME11: u32 = 346;
    pub const PLAYER_FIELD_INV_SLOT_FIXME12: u32 = 348;
    pub const PLAYER_FIELD_INV_SLOT_FIXME13: u32 = 350;
    pub const PLAYER_FIELD_INV_SLOT_FIXME14: u32 = 352;
    pub const PLAYER_FIELD_INV_SLOT_FIXME15: u32 = 354;
    pub const PLAYER_FIELD_INV_SLOT_FIXME16: u32 = 356;
    pub const PLAYER_FIELD_INV_SLOT_FIXME17: u32 = 358;
    pub const PLAYER_FIELD_INV_SLOT_FIXME18: u32 = 360;
    pub const PLAYER_FIELD_INV_SLOT_FIXME19: u32 = 362;
    pub const PLAYER_FIELD_INV_SLOT_FIXME20: u32 = 364;
    pub const PLAYER_FIELD_INV_SLOT_FIXME21: u32 = 366;
    pub const PLAYER_FIELD_INV_SLOT_FIXME22: u32 = 368;
    pub const PLAYER_FIELD_PACK_SLOT_1: u32 = 370;
    pub const PLAYER_FIELD_BANK_SLOT_1: u32 = 402;
    pub const PLAYER_FIELD_BANKBAG_SLOT_1: u32 = 458;
    pub const PLAYER_FIELD_VENDORBUYBACK_SLOT_1: u32 = 472;
    pub const PLAYER_FIELD_KEYRING_SLOT_1: u32 = 496;
    pub const PLAYER_FIELD_CURRENCYTOKEN_SLOT_1: u32 = 560;
    pub const PLAYER_FARSIGHT: u32 = 624;
    pub const PLAYER_FIELD_KNOWN_TITLES: u32 = 626;
    pub const PLAYER_FIELD_KNOWN_TITLES1: u32 = 628;
    pub const PLAYER_FIELD_KNOWN_TITLES2: u32 = 630;
    pub const PLAYER_FIELD_KNOWN_CURRENCIES: u32 = 632;
    pub const PLAYER_XP: u32 = 634;
    pub const PLAYER_NEXT_LEVEL_XP: u32 = 635;
    pub const PLAYER_SKILL_INFO_1_1: u32 = 636;
    pub const PLAYER_CHARACTER_POINTS1: u32 = 1020;
    pub const PLAYER_CHARACTER_POINTS2: u32 = 1021;
    pub const PLAYER_TRACK_CREATURES: u32 = 1022;
    pub const PLAYER_TRACK_RESOURCES: u32 = 1023;
    pub const PLAYER_BLOCK_PERCENTAGE: u32 = 1024;
    pub const PLAYER_DODGE_PERCENTAGE: u32 = 1025;
    pub const PLAYER_PARRY_PERCENTAGE: u32 = 1026;
    pub const PLAYER_EXPERTISE: u32 = 1027;
    pub const PLAYER_OFFHAND_EXPERTISE: u32 = 1028;
    pub const PLAYER_CRIT_PERCENTAGE: u32 = 1029;
    pub const PLAYER_RANGED_CRIT_PERCENTAGE: u32 = 1030;
    pub const PLAYER_OFFHAND_CRIT_PERCENTAGE: u32 = 1031;
    pub const PLAYER_SPELL_CRIT_PERCENTAGE1: u32 = 1032;
    pub const PLAYER_SPELL_CRIT_PERCENTAGE2: u32 = 1033;
    pub const PLAYER_SPELL_CRIT_PERCENTAGE3: u32 = 1034;
    pub const PLAYER_SPELL_CRIT_PERCENTAGE4: u32 = 1035;
    pub const PLAYER_SPELL_CRIT_PERCENTAGE5: u32 = 1036;
    pub const PLAYER_SPELL_CRIT_PERCENTAGE6: u32 = 1037;
    pub const PLAYER_SPELL_CRIT_PERCENTAGE7: u32 = 1038;
    pub const PLAYER_SHIELD_BLOCK: u32 = 1039;
    pub const PLAYER_SHIELD_BLOCK_CRIT_PERCENTAGE: u32 = 1040;
    pub const PLAYER_EXPLORED_ZONES_1: u32 = 1041;
    pub const PLAYER_REST_STATE_EXPERIENCE: u32 = 1169;
    pub const PLAYER_FIELD_COINAGE: u32 = 1170;
    pub const PLAYER_FIELD_MOD_DAMAGE_DONE_POS: u32 = 1171;
    pub const PLAYER_FIELD_MOD_DAMAGE_DONE_NEG: u32 = 1178;
    pub const PLAYER_FIELD_MOD_DAMAGE_DONE_PCT1: u32 = 1185;
    pub const PLAYER_FIELD_MOD_DAMAGE_DONE_PCT2: u32 = 1186;
    pub const PLAYER_FIELD_MOD_DAMAGE_DONE_PCT3: u32 = 1187;
    pub const PLAYER_FIELD_MOD_DAMAGE_DONE_PCT4: u32 = 1188;
    pub const PLAYER_FIELD_MOD_DAMAGE_DONE_PCT5: u32 = 1189;
    pub const PLAYER_FIELD_MOD_DAMAGE_DONE_PCT6: u32 = 1190;
    pub const PLAYER_FIELD_MOD_DAMAGE_DONE_PCT7: u32 = 1191;
    pub const PLAYER_FIELD_MOD_HEALING_DONE_POS: u32 = 1192;
    pub const PLAYER_FIELD_MOD_HEALING_PCT: u32 = 1193;
    pub const PLAYER_FIELD_MOD_HEALING_DONE_PCT: u32 = 1194;
    pub const PLAYER_FIELD_MOD_TARGET_RESISTANCE: u32 = 1195;
    pub const PLAYER_FIELD_MOD_TARGET_PHYSICAL_RESISTANCE: u32 = 1196;
    pub const PLAYER_FIELD_BYTES: u32 = 1197;
    pub const PLAYER_AMMO_ID: u32 = 1198;
    pub const PLAYER_SELF_RES_SPELL: u32 = 1199;
    pub const PLAYER_FIELD_PVP_MEDALS: u32 = 1200;
    pub const PLAYER_FIELD_BUYBACK_PRICE_1: u32 = 1201;
    pub const PLAYER_FIELD_BUYBACK_PRICE_2: u32 = 1202;
    pub const PLAYER_FIELD_BUYBACK_PRICE_3: u32 = 1203;
    pub const PLAYER_FIELD_BUYBACK_PRICE_4: u32 = 1204;
    pub const PLAYER_FIELD_BUYBACK_PRICE_5: u32 = 1205;
    pub const PLAYER_FIELD_BUYBACK_PRICE_6: u32 = 1206;
    pub const PLAYER_FIELD_BUYBACK_PRICE_7: u32 = 1207;
    pub const PLAYER_FIELD_BUYBACK_PRICE_8: u32 = 1208;
    pub const PLAYER_FIELD_BUYBACK_PRICE_9: u32 = 1209;
    pub const PLAYER_FIELD_BUYBACK_PRICE_10: u32 = 1210;
    pub const PLAYER_FIELD_BUYBACK_PRICE_11: u32 = 1211;
    pub const PLAYER_FIELD_BUYBACK_PRICE_12: u32 = 1212;
    pub const PLAYER_FIELD_BUYBACK_TIMESTAMP_1: u32 = 1213;
    pub const PLAYER_FIELD_BUYBACK_TIMESTAMP_2: u32 = 1214;
    pub const PLAYER_FIELD_BUYBACK_TIMESTAMP_3: u32 = 1215;
    pub const PLAYER_FIELD_BUYBACK_TIMESTAMP_4: u32 = 1216;
    pub const PLAYER_FIELD_BUYBACK_TIMESTAMP_5: u32 = 1217;
    pub const PLAYER_FIELD_BUYBACK_TIMESTAMP_6: u32 = 1218;
    pub const PLAYER_FIELD_BUYBACK_TIMESTAMP_7: u32 = 1219;
    pub const PLAYER_FIELD_BUYBACK_TIMESTAMP_8: u32 = 1220;
    pub const PLAYER_FIELD_BUYBACK_TIMESTAMP_9: u32 = 1221;
    pub const PLAYER_FIELD_BUYBACK_TIMESTAMP_10: u32 = 1222;
    pub const PLAYER_FIELD_BUYBACK_TIMESTAMP_11: u32 = 1223;
    pub const PLAYER_FIELD_BUYBACK_TIMESTAMP_12: u32 = 1224;
    pub const PLAYER_FIELD_KILLS: u32 = 1225;
    pub const PLAYER_FIELD_TODAY_CONTRIBUTION: u32 = 1226;
    pub const PLAYER_FIELD_YESTERDAY_CONTRIBUTION: u32 = 1227;
    pub const PLAYER_FIELD_LIFETIME_HONORABLE_KILLS: u32 = 1228;
    pub const PLAYER_FIELD_BYTES2: u32 = 1229;
    pub const PLAYER_FIELD_WATCHED_FACTION_INDEX: u32 = 1230;
    pub const PLAYER_FIELD_COMBAT_RATING_1: u32 = 1231;
    pub const PLAYER_FIELD_ARENA_TEAM_INFO_1_1: u32 = 1256;
    pub const PLAYER_FIELD_HONOR_CURRENCY: u32 = 1277;
    pub const PLAYER_FIELD_ARENA_CURRENCY: u32 = 1278;
    pub const PLAYER_FIELD_MAX_LEVEL: u32 = 1279;
    pub const PLAYER_FIELD_DAILY_QUESTS_1: u32 = 1280;
    pub const PLAYER_RUNE_REGEN_1: u32 = 1305;
    pub const PLAYER_RUNE_REGEN_2: u32 = 1306;
    pub const PLAYER_RUNE_REGEN_3: u32 = 1307;
    pub const PLAYER_RUNE_REGEN_4: u32 = 1308;
    pub const PLAYER_NO_REAGENT_COST_1: u32 = 1309;
    pub const PLAYER_FIELD_GLYPH_SLOTS_1: u32 = 1312;
    pub const PLAYER_FIELD_GLYPH_SLOTS_2: u32 = 1313;
    pub const PLAYER_FIELD_GLYPH_SLOTS_3: u32 = 1314;
    pub const PLAYER_FIELD_GLYPH_SLOTS_4: u32 = 1315;
    pub const PLAYER_FIELD_GLYPH_SLOTS_5: u32 = 1316;
    pub const PLAYER_FIELD_GLYPH_SLOTS_6: u32 = 1317;
    pub const PLAYER_FIELD_GLYPHS_1: u32 = 1318;
    pub const PLAYER_FIELD_GLYPHS_2: u32 = 1319;
    pub const PLAYER_FIELD_GLYPHS_3: u32 = 1320;
    pub const PLAYER_FIELD_GLYPHS_4: u32 = 1321;
    pub const PLAYER_FIELD_GLYPHS_5: u32 = 1322;
    pub const PLAYER_FIELD_GLYPHS_6: u32 = 1323;
    pub const PLAYER_GLYPHS_ENABLED: u32 = 1324;
    pub const PLAYER_PET_SPELL_POWER: u32 = 1325;
}