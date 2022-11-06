use crate::bonus::gear::weapon::{
    crossbow_weapon_trait::CrossbowWeaponTrait, weapon_trait::WeaponTrait,
};

struct ZaryteCrossbow {}

impl WeaponTrait for ZaryteCrossbow {}
impl CrossbowWeaponTrait for ZaryteCrossbow {}
