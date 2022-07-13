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