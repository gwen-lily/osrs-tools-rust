use crate::data::{
    MagicStance::*,
    MeleeDamageType::*,
    MeleeStance,
    Stance::*,
    StyleName::{self, Bash, DefensiveSpell, Fend, Focus, Jab, Pound, StandardSpell, Swipe},
    DT,
};

use super::StylesCategory::*;
use super::StylesMap;
use crate::style_mod::style::{Style, StyleCollection};

pub fn add_all_magic_styles(map: &mut StylesMap) {
    // BladedStaves
    let key = BladedStaves;
    let val = StyleCollection::new(
        vec![
            Style {
                name: Jab,
                dt: DT::Melee(Stab),
                stance: Melee(MeleeStance::Accurate),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Swipe,
                dt: DT::Melee(Slash),
                stance: Melee(MeleeStance::Aggressive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Fend,
                dt: DT::Melee(Crush),
                stance: Melee(MeleeStance::Defensive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: StandardSpell,
                dt: DT::Magic,
                stance: Magic(NoStyle),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: DefensiveSpell,
                dt: DT::Magic,
                stance: Magic(NoStyle),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
        ],
        1,
    );

    map.insert(key, val);

    // PoweredStaves
    let key = PoweredStaves;
    let val = StyleCollection::new(
        vec![
            Style {
                name: StyleName::Accurate,
                dt: DT::Magic,
                stance: Magic(Accurate),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: StyleName::Longrange,
                dt: DT::Magic,
                stance: Magic(Longrange),
                attack_speed_mod: None,
                attack_range_mod: Some(2),
            },
        ],
        0,
    );

    map.insert(key, val);

    // Staves
    let key = Staves;
    let val = StyleCollection::new(
        vec![
            Style {
                name: Bash,
                dt: DT::Melee(Crush),
                stance: Melee(MeleeStance::Accurate),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Pound,
                dt: DT::Melee(Crush),
                stance: Melee(MeleeStance::Aggressive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: Focus,
                dt: DT::Melee(Crush),
                stance: Melee(MeleeStance::Defensive),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: StandardSpell,
                dt: DT::Magic,
                stance: Magic(NoStyle),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: DefensiveSpell,
                dt: DT::Magic,
                stance: Magic(NoStyle),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
        ],
        1,
    );

    map.insert(key, val);
}
