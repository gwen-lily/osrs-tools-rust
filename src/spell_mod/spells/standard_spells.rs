use crate::spell_mod::spell::Aspect::*;
use crate::spell_mod::spell::Spell;

use super::SpellName::*;
use super::SpellsMap;

pub fn add_standard_spells(map: &mut SpellsMap) {
    // Strike
    let key = WindStrike;
    let val = Spell::standard(2, Some(Wind));
    map.insert(key, val);

    let key = WaterStrike;
    let val = Spell::standard(4, Some(Water));
    map.insert(key, val);

    let key = EarthStrike;
    let val = Spell::standard(6, Some(Earth));
    map.insert(key, val);

    let key = FireStrike;
    let val = Spell::standard(8, Some(Fire));
    map.insert(key, val);

    // Bolt
    let key = WindBolt;
    let val = Spell::standard(9, Some(Wind));
    map.insert(key, val);

    let key = WaterBolt;
    let val = Spell::standard(10, Some(Water));
    map.insert(key, val);

    let key = EarthBolt;
    let val = Spell::standard(11, Some(Earth));
    map.insert(key, val);

    let key = FireBolt;
    let val = Spell::standard(12, Some(Fire));
    map.insert(key, val);

    // Blast
    let key = WindBlast;
    let val = Spell::standard(13, Some(Wind));
    map.insert(key, val);

    let key = WaterBlast;
    let val = Spell::standard(14, Some(Water));
    map.insert(key, val);

    let key = EarthBlast;
    let val = Spell::standard(15, Some(Earth));
    map.insert(key, val);

    let key = FireBlast;
    let val = Spell::standard(16, Some(Fire));
    map.insert(key, val);

    // Wave
    let key = WindWave;
    let val = Spell::standard(17, Some(Wind));
    map.insert(key, val);

    let key = WaterWave;
    let val = Spell::standard(18, Some(Water));
    map.insert(key, val);

    let key = EarthWave;
    let val = Spell::standard(19, Some(Earth));
    map.insert(key, val);

    let key = FireWave;
    let val = Spell::standard(20, Some(Fire));
    map.insert(key, val);

    // Surge
    let key = WindSurge;
    let val = Spell::standard(21, Some(Wind));
    map.insert(key, val);

    let key = WaterSurge;
    let val = Spell::standard(22, Some(Water));
    map.insert(key, val);

    let key = EarthSurge;
    let val = Spell::standard(23, Some(Earth));
    map.insert(key, val);

    let key = FireSurge;
    let val = Spell::standard(24, Some(Fire));
    map.insert(key, val);

    // Uniques
    let key = CrumbleUndead;
    let val = Spell::standard(15, None);
    map.insert(key, val);

    let key = IbanBlast;
    let val = Spell::standard(25, None);
    map.insert(key, val);

    // God spells
    let key = SaradominStrike;
    let val = Spell::standard(20, None);
    map.insert(key, val);

    let key = ClawsOfGuthix;
    let val = Spell::standard(20, None);
    map.insert(key, val);

    let key = FlamesOfZamorak;
    let val = Spell::standard(20, None);
    map.insert(key, val);
}
