use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct StyleStats {
    /// Style associated values
    pub melee_attack: u32,
    pub melee_strength: u32,
    pub ranged_attack: u32,
    pub ranged_strength: u32,
    pub magic_attack: u32,
    pub defence: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Agg {
    // aggressive bonuses
    melee: MeleeAgg,
    ranged: RangedAgg,
    magic: MagicAgg,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct MeleeAgg {
    attack: AttackMeleeAgg,
    strength: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct AttackMeleeAgg {
    stab: i32,
    slash: i32,
    crush: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct RangedAgg {
    attack: i32,
    strength: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct MagicAgg {
    attack: i32,
    strength: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct Def {
    melee: MeleeDef,
    ranged: i32,
    magic: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct MeleeDef {
    stab: i32,
    slash: i32,
    crush: i32,
}

/// Add trait

impl Add for StyleStats {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            melee_attack: self.melee_attack + rhs.melee_attack,
            melee_strength: self.melee_strength + rhs.melee_strength,
            ranged_attack: self.ranged_attack + rhs.ranged_attack,
            ranged_strength: self.ranged_strength + rhs.ranged_strength,
            magic_attack: self.magic_attack + rhs.magic_attack,
            defence: self.defence + rhs.defence,
        }
    }
}

/// Agg & Def implementation is code duplication?
///
impl Add for Agg {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            melee: self.melee + rhs.melee,
            ranged: self.ranged + rhs.ranged,
            magic: self.magic + rhs.magic,
        }
    }
}

impl Add for Def {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            melee: self.melee + rhs.melee,
            ranged: self.ranged + rhs.ranged,
            magic: self.magic + rhs.magic,
        }
    }
}

impl Add for MeleeAgg {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            attack: self.attack + rhs.attack,
            strength: self.strength + rhs.strength,
        }
    }
}

impl Add for AttackMeleeAgg {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            stab: self.stab + rhs.stab,
            slash: self.slash + rhs.slash,
            crush: self.crush + rhs.crush,
        }
    }
}

impl Add for RangedAgg {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            attack: self.attack + rhs.attack,
            strength: self.strength + rhs.strength,
        }
    }
}

impl Add for MagicAgg {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            attack: self.attack + rhs.attack,
            strength: self.strength + rhs.strength,
        }
    }
}

impl Add for MeleeDef {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            stab: self.stab + rhs.stab,
            slash: self.slash + rhs.slash,
            crush: self.crush + rhs.crush,
        }
    }
}

/// Default implementation

impl Default for Agg {
    fn default() -> Self {
        Self {
            melee: MeleeAgg::default(),
            ranged: RangedAgg::default(),
            magic: MagicAgg::default(),
        }
    }
}
