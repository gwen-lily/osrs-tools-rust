use crate::character::monster::MonsterName;
use crate::character::{Monster, Player};
use crate::data::Skill;
use crate::modifiers::{ConditionalModifier, DmgMod};
use crate::STYLES_MAP;

use crate::bonus::GearLike;
use crate::style::StylesCategory;

pub(crate) struct GuardianModifier<'a> {
    pub(crate) player: &'a Player,
    pub(crate) target: &'a Monster,
}

impl<'a> ConditionalModifier for GuardianModifier<'a> {
    fn activate(&self) -> bool {
        self.target.name == MonsterName::StoneGuardian
    }
}

impl<'a> DmgMod for GuardianModifier<'a> {
    fn damage_mod(&self) -> Option<f64> {
        if !self.activate() {
            return None;
        };

        let style = self.player.style;
        let pick_styles = STYLES_MAP.get(&StylesCategory::Pickaxes).unwrap();

        if pick_styles.styles.contains(style) {
            let mining_req: u32 = *self
                .player
                .weapon()
                .get_lvl_reqs()
                .get(&Skill::Mining)
                .unwrap() as u32;

            let mining_clmp: u32 = self.player.get_visible_level(Skill::Mining).min(100);
            let dm: f64 = (50 + mining_clmp + mining_req) as f64 / 150.0;
            Some(dm)
        } else {
            None
        }
    }
}
