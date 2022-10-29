use crate::{
    bonus::{BonusLike, BonusStats},
    character::{monster::Monster, player::Player},
    combat::base_damage,
    data::{Skill, DT},
    modifiers::{DmgBuff, PlayerModifiers},
    spell::Spell,
    OsrsError,
};

use crate::modifiers::player::powered_staff::PoweredStaffModifiers;

#[derive(Debug, Builder)]
#[builder(pattern = "owned", build_fn(validate = "Self::validate"))]
pub struct MaxHit {
    pub player: Player,
    pub target: Monster,
    pub special_attack: bool,
    pub distance: u8,
    pub adds: u8,
}

impl MaxHitBuilder {
    /// validation function
    fn validate(&self) -> Result<(), String> {
        if let Some(spec_atk) = self.special_attack {
            match spec_atk {
                true => {
                    if let Some(_spc_wpn) = &self.player.as_ref().unwrap().weapon().special_weapon {
                        Ok(())
                    } else {
                        Err(
                            "Attempted to perform special attack with non-special weapon"
                                .to_string(),
                        )
                    }
                }
                false => Ok(()),
            }
        } else {
            Ok(())
        }
    }
}

impl MaxHit {
    fn explore_the_data_available(&self) -> crate::Result<()> {
        use DT::*;
        let ply: &Player = &self.player;
        let tgt: &Monster = &self.target;

        let dt: DT = ply.style.dt;
        let attack_skill: Skill = match dt {
            Melee(_) => Skill::Attack,
            Ranged => Skill::Ranged,
            Magic => Skill::Magic,
            Typeless => return Err(OsrsError::Typeless),
        };

        let effective_attack_level: i32 = ply.get_effective_level(dt, attack_skill);
        let bonus_stats: &BonusStats = ply.equipment_info.equipment.get_bonus_stats();
        let gear_attack_bonus: i32 = *bonus_stats.get(&(dt, attack_skill)).unwrap();

        let base_max: u8 = match dt {
            Melee(_) | Ranged => {
                let strength_skill: Skill = match dt {
                    Melee(_) => Skill::Strength,
                    Ranged => Skill::Ranged,
                    Magic => return Err(OsrsError::Combat),
                    Typeless => return Err(OsrsError::Typeless),
                };
                let effective_strength_level: i32 = ply.get_effective_level(dt, strength_skill);
                let gear_strength_bonus: i32 = *bonus_stats.get(&(dt, strength_skill)).unwrap();
                let base_damage: f64 = base_damage(&effective_strength_level, &gear_strength_bonus);
                base_damage.trunc() as u8
            }
            Magic => {
                // Magic base max comes from spells
                use crate::spell::Spellbook::*;
                let spl: &Spell = ply.spell.unwrap();

                match spl.spellbook {
                    Standard | Ancient | Lunar | Arceus => spl.base_max,
                    Powered => {
                        let calc: PoweredStaffModifiers = PoweredStaffModifiers { player: ply };
                        spl.base_max + calc.damage_buff().unwrap()
                    }
                }
            }
            Typeless => return Err(OsrsError::Typeless),
        };

        let pmods: PlayerModifiers = PlayerModifiers {
            player: ply,
            target: tgt,
            // special_attack: self.special_attack,
            distance: &self.distance,
            // adds: &self.adds,
        };

        let arms = pmods.get_all_arms();
        let drms = pmods.get_all_drms();
        let gear_bonus_mods = pmods.get_all_bns_mods();
        let dms = pmods.get_all_dms();
        let dmg_buffs = pmods.get_all_dmg_buffs();
        let cmbs = pmods.get_all_cmb_mods();

        Ok(())
    }
}
