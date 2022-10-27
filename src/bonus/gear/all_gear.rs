pub mod example;

use super::super::gear::Gear;
use std::collections::HashMap;
use strum_macros::EnumIter;

/// A HashMap<GearName, Gear> type alias.
pub type GearMap = HashMap<GearName, Gear>;

/// An enum of the name of every Gear.
#[derive(Debug, EnumIter, Hash, Eq, PartialEq, PartialOrd, Clone, Copy)]
pub enum GearName {
    FooBarBaz,
    Chinchompa,
    RedChinchompa,
    BlackChinchompa,
    DinhsBulwark,
    ToxicBlowpipe,
    VoidHelmet,
    VoidBody,
    VoidLegs,
    VoidGloves,
    EliteVoidBody,
    EliteVoidLegs,
    TumekensShadow,
    SanguinestiStaff,
    TridentOfTheSwamp,
    TridentOfTheSeas,
}
