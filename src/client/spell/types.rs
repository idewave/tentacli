#[non_exhaustive]
pub struct SpellCastTargetType;

#[allow(dead_code)]
impl SpellCastTargetType {
    pub const TARGET_FLAG_SELF: u32 = 0x00000000;
    // not used in any spells as of 2.4.3 (can be set dynamically)
    pub const TARGET_FLAG_UNUSED1: u32 = 0x00000001;
    // pguid
    pub const TARGET_FLAG_UNIT: u32 = 0x00000002;
    // not used in any spells as of 2.4.3 (can be set dynamically) - raid member
    pub const TARGET_FLAG_UNIT_RAID: u32 = 0x00000004;
    // not used in any spells as of 2.4.3 (can be set dynamically) - party member
    pub const TARGET_FLAG_UNIT_PARTY: u32 = 0x00000008;
    // pguid
    pub const TARGET_FLAG_ITEM: u32 = 0x00000010;
    // 3xfloat
    pub const TARGET_FLAG_SOURCE_LOCATION: u32 = 0x00000020;
    // 3xfloat
    pub const TARGET_FLAG_DEST_LOCATION: u32 = 0x00000040;
    // CanAttack == true
    pub const TARGET_FLAG_UNIT_ENEMY: u32 = 0x00000080;
    // CanAssist == true
    pub const TARGET_FLAG_UNIT_ALLY: u32 = 0x00000100;
    // pguid, CanAssist == false
    pub const TARGET_FLAG_CORPSE_ENEMY: u32 = 0x00000200;
    // skinning-like effects
    pub const TARGET_FLAG_UNIT_DEAD: u32 = 0x00000400;
    // pguid, 0 spells in 2.4.3
    pub const TARGET_FLAG_GAMEOBJECT: u32 = 0x00000800;
    // pguid, 0 spells
    pub const TARGET_FLAG_TRADE_ITEM: u32 = 0x00001000;
    // string, 0 spells
    pub const TARGET_FLAG_STRING: u32 = 0x00002000;
    // 199 spells, opening object/lock
    pub const TARGET_FLAG_LOCKED: u32 = 0x00004000;
    // pguid, CanAssist == true
    pub const TARGET_FLAG_CORPSE_ALLY: u32 = 0x00008000;
    // pguid, not used in any spells as of 2.4.3 (can be set dynamically)
    pub const TARGET_FLAG_UNIT_MINIPET: u32 = 0x00010000;
    // used in glyph spells
    pub const TARGET_FLAG_GLYPH: u32 = 0x00020000;

    pub const TARGET_FLAG_UNK3: u32 = 0x00040000;
    pub const TARGET_FLAG_VISUAL_CHAIN: u32 = 0x00080000;
}

#[non_exhaustive]
pub struct CastResult;

#[allow(dead_code)]
impl CastResult {
    pub const FAILED: u8 = 0;
    pub const AFFECTING_COMBAT: u8 = 1;
    pub const ALREADY_AT_FULL_HEALTH: u8 = 2;
    pub const ALREADY_AT_FULL_MANA: u8 = 3;
    pub const ALREADY_AT_FULL_POWER: u8 = 4;
    pub const ALREADY_BEING_TAMED: u8 = 5;
    pub const ALREADY_HAVE_CHARM: u8 = 6;
    pub const ALREADY_HAVE_SUMMON: u8 = 7;
    pub const ALREADY_OPEN: u8 = 8;
    pub const AURA_BOUNCED: u8 = 9;
    pub const AUTOTRACK_INTERRUPTED: u8 = 10;
    pub const BAD_IMPLICIT_TARGETS: u8 = 11;
    pub const BAD_TARGETS: u8 = 12;
    pub const CANT_BE_CHARMED: u8 = 13;
    pub const CANT_BE_DISENCHANTED: u8 = 14;
    pub const CANT_BE_DISENCHANTED_SKILL: u8 = 15;
    pub const CANT_BE_MILLED: u8 = 16;
    pub const CANT_BE_PROSPECTED: u8 = 17;
    pub const CANT_CAST_ON_TAPPED: u8 = 18;
    pub const CANT_DUEL_WHILE_INVISIBLE: u8 = 19;
    pub const CANT_DUEL_WHILE_STEALTHED: u8 = 20;
    pub const CANT_STEALTH: u8 = 21;
    pub const CASTER_AURASTATE: u8 = 22;
    pub const CASTER_DEAD: u8 = 23;
    pub const CHARMED: u8 = 24;
    pub const CHEST_IN_USE: u8 = 25;
    pub const CONFUSED: u8 = 26;
    pub const DONT_REPORT: u8 = 27;
    pub const EQUIPPED_ITEM: u8 = 28;
    pub const EQUIPPED_ITEM_CLASS: u8 = 29;
    pub const EQUIPPED_ITEM_CLASS_MAINHAND: u8 = 30;
    pub const EQUIPPED_ITEM_CLASS_OFFHAND: u8 = 31;
    pub const ERROR: u8 = 32;
    pub const FIZZLE: u8 = 33;
    pub const FLEEING: u8 = 34;
    pub const FOOD_LOWLEVEL: u8 = 35;
    pub const HIGHLEVEL: u8 = 36;
    pub const HUNGER_SATIATED: u8 = 37;
    pub const IMMUNE: u8 = 38;
    pub const INCORRECT_AREA: u8 = 39;
    pub const INTERRUPTED: u8 = 40;
    pub const INTERRUPTED_COMBAT: u8 = 41;
    pub const ITEM_ALREADY_ENCHANTED: u8 = 42;
    pub const ITEM_GONE: u8 = 43;
    pub const ITEM_NOT_FOUND: u8 = 44;
    pub const ITEM_NOT_READY: u8 = 45;
    pub const LEVEL_REQUIREMENT: u8 = 46;
    pub const LINE_OF_SIGHT: u8 = 47;
    pub const LOWLEVEL: u8 = 48;
    pub const LOW_CASTLEVEL: u8 = 49;
    pub const MAINHAND_EMPTY: u8 = 50;
    pub const MOVING: u8 = 51;
    pub const NEED_AMMO: u8 = 52;
    pub const NEED_AMMO_POUCH: u8 = 53;
    pub const NEED_EXOTIC_AMMO: u8 = 54;
    pub const NEED_MORE_ITEMS: u8 = 55;
    pub const NOPATH: u8 = 56;
    pub const NOT_BEHIND: u8 = 57;
    pub const NOT_FISHABLE: u8 = 58;
    pub const NOT_FLYING: u8 = 59;
    pub const NOT_HERE: u8 = 60;
    pub const NOT_INFRONT: u8 = 61;
    pub const NOT_IN_CONTROL: u8 = 62;
    pub const NOT_KNOWN: u8 = 63;
    pub const NOT_MOUNTED: u8 = 64;
    pub const NOT_ON_TAXI: u8 = 65;
    pub const NOT_ON_TRANSPORT: u8 = 66;
    pub const NOT_READY: u8 = 67;
    pub const NOT_SHAPESHIFT: u8 = 68;
    pub const NOT_STANDING: u8 = 69;
    pub const NOT_TRADEABLE: u8 = 70;
    pub const NOT_TRADING: u8 = 71;
    pub const NOT_UNSHEATHED: u8 = 72;
    pub const NOT_WHILE_GHOST: u8 = 73;
    pub const NOT_WHILE_LOOTING: u8 = 74;
    pub const NO_AMMO: u8 = 75;
    pub const NO_CHARGES_REMAIN: u8 = 76;
    pub const NO_CHAMPION: u8 = 77;
    pub const NO_COMBO_POINTS: u8 = 78;
    pub const NO_DUELING: u8 = 79;
    pub const NO_ENDURANCE: u8 = 80;
    pub const NO_FISH: u8 = 81;
    pub const NO_ITEMS_WHILE_SHAPESHIFTED: u8 = 82;
    pub const NO_MOUNTS_ALLOWED: u8 = 83;
    pub const NO_PET: u8 = 84;
    pub const NO_POWER: u8 = 85;
    pub const NOTHING_TO_DISPEL: u8 = 86;
    pub const NOTHING_TO_STEAL: u8 = 87;
    pub const ONLY_ABOVEWATER: u8 = 88;
    pub const ONLY_DAYTIME: u8 = 89;
    pub const ONLY_INDOORS: u8 = 90;
    pub const ONLY_MOUNTED: u8 = 91;
    pub const ONLY_NIGHTTIME: u8 = 92;
    pub const ONLY_OUTDOORS: u8 = 93;
    pub const ONLY_SHAPESHIFT: u8 = 94;
    pub const ONLY_STEALTHED: u8 = 95;
    pub const ONLY_UNDERWATER: u8 = 96;
    pub const OUT_OF_RANGE: u8 = 97;
    pub const PACIFIED: u8 = 98;
    pub const POSSESSED: u8 = 99;
    pub const REAGENTS: u8 = 100;
    pub const REQUIRES_AREA: u8 = 101;
    pub const REQUIRES_SPELL_FOCUS: u8 = 102;
    pub const ROOTED: u8 = 103;
    pub const SILENCED: u8 = 104;
    pub const SPELL_IN_PROGRESS: u8 = 105;
    pub const SPELL_LEARNED: u8 = 106;
    pub const SPELL_UNAVAILABLE: u8 = 107;
    pub const STUNNED: u8 = 108;
    pub const TARGETS_DEAD: u8 = 109;
    pub const TARGET_AFFECTING_COMBAT: u8 = 110;
    pub const TARGET_AURASTATE: u8 = 111;
    pub const TARGET_DUELING: u8 = 112;
    pub const TARGET_ENEMY: u8 = 113;
    pub const TARGET_ENRAGED: u8 = 114;
    pub const TARGET_FRIENDLY: u8 = 115;
    pub const TARGET_IN_COMBAT: u8 = 116;
    pub const TARGET_IS_PLAYER: u8 = 117;
    pub const TARGET_IS_PLAYER_CONTROLLED: u8 = 118;
    pub const TARGET_NOT_DEAD: u8 = 119;
    pub const TARGET_NOT_IN_PARTY: u8 = 120;
    pub const TARGET_NOT_LOOTED: u8 = 121;
    pub const TARGET_NOT_PLAYER: u8 = 122;
    pub const TARGET_NO_POCKETS: u8 = 123;
    pub const TARGET_NO_WEAPONS: u8 = 124;
    pub const TARGET_NO_RANGED_WEAPONS: u8 = 125;
    pub const TARGET_UNSKINNABLE: u8 = 126;
    pub const THIRST_SATIATED: u8 = 127;
    pub const TOO_CLOSE: u8 = 128;
    pub const TOO_MANY_OF_ITEM: u8 = 129;
    pub const TOTEM_CATEGORY: u8 = 130;
    pub const TOTEMS: u8 = 131;
    pub const TRY_AGAIN: u8 = 132;
    pub const UNIT_NOT_BEHIND: u8 = 133;
    pub const UNIT_NOT_INFRONT: u8 = 134;
    pub const WRONG_PET_FOOD: u8 = 135;
    pub const NOT_WHILE_FATIGUED: u8 = 136;
    pub const TARGET_NOT_IN_INSTANCE: u8 = 137;
    pub const NOT_WHILE_TRADING: u8 = 138;
    pub const TARGET_NOT_IN_RAID: u8 = 139;
    pub const TARGET_FREEFORALL: u8 = 140;
    pub const NO_EDIBLE_CORPSES: u8 = 141;
    pub const ONLY_BATTLEGROUNDS: u8 = 142;
    pub const TARGET_NOT_GHOST: u8 = 143;
    pub const TRANSFORM_UNUSABLE: u8 = 144;
    pub const WRONG_WEATHER: u8 = 145;
    pub const DAMAGE_IMMUNE: u8 = 146;
    pub const PREVENTED_BY_MECHANIC: u8 = 147;
    pub const PLAY_TIME: u8 = 148;
    pub const REPUTATION: u8 = 149;
    pub const MIN_SKILL: u8 = 150;
    pub const NOT_IN_ARENA: u8 = 151;
    pub const NOT_ON_SHAPESHIFT: u8 = 152;
    pub const NOT_ON_STEALTHED: u8 = 153;
    pub const NOT_ON_DAMAGE_IMMUNE: u8 = 154;
    pub const NOT_ON_MOUNTED: u8 = 155;
    pub const TOO_SHALLOW: u8 = 156;
    pub const TARGET_NOT_IN_SANCTUARY: u8 = 157;
    pub const TARGET_IS_TRIVIAL: u8 = 158;
    pub const BM_OR_INVISGOD: u8 = 159;
    pub const EXPERT_RIDING_REQUIREMENT: u8 = 160;
    pub const ARTISAN_RIDING_REQUIREMENT: u8 = 161;
    pub const NOT_IDLE: u8 = 162;
    pub const NOT_INACTIVE: u8 = 163;
    pub const PARTIAL_PLAYTIME: u8 = 164;
    pub const NO_PLAYTIME: u8 = 165;
    pub const NOT_IN_BATTLEGROUND: u8 = 166;
    pub const NOT_IN_RAID_INSTANCE: u8 = 167;
    pub const ONLY_IN_ARENA: u8 = 168;
    pub const TARGET_LOCKED_TO_RAID_INSTANCE: u8 = 169;
    pub const ON_USE_ENCHANT: u8 = 170;
    pub const NOT_ON_GROUND: u8 = 171;
    pub const CUSTOM_ERROR: u8 = 172;
    pub const CANT_DO_THAT_RIGHT_NOW: u8 = 173;
    pub const TOO_MANY_SOCKETS: u8 = 174;
    pub const INVALID_GLYPH: u8 = 175;
    pub const UNIQUE_GLYPH: u8 = 176;
    pub const GLYPH_SOCKET_LOCKED: u8 = 177;
    pub const NO_VALID_TARGETS: u8 = 178;
    pub const ITEM_AT_MAX_CHARGES: u8 = 179;
    pub const NOT_IN_BARBERSHOP: u8 = 180;
    pub const FISHING_TOO_LOW: u8 = 181;
    pub const ITEM_ENCHANT_TRADE_WINDOW: u8 = 182;
    pub const SUMMON_PENDING: u8 = 183;
    pub const MAX_SOCKETS: u8 = 184;
    pub const PET_CAN_RENAME: u8 = 185;
    pub const TARGET_CANNOT_BE_RESURRECTED: u8 = 186;
    // actually doesn't exist in client
    pub const UNKNOWN: u8 = 187;

    // custom value, don't must be send to client
    pub const SPELL_NOT_FOUND: u8 = 254;
    pub const SPELL_CAST_OK: u8 = 255;
}