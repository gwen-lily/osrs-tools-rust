use std::collections::HashMap;

use crate::{
    data::Slot,
    loadout::gear::{Gear, HasGearStats},
    stat::{
        aux_stat::{Agg, Def},
        basic_stat::Levels,
    },
};

/// Equipment type is a HashMap linking Slot to Gear.
pub type Equipment = HashMap<Slot, Gear>;

/** Implement HasGearStats for Equipment, which aggregates individual gear fields
 *  into a returned combination object. For most field getters, this is simple addition.
 */
impl HasGearStats for Equipment {
    /// Get the sum of all aggressive stats
    fn get_agg(&self) -> Agg {
        let mut agg: Agg = Agg::default();

        for gear in self.values() {
            agg = agg + gear.get_agg();
        }

        agg
    }

    /// Get the sum of all defensive stats
    fn get_def(&self) -> Def {
        let mut def: Def = Def::default();

        for gear in self.values() {
            def = def + gear.get_def();
        }

        def
    }

    /// Get the sum prayer bonus
    fn get_pry(&self) -> u32 {
        let mut pry: u32 = 0;

        for gear in self.values() {
            pry += gear.get_pry();
        }

        pry
    }

    /// Get the minimum required stats to equip ALL of the gear in Equipment.
    fn get_lvl_reqs(&self) -> Levels {
        let mut levels: Levels = Levels::new();

        for gear in self.values() {
            for (skill, gear_req) in gear.get_lvl_reqs().iter() {
                match levels.get(skill) {
                    Some(cur_req) => {
                        if gear_req > cur_req {
                            levels.remove(skill);
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
