use crate::bonus::WeaponTrait;
use crate::combat::{multiply_then_trunc, DamageDistribution, Hitsplats};
use crate::modifiers::PlayerModifiers;

struct DragonClaws {}

impl WeaponTrait for DragonClaws {
    fn damage_distribution(&self, pmods: &PlayerModifiers) -> Hitsplats {
        let max_hit: u32 = self.actual_max(pmods);
        let accuracy: f64 = self.accuracy(pmods);
        let hp_cap: Option<u32> = Some(self.hp_cap(pmods));

        // scenario A: first attack is successful (4-2-1-1)
        let p_a: f64 = accuracy;
        let min_hit_a: u32 = multiply_then_trunc(max_hit, 0.5);
        let max_hit_a: u32 = max_hit - 1;

        let dmg_a1: Vec<u32> = (min_hit_a..=max_hit_a).collect();
        let dmg_a2: Vec<u32> = dmg_a1
            .iter()
            .map(|&d| multiply_then_trunc(d, 0.5))
            .collect();
        let dmg_a3: Vec<u32> = dmg_a2
            .iter()
            .map(|&d| multiply_then_trunc(d, 0.5))
            .collect();
        let dmg_a4: &Vec<u32> = &dmg_a3;
        let n_a: usize = dmg_a1.len();

        // scenario B: second attack is successful (0-4-2-2)
        let p_b: f64 = accuracy * (1.0 - accuracy);
        let min_hit_b: u32 = multiply_then_trunc(max_hit, 3.0 / 8.0);
        let max_hit_b: u32 = multiply_then_trunc(max_hit, 7.0 / 8.0);

        let dmg_b2: Vec<u32> = (min_hit_b..=max_hit_b).collect();
        let dmg_b3: Vec<u32> = dmg_b2
            .iter()
            .map(|&d| multiply_then_trunc(d, 0.5))
            .collect();
        let dmg_b4: &Vec<u32> = &dmg_b3;
        let n_b: usize = dmg_b2.len();

        // scenario C: third attack is successful (0-0-3-3)
        let p_c: f64 = accuracy * (1.0 - accuracy).powi(2);
        let min_hit_c: u32 = multiply_then_trunc(max_hit, 1.0 / 4.0);
        let max_hit_c: u32 = multiply_then_trunc(max_hit, 3.0 / 4.0);

        let dmg_c3: Vec<u32> = (min_hit_c..=max_hit_c).collect();
        let dmg_c4: &Vec<u32> = &dmg_c3;
        let n_c: usize = dmg_c3.len();

        // scenario D: fourth attack is successful (0-0-0-5)
        let p_d: f64 = accuracy * (1.0 - accuracy).powi(3);
        let min_hit_d: u32 = multiply_then_trunc(max_hit, 1.0 / 4.0);
        let max_hit_d: u32 = multiply_then_trunc(max_hit, 5.0 / 4.0);

        let dmg_d4: Vec<u32> = (min_hit_d..=max_hit_d).collect();
        let n_d: usize = dmg_d4.len();

        // scenario E: the big buh (0-0-0-0) / (0-0-1-1)
        let p_e = (1.0 - accuracy).powi(4);

        // complete probability check
        let p_sum: f64 = p_a + p_b + p_c + p_d + p_e;
        assert!(approx_eq!(f64, p_sum, 1.0, ulps = 2));

        // hitsplat 1
        let hs_1_dmg: Vec<u32> = (0..=max_hit_a).collect();
        let mut hs_1_prb: Vec<f64> = Vec::new();

        for dmg in hs_1_dmg.iter() {
            let mut dmg_prb: f64 = 0.0;

            if dmg_a1.contains(dmg) {
                dmg_prb += p_a / n_a as f64;
            };

            hs_1_prb.push(dmg_prb);
        }

        // miss probability
        hs_1_prb[0] += p_b + p_c + p_d + p_e;

        // hitsplat 2
        let hs_2_dmg: Vec<u32> = (0..=max_hit_b).collect();
        let mut hs_2_prb: Vec<f64> = Vec::new();

        for dmg in hs_2_dmg.iter() {
            let mut dmg_prb: f64 = 0.0;

            if dmg_a2.contains(dmg) {
                dmg_prb += p_a / n_a as f64;
            };

            if dmg_b2.contains(dmg) {
                dmg_prb += p_b / n_b as f64;
            };

            hs_2_prb.push(dmg_prb);
        }

        // miss probability
        hs_2_prb[0] += p_c + p_d + p_e;

        // hitsplat 3
        let hs_3_dmg: Vec<u32> = (0..=max_hit_c).collect();
        let mut hs_3_prb: Vec<f64> = Vec::new();

        for dmg in hs_3_dmg.iter() {
            let mut dmg_prb: f64 = 0.0;

            if dmg_a3.contains(dmg) {
                dmg_prb += p_a / n_a as f64;
            };

            if dmg_b3.contains(dmg) {
                dmg_prb += p_b / n_b as f64;
            };

            if dmg_c3.contains(dmg) {
                dmg_prb += p_c / n_c as f64;
            }

            hs_3_prb.push(dmg_prb);
        }

        // miss probability
        hs_3_prb[0] += p_d + 0.5 * p_e;
        hs_3_prb[1] += p_e * 0.5;

        // hitsplat 4
        let hs_4_dmg: Vec<u32> = (0..=max_hit_c).collect();
        let mut hs_4_prb: Vec<f64> = Vec::new();

        for dmg in hs_4_dmg.iter() {
            let mut dmg_prb: f64 = 0.0;

            if dmg_a4.contains(dmg) {
                dmg_prb += p_a / n_a as f64;
            };

            if dmg_b4.contains(dmg) {
                dmg_prb += p_b / n_b as f64;
            };

            if dmg_c4.contains(dmg) {
                dmg_prb += p_c / n_c as f64;
            }

            if dmg_d4.contains(dmg) {
                dmg_prb += p_d / n_d as f64;
            }

            hs_4_prb.push(dmg_prb);
        }

        // miss probability
        hs_4_prb[0] += p_e * 0.5;
        hs_4_prb[1] += p_e * 0.5;

        vec![
            DamageDistribution::new(hs_1_dmg, hs_1_prb, hp_cap),
            DamageDistribution::new(hs_2_dmg, hs_2_prb, hp_cap),
            DamageDistribution::new(hs_3_dmg, hs_3_prb, hp_cap),
            DamageDistribution::new(hs_4_dmg, hs_4_prb, hp_cap),
        ]
    }
}
