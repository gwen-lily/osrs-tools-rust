use crate::bonus::{BonusLike, BonusStats};
use crate::character::{Monster, Player};
use crate::combat::{self, accuracy, base_damage, maximum_roll, DamageDistribution, Hitsplats};
use crate::modifiers::player::{
    dinhs::DinhsModifier, powered_staff::PoweredStaffModifiers, tumeken::TumekensModifier,
};
use crate::modifiers::{BnsMod, CmbMod, DmgBuff, PlayerModifiers};
use crate::spell::Spell;
use crate::{levels, CombatAspect, Skill, DT};

/// A vanilla weapon with completely default behavior wrt. what constitutes a "weapon" function

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Copy, Clone, Default)]
pub struct NormalWeaponTrait {}

impl WeaponTrait for NormalWeaponTrait {}

pub trait WeaponTrait {
    fn attack_skill(&self, player: &Player) -> Skill {
        let dt: DT = player.style.dt;
        levels::combat_skill(dt, CombatAspect::Attack)
    }

    fn strength_skill(&self, player: &Player) -> Skill {
        let dt: DT = player.style.dt;
        levels::combat_skill(dt, CombatAspect::Strength)
    }

    fn defence_skill(&self, pmods: &PlayerModifiers) -> Skill {
        // Determine dt
        let player = pmods.player;

        // If there's a weapon...
        let dt: DT = if let Some(wpn) = &player.equipment_info.equipment.weapon {
            // If that weapon is a special weapon...
            if let Some(spec_wpn) = &wpn.special_weapon_info {
                // If that special weapon has special functions...
                if let Some(spec_fns) = &wpn.unique_spec_fns {
                    // If those special functions produce a unique dt...
                    if let Some(spec_dt) = spec_fns.get_special_defence_roll(pmods) {
                        spec_dt
                    } else {
                        player.style.dt
                    }
                } else {
                    // Or if the basic special info is enough (no tricks)
                    if let Some(spec_dt) = spec_wpn.special_defence_roll {
                        spec_dt
                    } else {
                        player.style.dt
                    }
                }
            // If that weapon is not special, easy
            } else {
                player.style.dt
            }
        } else {
            player.style.dt
        };

        levels::combat_skill(dt, CombatAspect::Defence)
    }

    fn get_invisible_level(&self, pmods: &PlayerModifiers, dt: DT, aspect: CombatAspect) -> u32 {
        pmods.player.get_invisible_level(dt, aspect)
    }

    fn get_effective_level(&self, pmods: &PlayerModifiers, dt: DT, aspect: CombatAspect) -> u32 {
        pmods.player.get_effective_level(dt, aspect)
    }

    fn bonus_stats(&self, pmods: &PlayerModifiers) -> BonusStats {
        let player: &Player = pmods.player;
        let target: &Monster = pmods.target;

        let mut bs: BonusStats = player.equipment_info.equipment.get_bonus_stats().clone();

        // dinhs
        let dinhs_mod = DinhsModifier { player };

        if let Some(bs_dinhs) = dinhs_mod.bonus_mod() {
            for ((dt, skill), bns_mod) in bs_dinhs.iter() {
                if let Some(prev_val) = bs.get(&(*dt, *skill)) {
                    let new_val: i32 = prev_val + bns_mod;
                    bs.insert((*dt, *skill), new_val);
                } else {
                    panic!()
                }
            }
        }

        // tumeken
        let tummy_mod = TumekensModifier { player, target };

        if let Some(bs_tum) = tummy_mod.combat_mod() {
            for ((dt, skill), cmb_mod) in bs_tum.iter() {
                if let Some(prev_val) = bs.get(&(*dt, *skill)) {
                    let new_val: i32 = (*prev_val as f64 * *cmb_mod).trunc() as i32;
                    bs.insert((*dt, *skill), new_val);
                } else {
                    panic!()
                }
            }
        }

        bs
    }

    fn base_max(&self, pmods: &PlayerModifiers) -> u32 {
        let player: &Player = pmods.player;
        let dt: DT = player.style.dt;

        match dt {
            DT::Melee(_) | DT::Ranged => {
                let effective_strength_level: u32 =
                    self.get_effective_level(pmods, dt, CombatAspect::Strength);

                let gear_str_bns: i32 = *self
                    .bonus_stats(pmods)
                    .get(&(dt, CombatAspect::Strength))
                    .unwrap();

                base_damage(effective_strength_level, gear_str_bns).trunc() as u32
            }
            DT::Magic => {
                use crate::spell::Spellbook::*;
                let spl: &Spell = player.spell.unwrap();

                match spl.spellbook {
                    Standard | Ancient | Lunar | Arceus => spl.base_max,
                    Powered => {
                        let pwr_stf_mod: PoweredStaffModifiers = PoweredStaffModifiers { player };
                        spl.base_max + pwr_stf_mod.damage_buff().unwrap()
                    }
                }
            }
            DT::Typeless => todo!(),
        }
    }

    fn actual_max(&self, pmods: &PlayerModifiers) -> u32 {
        let base_max: u32 = self.base_max(pmods);
        let mut dms: Vec<f64> = pmods.get_all_dms();

        if *pmods.special_attack {
            if let Some(spec_dms) = &pmods
                .player
                .weapon()
                .special_weapon_info
                .as_ref()
                .unwrap()
                .special_dms
            {
                dms.append(&mut spec_dms.clone());
            }
        }

        combat::max_hit(base_max, &dms)
    }

    fn attack_maximum_roll(&self, pmods: &PlayerModifiers) -> u32 {
        let player: &Player = pmods.player;
        let dt = player.style.dt;

        let eff_atk_lvl = self.get_effective_level(pmods, dt, CombatAspect::Attack);
        let gear_atk_bns: i32 = *self
            .bonus_stats(pmods)
            .get(&(dt, CombatAspect::Attack))
            .unwrap();
        let mut arm_mods: Vec<f64> = pmods.get_all_arms();

        if *pmods.special_attack {
            if let Some(spec_arms) = &player
                .weapon()
                .special_weapon_info
                .as_ref()
                .unwrap()
                .special_arms
            {
                arm_mods.append(&mut spec_arms.clone());
            }
        }

        maximum_roll(eff_atk_lvl, gear_atk_bns, &arm_mods)
    }

    fn defence_maximum_roll(&self, pmods: &PlayerModifiers) -> u32 {
        let player: &Player = pmods.player;
        let target: &Monster = pmods.target;

        let dt = player.style.dt;

        // get the defence skill & level of the target, based on the player's weapon / spec weapon
        let defence_skill: Skill = self.defence_skill(pmods);
        let effective_def_lvl: u32 = *target.levels.get(&defence_skill).unwrap() as u32;

        let monster_bns_stats: &BonusStats = target.monster_bonus.get_bonus_stats();
        let def_bns: i32 = *monster_bns_stats.get(&(dt, CombatAspect::Defence)).unwrap();

        let drms: &[f64] = &pmods.get_all_drms();

        maximum_roll(effective_def_lvl, def_bns, drms)
    }

    fn accuracy(&self, pmods: &PlayerModifiers) -> f64 {
        let atk_roll: u32 = self.attack_maximum_roll(pmods);
        let def_roll: u32 = self.defence_maximum_roll(pmods);

        accuracy(atk_roll, def_roll)
    }

    fn hp_cap(&self, pmods: &PlayerModifiers) -> u32 {
        let target: &Monster = pmods.target;
        *target.levels.get(&Skill::Hitpoints).unwrap() as u32
    }

    fn damage_distribution(&self, pmods: &PlayerModifiers) -> Hitsplats {
        let max_hit: u32 = self.actual_max(pmods);
        let accuracy: f64 = self.accuracy(pmods);
        let hp_cap: u32 = self.hp_cap(pmods);

        let hs = DamageDistribution::simple(max_hit, accuracy, Some(hp_cap));
        vec![hs]
    }
}
