use crate::{
    bonus::{BonusLike, BonusStats, Gear, GearName, Slot},
    character::{player::Player, Monster},
    data::{MeleeDamageType, MonsterAttribute, MonsterLocation, Skill, DT},
    spell::{Spell, Spellbook},
    CombatMap, STYLES_MAP,
};

use super::{ArMod, BnsMod, CmbMod, DmgBuff, DmgMod};

// pub(crate) fn get_all_modifiers(player: &Player, target: &Monster) ->

pub(crate) struct VoidModifiers<'a> {
    player: &'a Player,
}

pub(crate) struct PoweredStaffModifiers<'a> {
    pub(crate) player: &'a Player,
}

pub(crate) struct DinhsModifier<'a> {
    pub(crate) player: &'a Player,
}

pub(crate) struct SmokeModifier<'a> {
    pub(crate) player: &'a Player,
}

pub(crate) struct TumekensModifier<'a> {
    pub(crate) player: &'a Player,
    pub(crate) target: &'a Monster,
}

pub(crate) struct SalveModifier<'a> {
    pub(crate) player: &'a Player,
    pub(crate) target: &'a Monster,
}
pub(crate) struct MagicDamageModifier<'a> {
    pub(crate) player: &'a Player,
}

pub(crate) struct SlayerModifier<'a> {
    pub(crate) player: &'a Player,
    pub(crate) target: &'a Monster,
}

// pub(crate) struct FooModifier<'a> {
//     pub(crate) player: &'a Player,
// }

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

impl<'a> DmgBuff for PoweredStaffModifiers<'a> {
    fn damage_buff(&self) -> Option<u8> {
        use crate::bonus::GearName::*;
        let wpn: &Gear = self.player.weapon();
        let visible_magic_level: i32 = self.player.get_visible_level(Skill::Magic);

        match wpn.name {
            TumekensShadow => {
                let bonus_dmg: u8 = (visible_magic_level / 3) as u8;
                Some(bonus_dmg)
            }
            TridentOfTheSeas | TridentOfTheSwamp | SanguinestiStaff => {
                let clmpd_lvl: i32 = visible_magic_level.max(75);
                let diff_lvl: i32 = clmpd_lvl - 75;
                let bonus_dmg: u8 = (diff_lvl / 3) as u8;
                Some(bonus_dmg)
            }
            _ => panic!(), // how you got here without a powered staff is beyond me
        }
    }
}

impl<'a> CmbMod for TumekensModifier<'a> {
    fn combat_mod(&self) -> Option<CombatMap<f64>> {
        use crate::bonus::GearName::*;
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

impl<'a> BnsMod for DinhsModifier<'a> {
    fn bonus_mod(&self) -> Option<BonusStats> {
        use crate::style::StylesCategory::Bulwarks;
        if let Some(bulwark_styles) = STYLES_MAP.get(&Bulwarks) {
            if let Some(pummel) = bulwark_styles.styles.get(0) {
                if self.player.style == pummel {
                    use crate::data::MeleeDamageType::*;
                    use crate::data::Skill::{Defence, Strength};
                    use crate::data::DT::*;

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

                    return Some(map);
                }
            }
        }
        None
    }
}

impl<'a> ArMod for SalveModifier<'a> {
    fn accuracy_roll_mod(&self) -> Option<f64> {
        use crate::bonus::GearName::*;
        let tgt: &Monster = self.target;

        if let Some(attrs) = &tgt.attributes {
            if attrs.contains(&MonsterAttribute::Undead) {
                if let Some(neck) = self.player.eqpd().get(&Slot::Neck) {
                    match neck.name {
                        SalveAmuletI => Some(7.0 / 6.0),
                        SalveAmuletEI => Some(1.2),
                        _ => None,
                    };
                };
            };
        };

        None
    }
}

impl<'a> DmgMod for SalveModifier<'a> {
    fn damage_mod(&self) -> Option<f64> {
        self.accuracy_roll_mod()
    }
}

impl<'a> ArMod for SmokeModifier<'a> {
    fn accuracy_roll_mod(&self) -> Option<f64> {
        use crate::bonus::GearName::*;
        let wpn_name: GearName = self.player.weapon().name;

        match wpn_name {
            SmokeBattlestaff | MysticSmokeStaff => match self.player.spell {
                Some(spl) => {
                    if spl.spellbook == Spellbook::Standard {
                        Some(1.10)
                    } else {
                        None
                    }
                }
                None => None,
            },
            _ => None,
        }
    }
}

impl<'a> DmgMod for SmokeModifier<'a> {
    fn damage_mod(&self) -> Option<f64> {
        use crate::bonus::GearName::*;
        let wpn_name: GearName = self.player.weapon().name;

        match wpn_name {
            SmokeBattlestaff | MysticSmokeStaff => match self.player.spell {
                Some(spl) => {
                    if spl.spellbook == Spellbook::Standard {
                        Some(0.10) // value ADDED not multiplied
                    } else {
                        None
                    }
                }
                None => None,
            },
            _ => None,
        }
    }
}

impl<'a> DmgMod for MagicDamageModifier<'a> {
    fn damage_mod(&self) -> Option<f64> {
        let ply: &Player = self.player;

        if ply.spell.is_none() || ply.style.dt != DT::Magic {
            return None;
        };

        // Strange definition, preserves the i32 though
        let bns: &BonusStats = ply.equipment_info.equipment.get_bonus_stats();
        let mag_dmg_bonus: f64 =
            (*bns.get(&(DT::Magic, Skill::Strength)).unwrap_or(&0) as f64) / 1000.0;

        let val: f64 = 1.0 + mag_dmg_bonus;
        Some(val)
    }
}

impl<'a> ArMod for SlayerModifier<'a> {
    fn accuracy_roll_mod(&self) -> Option<f64> {
        let ply: &Player = self.player;
        let tgt: &Monster = self.target;

        if ply.weapon().name == GearName::SeerCull {
            return None;
        };

        if let Some(task) = ply.slayer_task {
            if let Some(slyr) = tgt.slayer {
                if task != slyr {
                    return None;
                }
            } else {
                return None;
            }
        } else {
            return None;
        };

        if let Some(head) = ply.eqpd().get(&Slot::Head) {
            match head.name {
                GearName::SlayerHelmetI => match ply.style.dt {
                    DT::Melee(_) => Some(7.0 / 6.0),
                    DT::Ranged => Some(1.15),
                    DT::Magic => Some(1.15),
                    _ => panic!("typeless?"),
                },
                _ => None,
            }
        } else {
            None
        }
    }
}

impl<'a> DmgMod for SlayerModifier<'a> {
    fn damage_mod(&self) -> Option<f64> {
        let ply: &Player = self.player;
        let tgt: &Monster = self.target;

        if ply.weapon().name == GearName::SeerCull {
            return None;
        };

        if let Some(task) = ply.slayer_task {
            if let Some(slyr) = tgt.slayer {
                if task != slyr {
                    return None;
                }
            } else {
                return None;
            }
        } else {
            return None;
        };

        if let Some(head) = ply.eqpd().get(&Slot::Head) {
            match head.name {
                GearName::SlayerHelmetI => match ply.style.dt {
                    DT::Melee(_) => Some(7.0 / 6.0),
                    DT::Ranged => Some(1.15),
                    DT::Magic => Some(1.15),
                    _ => panic!("typeless?"),
                },
                _ => None,
            }
        } else {
            None
        }
    }
}
