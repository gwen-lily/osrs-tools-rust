use crate::character::Player;
use crate::modifiers::ArMod;
use crate::style::StyleName::*;

pub(crate) struct ChinModifier<'a> {
    pub(crate) player: &'a Player,
    pub(crate) distance: &'a u8,
}

impl<'a> ArMod for ChinModifier<'a> {
    fn accuracy_roll_mod(&self) -> Option<f64> {
        let ply = self.player;
        let dst = *self.distance;

        match ply.style.name {
            ShortFuse => {
                if dst <= 3 {
                    Some(1.00)
                } else if (4..=6).contains(&dst) {
                    Some(0.75)
                } else {
                    Some(0.50)
                }
            }
            MediumFuse => {
                if (4..=6).contains(&dst) {
                    Some(1.00)
                } else {
                    Some(0.75)
                }
            }
            LongFuse => {
                if dst <= 3 {
                    Some(0.50)
                } else if (4..=6).contains(&dst) {
                    Some(0.75)
                } else {
                    Some(1.00)
                }
            }
            _ => None,
        }
    }
}
