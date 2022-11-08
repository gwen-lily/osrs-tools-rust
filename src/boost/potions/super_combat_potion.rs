use crate::{
    boost::{basic_boost_map, BoostMap},
    data::Skill::{self, *},
};

use super::{DivinePotion, Potion};

struct SuperCombatPotion {
    boosts: BoostMap,
}

impl Potion for SuperCombatPotion {
    fn get_boosts(&self) -> &BoostMap {
        &self.boosts
    }
}

impl DivinePotion for SuperCombatPotion {}

impl SuperCombatPotion {
    pub fn new() -> Self {
        let skills: [Skill; 3] = [Attack, Strength, Defence];
        let boosts: BoostMap = basic_boost_map(5, 0.15, true, &skills);
        Self { boosts }
    }
}
