use crate::spell_mod::spell::PoweredSpell;
use crate::spell_mod::spell::TumekensPoweredSpell;

use super::SpellName::*;
use super::SpellsMap;

pub fn add_powered_spells(map: &mut SpellsMap) {
    for vis_lvl in 1..126 {
        let key = TridentOfTheSeas(vis_lvl);
        let val = Box::new(PoweredSpell::new(20, vis_lvl));
        map.insert(key, val);

        let key = TridentOfTheSwamp(vis_lvl);
        let val = Box::new(PoweredSpell::new(23, vis_lvl));
        map.insert(key, val);

        let key = SanguinestiStaff(vis_lvl);
        let val = Box::new(PoweredSpell::new(24, vis_lvl));
        map.insert(key, val);

        let key = TumekensShadow(vis_lvl);
        let val = Box::new(TumekensPoweredSpell::new(vis_lvl));
        map.insert(key, val);
    }
}
