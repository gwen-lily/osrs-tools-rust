use crate::spell_mod::spell::Spell;

use super::SpellName::*;
use super::SpellsMap;

pub fn add_powered_spells(map: &mut SpellsMap) {
    let key = TridentOfTheSeas;
    let val = Spell::powered(20, 4);
    map.insert(key, val);

    let key = TridentOfTheSwamp;
    let val = Spell::powered(23, 4);
    map.insert(key, val);

    let key = SanguinestiStaff;
    let val = Spell::powered(24, 4);
    map.insert(key, val);

    let key = TumekensShadow;
    let val = Spell::powered(1, 5);
    map.insert(key, val);
}
