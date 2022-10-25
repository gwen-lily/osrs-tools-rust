pub mod monster;
pub mod player;

use crate::levels::HasCombatStats;

pub trait CharCombat: HasCombatStats {}
