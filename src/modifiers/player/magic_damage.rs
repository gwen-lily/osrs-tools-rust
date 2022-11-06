use crate::character::Player;
use crate::modifiers::DmgMod;

use crate::bonus::{BonusLike, BonusStats};
use crate::data::{CombatAspect, DT};

pub(crate) struct MagicDamageModifier<'a> {
    pub(crate) player: &'a Player,
}

impl<'a> DmgMod for MagicDamageModifier<'a> {
    fn damage_mod(&self) -> Option<f64> {
        let ply: &Player = self.player;

        if ply.spell.is_none() || ply.style.dt != DT::Magic {
            return None;
        };

        // Strange definition, preserves the i32 though
        let bns: &BonusStats = ply.equipment_info.equipment.get_bonus_stats();
        let mag_dmg_bonus: f64 =
            (*bns.get(&(DT::Magic, CombatAspect::Strength)).unwrap_or(&0) as f64) / 1000.0;

        let val: f64 = 1.0 + mag_dmg_bonus;
        Some(val)
    }
}
