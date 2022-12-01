use crate::character::{Monster, MonsterLocation, Player};
use crate::modifiers::{ArMod, ConditionalModifier, DmgMod};

use crate::bonus::GearName::*;

pub(crate) struct WildernessModifier<'a> {
    pub(crate) player: &'a Player,
    pub(crate) target: &'a Monster,
}

impl<'a> ConditionalModifier for WildernessModifier<'a> {
    fn activate(&self) -> bool {
        match self.player.weapon().gear_info.name {
            ViggorasChainmace | CrawsBow | ThammaronsSceptre => {
                self.target.location == MonsterLocation::Wilderness
            }
            _ => false,
        }
    }
}

impl<'a> ArMod for WildernessModifier<'a> {
    fn accuracy_roll_mod(&self) -> Option<f64> {
        if !self.activate() {
            return None;
        };

        match self.player.weapon().gear_info.name {
            ViggorasChainmace => Some(1.5),
            CrawsBow => Some(1.5),
            ThammaronsSceptre => Some(2.0),
            _ => panic!(),
        }
    }
}

impl<'a> DmgMod for WildernessModifier<'a> {
    fn damage_mod(&self) -> Option<f64> {
        if !self.activate() {
            return None;
        };

        match self.player.weapon().gear_info.name {
            ViggorasChainmace => Some(1.5),
            CrawsBow => Some(1.5),
            ThammaronsSceptre => Some(1.25),
            _ => panic!(),
        }
    }
}
