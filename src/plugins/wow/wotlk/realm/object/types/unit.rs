use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::Serialize;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Gender {
    Male = 0,
    Female = 1,
    None = 2,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Race {
    Human = 1,
    Orc = 2,
    Dwarf = 3,
    NightElf = 4,
    Undead = 5,
    Tauren = 6,
    Gnome = 7,
    Troll = 8,
    Goblin = 9,
    BloodElf = 10,
    Draenei = 11,
    FelOrc = 12,
    Naga = 13,
    Broken = 14,
    Skeleton = 15,
    Vrykul = 16,
    Tuskarr = 17,
    ForestTroll = 18,
    Taunka = 19,
    NorthrendSkeleton = 20,
    IceTroll = 21,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Class {
    Warrior = 1,
    Paladin = 2,
    Hunter = 3,
    Rogue = 4,
    Priest = 5,
    DeathKnight = 6,
    Shaman = 7,
    Mage = 8,
    Warlock = 9,
    Druid = 11,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum PowerType {
    Mana = 0,
    Rage = 1,
    Focus = 2,
    Energy = 3,
    Happiness = 4,
    Runes = 5,
    RunicPower = 6,
}
