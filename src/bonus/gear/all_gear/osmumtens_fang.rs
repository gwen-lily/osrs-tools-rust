use crate::bonus::gear::weapon::weapon_trait::WeaponTrait;
use crate::character::{Monster, MonsterAttribute};
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
        let atk_roll: i32 = self.attack_maximum_roll(pmods);
        let def_roll: i32 = self.defence_maximum_roll(pmods);

        if atk_roll >= def_roll {
            let num: i32 = (def_roll + 2) * (2 * def_roll + 3);
            let den: i32 = 6 * (atk_roll + 1).pow(2);

            1.0 - (num as f64) / (den as f64)
        } else {
            let num: i32 = atk_roll * (4 * atk_roll + 5);
            let den: i32 = 6 * (atk_roll + 1) * (def_roll + 1);

            (num as f64) / (den as f64)
        }
    }
}
