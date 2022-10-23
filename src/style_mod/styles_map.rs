pub mod magic_styles_map;
pub mod melee_styles_map;
pub mod ranged_styles_map;

use std::collections::HashMap;
#[allow(unused_imports)]
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::style_mod::style::Styles;
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
    Pickaxes,
    Polearms,
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

    map
}

// pub const ALL_STYLES: StylesMap = get_all_player_styles();
