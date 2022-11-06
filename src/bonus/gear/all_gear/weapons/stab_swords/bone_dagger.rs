use crate::bonus::gear::weapon::weapon_trait::WeaponTrait;
use crate::combat;
use crate::modifiers::PlayerModifiers;

struct BoneDagger {}

impl WeaponTrait for BoneDagger {
    fn accuracy(&self, pmods: &PlayerModifiers) -> f64 {
        if *pmods.special_attack {
            if let Some(last) = &pmods.target.last_attacked_by {
                if pmods.player != last {
                    return 1.0;
                }
            } else {
                return 1.0;
            }
        }

        let atk_roll: u32 = self.attack_maximum_roll(pmods);
        let def_roll: u32 = self.defence_maximum_roll(pmods);

        combat::accuracy(atk_roll, def_roll)
    }
}
