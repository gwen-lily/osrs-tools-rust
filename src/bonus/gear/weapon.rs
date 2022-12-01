pub(crate) mod chinchompa_weapon_trait;
pub(crate) mod crossbow_weapon_trait;
pub(crate) mod weapon_trait;

use crate::{
    bonus::{BonusLike, BonusStats, GearLike},
    levels::Levels,
    style::{Styles, StylesCategory},
    STYLES_MAP,
};

use weapon_trait::WeaponTrait;

use self::weapon_trait::NormalWeaponTrait;

use super::special_weapon::{SpecialWeaponInfo, SpecialWeaponTrait};
use super::GearInfo;

/// Weapon struct which represents a Gear item with more fields and implementation.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct WeaponInfo {
    pub styles: &'static Styles,
    pub base_attack_speed: u8,
    pub two_handed: bool,
    pub base_attack_range: u8,
}

impl WeaponInfo {
    pub fn new(
        styles: &'static Styles,
        base_attack_speed: u8,
        two_handed: bool,
        base_attack_range: u8,
    ) -> Self {
        Self {
            styles,
            base_attack_speed,
            two_handed,
            base_attack_range,
        }
    }
}

impl Default for WeaponInfo {
    fn default() -> Self {
        Self {
            styles: STYLES_MAP.get(&StylesCategory::UnarmedWeapons).unwrap(),
            base_attack_speed: 4,
            two_handed: false,
            base_attack_range: 1,
        }
    }
}

pub struct Weapon {
    pub gear_info: GearInfo,
    pub weapon_info: WeaponInfo,
    pub unique_fns: Box<dyn WeaponTrait>,
    pub special_weapon_info: Option<SpecialWeaponInfo>,
    pub unique_spec_fns: Option<Box<dyn SpecialWeaponTrait>>,
}

impl BonusLike for Weapon {
    fn get_bonus_stats(&self) -> &BonusStats {
        &self.gear_info.bonus_stats
    }
}

impl GearLike for Weapon {
    fn get_pry(&self) -> &u32 {
        &self.gear_info.pry
    }
    fn get_lvl_reqs(&self) -> &Levels {
        &self.gear_info.lvl_reqs
    }
}

impl WeaponTrait for Weapon {}

impl Weapon {
    pub fn new(
        gear_info: GearInfo,
        weapon_info: WeaponInfo,
        special_weapon_info: Option<SpecialWeaponInfo>,
        unique_fns: Box<dyn WeaponTrait>,
        unique_spec_fns: Option<Box<dyn SpecialWeaponTrait>>,
    ) -> Self {
        Self {
            gear_info,
            weapon_info,
            unique_fns,
            special_weapon_info,
            unique_spec_fns,
        }
    }

    fn hands() -> Self {
        Self {
            gear_info: GearInfo::default(),
            weapon_info: WeaponInfo::default(),
            special_weapon_info: None,
            unique_fns: Box::new(NormalWeaponTrait::default()),
            unique_spec_fns: None,
        }
    }
}

impl Default for Weapon {
    fn default() -> Self {
        Weapon::hands()
    }
}
