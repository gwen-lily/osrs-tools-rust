use crate::{
    boost_mod::boost::Boost, effect_mod::effect::EffectLike, loadout::equipment::Equipment,
    prayer_mod::prayer::PrayerCollection, stat::basic_stat::Levels,
};

#[allow(dead_code)]
pub struct Player {
    equipment: Equipment,
    levels: Levels,
    prayers: Option<PrayerCollection>,
    boosts: Boost,
    effects: Vec<Box<dyn EffectLike>>,
}
