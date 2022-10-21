use crate::data::{MeleeDamageType::*, MeleeStance::*, Stance::*, DT};

use crate::data::StyleName::{
    self, Block, Chop, Deflect, Fend, Flick, Hack, Impale, Jab, Kick, Lash, Lunge, Pound, Pummel,
    Punch, Reap, Smash, Spike, Swipe,
};

use super::StylesCategory::*;
use super::StylesMap;
use crate::style_mod::style::{Style, StyleCollection};

pub fn add_all_melee_styles(map: &mut StylesMap) {
    // Two handed swords
    let key = TwoHandedSwords;
    let val = StyleCollection::new(
        vec![
            Style {
                name: Chop,
                dt: DT::Melee(Slash),
                stance: Melee(Aggressive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: StyleName::Slash,
                dt: DT::Melee(Slash),
                stance: Melee(Aggressive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Smash,
                dt: DT::Melee(Crush),
                stance: Melee(Aggressive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Block,
                dt: DT::Melee(Slash),
                stance: Melee(Defensive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
        ],
        1,
    );

    map.insert(key, val);

    // Axes
    let key = Axes;
    let val = StyleCollection::new(
        vec![
            Style {
                name: Chop,
                dt: DT::Melee(Slash),
                stance: Melee(Accurate),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Hack,
                dt: DT::Melee(Slash),
                stance: Melee(Aggressive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Smash,
                dt: DT::Melee(Crush),
                stance: Melee(Aggressive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Block,
                dt: DT::Melee(Slash),
                stance: Melee(Defensive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
        ],
        1,
    );

    map.insert(key, val);

    // BluntWeapons
    let key = BluntWeapons;
    let val = StyleCollection::new(
        vec![
            Style {
                name: Pound,
                dt: DT::Melee(Crush),
                stance: Melee(Accurate),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Pummel,
                dt: DT::Melee(Crush),
                stance: Melee(Aggressive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Block,
                dt: DT::Melee(Crush),
                stance: Melee(Defensive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
        ],
        0,
    );

    map.insert(key, val);

    // Bludgeons
    let key = Bludgeons;
    let val = StyleCollection::new(
        vec![Style {
            name: Pummel,
            dt: DT::Melee(Crush),
            stance: Melee(Aggressive),
            attack_speed_mod: None,
            attack_range_mod: None,
        }],
        0,
    );

    map.insert(key, val);

    // Bulwarks
    let key = Bulwarks;
    let val = StyleCollection::new(
        vec![
            Style {
                name: Pummel,
                dt: DT::Melee(Crush),
                stance: Melee(Accurate),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Block,
                dt: DT::Melee(Crush),
                stance: Melee(Defensive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
        ],
        1,
    );

    map.insert(key, val);

    // Claws
    let key = Claws;
    let val = StyleCollection::new(
        vec![
            Style {
                name: Chop,
                dt: DT::Melee(Slash),
                stance: Melee(Accurate),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: StyleName::Slash,
                dt: DT::Melee(Slash),
                stance: Melee(Aggressive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Lunge,
                dt: DT::Melee(Stab),
                stance: Melee(Controlled),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Block,
                dt: DT::Melee(Slash),
                stance: Melee(Defensive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
        ],
        1,
    );

    map.insert(key, val);

    // Pickaxes
    let key = Pickaxes;
    let val = StyleCollection::new(
        vec![
            Style {
                name: Spike,
                dt: DT::Melee(Stab),
                stance: Melee(Accurate),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Impale,
                dt: DT::Melee(Stab),
                stance: Melee(Aggressive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Smash,
                dt: DT::Melee(Crush),
                stance: Melee(Aggressive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Block,
                dt: DT::Melee(Stab),
                stance: Melee(Defensive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
        ],
        2,
    );

    map.insert(key, val);

    // Polearms
    let key = Polearms;
    let val = StyleCollection::new(
        vec![
            Style {
                name: Jab,
                dt: DT::Melee(Stab),
                stance: Melee(Controlled),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Swipe,
                dt: DT::Melee(Slash),
                stance: Melee(Aggressive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Fend,
                dt: DT::Melee(Stab),
                stance: Melee(Defensive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
        ],
        0,
    );

    map.insert(key, val);

    // Scythes
    let key = Scythes;
    let val = StyleCollection::new(
        vec![
            Style {
                name: Reap,
                dt: DT::Melee(Slash),
                stance: Melee(Accurate),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Chop,
                dt: DT::Melee(Slash),
                stance: Melee(Aggressive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Jab,
                dt: DT::Melee(Crush),
                stance: Melee(Aggressive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Block,
                dt: DT::Melee(Slash),
                stance: Melee(Defensive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
        ],
        1,
    );

    map.insert(key, val);

    // SlashSwords
    let key = SlashSwords;
    let val = StyleCollection::new(
        vec![
            Style {
                name: Chop,
                dt: DT::Melee(Slash),
                stance: Melee(Accurate),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: StyleName::Slash,
                dt: DT::Melee(Slash),
                stance: Melee(Aggressive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Lunge,
                dt: DT::Melee(Stab),
                stance: Melee(Controlled),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Block,
                dt: DT::Melee(Slash),
                stance: Melee(Defensive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
        ],
        1,
    );

    map.insert(key, val);

    // Spears
    let key = Spears;
    let val = StyleCollection::new(
        vec![
            Style {
                name: Lunge,
                dt: DT::Melee(Stab),
                stance: Melee(Controlled),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Swipe,
                dt: DT::Melee(Slash),
                stance: Melee(Controlled),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Pound,
                dt: DT::Melee(Crush),
                stance: Melee(Controlled),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Block,
                dt: DT::Melee(Stab),
                stance: Melee(Defensive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
        ],
        0,
    );

    map.insert(key, val);

    // SpikedWeapons
    let key = SpikedWeapons;
    let val = StyleCollection::new(
        vec![
            Style {
                name: Pound,
                dt: DT::Melee(Crush),
                stance: Melee(Accurate),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Pummel,
                dt: DT::Melee(Crush),
                stance: Melee(Aggressive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Spike,
                dt: DT::Melee(Stab),
                stance: Melee(Controlled),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Block,
                dt: DT::Melee(Crush),
                stance: Melee(Defensive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
        ],
        1,
    );

    map.insert(key, val);

    // StabSwords
    let key = StabSwords;
    let val = StyleCollection::new(
        vec![
            Style {
                name: StyleName::Stab,
                dt: DT::Melee(Stab),
                stance: Melee(Accurate),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Lunge,
                dt: DT::Melee(Stab),
                stance: Melee(Aggressive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: StyleName::Slash,
                dt: DT::Melee(Slash),
                stance: Melee(Aggressive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Block,
                dt: DT::Melee(Stab),
                stance: Melee(Defensive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
        ],
        1,
    );

    map.insert(key, val);

    // UnarmedWeapons
    let key = UnarmedWeapons;
    let val = StyleCollection::new(
        vec![
            Style {
                name: Punch,
                dt: DT::Melee(Crush),
                stance: Melee(Accurate),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Kick,
                dt: DT::Melee(Crush),
                stance: Melee(Aggressive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Block,
                dt: DT::Melee(Crush),
                stance: Melee(Defensive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
        ],
        1,
    );

    map.insert(key, val);

    // Whips
    let key = Whips;
    let val = StyleCollection::new(
        vec![
            Style {
                name: Flick,
                dt: DT::Melee(Slash),
                stance: Melee(Accurate),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Lash,
                dt: DT::Melee(Slash),
                stance: Melee(Controlled),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Deflect,
                dt: DT::Melee(Slash),
                stance: Melee(Defensive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
        ],
        1,
    );

    map.insert(key, val);
}
