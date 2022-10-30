use crate::character::Player;
use crate::modifiers::{ConditionalModifier, DmgMod};

use crate::bonus::SetName::DharokSet;
use crate::Skill::*;

pub(crate) struct DharokModifier<'a> {
    pub(crate) player: &'a Player,
}

impl<'a> ConditionalModifier for DharokModifier<'a> {
    fn activate(&self) -> bool {
        self.player.equipment_info.set_equipped(&DharokSet)
    }
}

impl<'a> DmgMod for DharokModifier<'a> {
    fn damage_mod(&self) -> Option<f64> {
        if self.activate() == false {
            return None;
        };

        let base_hp: i32 = *self.player.levels.get(&Hitpoints).unwrap();
        let curr_hp: i32 = self.player.get_visible_level(&Hitpoints);
        let dmg_mod: f64 = 1.0 + { (base_hp - curr_hp) as f64 / 100.0 * (base_hp as f64 / 100.0) };

        Some(dmg_mod)
    }
}
