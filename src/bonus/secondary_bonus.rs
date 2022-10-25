use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Agg {
    // aggressive bonuses
    pub melee: MeleeAgg,
    pub ranged: RangedAgg,
    pub magic: MagicAgg,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct MeleeAgg {
    pub attack: AttackMeleeAgg,
    pub strength: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct AttackMeleeAgg {
    pub stab: i32,
    pub slash: i32,
    pub crush: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct RangedAgg {
    pub attack: i32,
    pub strength: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct MagicAgg {
    pub attack: i32,
    pub strength: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct Def {
    pub melee: MeleeDef,
    pub ranged: i32,
    pub magic: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct MeleeDef {
    pub stab: i32,
    pub slash: i32,
    pub crush: i32,
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
