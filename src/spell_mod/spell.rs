use std::cmp::max;

/// structs

/// God spells are included as standard spells too
/// Charge behavior should be handled as part of modifiers
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct StandardSpell {
    base_max: u8,
    attack_speed: u8,
}

pub struct AncientSpell {
    base_max: u8,
    #[allow(dead_code)]
    attack_speed: u8,
    #[allow(dead_code)]
    max_targets_hit: u8,
}

pub struct PoweredSpell {
    base_max: u8,
    #[allow(dead_code)]
    attack_speed: u8,
    visible_level: i32,
}

pub struct TumekensPoweredSpell {
    base_max: u8,
    #[allow(dead_code)]
    attack_speed: u8,
    visible_level: i32,
}

/// traits

pub trait SpellTrait {
    fn get_base_max(&self) -> u8;
    fn max_hit(&self) -> u8 {
        self.get_base_max()
    }
}

pub trait AoeSpell {
    fn get_max_targets_hit(&self) -> u8;
}

pub trait PoweredSpellTrait: SpellTrait {
    fn get_visible_level(&self) -> i32;

    fn max_hit(&self) -> u8 {
        let minimum_level: i32 = 75;
        let visible_level: i32 = self.get_visible_level();
        let adj_visible_level: i32 = max(minimum_level, visible_level);
        let level_diff: i32 = adj_visible_level - visible_level;
        let base_max: u8 = self.get_base_max();
        let max: u8 = base_max + (level_diff / 3) as u8;
        max
    }
}

pub trait TumekensPoweredSpellTrait: PoweredSpellTrait {
    fn max_hit(&self) -> u8 {
        let visible_level: i32 = self.get_visible_level();
        let base_max: u8 = self.get_base_max();
        let max: u8 = base_max + (visible_level / 3) as u8;
        max
    }
}

/// Trait implementation

impl SpellTrait for StandardSpell {
    fn get_base_max(&self) -> u8 {
        self.base_max
    }
}

impl SpellTrait for AncientSpell {
    fn get_base_max(&self) -> u8 {
        self.base_max
    }
}

impl SpellTrait for PoweredSpell {
    fn get_base_max(&self) -> u8 {
        self.base_max
    }
}

impl SpellTrait for TumekensPoweredSpell {
    fn get_base_max(&self) -> u8 {
        self.base_max
    }
}

impl PoweredSpellTrait for PoweredSpell {
    fn get_visible_level(&self) -> i32 {
        self.visible_level
    }
}

impl PoweredSpellTrait for TumekensPoweredSpell {
    fn get_visible_level(&self) -> i32 {
        self.visible_level
    }
}

impl TumekensPoweredSpellTrait for TumekensPoweredSpell {}

/// Default implementation

impl Default for StandardSpell {
    fn default() -> Self {
        Self {
            base_max: 0,
            attack_speed: 5,
        }
    }
}

impl Default for AncientSpell {
    fn default() -> Self {
        Self {
            base_max: 0,
            attack_speed: 5,
            max_targets_hit: 1,
        }
    }
}

impl Default for PoweredSpell {
    fn default() -> Self {
        Self {
            base_max: 0,
            attack_speed: 4,
            visible_level: 1,
        }
    }
}

impl Default for TumekensPoweredSpell {
    fn default() -> Self {
        Self {
            base_max: 1u8,
            attack_speed: 5u8,
            visible_level: 1,
        }
    }
}

/// Implementation

impl StandardSpell {
    pub fn new(base_max: u8) -> Self {
        Self {
            base_max,
            ..Default::default()
        }
    }
}

impl AncientSpell {
    pub fn new(base_max: u8, max_targets_hit: u8) -> Self {
        Self {
            base_max,
            max_targets_hit,
            ..Default::default()
        }
    }
}

impl PoweredSpell {
    pub fn new(base_max: u8) -> Self {
        Self {
            base_max,
            ..Default::default()
        }
    }
}

impl TumekensPoweredSpell {
    pub fn new() -> Self {
        Self::default()
    }
}
