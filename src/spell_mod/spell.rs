use std::cmp::max;

/// A StandardSpell is on the Standard spellbook and has limited properties.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct StandardSpell {
    base_max: u8,
    attack_speed: u8,
}

/// An AncientSpell is on the Ancient spellbok and has a few more properties.
pub struct AncientSpell {
    base_max: u8,
    #[allow(dead_code)]
    attack_speed: u8,
    #[allow(dead_code)]
    max_targets_hit: u8,
}

/// A PoweredSpell is one used by a PoweredStaff.
pub struct PoweredSpell {
    base_max: u8,
    #[allow(dead_code)]
    attack_speed: u8,
    visible_level: i32,
}

/// A TumekensPoweredSpell is used by TumekensShadow exclusively.
pub struct TumekensPoweredSpell {
    base_max: u8,
    #[allow(dead_code)]
    attack_speed: u8,
    visible_level: i32,
}

/// Basic spell behavior
pub trait SpellLike {
    fn get_base_max(&self) -> u8;
    fn get_attack_speed(&self) -> u8;
    fn max_hit(&self) -> u8 {
        self.get_base_max()
    }
}

/// Basic AoE spell behavior.
pub trait AoeSpellLike {
    fn get_max_targets_hit(&self) -> u8;
}

/// Trait for determining the max hit of PoweredSpells, which depends on visible level.
pub trait PoweredSpellLike: SpellLike {
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

/// Trait for determining the max hit of TumekensPoweredSpells, which depends on visible level.
pub trait TumekensPoweredSpellLike: PoweredSpellLike {
    fn max_hit(&self) -> u8 {
        let visible_level: i32 = self.get_visible_level();
        let base_max: u8 = self.get_base_max();
        let max: u8 = base_max + (visible_level / 3) as u8;
        max
    }
}

impl SpellLike for StandardSpell {
    fn get_base_max(&self) -> u8 {
        self.base_max
    }
    fn get_attack_speed(&self) -> u8 {
        self.attack_speed
    }
}

impl SpellLike for AncientSpell {
    fn get_base_max(&self) -> u8 {
        self.base_max
    }
    fn get_attack_speed(&self) -> u8 {
        self.attack_speed
    }
}

impl AoeSpellLike for AncientSpell {
    fn get_max_targets_hit(&self) -> u8 {
        self.max_targets_hit
    }
}

impl SpellLike for PoweredSpell {
    fn get_base_max(&self) -> u8 {
        self.base_max
    }
    fn get_attack_speed(&self) -> u8 {
        self.attack_speed
    }
}

impl SpellLike for TumekensPoweredSpell {
    fn get_base_max(&self) -> u8 {
        self.base_max
    }
    fn get_attack_speed(&self) -> u8 {
        self.attack_speed
    }
}

impl PoweredSpellLike for PoweredSpell {
    fn get_visible_level(&self) -> i32 {
        self.visible_level
    }
}

impl PoweredSpellLike for TumekensPoweredSpell {
    fn get_visible_level(&self) -> i32 {
        self.visible_level
    }
}

impl TumekensPoweredSpellLike for TumekensPoweredSpell {}

/// Default StandardSpell has an attack speed of 5 ticks. Max hit must be specified.
impl Default for StandardSpell {
    fn default() -> Self {
        Self {
            base_max: 0,
            attack_speed: 5,
        }
    }
}

/** Default AncientSpell has an attack speed of 5 ticks. base_max must be specified,
 *  and optionally max_targets_hit.
 */
impl Default for AncientSpell {
    fn default() -> Self {
        Self {
            base_max: 0,
            attack_speed: 5,
            max_targets_hit: 1,
        }
    }
}

/// Default PoweredSpell assumes a visible_level of 1 and an attack_speed of 4.
impl Default for PoweredSpell {
    fn default() -> Self {
        Self {
            base_max: 0,
            attack_speed: 4,
            visible_level: 1,
        }
    }
}

/** Default TumekensPoweredSpell assumes a visible level of 1, a base_max of 1, and a 5 tick
 *  attack speed.
 */
impl Default for TumekensPoweredSpell {
    fn default() -> Self {
        Self {
            base_max: 1u8,
            attack_speed: 5u8,
            visible_level: 1,
        }
    }
}

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

    pub fn single_target_spell(base_max: u8) -> Self {
        Self {
            base_max,
            max_targets_hit: 1,
            ..Default::default()
        }
    }

    pub fn aoe_spell(base_max: u8) -> Self {
        Self {
            base_max,
            max_targets_hit: 9,
            ..Default::default()
        }
    }
}

impl PoweredSpell {
    pub fn new(base_max: u8, visible_level: i32) -> Self {
        Self {
            base_max,
            visible_level,
            ..Default::default()
        }
    }
}

impl TumekensPoweredSpell {
    pub fn new(visible_level: i32) -> Self {
        Self {
            visible_level,
            ..Default::default()
        }
    }
}
