use crate::data::{
    RangedStance::*,
    Stance::*,
    StyleName::{self, LongFuse, MediumFuse, ShortFuse},
    DT,
};

use super::StylesCategory::*;
use super::StylesMap;
use crate::style::style::{Style, StyleCollection};

pub fn add_all_ranged_styles(map: &mut StylesMap) {
    // Bows
    let key = Bows;
    let val = StyleCollection::new(
        vec![
            Style {
                name: StyleName::Accurate,
                dt: DT::Ranged,
                stance: Ranged(Accurate),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: StyleName::Rapid,
                dt: DT::Ranged,
                stance: Ranged(Rapid),
                attack_speed_mod: Some(-1),
                attack_range_mod: None,
            },
            Style {
                name: StyleName::Longrange,
                dt: DT::Ranged,
                stance: Ranged(Longrange),
                attack_speed_mod: None,
                attack_range_mod: Some(2),
            },
        ],
        1,
    );

    map.insert(key, val);

    // Chinchompas
    let key = Chinchompas;
    let val = StyleCollection::new(
        vec![
            Style {
                name: ShortFuse,
                dt: DT::Ranged,
                stance: Ranged(Accurate),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: MediumFuse,
                dt: DT::Ranged,
                stance: Ranged(Rapid),
                attack_speed_mod: Some(-1),
                attack_range_mod: None,
            },
            Style {
                name: LongFuse,
                dt: DT::Ranged,
                stance: Ranged(Longrange),
                attack_speed_mod: None,
                attack_range_mod: Some(2),
            },
        ],
        1,
    );

    map.insert(key, val);

    // Crossbows
    let key = Crossbows;
    let val = StyleCollection::new(
        vec![
            Style {
                name: StyleName::Accurate,
                dt: DT::Ranged,
                stance: Ranged(Accurate),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: StyleName::Rapid,
                dt: DT::Ranged,
                stance: Ranged(Rapid),
                attack_speed_mod: Some(-1),
                attack_range_mod: None,
            },
            Style {
                name: StyleName::Longrange,
                dt: DT::Ranged,
                stance: Ranged(Longrange),
                attack_speed_mod: None,
                attack_range_mod: Some(2),
            },
        ],
        1,
    );

    map.insert(key, val);

    // Thrown
    let key = Thrown;
    let val = StyleCollection::new(
        vec![
            Style {
                name: StyleName::Accurate,
                dt: DT::Ranged,
                stance: Ranged(Accurate),
                attack_speed_mod: None,
                attack_range_mod: None,
            },
            Style {
                name: StyleName::Rapid,
                dt: DT::Ranged,
                stance: Ranged(Rapid),
                attack_speed_mod: Some(-1),
                attack_range_mod: None,
            },
            Style {
                name: StyleName::Longrange,
                dt: DT::Ranged,
                stance: Ranged(Longrange),
                attack_speed_mod: None,
                attack_range_mod: Some(2),
            },
        ],
        1,
    );

    map.insert(key, val);
}
