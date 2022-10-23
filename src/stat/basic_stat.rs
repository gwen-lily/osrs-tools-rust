use crate::data::Skill::{self, Attack, Defence, Hitpoints, Magic, Ranged, Strength};
use std::collections::HashMap;
use strum::IntoEnumIterator;

/// Type alias for HashMap<Skill, i32>
pub type Levels = HashMap<Skill, i32>;

/// Trait which certifies that a HashMap has the necessary combat stats
pub trait HasCombatStats {
    fn combat_stats(&self) -> Levels;
}

impl HasCombatStats for Levels {
    fn combat_stats(&self) -> Levels {
        let combat_skills: [Skill; 6] = [Attack, Strength, Defence, Ranged, Magic, Hitpoints];

        let msg: &str = "&Levels does not contain a value for";
        let mut combat_stats = Levels::new();

        for skill in combat_skills.iter() {
            let value: &i32 = self.get(skill).expect(msg);
            combat_stats.insert(*skill, *value);
        }

        combat_stats
    }
}

pub fn get_fresh_levels() -> Levels {
    let mut map: Levels = Levels::new();

    for skill in Skill::iter() {
        map.insert(skill, 1);
    }

    map.insert(Skill::Hitpoints, 10);
    map
}

pub fn get_maxed_levels() -> Levels {
    let mut map: Levels = Levels::new();

    for skill in Skill::iter() {
        map.insert(skill, 99);
    }

    map
}
