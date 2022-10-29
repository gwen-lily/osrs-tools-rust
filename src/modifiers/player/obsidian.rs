use crate::character::Player;
use crate::modifiers::{ArMod, ConditionalModifier, DmgMod};

use crate::bonus::SetName::ObsidianSet;

pub(crate) struct ObsidianModifier<'a> {
    pub(crate) player: &'a Player,
}

impl<'a> ConditionalModifier for ObsidianModifier<'a> {
    fn activate(&self) -> bool {
        self.player.equipment_info.set_equipped(&ObsidianSet)
            && self.player.equipment_info.obsidian_weapon_equipped()
    }
}

impl<'a> ArMod for ObsidianModifier<'a> {
    fn accuracy_roll_mod(&self) -> Option<f64> {
        if self.activate() == false {
            return None;
        };

        Some(1.1)
    }
}

impl<'a> DmgMod for ObsidianModifier<'a> {
    fn damage_mod(&self) -> Option<f64> {
        if self.activate() == false {
            return None;
        };

        Some(1.1)
    }
}
