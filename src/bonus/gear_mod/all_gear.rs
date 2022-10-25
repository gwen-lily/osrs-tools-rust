use std::collections::HashMap;

use strum_macros::EnumIter;

use super::gear::Gear;

pub type GearMap = HashMap<GearName, Gear>;

#[derive(Debug, EnumIter, Hash, Eq, PartialEq, PartialOrd, Clone, Copy)]
pub enum GearName {
    FooBarBaz,
}
