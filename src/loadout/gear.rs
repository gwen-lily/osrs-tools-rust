use crate::data::Slot;
use crate::levels::Levels;

use super::secondary_bonus::{Agg, Def};

/** Gear struct which represents a single Gear item. Gear implements HasGearStats. Gear derives
 *  default behavior, which yields a slotless husk with no bonuses or requirements
 */
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Gear {
    pub name: String,
    /* Slot must be Some(Slot), but we choose Option to allow
    for Default implementation */
    pub slot: Option<Slot>,
    pub(super) agg: Agg,
    pub(super) def: Def,
    pub(super) pry: u32,
    pub(super) lvl_reqs: Levels,
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
