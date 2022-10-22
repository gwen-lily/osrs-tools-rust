use crate::spell_mod::spell::StandardSpell;

use super::SpellName::*;
use super::SpellsMap;

pub fn add_standard_spells(map: &mut SpellsMap) {
    // Strike
    let key = WindStrike;
    let val = Box::new(StandardSpell::new(2));
    map.insert(key, val);

    let key = WaterStrike;
    let val = Box::new(StandardSpell::new(4));
    map.insert(key, val);

    let key = EarthStrike;
    let val = Box::new(StandardSpell::new(6));
    map.insert(key, val);

    let key = FireStrike;
    let val = Box::new(StandardSpell::new(8));
    map.insert(key, val);

    // Bolt
    let key = WindBolt;
    let val = Box::new(StandardSpell::new(9));
    map.insert(key, val);

    let key = WaterBolt;
    let val = Box::new(StandardSpell::new(10));
    map.insert(key, val);

    let key = EarthBolt;
    let val = Box::new(StandardSpell::new(11));
    map.insert(key, val);

    let key = FireBolt;
    let val = Box::new(StandardSpell::new(12));
    map.insert(key, val);

    // Blast
    let key = WindBlast;
    let val = Box::new(StandardSpell::new(13));
    map.insert(key, val);

    let key = WaterBlast;
    let val = Box::new(StandardSpell::new(14));
    map.insert(key, val);

    let key = EarthBlast;
    let val = Box::new(StandardSpell::new(15));
    map.insert(key, val);

    let key = FireBlast;
    let val = Box::new(StandardSpell::new(16));
    map.insert(key, val);

    // Wave
    let key = WindWave;
    let val = Box::new(StandardSpell::new(17));
    map.insert(key, val);

    let key = WaterWave;
    let val = Box::new(StandardSpell::new(18));
    map.insert(key, val);

    let key = EarthWave;
    let val = Box::new(StandardSpell::new(19));
    map.insert(key, val);

    let key = FireWave;
    let val = Box::new(StandardSpell::new(20));
    map.insert(key, val);

    // Surge
    let key = WindSurge;
    let val = Box::new(StandardSpell::new(21));
    map.insert(key, val);

    let key = WaterSurge;
    let val = Box::new(StandardSpell::new(22));
    map.insert(key, val);

    let key = EarthSurge;
    let val = Box::new(StandardSpell::new(23));
    map.insert(key, val);

    let key = FireSurge;
    let val = Box::new(StandardSpell::new(24));
    map.insert(key, val);

    // Uniques
    let key = CrumbleUndead;
    let val = Box::new(StandardSpell::new(15));
    map.insert(key, val);

    let key = IbanBlast;
    let val = Box::new(StandardSpell::new(25));
    map.insert(key, val);

    // God spells
    let key = SaradominStrike;
    let val = Box::new(StandardSpell::new(20));
    map.insert(key, val);

    let key = ClawsOfGuthix;
    let val = Box::new(StandardSpell::new(20));
    map.insert(key, val);

    let key = FlamesOfZamorak;
    let val = Box::new(StandardSpell::new(20));
    map.insert(key, val);
}
