use crate::boost::potions::{OverloadPotion, Potion};
use crate::boost::{basic_boost_map, BoostMap};
use crate::Skill::{self, *};

struct SmellingSalts {
    boosts: BoostMap,
}

impl Potion for SmellingSalts {
    fn get_boosts(&self) -> &BoostMap {
        &self.boosts
    }
}

impl OverloadPotion for SmellingSalts {
    fn get_duration(&self) -> u32 {
        800
    }

    fn get_damage(&self) -> u32 {
        0
    }

    fn get_run_energy_restore(&self) -> u32 {
        2500
    }
}

impl SmellingSalts {
    pub fn new() -> Self {
        let skills: [Skill; 5] = [Attack, Strength, Defence, Ranged, Magic];
        let boosts: BoostMap = basic_boost_map(11, 0.16, true, &skills);
        Self { boosts }
    }
}
