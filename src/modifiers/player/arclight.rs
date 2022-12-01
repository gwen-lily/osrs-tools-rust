use crate::character::{Monster, MonsterAttribute, Player};
use crate::modifiers::{ArMod, ConditionalModifier, DmgMod};

use crate::bonus::GearName::*;

pub(crate) struct ArclightModifier<'a> {
    pub(crate) player: &'a Player,
    pub(crate) target: &'a Monster,
}

impl<'a> ConditionalModifier for ArclightModifier<'a> {
    fn activate(&self) -> bool {
        let ply = self.player;
        let tgt = self.target;

        match ply.weapon().gear_info.name {
            Arclight | Darklight => {
                if let Some(attrs) = &tgt.attributes {
                    if attrs.contains(&MonsterAttribute::Demon) {
                        return true;
                    };
                };
            }
            _ => return false,
        }
        false
    }
}

impl<'a> ArMod for ArclightModifier<'a> {
    fn accuracy_roll_mod(&self) -> Option<f64> {
        if !self.activate() {
            return None;
        };

        Some(1.7)
    }
}

impl<'a> DmgMod for ArclightModifier<'a> {
    fn damage_mod(&self) -> Option<f64> {
        if !self.activate() {
            return None;
        };

        Some(1.7)
    }
}
