pub mod spells;

pub use spells::{SpellName, SpellsMap};

/// A StandardSpell is on the Standard spellbook and has limited properties.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Spell {
    pub name: SpellName,
    pub base_max: u8,
    pub attack_speed: u8,
    pub max_targets: u8,
    pub spellbook: Spellbook,
    pub aspect: Option<Aspect>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Hash)]
pub enum Spellbook {
    Standard,
    Ancient,
    Lunar,
    Arceus,
    Powered,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Hash)]
pub enum Aspect {
    Wind,
    Water,
    Earth,
    Fire,
    Smoke,
    Shadow,
    Blood,
    Ice,
}

use Spellbook::*;

impl Spell {
    pub fn standard(name: SpellName, base_max: u8, aspect: Option<Aspect>) -> Self {
        Self {
            name,
            base_max,
            attack_speed: 5,
            max_targets: 1,
            spellbook: Standard,
            aspect,
        }
    }

    pub fn ancient(name: SpellName, base_max: u8, aspect: Option<Aspect>) -> Self {
        Self {
            name,
            base_max,
            attack_speed: 5,
            max_targets: 1,
            spellbook: Ancient,
            aspect,
        }
    }

    pub fn aoe(name: SpellName, base_max: u8, aspect: Option<Aspect>) -> Self {
        Self {
            name,
            base_max,
            attack_speed: 5,
            max_targets: 9,
            spellbook: Ancient,
            aspect,
        }
    }

    pub fn powered(name: SpellName, base_max: u8, attack_speed: u8) -> Self {
        Self {
            name,
            base_max,
            attack_speed,
            max_targets: 1,
            spellbook: Powered,
            aspect: None,
        }
    }
}
