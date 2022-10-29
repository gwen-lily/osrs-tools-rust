use crate::character::{Monster, MonsterAttribute, Player};
use crate::modifiers::{ArMod, ConditionalModifier, DmgMod};

use crate::bonus::GearName::*;

pub(crate) struct KalphiteModifier<'a> {
    pub(crate) player: &'a Player,
    pub(crate) target: &'a Monster,
}

impl<'a> ConditionalModifier for KalphiteModifier<'a> {
    fn activate(&self) -> bool {
        if let Some(attrs) = &self.target.attributes {
            if !attrs.contains(&MonsterAttribute::Kalphite) {
                return false;
            };

            match self.player.weapon().name {
                Keris
                | KerisPartisan
                | KerisPartisanOfBreaching
                | KerisPartisanOfCorruption
                | KerisPartisanOfTheSun => return true,
                _ => return false,
            }
        }
        false
    }
}

impl<'a> ArMod for KalphiteModifier<'a> {
    fn accuracy_roll_mod(&self) -> Option<f64> {
        if self.activate() == false {
            return None;
        };

        match self.player.weapon().name {
            KerisPartisanOfBreaching => Some(1.33),
            Keris | KerisPartisan | KerisPartisanOfCorruption | KerisPartisanOfTheSun => None,
            _ => panic!(),
        }
    }
}

impl<'a> DmgMod for KalphiteModifier<'a> {
    fn damage_mod(&self) -> Option<f64> {
        if self.activate() == false {
            return None;
        };

        Some(1.33)
    }
}
