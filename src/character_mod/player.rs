use crate::{
    bonus::gear_mod::{equipment::Equipment, weapon::WeaponLike},
    boost_mod::boost::Boost,
    effect_mod::effect::EffectLike,
    levels::Levels,
    prayer_mod::prayer::PrayerCollection,
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
    // pub fn weapon(&self) -> Box<dyn WeaponLike> {}
}
