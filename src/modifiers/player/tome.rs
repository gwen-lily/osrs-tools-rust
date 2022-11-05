use crate::bonus::GearName::TomeOfFire;
use crate::bonus::Slot;
use crate::character::Player;
use crate::modifiers::{ConditionalModifier, DmgMod};
use crate::spell::Aspect;

pub(crate) struct TomeOfFireModifier<'a> {
    pub(crate) player: &'a Player,
}

impl<'a> ConditionalModifier for TomeOfFireModifier<'a> {
    fn activate(&self) -> bool {
        if let Some(spl) = self.player.spell {
            if let Some(aspct) = spl.aspect {
                if aspct == Aspect::Fire {
                    if let Some(shield) = self.player.eqpd().get(&Slot::Shield) {
                        return shield.gear_info.name == TomeOfFire;
                    }
                }
            }
        }
        false
    }
}

impl<'a> DmgMod for TomeOfFireModifier<'a> {
    fn damage_mod(&self) -> Option<f64> {
        if !self.activate() {
            return None;
        };

        Some(1.50)
    }
}
