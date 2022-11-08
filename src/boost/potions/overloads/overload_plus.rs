use crate::boost::potions::{OverloadPotion, Potion};
use crate::boost::{basic_boost_map, BoostMap};
use crate::Skill::{self, *};

struct OverloadPlus {
    boosts: BoostMap,
}

impl Potion for OverloadPlus {
    fn get_boosts(&self) -> &BoostMap {
        &self.boosts
    }
}

impl OverloadPotion for OverloadPlus {}

impl OverloadPlus {
    pub fn new() -> Self {
        let skills: [Skill; 5] = [Attack, Strength, Defence, Ranged, Magic];
        let boosts: BoostMap = basic_boost_map(6, 0.16, true, &skills);
        Self { boosts }
    }
}
