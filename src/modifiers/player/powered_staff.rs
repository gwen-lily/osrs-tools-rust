use crate::bonus::Gear;
use crate::character::Player;
use crate::data::Skill;
use crate::modifiers::DmgBuff;

use crate::bonus::GearName::*;

pub(crate) struct PoweredStaffModifiers<'a> {
    pub(crate) player: &'a Player,
}

impl<'a> DmgBuff for PoweredStaffModifiers<'a> {
    fn damage_buff(&self) -> Option<u8> {
        let wpn: &Gear = self.player.weapon();
        let visible_magic_level: i32 = self.player.get_visible_level(Skill::Magic);

        match wpn.name {
            TumekensShadow => {
                let bonus_dmg: u8 = (visible_magic_level / 3) as u8;
                Some(bonus_dmg)
            }
            TridentOfTheSeas | TridentOfTheSwamp | SanguinestiStaff => {
                let clmpd_lvl: i32 = visible_magic_level.max(75);
                let diff_lvl: i32 = clmpd_lvl - 75;
                let bonus_dmg: u8 = (diff_lvl / 3) as u8;
                Some(bonus_dmg)
            }
            _ => panic!(), // how you got here without a powered staff is beyond me
        }
    }
}