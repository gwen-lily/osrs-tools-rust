use crate::spell_mod::spell::StandardSpell;

use super::SpellName::*;
use super::SpellsMap;

pub fn add_standard_spells(map: &mut SpellsMap) {
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

    // let key = ;
    // let val = Box::new(StandardSpell::new());
    // map.insert(key, val);
}
