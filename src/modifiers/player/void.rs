use crate::character::Player;
use crate::data::{CombatAspect::*, MeleeDamageType, DT};
use crate::modifiers::CmbMod;
use crate::CombatMap;

use crate::bonus::SetName::*;

pub(crate) struct VoidModifiers<'a> {
    pub(crate) player: &'a Player,
}

impl<'a> CmbMod for VoidModifiers<'a> {
    fn combat_mod(&self) -> Option<CombatMap<f64>> {
        if self.player.equipment_info.set_equipped(NormalVoidSet) {
            let mut combat_mod: CombatMap<f64> = CombatMap::new();
            // Melee
            combat_mod.insert((DT::Melee(MeleeDamageType::Default), Attack), 1.10);
            combat_mod.insert((DT::Melee(MeleeDamageType::Default), Strength), 1.10);
            // Ranged
            combat_mod.insert((DT::Ranged, Attack), 1.10);
            combat_mod.insert((DT::Ranged, Strength), 1.10);
            // Magic
            combat_mod.insert((DT::Magic, Attack), 1.45);

            Some(combat_mod)
        } else if self.player.equipment_info.set_equipped(EliteVoidSet) {
            let mut combat_mod: CombatMap<f64> = CombatMap::new();
            // Melee
            combat_mod.insert((DT::Melee(MeleeDamageType::Default), Attack), 1.10);
            combat_mod.insert((DT::Melee(MeleeDamageType::Default), Strength), 1.10);
            // Ranged
            combat_mod.insert((DT::Ranged, Attack), 1.10);
            combat_mod.insert((DT::Ranged, Strength), 1.125);
            // Magic
            combat_mod.insert((DT::Magic, Attack), 1.45);
            combat_mod.insert((DT::Magic, Strength), 1.025);

            Some(combat_mod)
        } else {
            None
        }
    }
}
