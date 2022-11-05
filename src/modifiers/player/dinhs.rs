use crate::{
    bonus::{BonusLike, BonusStats},
    character::Player,
    data::MeleeDamageType::*,
    modifiers::{BnsMod, ConditionalModifier},
    style::StylesCategory::Bulwarks,
    Skill::{Defence, Strength},
    DT::*,
    STYLES_MAP,
};

pub(crate) struct DinhsModifier<'a> {
    pub(crate) player: &'a Player,
}

impl<'a> ConditionalModifier for DinhsModifier<'a> {
    fn activate(&self) -> bool {
        if let Some(bulwark_styles) = STYLES_MAP.get(&Bulwarks) {
            if let Some(pummel) = bulwark_styles.styles.get(0) {
                return self.player.style == pummel;
            }
        }
        false
    }
}

impl<'a> BnsMod for DinhsModifier<'a> {
    fn bonus_mod(&self) -> Option<BonusStats> {
        if !self.activate() {
            return None;
        };

        // Get the mean defensive bonus of these four stats
        let db: &BonusStats = self.player.equipment_info.equipment.get_bonus_stats();
        let sum_def_bonus: i32 = *db.get(&(Melee(Stab), Defence)).unwrap()
            + *db.get(&(Melee(Slash), Defence)).unwrap()
            + *db.get(&(Melee(Crush), Defence)).unwrap()
            + *db.get(&(Ranged, Defence)).unwrap();
        let mean_def_bonus = sum_def_bonus as f64 / 4.0;
        // And calculate the strength bonus
        let val: i32 = (((mean_def_bonus - 200.0) / 3.0) - 38.0).trunc() as i32;

        let mut map: BonusStats = BonusStats::new();
        map.insert((Melee(Default), Strength), val);

        Some(map)
    }
}
