use crate::character::{Monster, Player};
use crate::modifiers::{ArMod, ConditionalModifier, DmgMod};

use crate::bonus::GearName::*;
use crate::bonus::Slot::*;
use crate::DT::*;

pub(crate) struct SlayerModifier<'a> {
    pub(crate) player: &'a Player,
    pub(crate) target: &'a Monster,
}

impl<'a> ConditionalModifier for SlayerModifier<'a> {
    fn activate(&self) -> bool {
        let ply: &Player = self.player;
        let tgt: &Monster = self.target;

        if ply.weapon().name == SeerCull {
            return false;
        };

        if let Some(task) = ply.slayer_task {
            if let Some(slyr) = tgt.slayer {
                if task != slyr {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        };

        if let Some(head) = ply.eqpd().get(&Head) {
            match head.name {
                SlayerHelmetI => return true,
                _ => return false,
            }
        }

        true
    }
}

impl<'a> ArMod for SlayerModifier<'a> {
    fn accuracy_roll_mod(&self) -> Option<f64> {
        if self.activate() == false {
            return None;
        };

        match self.player.style.dt {
            Melee(_) => Some(7.0 / 6.0),
            Ranged => Some(1.15),
            Magic => Some(1.15),
            _ => panic!("typeless?"),
        }
    }
}

impl<'a> DmgMod for SlayerModifier<'a> {
    fn damage_mod(&self) -> Option<f64> {
        if self.activate() == false {
            return None;
        };

        match self.player.style.dt {
            Melee(_) => Some(7.0 / 6.0),
            Ranged => Some(1.15),
            Magic => Some(1.15),
            _ => panic!("typeless?"),
        }
    }
}
