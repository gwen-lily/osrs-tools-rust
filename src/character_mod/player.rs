use crate::{
    bonus::gear_mod::gear::Slot,
    bonus::gear_mod::{equipment::Equipment, gear::Gear},
    boost_mod::boost::Boost,
    data::Slayer,
    effect_mod::effect::EffectLike,
    levels::Levels,
    prayer_mod::prayer::{Prayer, Prayers},
};

/** The Player struct describes a player in OSRS.
 */
pub struct Player {
    equipment: Equipment,
    levels: Levels,
    prayers: Option<Prayers>,
    boosts: Boost,
    effects: Vec<Box<dyn EffectLike>>,
    kandarin_hard: bool,
    special_energy: u32,
    run_energy: u32,
    slayer_task: Option<Slayer>,
}

impl Player {
    pub fn weapon(&self) -> &Gear {
        if let Some(gear) = self.equipment.get(&Slot::Weapon) {
            gear
        } else {
            todo!() // &Gear::hands()
        }
    }

    // pub fn pray(&mut self, prayer: &'static Prayer) {
    //     match &self.prayers {
    //         Some(prys) => {
    //             prys.prayers.insert(0, prayer);
    //         }
    //         None => {
    //             self.prayers = Some(Prayers::new(vec![prayer]));
    //         }
    //     }
    // }
}

// impl Player {
//     /// Return a box with an item that implements WeaponLike.
//     pub fn weapon(&self) -> Weapon {
//         // If Equipment contains a Weapon, return a box with a clone of it.
//         // Otherwise, return a Box with the default hands in it.
//         if let Some(wpn) = self.equipment.get(&Slot::Weapon) {
//             return *wpn.clone();
//         } else {
//             todo!() // return Box::new(Weapon::hands());
//         }
//     }
// }
