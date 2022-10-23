use crate::{
    levels::Levels,
    style_mod::{style::Styles, styles_map::StylesMap},
};

use super::{
    super::{
        bonus_like::BonusLike,
        secondary_bonus::{Agg, Def},
    },
    gear_bonus::GearLike,
};

/// Weapon struct which represents a Gear item with more fields and implementation.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Weapon {
    pub name: String,
    pub(super) agg: Agg,
    pub(super) def: Def,
    pub(super) pry: u32,
    pub(super) lvl_reqs: Levels,
    styles: Styles,
    base_attack_speed: u8,
    two_handed: bool,
}

/// Weaponlike trait covers additional Weapon behavior
pub trait WeaponLike: GearLike {
    fn get_styles(&self) -> &Styles;
    fn get_base_attack_speed(&self) -> &u8;
    fn is_two_handed(&self) -> &bool;
}

impl BonusLike for Weapon {
    fn get_agg(&self) -> Agg {
        self.agg
    }
    fn get_def(&self) -> Def {
        self.def
    }
}

impl GearLike for Weapon {
    fn get_pry(&self) -> u32 {
        self.pry
    }

    fn get_lvl_reqs(&self) -> Levels {
        self.lvl_reqs.clone()
    }
}

impl WeaponLike for Weapon {
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

impl Weapon {
    pub fn new(
        name: String,
        agg: Agg,
        def: Def,
        pry: u32,
        lvl_reqs: Levels,
        styles: Styles,
        base_attack_speed: u8,
        two_handed: bool,
    ) -> Self {
        Self {
            name,
            agg,
            def,
            pry,
            lvl_reqs,
            styles,
            base_attack_speed,
            two_handed,
        }
    }

    // pub fn hands() -> Self {
    //     Self { }
    // }
}
