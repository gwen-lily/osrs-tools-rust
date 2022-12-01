use crate::character::{Monster, MonsterLocation, Player};
use crate::modifiers::{CmbMod, ConditionalModifier};
use crate::{CombatAspect, CombatMap, DT};

use crate::spell::SpellName;

pub(crate) struct TumekensModifier<'a> {
    pub(crate) player: &'a Player,
    pub(crate) target: &'a Monster,
}

impl<'a> ConditionalModifier for TumekensModifier<'a> {
    fn activate(&self) -> bool {
        if let Some(spl) = self.player.spell {
            if spl.name == SpellName::TumekensShadow {
                return true;
            }
        }
        false
    }
}

impl<'a> CmbMod for TumekensModifier<'a> {
    fn combat_mod(&self) -> Option<CombatMap<f64>> {
        if !self.activate() {
            return None;
        };

        let tum_mod: f64 = if self.target.location == MonsterLocation::Toa {
            4.0
        } else {
            3.0
        };

        let mut map: CombatMap<f64> = CombatMap::new();
        map.insert((DT::Magic, CombatAspect::Strength), tum_mod);

        Some(map)
    }
}
