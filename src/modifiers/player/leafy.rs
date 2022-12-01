use crate::character::{Monster, MonsterAttribute, Player};
use crate::modifiers::{ArMod, ConditionalModifier, DmgMod};

use crate::bonus::GearName::*;

pub(crate) struct LeafyModifier<'a> {
    pub(crate) player: &'a Player,
    pub(crate) target: &'a Monster,
}

impl<'a> ConditionalModifier for LeafyModifier<'a> {
    fn activate(&self) -> bool {
        if let Some(attrs) = &self.target.attributes {
            if !attrs.contains(&MonsterAttribute::Leafy) {
                return false;
            };

            match self.player.weapon().gear_info.name {
                LeafBladedSpear | LeafBladedSword | LeafBladedBattleaxe => return true,
                _ => return false,
            }
        }
        false
    }
}

impl<'a> ArMod for LeafyModifier<'a> {
    fn accuracy_roll_mod(&self) -> Option<f64> {
        if !self.activate() {
            return None;
        };

        Some(1.175)
    }
}

impl<'a> DmgMod for LeafyModifier<'a> {
    fn damage_mod(&self) -> Option<f64> {
        if !self.activate() {
            return None;
        };

        Some(1.175)
    }
}
