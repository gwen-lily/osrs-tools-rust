mod ancient_spells;
mod powered_spells;
mod standard_spells;

use crate::spell_mod::spell::SpellLike;
use std::collections::HashMap;

use ancient_spells::add_ancient_spells;
use powered_spells::add_powered_spells;
use standard_spells::add_standard_spells;

pub type SpellsMap = HashMap<SpellName, Box<dyn SpellLike>>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum SpellName {
    WindStrike,
    WaterStrike,
    EarthStrike,
    FireStrike,
    WindBolt,
    WaterBolt,
    EarthBolt,
    FireBolt,
    WindBlast,
    WaterBlast,
    EarthBlast,
    FireBlast,
    WindWave,
    WaterWave,
    EarthWave,
    FireWave,
    WindSurge,
    WaterSurge,
    EarthSurge,
    FireSurge,
    CrumbleUndead,
    IbanBlast,
    SaradominStrike,
    ClawsOfGuthix,
    FlamesOfZamorak,
    SmokeRush,
    ShadowRush,
    BloodRush,
    IceRush,
    SmokeBurst,
    ShadowBurst,
    BloodBurst,
    IceBurst,
    SmokeBlitz,
    ShadowBlitz,
    BloodBlitz,
    IceBlitz,
    SmokeBarrage,
    ShadowBarrage,
    BloodBarrage,
    IceBarrage,
    TridentOfTheSeas(i32),
    TridentOfTheSwamp(i32),
    SanguinestiStaff(i32),
    TumekensShadow(i32),
}

#[allow(dead_code)]
pub fn get_all_spells() -> SpellsMap {
    let mut map = SpellsMap::new();

    add_standard_spells(&mut map);
    add_powered_spells(&mut map);
    add_ancient_spells(&mut map);

    map
}
