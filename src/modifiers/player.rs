use crate::{character::player::Player, levels::Levels};

use super::LvlMod;

pub struct VoidModifiers<'a> {
    player: &'a Player,
}

impl<'a> LvlMod for VoidModifiers<'a> {
    fn levels_mod(&self) -> Option<Levels> {
        let ply: &Player = self.player;

        // Regular void equipped, I can see the false being confusing
        if ply.equipment_info.void_equipped(false) {
            let mut lvl_mod = Levels::new();

            Some(lvl_mod)
        // Elite void equipped
        } else if ply.equipment_info.void_equipped(true) {
            let mut lvl_mod = Levels::new();

            Some(lvl_mod)
        } else {
            None
        }
    }
}
