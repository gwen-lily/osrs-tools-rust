use crate::bonus::Weapon;
use crate::character::Player;
use crate::data::Skill;
use crate::modifiers::DmgBuff;

use crate::bonus::GearName::*;

pub(crate) struct PoweredStaffModifiers<'a> {
    pub(crate) player: &'a Player,
}

impl<'a> DmgBuff for PoweredStaffModifiers<'a> {
    fn damage_buff(&self) -> Option<u32> {
        let wpn: &Weapon = self.player.weapon();
        let visible_magic_level: u32 = self.player.get_visible_level(Skill::Magic);

        match wpn.gear_info.name {
            TumekensShadow => {
                let bonus_dmg: u32 = visible_magic_level / 3;
                Some(bonus_dmg)
            }
            TridentOfTheSeas | TridentOfTheSwamp | SanguinestiStaff => {
                let clmpd_lvl: u32 = visible_magic_level.max(75);
                let diff_lvl: u32 = clmpd_lvl - 75;
                let bonus_dmg: u32 = diff_lvl / 3;
                Some(bonus_dmg)
            }
            _ => panic!(), // how you got here without a powered staff is beyond me
        }
    }
}
