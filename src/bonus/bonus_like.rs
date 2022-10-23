use super::secondary_bonus::{Agg, Def};

/// Trait which ensures a struct has gearlike/monster secondary bonuses.
pub trait BonusLike {
    fn get_agg(&self) -> Agg;
    fn get_def(&self) -> Def;
}
