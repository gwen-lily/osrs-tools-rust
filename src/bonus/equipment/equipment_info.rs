use crate::{
    bonus::{Equipment, EquipmentMap, EquipmentNameMap},
    GEAR_SETS_MAP,
};

///
/// EquipmentInfo provides information functions on the sets and gear contained within.
#[derive(Debug, PartialEq, Eq)]
pub struct EquipmentInfo {
    pub equipment: Equipment,
}

impl EquipmentInfo {
    /// Return true if all eleven slots are filled
    pub fn complete(&self) -> bool {
        self.equipment.equipment.keys().count() == 11
    }

    /// Return true if all gear in eqp is found in self.equipment
    fn equipped(&self, eqp: &EquipmentMap) -> bool {
        for (slot, gear) in eqp.iter() {
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

    fn equipped_name(&self, eqp: &EquipmentNameMap) -> bool {
        for (slot, gear_name) in eqp.iter() {
            if let Some(eqpd_gear) = self.equipment.equipment.get(slot) {
                if eqpd_gear.name != *gear_name {
                    return false;
                };
            } else {
                return false;
            }
        }

        true
    }

    pub fn void_equipped(&self, elite: bool) -> bool {
        use super::sets::SetName::*;

        let set_eqp_map: &EquipmentNameMap = GEAR_SETS_MAP.get(&VoidSet(elite)).unwrap();
        self.equipped_name(set_eqp_map)
    }
}
