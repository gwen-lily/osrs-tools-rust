use crate::character::Player;
use crate::modifiers::{ArMod, ConditionalModifier, DmgMod};

use crate::bonus::GearName::{self, *};
use crate::spell::Spellbook;

pub(crate) struct SmokeModifier<'a> {
    pub(crate) player: &'a Player,
}

impl<'a> ConditionalModifier for SmokeModifier<'a> {
    fn activate(&self) -> bool {
        let wpn_name: GearName = self.player.weapon().name;

        match wpn_name {
            SmokeBattlestaff | MysticSmokeStaff => match self.player.spell {
                Some(spl) => spl.spellbook == Spellbook::Standard,
                None => false,
            },
            _ => false,
        }
    }
}

impl<'a> ArMod for SmokeModifier<'a> {
    fn accuracy_roll_mod(&self) -> Option<f64> {
        if self.activate() {
            Some(1.10)
        } else {
            None
        }
    }
}

impl<'a> DmgMod for SmokeModifier<'a> {
    fn damage_mod(&self) -> Option<f64> {
        if self.activate() {
            Some(0.10) // value ADDED not multiplied
        } else {
            None
        }
    }
}
