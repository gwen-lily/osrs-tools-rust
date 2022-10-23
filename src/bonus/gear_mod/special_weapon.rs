use crate::{data::DT, levels::Levels, style_mod::style::Styles};

use super::{
    super::{
        bonus_like::BonusLike,
        secondary_bonus::{Agg, Def},
    },
    gear_bonus::GearLike,
    weapon::WeaponLike,
};

/// SpecialWeapon struct which represents a Weapon item with more fields and implementation.
#[derive(Debug, PartialEq, Clone)]
pub struct SpecialWeapon {
    pub name: String,
    pub(super) agg: Agg,
    pub(super) def: Def,
    pub(super) pry: u32,
    pub(super) lvl_reqs: Levels,
    styles: Styles,
    base_attack_speed: u8,
    two_handed: bool,
    special_arms: Option<Vec<f64>>,
    special_dms: Option<Vec<f64>>,
    special_defence_roll: Option<DT>,
}

/// SpecialWeaponlike trait covers behavior of Weapon, SpecialWeapon, etc...
pub trait SpecialWeaponLike {
    fn get_special_arms(&self) -> &Option<Vec<f64>>;
    fn get_special_dms(&self) -> &Option<Vec<f64>>;
    fn get_special_defence_roll(&self) -> &Option<DT>;
}

impl BonusLike for SpecialWeapon {
    fn get_agg(&self) -> Agg {
        self.agg
    }
    fn get_def(&self) -> Def {
        self.def
    }
}

impl GearLike for SpecialWeapon {
    fn get_pry(&self) -> u32 {
        self.pry
    }

    fn get_lvl_reqs(&self) -> Levels {
        self.lvl_reqs.clone()
    }
}

impl WeaponLike for SpecialWeapon {
    fn get_styles(&self) -> &Styles {
        &self.styles
    }

    fn get_base_attack_speed(&self) -> &u8 {
        &self.base_attack_speed
    }

    fn is_two_handed(&self) -> &bool {
        &self.two_handed
    }
}

impl SpecialWeaponLike for SpecialWeapon {
    fn get_special_arms(&self) -> &Option<Vec<f64>> {
        &self.special_arms
    }

    fn get_special_dms(&self) -> &Option<Vec<f64>> {
        &self.special_dms
    }

    fn get_special_defence_roll(&self) -> &Option<DT> {
        &self.special_defence_roll
    }
}
