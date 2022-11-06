use crate::{data::DT, modifiers::PlayerModifiers, OsrsError, Result};

/// SpecialWeapon struct which represents a Weapon item with more fields and implementation.
#[derive(Debug, PartialEq, Clone)]
pub struct SpecialWeaponInfo {
    pub special_arms: Option<Vec<f64>>,
    pub special_dms: Option<Vec<f64>>,
    pub special_defence_roll: Option<DT>,
}

impl SpecialWeaponInfo {
    pub fn new(
        special_arms: Option<Vec<f64>>,
        special_dms: Option<Vec<f64>>,
        special_defence_roll: Option<DT>,
    ) -> Self {
        Self {
            special_arms,
            special_dms,
            special_defence_roll,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub(crate) struct NormalSpecialWeapon {}

impl SpecialWeaponTrait for NormalSpecialWeapon {}

pub trait SpecialWeaponTrait {
    fn get_special_arms(&self, pmods: &PlayerModifiers) -> Option<Vec<f64>> {
        let spec_wpn_info: SpecialWeaponInfo = self.get_special_weapon_info(pmods).expect("foo");
        spec_wpn_info.special_arms
    }

    fn get_special_dms(&self, pmods: &PlayerModifiers) -> Option<Vec<f64>> {
        let spec_wpn_info: SpecialWeaponInfo = self.get_special_weapon_info(pmods).expect("bar");
        spec_wpn_info.special_dms
    }

    fn get_special_defence_roll(&self, pmods: &PlayerModifiers) -> Option<DT> {
        let spec_wpn_info: SpecialWeaponInfo = self.get_special_weapon_info(pmods).expect("baz");
        spec_wpn_info.special_defence_roll
    }

    fn get_special_weapon_info(&self, pmods: &PlayerModifiers) -> Result<SpecialWeaponInfo> {
        if let Some(wpn) = &pmods.player.equipment_info.equipment.weapon {
            if let Some(spec_wpn) = &wpn.special_weapon_info {
                Ok(spec_wpn.clone())
            } else {
                Err(OsrsError::SpecialWeapon)
            }
        } else {
            Err(OsrsError::SpecialWeapon)
        }
    }
}
