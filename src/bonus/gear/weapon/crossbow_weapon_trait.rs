use super::weapon_trait::WeaponTrait;
use crate::{
    bonus::{Gear, Slot},
    character::Player,
    combat::{crossbow_proc_chance, multiply_then_trunc, DamageDistribution, Hitsplats},
    data::bolts::{
        DIAMOND_BOLTS_DMG, DIAMOND_BOLTS_PROC, RUBY_BOLTS_HP_CAP, RUBY_BOLTS_HP_RATIO,
        RUBY_BOLTS_PROC,
    },
    modifiers::PlayerModifiers,
};

pub trait CrossbowWeaponTrait: WeaponTrait {
    /// Get the bolt effect proc chance
    fn proc_chance(&self, pmods: &PlayerModifiers) -> f64 {
        use crate::bonus::GearName::*;

        let player: &Player = pmods.player;

        let base_proc: f64 = if let Some(ammo) = player
            .equipment_info
            .equipment
            .equipment
            .get(&Slot::Ammunition)
        {
            if player.equipment_info.crossbow_equipped() {
                match ammo.gear_info.name {
                    DiamondBoltsE | DiamondDragonBoltsE => DIAMOND_BOLTS_PROC,
                    RubyBoltsE | RubyDragonBoltsE => RUBY_BOLTS_PROC,
                    _ => 0.0,
                }
            } else {
                0.0
            }
        } else {
            0.0
        };

        crossbow_proc_chance(base_proc, pmods)
    }

    /// Shadows the signature of WeaponTrait::damage_distribution
    fn damage_distribution(&self, pmods: &PlayerModifiers) -> Hitsplats {
        let player: &Player = pmods.player;

        let proc_chance: f64 = self.proc_chance(pmods);
        let normal_accuracy: f64 = self.accuracy(pmods);
        let normal_max_hit: u32 = self.actual_max(pmods);
        let hp_cap: u32 = self.hp_cap(pmods);

        if player.equipment_info.enchanted_bolts_equipped() {
            let ammo: &Gear = player.eqpd().get(&Slot::Ammunition).unwrap();
            use crate::bonus::GearName::*;

            match ammo.gear_info.name {
                RubyBoltsE | RubyDragonBoltsE => {
                    let hp_ratio: f64 = if player.equipment_info.zaryte_crossbow_equipped() {
                        RUBY_BOLTS_HP_RATIO * 1.10
                    } else {
                        RUBY_BOLTS_HP_RATIO
                    };

                    // The damage dealt by the effect before setting a hard cap
                    let uncapped_effect_max_hit: u32 = multiply_then_trunc(hp_cap, hp_ratio);
                    // The damage dealt by the effect at the cap
                    let maximum_effect_max_hit: u32 =
                        multiply_then_trunc(RUBY_BOLTS_HP_CAP, hp_ratio);

                    // The max hit of the effect, capped
                    let effect_max_hit: u32 = uncapped_effect_max_hit.min(maximum_effect_max_hit);
                    // The true max hit across spec & non-spec hits
                    let true_max_hit: u32 = normal_max_hit.max(effect_max_hit);

                    let damage: Vec<u32> = (0..=true_max_hit).collect();
                    let mut probability: Vec<f64> = Vec::new();

                    for dmg in damage.iter() {
                        let mut dmg_prb: f64 = 0.0;

                        if *dmg == 0 {
                            dmg_prb += (1.0 - proc_chance) * (1.0 - normal_accuracy);
                        };

                        if *dmg <= normal_max_hit {
                            dmg_prb +=
                                (1.0 - proc_chance) * normal_accuracy / (normal_max_hit + 1) as f64;
                        };

                        if *dmg == effect_max_hit {
                            dmg_prb += proc_chance;
                        };

                        probability.push(dmg_prb);
                    }

                    vec![DamageDistribution::new(damage, probability, Some(hp_cap))]
                }
                DiamondBoltsE | DiamondDragonBoltsE => {
                    let dmg_mod: f64 = if player.equipment_info.zaryte_crossbow_equipped() {
                        DIAMOND_BOLTS_DMG * 1.10
                    } else {
                        DIAMOND_BOLTS_DMG
                    };

                    let effect_max_hit: u32 = multiply_then_trunc(normal_max_hit, dmg_mod);

                    let damage: Vec<u32> = (0..=effect_max_hit).collect();
                    let mut probability: Vec<f64> = Vec::new();

                    for dmg in 0..=effect_max_hit {
                        let mut dmg_prb: f64 = 0.0;

                        // Miss probability
                        if dmg == 0 {
                            dmg_prb += (1.0 - proc_chance) * (1.0 - normal_accuracy);
                        };

                        // Normal hit distribution
                        if dmg <= normal_max_hit {
                            dmg_prb +=
                                (1.0 - proc_chance) * normal_accuracy / (normal_max_hit + 1) as f64
                        };

                        // Proc hit distribution
                        dmg_prb += proc_chance / (effect_max_hit + 1) as f64;
                        probability.push(dmg_prb);
                    }

                    vec![DamageDistribution::new(damage, probability, Some(hp_cap))]
                }
                DragonstoneDragonBoltsE | DragonstoneBoltsE => todo!(),
                OnyxDragonBoltsE | OnyxBoltsE => todo!(),
                _ => vec![],
            }
        } else {
            WeaponTrait::damage_distribution(self, pmods)
        }
    }
}
