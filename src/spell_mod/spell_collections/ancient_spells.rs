use crate::spell_mod::spell::AncientSpell;

use super::SpellName::*;
use super::SpellsMap;

pub fn add_ancient_spells(map: &mut SpellsMap) {
    let key = SmokeBurst;
    let val = Box::new(AncientSpell::new(17, 9));
    map.insert(key, val);

    let key = ShadowBurst;
    let val = Box::new(AncientSpell::new(18, 9));
    map.insert(key, val);

    let key = BloodBurst;
    let val = Box::new(AncientSpell::new(21, 9));
    map.insert(key, val);

    let key = IceBurst;
    let val = Box::new(AncientSpell::new(22, 9));
    map.insert(key, val);
}
