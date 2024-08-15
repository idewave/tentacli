use anyhow::{Result as AnyResult};
use std::fmt::{Debug};
use std::io::BufRead;
use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt};
use serde::{Serialize};

use crate::errors::FieldError;
use crate::BinaryConverter;
use crate::types::movement::MovementInfo;
use crate::types::position::{Point3D, Vector3D};
use crate::types::update_data::UpdateData;

#[derive(Serialize, Clone, Default, Debug)]
pub struct Player {
    pub guid: u64,
    pub name: String,
    pub race: u8,
    pub class: u8,
    pub gender: u8,
    pub level: u8,
    #[serde(skip_serializing_if = "UpdateData::is_default")]
    pub update_data: UpdateData,
    #[serde(skip_serializing_if = "MovementInfo::is_default")]
    pub movement_info: MovementInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Vector3D>,
}

impl Player {
    pub fn new(guid: u64, name: String, race: u8, class: u8, gender: u8) -> Self {
        Self {
            guid,
            name,
            race,
            class,
            gender,
            ..Self::default()
        }
    }
}

impl BinaryConverter for Player {
    fn write_into(&mut self, _: &mut Vec<u8>) -> AnyResult<()> {
        todo!()
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
        let label = "Player";

        let guid = reader.read_u64::<LittleEndian>()
            .map_err(|e| FieldError::CannotRead(e, format!("guid:u64 ({})", label)))?;

        let mut name_buf = Vec::new();
        reader.read_until(0, &mut name_buf)
            .map_err(|e| FieldError::CannotRead(e, format!("name_buf:Vec<u8> ({})", label)))?;
        let name = String::from_utf8(
            name_buf[..(name_buf.len() - 1)].to_vec()
        ).map_err(|e| FieldError::InvalidString(e, label.to_owned()))?;

        let race = reader.read_u8()
            .map_err(|e| FieldError::CannotRead(e, format!("race:u8 ({})", label)))?;
        let class = reader.read_u8()
            .map_err(|e| FieldError::CannotRead(e, format!("class:u8 ({})", label)))?;
        let gender = reader.read_u8()
            .map_err(|e| FieldError::CannotRead(e, format!("gender:u8 ({})", label)))?;

        let _skin = reader.read_u8()
            .map_err(|e| FieldError::CannotRead(e, format!("skin:u8 ({})", label)))?;
        let _face = reader.read_u8()
            .map_err(|e| FieldError::CannotRead(e, format!("face:u8 ({})", label)))?;
        let _hair_style = reader.read_u8()
            .map_err(|e| FieldError::CannotRead(e, format!("hair_style:u8 ({})", label)))?;
        let _hair_color = reader.read_u8()
            .map_err(|e| FieldError::CannotRead(e, format!("hair_color:u8 ({})", label)))?;

        let _facial_hair = reader.read_u8()
            .map_err(|e| FieldError::CannotRead(e, format!("facial_hair:u8 ({})", label)))?;
        let level = reader.read_u8()
            .map_err(|e| FieldError::CannotRead(e, format!("level:u8 ({})", label)))?;

        let _zone_id = reader.read_u32::<LittleEndian>()
            .map_err(|e| FieldError::CannotRead(e, format!("zone_id:u32 ({})", label)))?;
        let _map_id = reader.read_u32::<LittleEndian>()
            .map_err(|e| FieldError::CannotRead(e, format!("map_id:u32 ({})", label)))?;

        let _location = Point3D::read_from(reader, &mut vec![])?;

        let _guild_id = reader.read_u32::<LittleEndian>()
            .map_err(|e| FieldError::CannotRead(e, format!("guild_id:u32 ({})", label)))?;
        let _char_flags = reader.read_u32::<LittleEndian>()
            .map_err(|e| FieldError::CannotRead(e, format!("char_flags:u32 ({})", label)))?;
        let _char_customize_flags = reader.read_u32::<LittleEndian>()
            .map_err(|e| FieldError::CannotRead(e, format!("char_customize_flags:u32 ({})", label)))?;

        let _first_login = reader.read_u8()
            .map_err(|e| FieldError::CannotRead(e, format!("first_login:u8 ({})", label)))?;

        let _pet_display_id = reader.read_u32::<LittleEndian>()
            .map_err(|e| FieldError::CannotRead(e, format!("pet_display_id:u32 ({})", label)))?;
        let _pet_level = reader.read_u32::<LittleEndian>()
            .map_err(|e| FieldError::CannotRead(e, format!("pet_level:u32 ({})", label)))?;
        let _pet_family = reader.read_u32::<LittleEndian>()
            .map_err(|e| FieldError::CannotRead(e, format!("pet_family:u32 ({})", label)))?;

        // inventory
        for _ in 0..23 {
            reader.read_u32::<LittleEndian>()
                .map_err(|e| FieldError::CannotRead(e, format!("inventory:u32 ({})", label)))?;
            reader.read_u8()
                .map_err(|e| FieldError::CannotRead(e, format!("inventory:u8 ({})", label)))?;
            reader.read_u32::<LittleEndian>()
                .map_err(|e| FieldError::CannotRead(e, format!("inventory:u32_2 ({})", label)))?;
        }

        let mut player = Player::new(guid, name, race, class, gender);
        player.level = level;

        Ok(player)
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

#[non_exhaustive]
pub struct CharacterCreateResponseCode;

#[allow(dead_code)]
impl CharacterCreateResponseCode {
    pub const CHAR_CREATE_IN_PROGRESS: u8 = 46;
    pub const CHAR_CREATE_SUCCESS: u8 = 47;
    pub const CHAR_CREATE_ERROR: u8 = 48;
    pub const CHAR_CREATE_FAILED: u8 = 49;
    pub const CHAR_CREATE_NAME_IN_USE: u8 = 50;
    pub const CHAR_CREATE_DISABLED: u8 = 51;
    pub const CHAR_CREATE_PVP_TEAMS_VIOLATION: u8 = 52;
    pub const CHAR_CREATE_SERVER_LIMIT: u8 = 53;
    pub const CHAR_CREATE_ACCOUNT_LIMIT: u8 = 54;
    pub const CHAR_CREATE_SERVER_QUEUE: u8 = 55;
    pub const CHAR_CREATE_ONLY_EXISTING: u8 = 56;
    pub const CHAR_CREATE_EXPANSION: u8 = 57;
    pub const CHAR_CREATE_EXPANSION_CLASS: u8 = 58;
    pub const CHAR_CREATE_LEVEL_REQUIREMENT: u8 = 59;
    pub const CHAR_CREATE_UNIQUE_CLASS_LIMIT: u8 = 60;
    pub const CHAR_CREATE_CHARACTER_IN_GUILD: u8 = 61;
    pub const CHAR_CREATE_RESTRICTED_RACECLASS: u8 = 62;
    pub const CHAR_CREATE_CHARACTER_CHOOSE_RACE: u8 = 63;
    pub const CHAR_CREATE_CHARACTER_ARENA_LEADER: u8 = 64;
    pub const CHAR_CREATE_CHARACTER_DELETE_MAIL: u8 = 65;
    pub const CHAR_CREATE_CHARACTER_SWAP_FACTION: u8 = 66;
    pub const CHAR_CREATE_CHARACTER_RACE_ONLY: u8 = 67;
    pub const CHAR_CREATE_CHARACTER_GOLD_LIMIT: u8 = 68;
    pub const CHAR_CREATE_FORCE_LOGIN: u8 = 69;
}