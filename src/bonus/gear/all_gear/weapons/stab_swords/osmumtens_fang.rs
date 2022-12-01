use crate::bonus::gear::weapon::weapon_trait::WeaponTrait;
use crate::character::{Monster, MonsterAttribute};
use crate::combat::damage_distribution::Hitsplats;
use crate::combat::DamageDistribution;
use crate::modifiers::PlayerModifiers;

pub struct OsmumtensFang {}

impl WeaponTrait for OsmumtensFang {
    fn accuracy(&self, pmods: &PlayerModifiers) -> f64 {
        let target: &Monster = pmods.target;

        // inside toa
        if let Some(attrs) = &target.attributes {
            if attrs.contains(&MonsterAttribute::Toa) {
                let base_accuracy: f64 = WeaponTrait::accuracy(self, pmods);

                return 1.0 - base_accuracy.powi(2);
            };
        };

        // outside toa
        let atk_roll: u32 = self.attack_maximum_roll(pmods);
        let def_roll: u32 = self.defence_maximum_roll(pmods);

        if atk_roll >= def_roll {
            let num: u32 = (def_roll + 2) * (2 * def_roll + 3);
            let den: u32 = 6 * (atk_roll + 1).pow(2);

            1.0 - (num as f64) / (den as f64)
        } else {
            let num: u32 = atk_roll * (4 * atk_roll + 5);
            let den: u32 = 6 * (atk_roll + 1) * (def_roll + 1);

            (num as f64) / (den as f64)
        }
    }

    fn damage_distribution(&self, pmods: &PlayerModifiers) -> Hitsplats {
        let default_max: u32 = self.actual_max(pmods);

        let min_hit: u32 = (0.15 * default_max as f64).trunc() as u32;
        let max_hit: u32 = if *pmods.special_attack {
            default_max
        } else {
            default_max - min_hit
        };

        let hit_options: u32 = max_hit - min_hit + 1;

        let accuracy: f64 = self.accuracy(pmods);
        let prb: f64 = accuracy / hit_options as f64;

        let mut damage: Vec<u32> = Vec::new();
        let mut probability: Vec<f64> = Vec::new();

        for i in 0..=max_hit {
            damage.push(i);

            if (min_hit..=max_hit).contains(&i) {
                probability.push(prb);
            } else {
                probability.push(0.0);
            }
        }

        probability[0] += 1.0 - accuracy;

        let hp_cap: u32 = self.hp_cap(pmods);

        let hs = DamageDistribution::new(damage, probability, Some(hp_cap));
        vec![hs]
    }
}
