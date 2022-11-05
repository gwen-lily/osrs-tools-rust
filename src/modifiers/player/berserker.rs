use crate::bonus::EquipmentNameMap;
use crate::character::Player;
use crate::modifiers::{ConditionalModifier, DmgMod};

use crate::bonus::GearName::BerserkerNecklace;
use crate::bonus::Slot::Neck;

pub(crate) struct BerserkerNecklaceModifier<'a> {
    pub(crate) player: &'a Player,
}

impl<'a> ConditionalModifier for BerserkerNecklaceModifier<'a> {
    fn activate(&self) -> bool {
        let eqp_info = &self.player.equipment_info;

        let mut map: EquipmentNameMap = EquipmentNameMap::new();
        map.insert(Neck, BerserkerNecklace);

        eqp_info.equipped_name(&map) && eqp_info.obsidian_weapon_equipped()
    }
}

impl<'a> DmgMod for BerserkerNecklaceModifier<'a> {
    fn damage_mod(&self) -> Option<f64> {
        if !self.activate() {
            return None;
        };

        Some(1.2)
    }
}
