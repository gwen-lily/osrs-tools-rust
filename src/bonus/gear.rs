// modules
pub mod all_gear;
pub(crate) mod special_weapon;
pub(crate) mod weapon;

// imports
use crate::bonus::GearLike;
use crate::levels::Levels;
use strum_macros::EnumIter;

// higher level access to nested module
pub use all_gear::{GearMap, GearName};
pub use weapon::Weapon;

use super::{BonusLike, BonusStats};

/** GearInfo struct which represents a single GearInfo item. GearInfo implements HasGearInfoStats. GearInfo derives
 *  default behavior, which yields a slotless husk with no bonuses or requirements
 */
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GearInfo {
    pub name: GearName,
    pub slot: Slot,
    bonus_stats: BonusStats,
    pry: u32,
    lvl_reqs: Levels,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Gear {
    pub gear_info: GearInfo,
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
    fn get_bonus_stats(&self) -> &BonusStats {
        &self.gear_info.bonus_stats
    }
}

impl GearLike for Gear {
    fn get_pry(&self) -> &u32 {
        &self.gear_info.pry
    }
    fn get_lvl_reqs(&self) -> &Levels {
        &self.gear_info.lvl_reqs
    }
}

impl GearInfo {
    pub fn new(
        name: GearName,
        slot: Slot,
        bonus_stats: BonusStats,
        pry: u32,
        lvl_reqs: Levels,
    ) -> Self {
        Self {
            name,
            slot,
            bonus_stats,
            pry,
            lvl_reqs,
        }
    }

    fn hands() -> Self {
        Self {
            name: GearName::Hands,
            slot: Slot::Weapon,
            bonus_stats: BonusStats::new(),
            pry: 0,
            lvl_reqs: Levels::new(),
        }
    }
}

// hands default gear
impl Default for GearInfo {
    fn default() -> Self {
        GearInfo::hands()
    }
}
