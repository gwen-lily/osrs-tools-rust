use crate::spell_mod::spell::AncientSpell;

use super::SpellName::*;
use super::SpellsMap;

pub fn add_ancient_spells(map: &mut SpellsMap) {
    // Rush
    let key = SmokeRush;
    let val = Box::new(AncientSpell::single_target_spell(13));
    map.insert(key, val);

    let key = ShadowRush;
    let val = Box::new(AncientSpell::single_target_spell(14));
    map.insert(key, val);

    let key = BloodRush;
    let val = Box::new(AncientSpell::single_target_spell(15));
    map.insert(key, val);

    let key = IceRush;
    let val = Box::new(AncientSpell::single_target_spell(16));
    map.insert(key, val);

    // Burst
    let key = SmokeBurst;
    let val = Box::new(AncientSpell::aoe_spell(17));
    map.insert(key, val);

    let key = ShadowBurst;
    let val = Box::new(AncientSpell::aoe_spell(18));
    map.insert(key, val);

    let key = BloodBurst;
    let val = Box::new(AncientSpell::aoe_spell(21));
    map.insert(key, val);

    let key = IceBurst;
    let val = Box::new(AncientSpell::aoe_spell(22));
    map.insert(key, val);

    // Blitz
    let key = SmokeBlitz;
    let val = Box::new(AncientSpell::single_target_spell(23));
    map.insert(key, val);

    let key = ShadowBlitz;
    let val = Box::new(AncientSpell::single_target_spell(24));
    map.insert(key, val);

    let key = BloodBlitz;
    let val = Box::new(AncientSpell::single_target_spell(25));
    map.insert(key, val);

    let key = IceBlitz;
    let val = Box::new(AncientSpell::single_target_spell(26));
    map.insert(key, val);

    // Barrage
    let key = SmokeBarrage;
    let val = Box::new(AncientSpell::aoe_spell(27));
    map.insert(key, val);

    let key = ShadowBarrage;
    let val = Box::new(AncientSpell::aoe_spell(28));
    map.insert(key, val);

    let key = BloodBarrage;
    let val = Box::new(AncientSpell::aoe_spell(29));
    map.insert(key, val);

    let key = IceBarrage;
    let val = Box::new(AncientSpell::aoe_spell(30));
    map.insert(key, val);
}
