use crate::character::Player;
use crate::modifiers::{ConditionalModifier, DmgBuff};
use crate::spell::SpellName::*;

pub(crate) struct ChaosGauntletModifier<'a> {
    pub(crate) player: &'a Player,
}

impl<'a> ConditionalModifier for ChaosGauntletModifier<'a> {
    fn activate(&self) -> bool {
        if let Some(spl) = self.player.spell {
            match spl.name {
                WindBolt | WaterBolt | EarthBolt | FireBolt => true,
                _ => false,
            };
        }
        false
    }
}

impl<'a> DmgBuff for ChaosGauntletModifier<'a> {
    fn damage_buff(&self) -> Option<u8> {
        if !self.activate() {
            return None;
        };

        Some(3)
    }
}
