use crate::{
    boost::{basic_boost_map, BoostMap},
    data::Skill::{self, *},
};

use super::{DivinePotion, Potion};

struct RangingPotion {
    boosts: BoostMap,
}

impl Potion for RangingPotion {
    fn get_boosts(&self) -> &BoostMap {
        &self.boosts
    }
}

impl DivinePotion for RangingPotion {}

impl RangingPotion {
    pub fn new() -> Self {
        let skills: [Skill; 1] = [Ranged];
        let boosts: BoostMap = basic_boost_map(4, 0.10, true, &skills);
        Self { boosts }
    }
}
