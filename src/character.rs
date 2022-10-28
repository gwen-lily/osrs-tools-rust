pub mod monster;
pub mod player;

use crate::levels::HasCombatStats;
pub use monster::Monster;
pub use player::Player;

pub trait CharCombat: HasCombatStats {}
