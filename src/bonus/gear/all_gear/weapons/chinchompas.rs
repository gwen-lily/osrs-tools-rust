use crate::bonus::gear::weapon::{
    chinchompa_weapon_trait::ChinchompaWeaponTrait, weapon_trait::WeaponTrait,
};

pub struct Chinchompa {}
pub struct RedChinchompa {}
pub struct BlackChinchompa {}

impl WeaponTrait for Chinchompa {}
impl ChinchompaWeaponTrait for Chinchompa {}

impl WeaponTrait for RedChinchompa {}
impl ChinchompaWeaponTrait for RedChinchompa {}

impl WeaponTrait for BlackChinchompa {}
impl ChinchompaWeaponTrait for BlackChinchompa {}
