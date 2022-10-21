use crate::data::Slot;
use crate::stat::{
    aux_stat::{Agg, Def},
    basic_stat::Levels,
};

/// Structs

#[derive(Debug, PartialEq, Eq)]
pub struct Gear {
    pub name: String,
    pub slot: Option<Slot>,
    agg: Agg,
    def: Def,
    pry: u32,
    lvl_reqs: Levels,
}

/// Traits

pub trait HasGearStats {
    fn get_agg(&self) -> Agg;
    fn get_def(&self) -> Def;
    fn get_pry(&self) -> u32;
    fn get_lvl_reqs(&self) -> Levels;
}

/// Trait implementation

impl HasGearStats for Gear {
    fn get_agg(&self) -> Agg {
        self.agg.clone()
    }
    fn get_def(&self) -> Def {
        self.def.clone()
    }
    fn get_pry(&self) -> u32 {
        self.pry
    }
    fn get_lvl_reqs(&self) -> Levels {
        self.lvl_reqs.clone()
    }
}

/// Default implementation

impl Default for Gear {
    fn default() -> Self {
        Self {
            name: Default::default(),
            slot: None, // this should panic
            agg: Default::default(),
            def: Default::default(),
            pry: 0,
            lvl_reqs: Default::default(),
        }
    }
}
