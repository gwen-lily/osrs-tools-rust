use super::{
    gear::Gear,
    has_bonus::HasBonus,
    secondary_bonus::{Agg, Def},
};

use crate::levels::Levels;

/// Trait for higher level access to prayer and level_requirements.
pub trait HasGearStats: HasBonus {
    fn get_pry(&self) -> u32;
    fn get_lvl_reqs(&self) -> Levels;
}

impl HasBonus for Gear {
    fn get_agg(&self) -> Agg {
        self.agg
    }
    fn get_def(&self) -> Def {
        self.def
    }
}

/// Implementing HasGearStats for Gear basically provides copies / clones of private fields
impl HasGearStats for Gear {
    fn get_pry(&self) -> u32 {
        self.pry
    }
    fn get_lvl_reqs(&self) -> Levels {
        self.lvl_reqs.clone()
    }
}
