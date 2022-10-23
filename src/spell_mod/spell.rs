/// A StandardSpell is on the Standard spellbook and has limited properties.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Spell {
    pub base_max: u8,
    pub attack_speed: u8,
    pub max_targets: u8,
    pub aspect: Option<Aspect>,
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

impl Spell {
    pub fn standard(base_max: u8, aspect: Option<Aspect>) -> Self {
        Self {
            base_max,
            attack_speed: 5,
            max_targets: 1,
            aspect,
        }
    }

    pub fn aoe(base_max: u8, aspect: Option<Aspect>) -> Self {
        Self {
            base_max,
            attack_speed: 5,
            max_targets: 9,
            aspect,
        }
    }

    pub fn powered(base_max: u8, attack_speed: u8) -> Self {
        Self {
            base_max,
            attack_speed,
            max_targets: 1,
            aspect: None,
        }
    }
}
