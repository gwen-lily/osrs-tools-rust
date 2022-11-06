pub(crate) mod bolts;
pub(crate) mod weapons;

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

/// The corresponding accuracy, strength, and defensive values associated with DT
#[derive(Debug, EnumIter, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum CombatAspect {
    Attack,
    Strength,
    Defence,
}

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
