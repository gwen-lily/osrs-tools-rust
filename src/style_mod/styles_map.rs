pub mod magic_styles_map;
pub mod melee_styles_map;
pub mod ranged_styles_map;

use std::collections::HashMap;
#[allow(unused_imports)]
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    data::{MeleeDamageType, DT},
    stance_mod::stance::*,
    style_mod::style::{Style, StyleName, Styles},
};
use magic_styles_map::add_all_magic_styles;
use melee_styles_map::add_all_melee_styles;
use ranged_styles_map::add_all_ranged_styles;

pub type StylesMap = HashMap<StylesCategory, Styles>;

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum StylesCategory {
    TwoHandedSwords,
    Axes,
    BluntWeapons,
    Bludgeons,
    Bulwarks,
    Claws,
    Partisans,
    Pickaxes,
    Polearms,
    Polestaves,
    Salamanders,
    Scythes,
    SlashSwords,
    Spears,
    SpikedWeapons,
    StabSwords,
    UnarmedWeapons,
    Whips,
    Bows,
    Chinchompas,
    Crossbows,
    Thrown,
    BladedStaves,
    PoweredStaves,
    Staves,
}

#[allow(dead_code)]
pub fn get_all_player_styles() -> StylesMap {
    let mut map: StylesMap = StylesMap::new();

    add_all_melee_styles(&mut map);
    add_all_ranged_styles(&mut map);
    add_all_magic_styles(&mut map);
    add_salamanders_styles(&mut map);

    map
}

fn add_salamanders_styles(map: &mut StylesMap) {
    use StyleName::*;
    use DT::*;

    let key = StylesCategory::Salamanders;
    let val = Styles::new(
        vec![
            Style::new(
                Scorch,
                Melee(MeleeDamageType::Slash),
                Stance::Melee(MeleeStance::Aggressive),
                None,
                None,
            ),
            Style::new(
                Flare,
                Ranged,
                Stance::Ranged(RangedStance::Accurate),
                Some(-1),
                None,
            ),
            Style::new(
                Blaze,
                Magic,
                Stance::Magic(MagicStance::Defensive),
                None,
                None,
            ),
        ],
        1,
    );
    map.insert(key, val);
}
