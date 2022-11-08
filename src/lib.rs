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
pub mod simulation;
pub mod spell;
pub mod stance;
pub mod style;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate float_cmp;

use std::collections::HashMap;

use bonus::Slot;
use data::CombatAspect;

use crate::{
    bonus::{get_all_gear_sets, GearSetMap},
    data::{Skill, DT},
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

pub(crate) type CombatMap<T> = HashMap<(DT, CombatAspect), T>;
pub(crate) type SkillMap<T> = HashMap<Skill, T>;

/// Represents fail cases of using this software
#[derive(thiserror::Error, Debug)]
pub enum OsrsError {
    /// Returned when a CombatMap doesn't act right
    #[error("An error occurred while dealing with a CombatMap")]
    CombatMap(Option<Vec<DT>>, Option<Vec<CombatAspect>>),

    /// Returned when a Typeless calculation is executed in a non-sensical way
    #[error("A Typeless calculation was attempted")]
    Typeless,

    /// Returned when a combat calculation don't act right
    #[error("An error occurred while dealing with a combat calculation")]
    Combat,

    /// Returned when a special attack is performed without a special weapon
    #[error("Tried to perform a special attack with a non-special weapon")]
    SpecialWeapon,

    /// Returned when a spell is called for but none is found
    #[error("Tried to cast a spell, but none was found")]
    Spell,

    /// Returned when a two-handed weapon is equipped with a shield. The optional contained
    /// Slot is the one that attempted to equip most recently, causing the invalid state.
    #[error("Tried to equip a two-handed weapon & shield at the same time")]
    TwoHandedError(Option<Slot>),
}

/// A type alias used for convenience and consiceness throughout the library.
pub type Result<T> = std::result::Result<T, OsrsError>;
