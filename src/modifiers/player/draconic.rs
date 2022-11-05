use crate::character::{Monster, MonsterAttribute, Player};
use crate::modifiers::{ArMod, ConditionalModifier, DmgMod};

use crate::bonus::GearName::*;

pub(crate) struct DraconicModifier<'a> {
    pub(crate) player: &'a Player,
    pub(crate) target: &'a Monster,
}

impl<'a> ConditionalModifier for DraconicModifier<'a> {
    fn activate(&self) -> bool {
        if let Some(attrs) = &self.target.attributes {
            if !attrs.contains(&MonsterAttribute::Draconic) {
                return false;
            };

            match self.player.weapon().gear_info.name {
                DragonHunterLance | DragonHunterCrossbow => return true,
                _ => return false,
            }
        }
        false
    }
}

impl<'a> ArMod for DraconicModifier<'a> {
    fn accuracy_roll_mod(&self) -> Option<f64> {
        if !self.activate() {
            return None;
        };

        match self.player.weapon().gear_info.name {
            DragonHunterLance => Some(1.2),
            DragonHunterCrossbow => Some(1.3),
            _ => None,
        }
    }
}

impl<'a> DmgMod for DraconicModifier<'a> {
    fn damage_mod(&self) -> Option<f64> {
        if !self.activate() {
            return None;
        };

        match self.player.weapon().gear_info.name {
            DragonHunterLance => Some(1.2),
            DragonHunterCrossbow => Some(1.3),
            _ => None,
        }
    }
}
