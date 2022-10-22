use crate::spell_mod::spell::PoweredSpell;
use crate::spell_mod::spell::TumekensPoweredSpell;

use super::SpellName::*;
use super::SpellsMap;

pub fn add_powered_spells(map: &mut SpellsMap) {
    for vis_lvl in 1..126 {
        let key = TridentOfTheSeas(vis_lvl);
        let val = Box::new(PoweredSpell::new(vis_lvl as u8));
        map.insert(key, val);

        let key = TumekensShadow(vis_lvl);
        let val = Box::new(TumekensPoweredSpell::new());
        map.insert(key, val);
    }
}
