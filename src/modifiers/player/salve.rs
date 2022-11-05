use crate::character::{Monster, MonsterAttribute, Player};
use crate::modifiers::{ArMod, ConditionalModifier, DmgMod};

use crate::bonus::{GearName::*, Slot::*};

pub(crate) struct SalveModifier<'a> {
    pub(crate) player: &'a Player,
    pub(crate) target: &'a Monster,
}

impl<'a> ConditionalModifier for SalveModifier<'a> {
    fn activate(&self) -> bool {
        let tgt: &Monster = self.target;

        if let Some(attrs) = &tgt.attributes {
            if attrs.contains(&MonsterAttribute::Undead) {
                if let Some(neck) = self.player.eqpd().get(&Neck) {
                    match neck.gear_info.name {
                        SalveAmuletI | SalveAmuletEI => return true,
                        _ => return false,
                    };
                };
            }
        }
        false
    }
}

impl<'a> ArMod for SalveModifier<'a> {
    fn accuracy_roll_mod(&self) -> Option<f64> {
        if !self.activate() {
            return None;
        };

        match self.player.eqpd().get(&Neck).unwrap().gear_info.name {
            SalveAmuletI => Some(7.0 / 6.0),
            SalveAmuletEI => Some(1.2),
            _ => panic!(),
        }
    }
}

impl<'a> DmgMod for SalveModifier<'a> {
    fn damage_mod(&self) -> Option<f64> {
        if !self.activate() {
            return None;
        };

        match self.player.eqpd().get(&Neck).unwrap().gear_info.name {
            SalveAmuletI => Some(7.0 / 6.0),
            SalveAmuletEI => Some(1.2),
            _ => panic!(),
        }
    }
}
