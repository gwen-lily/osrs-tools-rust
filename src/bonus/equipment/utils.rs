use crate::{
    bonus::{Gear, Weapon},
    style::{Style, Styles},
};

use itertools::{iproduct, Itertools};

use super::{Equipment, EquipmentMap, SlotMap};

pub fn get_all_equipment_style_products(
    gear: Vec<Gear>,
    weapons: Vec<Weapon>,
    styles: Option<Vec<Styles>>,
) -> Vec<(EquipmentMap, Style)> {
    let mut geare: SlotMap<Vec<Gear>> = SlotMap::new();

    // First set of axes for the cartesian product, each slot except weapon
    for g in gear.into_iter() {
        match geare.get(&g.gear_info.slot) {
            Some(v) => v.push(g.clone()),
            None => _ = geare.insert(g.gear_info.slot, Vec::new()),
        }

        if let Some(v) = geare.get(&g.gear_info.slot) {
            v.push(g.clone());
        } else {
            geare.insert(g.gear_info.slot, vec![g]);
        }
    }

    // Then the pairings of weapon & style that are logically consistent
    let mut wpn_sty_tups: Vec<(Weapon, Style)> = Vec::new();

    // is this allowed?
    if let Some(isles) = styles {
        for (wpn, wiles) in iproduct!(weapons.iter(), isles.iter()) {
            for sty in &wiles.styles {
                if wpn.weapon_info.styles.styles.contains(&sty) {
                    wpn_sty_tups.push((wpn, sty));
                }
            }
        }
    } else {
        for wpn in weapons.iter() {
            for sty in wpn.weapon_info.styles.styles.iter() {
                wpn_sty_tups.push((wpn, sty));
            }
        }
    }

    let v: Vec<(EquipmentMap, Style)> = vec![];
    v
}
