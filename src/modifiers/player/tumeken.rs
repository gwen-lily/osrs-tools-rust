use crate::character::{Monster, MonsterLocation, Player};
use crate::modifiers::CmbMod;
use crate::{CombatMap, Skill, DT};

use crate::bonus::GearName::*;

pub(crate) struct TumekensModifier<'a> {
    pub(crate) player: &'a Player,
    pub(crate) target: &'a Monster,
}

impl<'a> CmbMod for TumekensModifier<'a> {
    fn combat_mod(&self) -> Option<CombatMap<f64>> {
        let ply = self.player;
        let tgt = self.target;

        let tum_mod: f64 =
            if ply.weapon().name == TumekensShadow && tgt.location == MonsterLocation::Toa {
                4.0
            } else {
                3.0
            };

        let mut map = CombatMap::new();
        map.insert((DT::Magic, Skill::Strength), tum_mod);

        Some(map)
    }
}
