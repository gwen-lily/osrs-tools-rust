mod equipment;
mod gear;
mod monster_bonus;

use crate::{
    data::{Skill, DT},
    levels::Levels,
};

pub use equipment::Equipment;
pub use gear::special_weapon::SpecialWeapon;
pub use gear::weapon::Weapon;
pub use gear::{Gear, GearMap, GearName, Slot};
pub use monster_bonus::MonsterBonus;

use std::collections::HashMap;

/// A BonusStats map matches (DT, Skill) keys to integer representations of modifiers
pub type BonusStats = HashMap<(DT, Skill), i32>;

/// Trait for basic bonus containers
pub trait BonusLike {
    fn get_bonus_stats(&self) -> &BonusStats;
}

/// Trait for higher level access to prayer and level_requirements.
pub trait GearLike {
    fn get_pry(&self) -> &u32;
    fn get_lvl_reqs(&self) -> &Levels;
}
