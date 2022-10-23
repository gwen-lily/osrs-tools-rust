use crate::{
    bonus::gear_mod::{equipment::Equipment, gear_bonus::GearLike, weapon::WeaponLike},
    boost_mod::boost::Boost,
    data::Slot,
    effect_mod::effect::EffectLike,
    levels::Levels,
    prayer_mod::prayer::PrayerCollection,
    style_mod::style::Styles,
};

/** The Player struct describes a player in OSRS.
 */
#[allow(dead_code)]
pub struct Player {
    equipment: Equipment,
    levels: Levels,
    prayers: Option<PrayerCollection>,
    boosts: Boost,
    effects: Vec<Box<dyn EffectLike>>,
}

impl Player {
    // pub fn weapon(&self) -> Box<dyn WeaponLike> {
    //     if let Some(wpn) = self.equipment.get(&Slot::Weapon) {
    //         return Box::new(wpn);
    //     } else {
    //         panic!("panic");
    //         // return Weapon::hands();
    //     }
    // }
}
