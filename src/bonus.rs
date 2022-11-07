mod equipment;
mod gear;
mod monster_bonus;

use crate::{levels::Levels, CombatMap};

pub(crate) use equipment::{get_all_gear_sets, GearSetMap};
pub use equipment::{Equipment, EquipmentInfo, EquipmentMap, EquipmentNameMap, SetName};
pub use gear::weapon::weapon_trait::WeaponTrait;
pub use gear::weapon::Weapon;
pub use gear::{Gear, GearMap, GearName, Slot};
pub use monster_bonus::MonsterBonus;

/// A BonusStats map matches (DT, Skill) keys to integer representations of modifiers
pub type BonusStats = CombatMap<i32>;

/// Trait for basic bonus containers
pub trait BonusLike {
    fn get_bonus_stats(&self) -> &BonusStats;
}

/// Trait for higher level access to prayer and level_requirements.
pub trait GearLike {
    fn get_pry(&self) -> &u32;
    fn get_lvl_reqs(&self) -> &Levels;
}
