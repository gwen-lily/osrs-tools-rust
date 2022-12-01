use crate::character::Player;
use crate::modifiers::{ConditionalModifier, DmgMod};

use crate::bonus::SetName::DharokSet;
use crate::Skill::*;

pub(crate) struct DharokModifier<'a> {
    pub(crate) player: &'a Player,
}

impl<'a> ConditionalModifier for DharokModifier<'a> {
    fn activate(&self) -> bool {
        self.player.equipment_info.set_equipped(DharokSet)
    }
}

impl<'a> DmgMod for DharokModifier<'a> {
    fn damage_mod(&self) -> Option<f64> {
        if !self.activate() {
            return None;
        };

        let base_hp: i32 = *self.player.levels.get(&Hitpoints).unwrap();
        let hp_diff: u32 = (base_hp - self.player.resources.hitpoints as i32).max(0) as u32;
        let dmg_mod: f64 = 1.0 + { hp_diff as f64 / 100.0 * (base_hp as f64 / 100.0) };

        Some(dmg_mod)
    }
}
