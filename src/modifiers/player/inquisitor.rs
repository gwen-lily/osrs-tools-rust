use crate::character::Player;
use crate::data::MeleeDamageType::Crush;
use crate::data::DT;
use crate::modifiers::{ArMod, ConditionalModifier, DmgMod};
use crate::GEAR_SETS_MAP;

use crate::bonus::{EquipmentMap, EquipmentNameMap, SetName::*};

pub(crate) struct InquisitorModifier<'a> {
    pub(crate) player: &'a Player,
}

impl<'a> ConditionalModifier for InquisitorModifier<'a> {
    fn activate(&self) -> bool {
        self.player.style.dt == DT::Melee(Crush)
    }
}

impl<'a> InquisitorModifier<'a> {
    fn inquisitor_pieces_equipped(&self) -> usize {
        if self.player.equipment_info.set_equipped(&InquisitorSet) {
            return 3;
        };

        let eqpd: &EquipmentMap = self.player.eqpd();
        let mut pieces_cnt: usize = 0;
        let inquis_pieces: &EquipmentNameMap = GEAR_SETS_MAP.get(&InquisitorSet).unwrap();

        for (slot, gear_name) in inquis_pieces.iter() {
            if let Some(g) = eqpd.get(slot) {
                if g.gear_info.name == *gear_name {
                    pieces_cnt += 1;
                };
            };
        }

        pieces_cnt
    }
}

impl<'a> ArMod for InquisitorModifier<'a> {
    fn accuracy_roll_mod(&self) -> Option<f64> {
        if !self.activate() {
            return None;
        };

        let set_bonus: f64 = 0.01;
        let pce_bonus: f64 = 0.005;

        match self.inquisitor_pieces_equipped() {
            0 => None,
            1 => Some(1.0 + pce_bonus),
            2 => Some(1.0 + 2.0 * pce_bonus),
            3 => Some(1.0 + 3.0 * pce_bonus + set_bonus),
            _ => panic!(),
        }
    }
}

impl<'a> DmgMod for InquisitorModifier<'a> {
    fn damage_mod(&self) -> Option<f64> {
        if !self.activate() {
            return None;
        };

        let set_bonus: f64 = 0.01;
        let pce_bonus: f64 = 0.005;

        match self.inquisitor_pieces_equipped() {
            0 => None,
            1 => Some(1.0 + pce_bonus),
            2 => Some(1.0 + 2.0 * pce_bonus),
            3 => Some(1.0 + 3.0 * pce_bonus + set_bonus),
            _ => panic!(),
        }
    }
}
