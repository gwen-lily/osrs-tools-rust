use crate::bonus::gear::weapon::weapon_trait::WeaponTrait;
use crate::data::{CombatAspect, DT};
use crate::levels;
use crate::modifiers::PlayerModifiers;
use crate::Skill;

pub struct Seercull {}

impl WeaponTrait for Seercull {
    fn get_invisible_level(&self, pmods: &PlayerModifiers, dt: DT, aspect: CombatAspect) -> u32 {
        // If the seercull performs a special attack...
        if *pmods.special_attack {
            // Invisible ranged strength level does not benefit from prayer bonus
            if dt == DT::Ranged && aspect == CombatAspect::Strength {
                let combat_skill: Skill = levels::combat_skill(dt, aspect);
                pmods.player.get_visible_level(combat_skill)
            } else {
                pmods.player.get_invisible_level(dt, aspect)
            }
        } else {
            pmods.player.get_invisible_level(dt, aspect)
        }
    }

    fn accuracy(&self, pmods: &PlayerModifiers) -> f64 {
        if *pmods.special_attack {
            1.0
        } else {
            WeaponTrait::accuracy(self, pmods)
        }
    }
}
