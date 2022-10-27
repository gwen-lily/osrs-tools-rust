use crate::{
    bonus::Gear,
    character::player::Player,
    data::{MeleeDamageType, Skill, DT},
    spell::Spell,
    CombatMap, Result,
};

use super::{CmbMod, DmgMod, DmgModFunction};

pub struct VoidModifiers<'a> {
    player: &'a Player,
}

impl<'a> CmbMod for VoidModifiers<'a> {
    fn combat_mod(&self) -> Option<CombatMap<f64>> {
        // Regular void equipped, I can see the false being confusing
        if self.player.equipment_info.void_equipped(false) {
            let mut combat_mod: CombatMap<f64> = CombatMap::new();
            // Melee
            combat_mod.insert((DT::Melee(MeleeDamageType::Default), Skill::Attack), 1.10);
            combat_mod.insert((DT::Melee(MeleeDamageType::Default), Skill::Strength), 1.10);
            // Ranged
            combat_mod.insert((DT::Ranged, Skill::Attack), 1.10);
            combat_mod.insert((DT::Ranged, Skill::Strength), 1.10);
            // Magic
            combat_mod.insert((DT::Magic, Skill::Attack), 1.45);

            Some(combat_mod)
        // Elite void equipped
        } else if self.player.equipment_info.void_equipped(true) {
            let mut combat_mod: CombatMap<f64> = CombatMap::new();
            // Melee
            combat_mod.insert((DT::Melee(MeleeDamageType::Default), Skill::Attack), 1.10);
            combat_mod.insert((DT::Melee(MeleeDamageType::Default), Skill::Strength), 1.10);
            // Ranged
            combat_mod.insert((DT::Ranged, Skill::Attack), 1.10);
            combat_mod.insert((DT::Ranged, Skill::Strength), 1.125);
            // Magic
            combat_mod.insert((DT::Magic, Skill::Attack), 1.45);
            combat_mod.insert((DT::Magic, Skill::Strength), 1.025);

            Some(combat_mod)
        } else {
            None
        }
    }
}

pub(crate) struct PoweredStaffModifiers<'a> {
    pub(crate) player: &'a Player,
}

impl<'a> PoweredStaffModifiers<'_> {
    pub(crate) fn get_powered_spell_base_max(&self) -> Result<u8> {
        use crate::bonus::GearName::*;
        let wpn: &Gear = self.player.weapon();
        let spl: &Spell = self.player.spell.unwrap();
        let visible_magic_level: i32 = self.player.get_visible_level(Skill::Magic);

        match wpn.name {
            TumekensShadow => {
                let val: u8 = spl.base_max + (visible_magic_level / 3) as u8;
                Ok(val)
            }
            TridentOfTheSeas | TridentOfTheSwamp | SanguinestiStaff => {
                let clmpd_lvl: i32 = visible_magic_level.max(75);
                let diff_lvl: i32 = clmpd_lvl - 75;
                let val: u8 = spl.base_max + (diff_lvl / 3) as u8;
                Ok(val)
            }
            _ => Err(crate::OsrsError::Spell),
        }
    }
}
