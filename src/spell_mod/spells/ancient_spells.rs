use crate::spell_mod::spell::Aspect::*;
use crate::spell_mod::spell::Spell;

use super::SpellName::*;
use super::SpellsMap;

pub fn add_ancient_spells(map: &mut SpellsMap) {
    // Rush
    let key = SmokeRush;
    let val = Spell::standard(13, Some(Smoke));
    map.insert(key, val);

    let key = ShadowRush;
    let val = Spell::standard(14, Some(Shadow));
    map.insert(key, val);

    let key = BloodRush;
    let val = Spell::standard(15, Some(Blood));
    map.insert(key, val);

    let key = IceRush;
    let val = Spell::standard(16, Some(Ice));
    map.insert(key, val);

    // Burst
    let key = SmokeBurst;
    let val = Spell::aoe(17, Some(Smoke));
    map.insert(key, val);

    let key = ShadowBurst;
    let val = Spell::aoe(18, Some(Shadow));
    map.insert(key, val);

    let key = BloodBurst;
    let val = Spell::aoe(21, Some(Blood));
    map.insert(key, val);

    let key = IceBurst;
    let val = Spell::aoe(22, Some(Ice));
    map.insert(key, val);

    // Blitz
    let key = SmokeBlitz;
    let val = Spell::standard(23, Some(Smoke));
    map.insert(key, val);

    let key = ShadowBlitz;
    let val = Spell::standard(24, Some(Shadow));
    map.insert(key, val);

    let key = BloodBlitz;
    let val = Spell::standard(25, Some(Blood));
    map.insert(key, val);

    let key = IceBlitz;
    let val = Spell::standard(26, Some(Ice));
    map.insert(key, val);

    // Barrage
    let key = SmokeBarrage;
    let val = Spell::aoe(27, Some(Smoke));
    map.insert(key, val);

    let key = ShadowBarrage;
    let val = Spell::aoe(28, Some(Shadow));
    map.insert(key, val);

    let key = BloodBarrage;
    let val = Spell::aoe(29, Some(Blood));
    map.insert(key, val);

    let key = IceBarrage;
    let val = Spell::aoe(30, Some(Ice));
    map.insert(key, val);
}
