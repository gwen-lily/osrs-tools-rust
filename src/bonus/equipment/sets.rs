use std::collections::HashMap;

use super::super::GearName::*;
use super::super::Slot::*;
use super::EquipmentNameMap;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum SetName {
    VoidSet(bool),
}

pub type GearSetMap = HashMap<SetName, EquipmentNameMap>;

fn get_void_set() -> EquipmentNameMap {
    let mut map = EquipmentNameMap::new();

    map.insert(Head, VoidHelmet);
    map.insert(Body, VoidBody);
    map.insert(Legs, VoidLegs);
    map.insert(Feet, VoidGloves);

    map
}

fn get_elite_void_set() -> EquipmentNameMap {
    let mut map = EquipmentNameMap::new();

    map.insert(Head, VoidHelmet);
    map.insert(Body, EliteVoidBody);
    map.insert(Legs, EliteVoidLegs);
    map.insert(Feet, VoidGloves);

    map
}

pub fn get_all_gear_sets() -> GearSetMap {
    use SetName::*;

    let mut map = GearSetMap::new();

    map.insert(VoidSet(false), get_void_set());
    map.insert(VoidSet(true), get_elite_void_set());

    map
}
