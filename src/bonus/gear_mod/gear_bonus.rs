use super::super::{
    bonus_like::BonusLike,
    secondary_bonus::{Agg, Def},
};
use super::gear::Gear;

use crate::levels::Levels;

/// Trait for higher level access to prayer and level_requirements.
pub trait GearLike: BonusLike {
    fn get_pry(&self) -> u32;
    fn get_lvl_reqs(&self) -> Levels;
}

impl BonusLike for Gear {
    fn get_agg(&self) -> Agg {
        self.agg
    }
    fn get_def(&self) -> Def {
        self.def
    }
}

/// Implementing GearLike for Gear basically provides copies / clones of private fields
impl GearLike for Gear {
    fn get_pry(&self) -> u32 {
        self.pry
    }
    fn get_lvl_reqs(&self) -> Levels {
        self.lvl_reqs.clone()
    }
}
