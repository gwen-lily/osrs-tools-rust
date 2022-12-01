mod ancient_spells;
mod powered_spells;
mod standard_spells;

use std::collections::HashMap;

use ancient_spells::add_ancient_spells;
use powered_spells::add_powered_spells;
use standard_spells::add_standard_spells;

use crate::spell::Spell;

pub type SpellsMap = HashMap<SpellName, Spell>;

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
    TridentOfTheSeas,
    TridentOfTheSwamp,
    SanguinestiStaff,
    TumekensShadow,
}

#[allow(dead_code)]
pub fn get_all_spells() -> SpellsMap {
    let mut map = SpellsMap::new();

    add_standard_spells(&mut map);
    add_powered_spells(&mut map);
    add_ancient_spells(&mut map);

    map
}
