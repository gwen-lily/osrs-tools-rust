pub mod monster;
pub mod player;

use crate::levels::HasCombatStats;
pub use monster::{Monster, MonsterAttribute, MonsterLocation, Slayer, VampyreTier};
pub use player::Player;

pub trait CharCombat: HasCombatStats {}
