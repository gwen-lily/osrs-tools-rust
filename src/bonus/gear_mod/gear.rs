use crate::levels::Levels;
use strum_macros::EnumIter;

use super::{
    super::secondary_bonus::{Agg, Def},
    special_weapon::SpecialWeapon,
    weapon::Weapon,
};

/** Gear struct which represents a single Gear item. Gear implements HasGearStats. Gear derives
 *  default behavior, which yields a slotless husk with no bonuses or requirements
 */
#[derive(Debug, PartialEq, Eq)]
pub struct Gear {
    pub name: String,
    pub slot: Slot,
    pub(super) agg: Agg,
    pub(super) def: Def,
    pub(super) pry: u32,
    pub(super) lvl_reqs: Levels,
    weapon: Option<Weapon>,
    special_weapon: Option<SpecialWeapon>,
}

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Slot {
    Head,
    Cape,
    Neck,
    Ammunition,
    Weapon,
    Body,
    Shield,
    Legs,
    Hands,
    Feet,
    Ring,
}

/// Implement new for gear which allows Gear to be constructed properly
impl Gear {
    pub fn new(name: String, slot: Slot, agg: Agg, def: Def, pry: u32, lvl_reqs: Levels) -> Self {
        Self {
            name,
            slot,
            agg,
            def,
            pry,
            lvl_reqs,
            weapon: None,
            special_weapon: None,
        }
    }

    pub fn weapon(
        name: String,
        agg: Agg,
        def: Def,
        pry: u32,
        lvl_reqs: Levels,
        weapon: Weapon,
    ) -> Self {
        Self {
            name,
            slot: Slot::Weapon,
            agg,
            def,
            pry,
            lvl_reqs,
            weapon: Some(weapon),
            special_weapon: None,
        }
    }

    pub fn special_weapon(
        name: String,
        agg: Agg,
        def: Def,
        pry: u32,
        lvl_reqs: Levels,
        weapon: Weapon,
        special_weapon: SpecialWeapon,
    ) -> Self {
        Self {
            name,
            slot: Slot::Weapon,
            agg,
            def,
            pry,
            lvl_reqs,
            weapon: Some(weapon),
            special_weapon: Some(special_weapon),
        }
    }
}
