#[macro_use]
extern crate lazy_static;

use osrs_tools::{
    prayer_mod::prayers::{get_all_prayers, PrayerMap},
    spell_mod::spells::{get_all_spells, SpellsMap},
    stance_mod::stance_stats::{get_all_stances, StanceMap},
    style_mod::styles_map::{get_all_player_styles, StylesMap},
};

lazy_static! {
    static ref STYLES_MAP: StylesMap = get_all_player_styles();
    static ref STANCE_MAP: StanceMap = get_all_stances();
    static ref PRAYER_MAP: PrayerMap = get_all_prayers();
    static ref SPELL_MAP: SpellsMap = get_all_spells();
}

fn main() {}
