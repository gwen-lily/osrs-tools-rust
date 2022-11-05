pub struct DamageDistribution {
    damage: Vec<u32>,
    probability: Vec<f64>,
}

impl DamageDistribution {
    pub fn new(damage: Vec<u32>, probability: Vec<f64>) -> Self {
        Self {
            damage,
            probability,
        }
    }

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

        // if max hit is greater than the hp cap
        if let Some(cap) = hp_cap {
            // clamped distribution from 0..=cap
            let damage_adj: Vec<u32> = (0..=cap).collect();
            let mut probability_adj: Vec<f64> = Vec::new();

            // from 0..cap, copy values
            for i in 0..cap {
                probability_adj.push(*probability.get(i as usize).unwrap());
            }

            let mut remainder_sum: f64 = 0.0;

            // from cap..=max_hit, sum probabilities into one value
            for i in cap..=max_hit {
                remainder_sum += probability.get(i as usize).unwrap();
            }

            // and assign to the capped hit probability
            probability_adj.push(remainder_sum);

            assert_eq!(damage_adj.len(), probability_adj.len());

            Self {
                damage: damage_adj,
                probability: probability_adj,
            }
        } else {
            Self {
                damage,
                probability,
            }
        }
    }
}
