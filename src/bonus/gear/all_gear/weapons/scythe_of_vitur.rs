use crate::bonus::gear::weapon::weapon_trait::WeaponTrait;
use crate::combat::damage_distribution::Hitsplats;
use crate::combat::DamageDistribution;
use crate::modifiers::PlayerModifiers;

pub struct ScytheOfVitur {}

impl WeaponTrait for ScytheOfVitur {
    fn damage_distribution(&self, pmods: &PlayerModifiers) -> Hitsplats {
        let max_hit: u32 = self.actual_max(pmods);
        let accuracy: f64 = self.accuracy(pmods);
        let hp_cap: u32 = self.hp_cap(pmods);

        let mut hitsplats: Hitsplats = vec![];

        // Standard hit
        hitsplats.push(DamageDistribution::simple(max_hit, accuracy, Some(hp_cap)));

        // Half dmg hit & quarter dmg hit
        for mod_pow in (-2..=-1).rev() {
            let sub_max: u32 = ((max_hit as f64) * 2.0_f64.powi(mod_pow)).trunc() as u32;
            let hs: DamageDistribution =
                DamageDistribution::simple(sub_max, accuracy, Some(hp_cap));
            hitsplats.push(hs);
        }

        assert_eq!(3, hitsplats.len());

        hitsplats
    }
}
