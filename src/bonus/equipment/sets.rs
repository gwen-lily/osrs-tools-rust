use std::collections::HashMap;

use super::super::GearName::*;
use super::super::Slot::*;
use super::EquipmentNameMap;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum SetName {
    NormalVoidSet,
    EliteVoidSet,
    ObsidianSet,
    DharokSet,
    CrystalSet,
    InquisitorSet,
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

fn get_obsidian_set() -> EquipmentNameMap {
    let mut map = EquipmentNameMap::new();

    map.insert(Head, ObsidianHelm);
    map.insert(Body, ObsidianPlatebody);
    map.insert(Legs, ObsidianPlatelegs);

    map
}

fn get_dharok_set() -> EquipmentNameMap {
    let mut map = EquipmentNameMap::new();

    map.insert(Head, DharoksGreathelm);
    map.insert(Body, DharoksPlatebody);
    map.insert(Legs, DharoksPlatelegs);
    map.insert(Weapon, DharoksGreataxe);

    map
}

fn get_crystal_set() -> EquipmentNameMap {
    let mut map = EquipmentNameMap::new();

    map.insert(Head, CrystalHelm);
    map.insert(Body, CrystalBody);
    map.insert(Legs, CrystalLegs);

    map
}

fn get_inquisitor_set() -> EquipmentNameMap {
    let mut map = EquipmentNameMap::new();

    map.insert(Head, InquisitorsGreatHelm);
    map.insert(Body, InquisitorsHauberk);
    map.insert(Legs, InquisitorsPlateskirt);

    map
}

pub fn get_all_gear_sets() -> GearSetMap {
    use SetName::*;

    let mut map = GearSetMap::new();

    map.insert(NormalVoidSet, get_void_set());
    map.insert(EliteVoidSet, get_elite_void_set());
    map.insert(ObsidianSet, get_obsidian_set());
    map.insert(CrystalSet, get_crystal_set());
    map.insert(InquisitorSet, get_inquisitor_set());

    map
}
