use crate::character::Player;
use crate::modifiers::{ArMod, ConditionalModifier, DmgMod};
use crate::GEAR_SETS_MAP;

use crate::bonus::{EquipmentMap, EquipmentNameMap, SetName::*};

pub(crate) struct CrystalModifier<'a> {
    pub(crate) player: &'a Player,
}

impl<'a> ConditionalModifier for CrystalModifier<'a> {
    fn activate(&self) -> bool {
        self.player.equipment_info.crystal_weapon_equipped()
    }
}

impl<'a> CrystalModifier<'a> {
    fn crystal_pieces_equipped(&self) -> usize {
        let eqpd: &EquipmentMap = self.player.eqpd();
        let mut pieces_cnt: usize = 0;
        let crystal_pieces: &EquipmentNameMap = GEAR_SETS_MAP.get(&CrystalSet).unwrap();

        for (slot, gear_name) in crystal_pieces.iter() {
            if let Some(g) = eqpd.get(slot) {
                if g.gear_info.name == *gear_name {
                    pieces_cnt += 1;
                };
            };
        }

        pieces_cnt
    }
}

impl<'a> ArMod for CrystalModifier<'a> {
    fn accuracy_roll_mod(&self) -> Option<f64> {
        if !self.activate() {
            return None;
        };

        let set_bonus: f64 = 0.12;
        let pce_bonus: f64 = 0.06;

        match self.crystal_pieces_equipped() {
            0 => None,
            1 => Some(1.0 + pce_bonus),
            2 => Some(1.0 + 2.0 * pce_bonus),
            3 => Some(1.0 + 3.0 * pce_bonus + set_bonus),
            _ => panic!(),
        }
    }
}

impl<'a> DmgMod for CrystalModifier<'a> {
    fn damage_mod(&self) -> Option<f64> {
        if !self.activate() {
            return None;
        };

        let set_bonus: f64 = 0.06;
        let pce_bonus: f64 = 0.03;

        match self.crystal_pieces_equipped() {
            0 => None,
            1 => Some(1.0 + pce_bonus),
            2 => Some(1.0 + 2.0 * pce_bonus),
            3 => Some(1.0 + 3.0 * pce_bonus + set_bonus),
            _ => panic!(),
        }
    }
}
