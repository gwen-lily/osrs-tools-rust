#[allow(unused_imports)]
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// An enum with the combat options and sub-stance options.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Stance {
    Melee(MeleeStance),
    Ranged(RangedStance),
    Magic(MagicStance),
}

/// All melee stances
#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MeleeStance {
    Accurate,
    Aggressive,
    Controlled,
    Defensive,
}

/// All ranged stances
#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RangedStance {
    Accurate,
    Rapid,
    Longrange,
}

/// All magic stances
#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MagicStance {
    Accurate,
    Longrange,
    NoStyle,
    Defensive,
}
