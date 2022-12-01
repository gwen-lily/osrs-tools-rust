use crate::{
    bonus::Slot,
    character::{Monster, Player},
    data::DT,
    modifiers::ConditionalModifier,
};

use crate::bonus::GearName::BrimstoneRing;

pub struct BrimstoneModifier<'a> {
    player: &'a Player,
    target: &'a Monster,
}

impl<'a> ConditionalModifier for BrimstoneModifier<'a> {
    fn activate(&self) -> bool {
        if self.player.style.dt != DT::Magic {
            return false;
        };

        if let Some(ring) = self.player.eqpd().get(&Slot::Ring) {
            if ring.gear_info.name == BrimstoneRing {
                return true;
            };
        }
        false
    }
}

// impl<'a> ArIntMod for BrimstoneModifier<'a> {
//     fn accuracy_roll_int_mod(&self) -> Option<i32> {
//         if self.activate() == false {
//             return false;
//         };
//
//         let r
//     }
// }
