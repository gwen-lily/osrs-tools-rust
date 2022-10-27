pub mod bonus;
pub mod boost;
pub mod character;
pub mod combat;
pub mod config;
pub mod data;
pub mod effect;
pub mod levels;
pub mod modifiers;
pub mod prayer;
pub mod spell;
pub mod stance;
pub mod style;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate derive_builder;

use std::collections::HashMap;

use data::{Skill, DT};

use crate::{
    bonus::{get_all_gear_sets, GearSetMap},
    prayer::prayers::{get_all_prayers, PrayerMap},
    spell::spells::{get_all_spells, SpellsMap},
    stance::{get_all_stances, StanceMap},
    style::styles_map::{get_all_player_styles, StylesMap},
};

lazy_static! {
    pub static ref GEAR_SETS_MAP: GearSetMap = get_all_gear_sets();
    pub static ref STYLES_MAP: StylesMap = get_all_player_styles();
    pub static ref STANCE_MAP: StanceMap = get_all_stances();
    pub static ref PRAYER_MAP: PrayerMap = get_all_prayers();
    pub static ref SPELL_MAP: SpellsMap = get_all_spells();
}

pub(crate) type CombatMap<T> = HashMap<(DT, Skill), T>;
