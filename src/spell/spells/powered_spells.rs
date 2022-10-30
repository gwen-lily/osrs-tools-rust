use crate::spell::{Spell, SpellName::*, SpellsMap};

pub fn add_powered_spells(map: &mut SpellsMap) {
    let key = TridentOfTheSeas;
    let val = Spell::powered(key, 20, 4);
    map.insert(key, val);

    let key = TridentOfTheSwamp;
    let val = Spell::powered(key, 23, 4);
    map.insert(key, val);

    let key = SanguinestiStaff;
    let val = Spell::powered(key, 24, 4);
    map.insert(key, val);

    let key = TumekensShadow;
    let val = Spell::powered(key, 1, 5);
    map.insert(key, val);
}
