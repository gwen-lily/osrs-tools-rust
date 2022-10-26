// modules
pub mod all_gear;
pub(crate) mod special_weapon;
pub(crate) mod weapon;

// imports
use crate::bonus::{BonusLike, GearLike};
use crate::levels::Levels;
use strum_macros::EnumIter;

use super::{Agg, Def};

// higher level access to nested module
pub use all_gear::{GearMap, GearName};
pub use special_weapon::SpecialWeapon;
pub use weapon::Weapon;

/** Gear struct which represents a single Gear item. Gear implements HasGearStats. Gear derives
 *  default behavior, which yields a slotless husk with no bonuses or requirements
 */
#[derive(Debug, PartialEq, Eq)]
pub struct Gear {
    pub name: GearName,
    pub slot: Slot,
    pub(super) agg: Agg,
    pub(super) def: Def,
    pub(super) pry: u32,
    pub(super) lvl_reqs: Levels,
    pub weapon: Option<Weapon>,
    pub special_weapon: Option<SpecialWeapon>,
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

impl BonusLike for Gear {
    fn get_agg(&self) -> Agg {
        self.agg
    }
    fn get_def(&self) -> Def {
        self.def
    }
}

impl GearLike for Gear {
    fn get_pry(&self) -> u32 {
        self.pry
    }
    fn get_lvl_reqs(&self) -> Levels {
        self.lvl_reqs.clone()
    }
}

impl Gear {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: GearName,
        slot: Slot,
        agg: Agg,
        def: Def,
        pry: u32,
        lvl_reqs: Levels,
        weapon: Option<Weapon>,
        special_weapon: Option<SpecialWeapon>,
    ) -> Self {
        Self {
            name,
            slot,
            agg,
            def,
            pry,
            lvl_reqs,
            weapon,
            special_weapon,
        }
    }

    /// Yep, geare. Clippy yells at me otherwise.
    pub fn geare(
        name: GearName,
        slot: Slot,
        agg: Agg,
        def: Def,
        pry: u32,
        lvl_reqs: Levels,
    ) -> Self {
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
        name: GearName,
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
        name: GearName,
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
