use crate::bonus::gear::weapon::weapon_trait::WeaponTrait;
use crate::character::Player;
use crate::combat;
use crate::data::Skill;
use crate::modifiers::PlayerModifiers;

struct AbyssalBludgeon {}

impl WeaponTrait for AbyssalBludgeon {
    fn actual_max(&self, pmods: &PlayerModifiers) -> u32 {
        let base_max: u32 = self.base_max(pmods);
        let mut dms: Vec<f64> = pmods.get_all_dms();

        if *pmods.special_attack {
            let player: &Player = pmods.player;
            let pp_diff: i32 =
                *player.levels.get(&Skill::Prayer).unwrap() - player.resources.prayer as i32;
            let pp_missing: u32 = 0_i32.max(pp_diff) as u32;
            // +0.5% for each prayer point missing
            let dmg_mod: f64 = 1.0 + (0.005 * pp_missing as f64);

            dms.push(dmg_mod)
        }

        combat::max_hit(base_max, &dms)
    }
}
