use crate::character::monster::MonsterName;
use crate::character::{Monster, Player};
use crate::modifiers::{ConditionalModifier, DmgMod};
use crate::spell::Aspect;

pub(crate) struct IceDemonModifier<'a> {
    pub(crate) player: &'a Player,
    pub(crate) target: &'a Monster,
}

impl<'a> ConditionalModifier for IceDemonModifier<'a> {
    fn activate(&self) -> bool {
        self.target.name == MonsterName::IceDemon
    }
}

impl<'a> DmgMod for IceDemonModifier<'a> {
    fn damage_mod(&self) -> Option<f64> {
        if !self.activate() {
            return None;
        };

        if let Some(spl) = self.player.spell {
            if let Some(aspct) = spl.aspect {
                if aspct == Aspect::Fire {
                    Some(1.50)
                } else {
                    Some(0.33)
                }
            } else {
                Some(0.33)
            }
        } else {
            Some(0.33)
        }
    }
}
