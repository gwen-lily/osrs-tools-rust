use crate::data::Skill::{self, Attack, Defence, Hitpoints, Magic, Ranged, Strength};
use std::collections::HashMap;

/// type alias and related behavior
///

pub type Levels = HashMap<Skill, i32>;
pub type CombatStats = HashMap<Skill, i32>;

/// Trait

trait HasCombatStats {
    fn combat_stats(&self) -> CombatStats;
}

/// Trait implementation

impl HasCombatStats for Levels {
    fn combat_stats(&self) -> CombatStats {
        let combat_skills: [Skill; 6] = [Attack, Strength, Defence, Ranged, Magic, Hitpoints];

        let msg: &str = "&Levels does not contain a value for";
        let mut combat_stats = CombatStats::new();

        for skill in combat_skills.iter() {
            let value: &i32 = &self.get(skill).expect(msg);
            combat_stats.insert(*skill, *value);
        }

        combat_stats
    }
}

// impl PlayerLevels {
//     /// Helper functions for creating PlayerLevels objects.
//
//     fn fresh() -> Self {
//         Self {
//             attack: Some(1),
//             strength: Some(1),
//             defence: Some(1),
//             ranged: Some(1),
//             prayer: Some(1),
//             magic: Some(1),
//             runecraft: Some(1),
//             construction: Some(1),
//             hitpoints: Some(10), // 10, not 1
//             agility: Some(1),
//             herblore: Some(1),
//             thieving: Some(1),
//             crafting: Some(1),
//             fletching: Some(1),
//             slayer: Some(1),
//             hunter: Some(1),
//             mining: Some(1),
//             smithing: Some(1),
//             fishing: Some(1),
//             cooking: Some(1),
//             firemaking: Some(1),
//             woodcutting: Some(1),
//             farming: Some(1),
//         }
//     }
//     fn maxed() -> Self {
//         Self {
//             attack: Some(99),
//             strength: Some(99),
//             defence: Some(99),
//             ranged: Some(99),
//             prayer: Some(99),
//             magic: Some(99),
//             runecraft: Some(99),
//             construction: Some(99),
//             hitpoints: Some(99),
//             agility: Some(99),
//             herblore: Some(99),
//             thieving: Some(99),
//             crafting: Some(99),
//             fletching: Some(99),
//             slayer: Some(99),
//             hunter: Some(99),
//             mining: Some(99),
//             smithing: Some(99),
//             fishing: Some(99),
//             cooking: Some(99),
//             firemaking: Some(99),
//             woodcutting: Some(99),
//             farming: Some(99),
//         }
//     }
// }
//
// /// Default implementation
//
// impl Default for PlayerLevels {
//     fn default() -> Self {
//         Self {
//             attack: None,
//             strength: None,
//             defence: None,
//             ranged: None,
//             prayer: None,
//             magic: None,
//             runecraft: None,
//             construction: None,
//             hitpoints: None,
//             agility: None,
//             herblore: None,
//             thieving: None,
//             crafting: None,
//             fletching: None,
//             slayer: None,
//             hunter: None,
//             mining: None,
//             smithing: None,
//             fishing: None,
//             cooking: None,
//             firemaking: None,
//             woodcutting: None,
//             farming: None,
//         }
//     }
// }
//
// impl Default for MonsterLevels {
//     fn default() -> Self {
//         Self {
//             attack: None,
//             strength: None,
//             defence: None,
//             ranged: None,
//             magic: None,
//             hitpoints: None,
//         }
//     }
// }
