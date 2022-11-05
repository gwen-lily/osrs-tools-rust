use crate::bonus::{BonusLike, BonusStats};
use crate::character::{Monster, Player};
use crate::combat::{accuracy, base_damage, maximum_roll};
use crate::modifiers::player::{
    dinhs::DinhsModifier, powered_staff::PoweredStaffModifiers, tumeken::TumekensModifier,
};
use crate::modifiers::{BnsMod, CmbMod, DmgBuff, PlayerModifiers};
use crate::spell::Spell;
use crate::{levels, OsrsError, Result};
use crate::{
    Skill::{self, *},
    DT,
};

/// A vanilla weapon with completely default behavior wrt. what constitutes a "weapon" function

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Copy, Clone, Default)]
pub struct NormalWeaponTrait {}

impl WeaponTrait for NormalWeaponTrait {}

pub trait WeaponTrait {
    fn attack_skill(&self, player: &Player) -> Result<Skill> {
        let dt: DT = player.style.dt;
        levels::combat_skill(dt, Attack)
    }

    fn strength_skill(&self, player: &Player) -> Result<Skill> {
        let dt: DT = player.style.dt;
        levels::combat_skill(dt, Strength)
    }

    fn defence_skill(&self, pmods: &PlayerModifiers) -> Result<Skill> {
        // Determine dt
        // If there's a weapon...
        let player = pmods.player;

        let dt: DT = if let Some(wpn) = &player.equipment_info.equipment.weapon {
            // If that weapon is a special weapon...
            if let Some(spec_wpn) = &wpn.special_weapon_info {
                // If that special weapon has special functions...
                if let Some(spec_fns) = &wpn.unique_spec_fns {
                    // If those special functions produce a unique dt...
                    if let Some(spec_dt) = spec_fns.get_special_defence_roll(pmods)? {
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

        levels::combat_skill(dt, Defence)
    }

    fn get_effective_level(&self, player: &Player, skill: Skill) -> Result<i32> {
        let dt: DT = player.style.dt;

        let combat_skill: Skill = match levels::combat_skill(dt, skill) {
            Ok(cmb_skl) => cmb_skl,
            Err(err) => return Err(err),
        };

        let effective_level: i32 = player.get_effective_level(&dt, &combat_skill);
        Ok(effective_level)
    }

    fn bonus_stats(&self, player: &Player, target: &Monster) -> BonusStats {
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

    fn base_max(&self, player: &Player, target: &Monster) -> Result<u32> {
        let dt: DT = player.style.dt;

        match dt {
            DT::Melee(_) | DT::Ranged => {
                let strength_skill = self.strength_skill(player)?;
                let effective_strength_level: i32 =
                    match self.get_effective_level(player, strength_skill) {
                        Ok(esl) => esl,
                        Err(err) => return Err(err),
                    };

                let gear_str_bns: i32 = *self
                    .bonus_stats(player, target)
                    .get(&(dt, strength_skill))
                    .unwrap();
                let base_dmg: f64 = base_damage(&effective_strength_level, &gear_str_bns);
                Ok(base_dmg.trunc() as u32)
            }
            DT::Magic => {
                use crate::spell::Spellbook::*;
                let spl: &Spell = player.spell.unwrap();

                let base_max: u32 = match spl.spellbook {
                    Standard | Ancient | Lunar | Arceus => spl.base_max,
                    Powered => {
                        let pwr_stf_mod: PoweredStaffModifiers = PoweredStaffModifiers { player };
                        spl.base_max + pwr_stf_mod.damage_buff().unwrap()
                    }
                };

                Ok(base_max)
            }
            DT::Typeless => Err(OsrsError::Typeless),
        }
    }

    // fn actual_max(&self, pmods: &PlayerModifiers) -> Result<u32> {}

    fn attack_maximum_roll(&self, pmods: &PlayerModifiers) -> i32 {
        let player: &Player = pmods.player;
        let target: &Monster = pmods.target;

        let dt = player.style.dt;
        let msg: &str = "maximum roll";

        let eff_atk_lvl = &self.get_effective_level(player, Attack).expect(msg);
        let attack_skill = self.attack_skill(player).expect(msg);
        let gear_atk_bns: i32 = *self
            .bonus_stats(player, target)
            .get(&(dt, attack_skill))
            .unwrap();

        let arm_mods: &[f64] = &pmods.get_all_arms();

        maximum_roll(eff_atk_lvl, &gear_atk_bns, arm_mods)
    }

    fn defence_maximum_roll(&self, pmods: &PlayerModifiers) -> i32 {
        let player: &Player = pmods.player;
        let target: &Monster = pmods.target;

        let dt = player.style.dt;
        let msg: &str = "maximum roll";

        // get the defence skill & level of the target, based on the player's weapon / spec weapon
        let defence_skill: &Skill = &self.defence_skill(pmods).expect(msg);
        let effective_def_lvl = target.levels.get(defence_skill).unwrap();

        let monster_bns_stats: &BonusStats = target.monster_bonus.get_bonus_stats();
        let def_bns: &i32 = monster_bns_stats.get(&(dt, *defence_skill)).unwrap();

        let drms: &[f64] = &pmods.get_all_drms();

        maximum_roll(effective_def_lvl, def_bns, drms)
    }

    fn accuracy(&self, pmods: &PlayerModifiers) -> f64 {
        let atk_roll: &i32 = &self.attack_maximum_roll(pmods);
        let def_roll: &i32 = &self.defence_maximum_roll(pmods);

        accuracy(atk_roll, def_roll)
    }

    // fn damage_distribution(&self, pmods: &PlayerModifiers) -> DamageDistribution {}
}
