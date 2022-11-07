use super::DamageDistribution;

pub struct DamageAnalysis {
    pub damage: Vec<u32>,
    pub probability: Vec<f64>,
    pub min_hit: u32,
    pub max_hit: u32,
    pub mean_hit: f64,
    pub probability_of_nonzero_damage: f64,
}

impl From<DamageDistribution> for DamageAnalysis {
    fn from(other: DamageDistribution) -> Self {
        let damage: Vec<u32> = other.damage;
        let probability: Vec<f64> = other.probability;

        let mut min_hit: u32 = 0;

        for prb in probability.iter() {
            if *prb > 0.0 {
                break;
            };
            min_hit += 1;
        }

        let max_hit: u32 = *damage.get(damage.len()).unwrap();

        let mut mean_hit: f64 = 0.0;

        for (dmg, prb) in damage.iter().zip(probability.iter()) {
            mean_hit += (*dmg as f64) * prb;
        }

        let probability_of_nonzero_damage: f64 = 1.0 - probability[0];

        Self {
            damage,
            probability,
            min_hit,
            max_hit,
            mean_hit,
            probability_of_nonzero_damage,
        }
    }
}
