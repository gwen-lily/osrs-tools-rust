pub type Hitsplats = Vec<DamageDistribution>;

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug)]
pub struct DamageDistribution {
    damage: Vec<u32>,
    probability: Vec<f64>,
}

impl DamageDistribution {
    pub fn new(damage: Vec<u32>, probability: Vec<f64>, hp_cap: Option<u32>) -> Self {
        // clamp to target Hitpoints if provided
        if let Some(cap) = hp_cap {
            let max_damage: u32 = (damage.len() - 1) as u32;
            if cap > max_damage {
                // clamped distribution from 0..=cap
                let damage_adj: Vec<u32> = (0..=cap).collect();
                let mut probability_adj: Vec<f64> = Vec::new();

                // from 0..cap, copy values
                for i in 0..cap {
                    probability_adj.push(*probability.get(i as usize).unwrap());
                }

                let mut remainder_sum: f64 = 0.0;

                // from cap..=max_hit, sum probabilities into one value
                for i in cap..=max_damage {
                    remainder_sum += probability.get(i as usize).unwrap();
                }

                // and assign to the capped hit probability
                probability_adj.push(remainder_sum);

                // Space of possible events
                let mut probability_space: f64 = 0.0;
                for prb in probability.iter() {
                    probability_space += prb;
                }

                // Assert complete and matching probability space
                assert_eq!(damage_adj.len(), probability_adj.len());
                assert!(approx_eq!(f64, 1.0, probability_space, ulps = 2));

                return Self {
                    damage: damage_adj,
                    probability: probability_adj,
                };
            }
        }

        // Space of possible events
        let mut probability_space: f64 = 0.0;
        for prb in probability.iter() {
            probability_space += prb;
        }

        // Assert complete and matching probability space
        assert_eq!(damage.len(), probability.len());
        assert!(approx_eq!(f64, 1.0, probability_space, ulps = 2));

        Self {
            damage,
            probability,
        }
    }

    /// The most common and simple way to create a damage distribution
    pub fn simple(max_hit: u32, accuracy: f64, hp_cap: Option<u32>) -> Self {
        let damage: Vec<u32> = (0..=max_hit).collect();
        let damage_options: usize = damage.len();

        let prb: f64 = accuracy * 1.0 / (damage_options as f64);
        let mut probability: Vec<f64> = Vec::new();

        for _ in damage.iter() {
            probability.push(prb);
        }

        // Probability of a miss
        probability[0] += 1.0 - accuracy;

        Self::new(damage, probability, hp_cap)
    }
}
