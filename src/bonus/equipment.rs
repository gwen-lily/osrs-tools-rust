mod equipment_info;
mod sets;

use std::collections::HashMap;

use crate::{
    bonus::{BonusLike, Gear, GearLike, Slot},
    levels::Levels,
};

use super::{BonusStats, GearName, Weapon};

pub use equipment_info::EquipmentInfo;
pub(crate) use sets::get_all_gear_sets;
pub use sets::{GearSetMap, SetName};

/// Equipment type is a HashMap linking Slot to Gear.

pub struct Equipment {
    pub equipment: EquipmentMap,
    pub weapon: Option<Weapon>,
    bonus_stats: BonusStats,
    lvl_reqs: Levels,
    pry: u32,
}

/// Type alias for HashMap<Slot, Gear>. The actual Gear item container.
pub type EquipmentMap = HashMap<Slot, Gear>;
/// Type alias for HashMap<Slot, GearName>. Used by developers to inquire about gear using a lookup
/// value
pub type EquipmentNameMap = HashMap<Slot, GearName>;

/// Implement BonusLike for Equipment, which is element-wise addition for its getters.
impl BonusLike for Equipment {
    fn get_bonus_stats(&self) -> &BonusStats {
        &self.bonus_stats
    }
}

impl GearLike for Equipment {
    /// Get the sum prayer bonus
    fn get_pry(&self) -> &u32 {
        &self.pry
    }

    fn get_lvl_reqs(&self) -> &Levels {
        &self.lvl_reqs
    }
}

impl Equipment {
    pub fn new(equipment: EquipmentMap, weapon: Option<Weapon>) -> Self {
        if let Some(_wpn) = equipment.get(&Slot::Weapon) {
            panic!("Weapon must be distinct, not part of an EquipmentMap")
        }

        Self {
            bonus_stats: Self::calc_bonus_stats(&equipment, &weapon),
            lvl_reqs: Self::calc_lvl_reqs(&equipment, &weapon),
            pry: Self::calc_pry(&equipment, &weapon),
            equipment,
            weapon,
        }
    }

    fn calc_bonus_stats(equipment: &EquipmentMap, weapon: &Option<Weapon>) -> BonusStats {
        let mut map = BonusStats::new();

        // Gear
        for gear in equipment.values() {
            for ((dt, skill), gear_val) in gear.get_bonus_stats().iter() {
                if let Some(existing_val) = map.get(&(*dt, *skill)) {
                    let new_val: i32 = existing_val + gear_val;
                    map.insert((*dt, *skill), new_val);
                } else {
                    map.insert((*dt, *skill), *gear_val);
                }
            }
        }

        // Weapon (optional)
        if let Some(wpn) = weapon {
            for ((dt, skill), gear_val) in wpn.get_bonus_stats().iter() {
                if let Some(existing_val) = map.get(&(*dt, *skill)) {
                    let new_val: i32 = existing_val + gear_val;
                    map.insert((*dt, *skill), new_val);
                } else {
                    map.insert((*dt, *skill), *gear_val);
                }
            }
        }

        map
    }

    /// Add the prayer bonus of all gear
    fn calc_pry(equipment: &EquipmentMap, weapon: &Option<Weapon>) -> u32 {
        let mut pry: u32 = 0;

        for gear in equipment.values() {
            pry += gear.get_pry();
        }

        if let Some(wpn) = weapon {
            pry += wpn.get_pry()
        }

        pry
    }

    /// Get the minimum required stats to equip ALL of the gear in Equipment.
    fn calc_lvl_reqs(equipment: &EquipmentMap, weapon: &Option<Weapon>) -> Levels {
        let mut levels: Levels = Levels::new();

        // Gear
        for gear in equipment.values() {
            for (skill, gear_req) in gear.get_lvl_reqs().iter() {
                match levels.get(skill) {
                    Some(cur_req) => {
                        if gear_req > cur_req {
                            levels.insert(*skill, *gear_req);
                        }
                    }
                    None => {
                        levels.insert(*skill, *gear_req);
                    }
                }
            }
        }

        // Weapon (optional)
        if let Some(wpn) = weapon {
            for (skill, gear_req) in wpn.get_lvl_reqs().iter() {
                match levels.get(skill) {
                    Some(cur_req) => {
                        if gear_req > cur_req {
                            levels.insert(*skill, *gear_req);
                        }
                    }
                    None => {
                        levels.insert(*skill, *gear_req);
                    }
                }
            }
        }

        levels
    }
}
