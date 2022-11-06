use crate::bonus::gear::weapon::{
    crossbow_weapon_trait::CrossbowWeaponTrait, weapon_trait::WeaponTrait,
};

struct ArmadylCrossbow {}

impl WeaponTrait for ArmadylCrossbow {}
impl CrossbowWeaponTrait for ArmadylCrossbow {}
