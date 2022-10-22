#[allow(unused_imports)]
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Stance {
    Melee(MeleeStance),
    Ranged(RangedStance),
    Magic(MagicStance),
}

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MeleeStance {
    Accurate,
    Aggressive,
    Controlled,
    Defensive,
}

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RangedStance {
    Accurate,
    Rapid,
    Longrange,
}

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MagicStance {
    Accurate,
    Longrange,
    NoStyle,
}
