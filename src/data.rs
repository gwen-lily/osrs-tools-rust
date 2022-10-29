#[allow(unused_imports)]
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Damage types

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[allow(dead_code)]
pub enum DT {
    Melee(MeleeDamageType),
    Ranged,
    Magic,
    Typeless,
}

#[derive(Debug, Default, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MeleeDamageType {
    Stab,
    Slash,
    Crush,
    #[default]
    Default,
}

/// Slot

/// Style

/// Skill

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Skill {
    Attack,
    Strength,
    Defence,
    Ranged,
    Prayer,
    Magic,
    Runecraft,
    Construction,
    Hitpoints,
    Agility,
    Herblore,
    Thieving,
    Crafting,
    Fletching,
    Slayer,
    Hunter,
    Mining,
    Smithing,
    Fishing,
    Cooking,
    Firemaking,
    Woodcutting,
    Farming,
}
