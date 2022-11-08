use crate::{
    boost::{basic_boost_map, BoostMap},
    data::Skill::{self, *},
};

use super::{DivinePotion, Potion};

struct ImbuedHeartPotion {
    boosts: BoostMap,
}

impl Potion for ImbuedHeartPotion {
    fn get_boosts(&self) -> &BoostMap {
        &self.boosts
    }
}

impl DivinePotion for ImbuedHeartPotion {}

impl ImbuedHeartPotion {
    pub fn new() -> Self {
        let skills: [Skill; 1] = [Ranged];
        let boosts: BoostMap = basic_boost_map(4, 0.10, true, &skills);
        Self { boosts }
    }
}
