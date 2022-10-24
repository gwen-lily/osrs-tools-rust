pub mod bonus;
pub mod boost_mod;
pub mod character_mod;
pub mod combat_mod;
pub mod config;
pub mod data;
pub mod effect_mod;
pub mod levels;
pub mod modifiers;
pub mod prayer_mod;
pub mod spell_mod;
pub mod stance_mod;
pub mod style_mod;

#[macro_use]
extern crate lazy_static;

use crate::{
    prayer_mod::prayers::{get_all_prayers, PrayerMap},
    spell_mod::spells::{get_all_spells, SpellsMap},
    stance_mod::stance_stats::{get_all_stances, StanceMap},
    style_mod::styles_map::{get_all_player_styles, StylesMap},
};

lazy_static! {
    pub static ref STYLES_MAP: StylesMap = get_all_player_styles();
    pub static ref STANCE_MAP: StanceMap = get_all_stances();
    pub static ref PRAYER_MAP: PrayerMap = get_all_prayers();
    pub static ref SPELL_MAP: SpellsMap = get_all_spells();
}
