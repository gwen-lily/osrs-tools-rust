mod magic_style_collections;
mod melee_style_collections;
mod ranged_style_collections;

use std::collections::HashMap;
#[allow(unused_imports)]
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::style_mod::style::StyleCollection;
use magic_style_collections::add_all_magic_styles;
use melee_style_collections::add_all_melee_styles;
use ranged_style_collections::add_all_ranged_styles;

pub type StylesMap = HashMap<StylesCategory, StyleCollection>;

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
pub fn get_all_player_styles(_: ()) -> StylesMap {
    let mut map = StylesMap::new();

    add_all_melee_styles(&mut map);
    add_all_ranged_styles(&mut map);
    add_all_magic_styles(&mut map);

    map
}
