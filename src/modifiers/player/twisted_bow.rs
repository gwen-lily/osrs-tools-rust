use crate::bonus::BonusLike;
use crate::bonus::GearName::*;
use crate::character::{Monster, MonsterAttribute, Player};
use crate::modifiers::{ArMod, ConditionalModifier, DmgMod};
use crate::{CombatAspect, Skill, DT};

pub(crate) struct TwistedBowModifier<'a> {
    pub(crate) player: &'a Player,
    pub(crate) target: &'a Monster,
}

impl<'a> ConditionalModifier for TwistedBowModifier<'a> {
    fn activate(&self) -> bool {
        self.player.weapon().gear_info.name == TwistedBow
    }
}

impl<'a> ArMod for TwistedBowModifier<'a> {
    fn accuracy_roll_mod(&self) -> Option<f64> {
        if !self.activate() {
            return None;
        };

        let accuracy_mod_ceil: f64 = 1.40;
        let magic: i32 = self.target_magic();

        let accuracy_mod_pcnt = {
            140 + { ((10 * 3 * magic) as f64 / 10.0 - 10.0) / 100.0 }.trunc() as i32 - {
                ((3 * magic) as f64 / 10.0 - 100.0).powi(2) / 100.0
            }
            .trunc()
                as i32
        };

        let accuracy_mod: f64 = accuracy_mod_pcnt as f64 / 100.0;
        let accuracy_mod_clmpd: f64 = accuracy_mod.min(accuracy_mod_ceil);

        Some(accuracy_mod_clmpd)
    }
}

impl<'a> DmgMod for TwistedBowModifier<'a> {
    fn damage_mod(&self) -> Option<f64> {
        if !self.activate() {
            return None;
        };

        let damage_mod_ceil: f64 = 2.50;
        let magic: i32 = self.target_magic();

        let damage_mod_pcnt = {
            250 + { ((10 * 3 * magic) as f64 / 10.0 - 14.0) / 100.0 }.trunc() as i32 - {
                ((3 * magic) as f64 / 10.0 - 140.0).powi(2) / 100.0
            }
            .trunc()
                as i32
        };

        let damage_mod: f64 = damage_mod_pcnt as f64 / 100.0;
        let damage_mod_clmpd: f64 = damage_mod.min(damage_mod_ceil);

        Some(damage_mod_clmpd)
    }
}

impl<'a> TwistedBowModifier<'a> {
    fn target_magic_ceiling(&self) -> i32 {
        if let Some(attrs) = &self.target.attributes {
            if attrs.contains(&MonsterAttribute::Xerician) {
                return 350;
            }
        }
        250
    }

    fn target_magic(&self) -> i32 {
        let magic_lvl: i32 = *self.target.levels.get(&Skill::Magic).unwrap();
        let magic_agg_bns: i32 = *self
            .target
            .monster_bonus
            .get_bonus_stats()
            .get(&(DT::Magic, CombatAspect::Attack))
            .unwrap();

        // order important
        magic_lvl
            .max(magic_agg_bns)
            .min(self.target_magic_ceiling())
    }
}
