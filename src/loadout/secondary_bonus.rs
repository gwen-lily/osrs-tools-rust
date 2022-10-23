use std::ops::Add;

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

/// Add field-wise for Agg
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

/// Add field-wise for Def
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

/// Add field-wise for MeleeAgg
impl Add for MeleeAgg {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            attack: self.attack + rhs.attack,
            strength: self.strength + rhs.strength,
        }
    }
}

/// Add field-wise for AttackMeleeAgg
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

/// Add field-wise for RangedAgg
impl Add for RangedAgg {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            attack: self.attack + rhs.attack,
            strength: self.strength + rhs.strength,
        }
    }
}

/// Add field-wise for MagicAgg
impl Add for MagicAgg {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            attack: self.attack + rhs.attack,
            strength: self.strength + rhs.strength,
        }
    }
}

/// Add field-wise for MeleeDef
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
