use crate::{
    boost_mod::boost::Boost, effect_mod::effect::EffectLike, levels::Levels,
    loadout::equipment::Equipment, prayer_mod::prayer::PrayerCollection,
};

/** The Player struct describes a player in OSRS.
 */
#[allow(dead_code)]
pub struct Player {
    pub equipment: Equipment,
    levels: Levels,
    prayers: Option<PrayerCollection>,
    boosts: Boost,
    effects: Vec<Box<dyn EffectLike>>,
}
