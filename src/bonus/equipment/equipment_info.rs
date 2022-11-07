use crate::{
    bonus::{
        Equipment, EquipmentMap, EquipmentNameMap,
        GearName::{self, *},
        Slot::{self, *},
    },
    GEAR_SETS_MAP,
};

use super::SetName;

///
/// EquipmentInfo provides information functions on the sets and gear contained within.
pub struct EquipmentInfo {
    pub equipment: Equipment,
}

impl EquipmentInfo {
    /// Return true if all eleven slots are filled
    pub fn complete(&self) -> bool {
        self.equipment.equipment.keys().count() == 10 && self.equipment.weapon.is_some()
    }

    /// Return true if all gear in eqpd is found in self.equipment
    #[allow(dead_code)]
    fn equipped(&self, eqpd: &EquipmentMap) -> bool {
        for (slot, gear) in eqpd.iter() {
            if let Some(eqpd_gear) = self.equipment.equipment.get(slot) {
                if eqpd_gear != gear {
                    return false;
                };
            } else {
                return false;
            }
        }

        true
    }

    /// Return true if either of the gear items is equipped in slot
    #[allow(dead_code)]
    fn any_equipped(&self, slot: Slot, names: &[GearName]) -> bool {
        if let Some(gear) = self.equipment.equipment.get(&slot) {
            for name in names.iter() {
                if gear.gear_info.name == *name {
                    return true;
                }
            }
        }

        false
    }

    pub(crate) fn equipped_name(&self, eqpd: &EquipmentNameMap) -> bool {
        for (slot, gear_name) in eqpd.iter() {
            if let Some(eqpd_gear) = self.equipment.equipment.get(slot) {
                // Compare the set name vs the name of the equipped, return false if mismatched
                if eqpd_gear.gear_info.name != *gear_name {
                    return false;
                };
            // If any piece of the set is not present as equipment, return false
            } else {
                return false;
            }
        }

        true
    }

    /// Return true iff all elements of a set are equipped
    pub(crate) fn set_equipped(&self, set_name: SetName) -> bool {
        let set_eqpd_map: &EquipmentNameMap = GEAR_SETS_MAP.get(&set_name).unwrap();
        self.equipped_name(set_eqpd_map)
    }

    /// Return true if an obsidian weapon which recieves a particular buff is equipped
    pub(crate) fn obsidian_weapon_equipped(&self) -> bool {
        if let Some(wpn) = &self.equipment.weapon {
            matches!(
                wpn.gear_info.name,
                ObsidianDagger | ObsidianMace | ObsidianMaul | ObsidianSword
            )
        } else {
            false
        }
    }

    /// Return true if either the bowfa or the crystal bow is equipped
    pub(crate) fn crystal_weapon_equipped(&self) -> bool {
        if let Some(wpn) = &self.equipment.weapon {
            matches!(wpn.gear_info.name, CrystalBow | BowOfFaerdhinen)
        } else {
            false
        }
    }

    /// Return true if either variant of bolt is equipped
    #[allow(dead_code)]
    fn enchanted_ruby_bolts(&self) -> bool {
        let names = [RubyBoltsE, RubyDragonBoltsE];
        self.any_equipped(Ammunition, &names)
    }

    /// Return true if either variant of bolt is equipped
    #[allow(dead_code)]
    fn enchanted_diamond_bolts(&self) -> bool {
        let names = [DiamondBoltsE, DiamondDragonBoltsE];
        self.any_equipped(Ammunition, &names)
    }

    /// Return true if either variant of bolt is equipped
    #[allow(dead_code)]
    fn enchanted_dragonstone_bolts(&self) -> bool {
        let names = [DragonstoneBoltsE, DragonstoneDragonBoltsE];
        self.any_equipped(Ammunition, &names)
    }

    /// Return true if either variant of bolt is equipped
    #[allow(dead_code)]
    fn enchanted_onyx_bolts(&self) -> bool {
        let names = [OnyxBoltsE, OnyxDragonBoltsE];
        self.any_equipped(Ammunition, &names)
    }

    /// Return true if any variant of enchanted bolts is equipped
    #[allow(dead_code)]
    pub(crate) fn enchanted_bolts_equipped(&self) -> bool {
        self.enchanted_ruby_bolts()
            || self.enchanted_diamond_bolts()
            || self.enchanted_dragonstone_bolts()
            || self.enchanted_onyx_bolts()
    }

    /** Return true if a crossbow that is capable of procuring a bolt effect is equipped. Note that
     * this does not return true for various "crossbows" such as Karil's or Hunter's
     */
    #[allow(dead_code)]
    pub(crate) fn crossbow_equipped(&self) -> bool {
        if let Some(wpn) = &self.equipment.weapon {
            match wpn.gear_info.name {
                ArmadylCrossbow | ZaryteCrossbow | DragonHunterCrossbow | BronzeCrossbow
                | BluriteCrossbow | IronCrossbow | SteelCrossbow | MithrilCrossbow
                | AdamantCrossbow | RuneCrossbow | DragonCrossbow => return true,
                _ => {}
            }
        }
        false
    }

    #[allow(dead_code)]
    pub(crate) fn armadyl_crossbow_equipped(&self) -> bool {
        if let Some(wpn) = &self.equipment.weapon {
            wpn.gear_info.name == ArmadylCrossbow
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub(crate) fn zaryte_crossbow_equipped(&self) -> bool {
        if let Some(wpn) = &self.equipment.weapon {
            wpn.gear_info.name == ZaryteCrossbow
        } else {
            false
        }
    }
}
