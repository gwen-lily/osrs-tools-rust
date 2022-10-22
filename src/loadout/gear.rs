use crate::data::Slot;
use crate::stat::{
    aux_stat::{Agg, Def},
    basic_stat::Levels,
};

/** Gear struct which represents a single Gear item. Gear implements HasGearStats. Gear derives
 *  default behavior, which yields a slotless husk with no bonuses or requirements
 */
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Gear {
    pub name: String,
    /* Slot must be Some(Slot), but we choose Option to allow
    for Default implementation */
    pub slot: Option<Slot>,
    agg: Agg,
    def: Def,
    pry: u32,
    lvl_reqs: Levels,
}

/// Trait which is useful for implementing higher-level access to Gear fields.
pub trait HasGearStats {
    fn get_agg(&self) -> Agg;
    fn get_def(&self) -> Def;
    fn get_pry(&self) -> u32;
    fn get_lvl_reqs(&self) -> Levels;
}

/// Implementing HasGearStats for Gear basically provides copies / clones of private fields
impl HasGearStats for Gear {
    fn get_agg(&self) -> Agg {
        self.agg
    }
    fn get_def(&self) -> Def {
        self.def
    }
    fn get_pry(&self) -> u32 {
        self.pry
    }
    fn get_lvl_reqs(&self) -> Levels {
        self.lvl_reqs.clone()
    }
}

/// Implement new for gear which allows Gear to be constructed properly
impl Gear {
    pub fn new(name: String, slot: Slot, agg: Agg, def: Def, pry: u32, lvl_reqs: Levels) -> Self {
        Self {
            name,
            slot: Some(slot),
            agg,
            def,
            pry,
            lvl_reqs,
        }
    }
}
