use crate::spell::{Aspect::*, Spell, SpellName::*, SpellsMap};

pub fn add_ancient_spells(map: &mut SpellsMap) {
    // Rush
    let key = SmokeRush;
    let val = Spell::ancient(key, 13, Some(Smoke));
    map.insert(key, val);

    let key = ShadowRush;
    let val = Spell::ancient(key, 14, Some(Shadow));
    map.insert(key, val);

    let key = BloodRush;
    let val = Spell::ancient(key, 15, Some(Blood));
    map.insert(key, val);

    let key = IceRush;
    let val = Spell::ancient(key, 16, Some(Ice));
    map.insert(key, val);

    // Burst
    let key = SmokeBurst;
    let val = Spell::aoe(key, 17, Some(Smoke));
    map.insert(key, val);

    let key = ShadowBurst;
    let val = Spell::aoe(key, 18, Some(Shadow));
    map.insert(key, val);

    let key = BloodBurst;
    let val = Spell::aoe(key, 21, Some(Blood));
    map.insert(key, val);

    let key = IceBurst;
    let val = Spell::aoe(key, 22, Some(Ice));
    map.insert(key, val);

    // Blitz
    let key = SmokeBlitz;
    let val = Spell::ancient(key, 23, Some(Smoke));
    map.insert(key, val);

    let key = ShadowBlitz;
    let val = Spell::ancient(key, 24, Some(Shadow));
    map.insert(key, val);

    let key = BloodBlitz;
    let val = Spell::ancient(key, 25, Some(Blood));
    map.insert(key, val);

    let key = IceBlitz;
    let val = Spell::ancient(key, 26, Some(Ice));
    map.insert(key, val);

    // Barrage
    let key = SmokeBarrage;
    let val = Spell::aoe(key, 27, Some(Smoke));
    map.insert(key, val);

    let key = ShadowBarrage;
    let val = Spell::aoe(key, 28, Some(Shadow));
    map.insert(key, val);

    let key = BloodBarrage;
    let val = Spell::aoe(key, 29, Some(Blood));
    map.insert(key, val);

    let key = IceBarrage;
    let val = Spell::aoe(key, 30, Some(Ice));
    map.insert(key, val);
}
