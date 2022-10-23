use crate::boost_mod::boost::Boost;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Effect {
    StaffOfTheDead,
    StaminaPotion,
    Potion(Boost),
    DivinePotion(Boost),
    Overload(Boost),
    RegenerateSpecialEnergy,
    UpdateStats,
    Olm(OlmEffect),
    PrayerDrain(i32),
    Frozen,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum OlmEffect {
    Burn,
    Acid,
    FallingCrystal,
}

pub trait EffectLike {
    fn get_effect(&self) -> Effect;
    fn modify_timer(&self, value: &i32);
    fn get_time_left(&self) -> i32;
}
