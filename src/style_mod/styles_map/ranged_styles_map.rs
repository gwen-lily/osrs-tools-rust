use crate::data::{
    StyleName::{self, LongFuse, MediumFuse, ShortFuse},
    DT,
};

use super::StylesCategory::*;
use super::StylesMap;
use crate::stance_mod::stance::{RangedStance::*, Stance::*};
use crate::style_mod::style::{Style, Styles};

pub fn add_all_ranged_styles(map: &mut StylesMap) {
    // Bows
    let key = Bows;
    let val = Styles::new(
        vec![
            Style::new(
                StyleName::Accurate,
                DT::Ranged,
                Ranged(Accurate),
                None,
                None,
            ),
            Style::new(StyleName::Rapid, DT::Ranged, Ranged(Rapid), Some(-1), None),
            Style::new(
                StyleName::Longrange,
                DT::Ranged,
                Ranged(Longrange),
                None,
                Some(2),
            ),
        ],
        1,
    );

    map.insert(key, val);

    // Chinchompas
    let key = Chinchompas;
    let val = Styles::new(
        vec![
            Style::new(ShortFuse, DT::Ranged, Ranged(Accurate), None, None),
            Style::new(MediumFuse, DT::Ranged, Ranged(Rapid), Some(-1), None),
            Style::new(LongFuse, DT::Ranged, Ranged(Longrange), None, Some(2)),
        ],
        1,
    );

    map.insert(key, val);

    // Crossbows
    let key = Crossbows;
    let val = Styles::new(
        vec![
            Style::new(
                StyleName::Accurate,
                DT::Ranged,
                Ranged(Accurate),
                None,
                None,
            ),
            Style::new(StyleName::Rapid, DT::Ranged, Ranged(Rapid), Some(-1), None),
            Style::new(
                StyleName::Longrange,
                DT::Ranged,
                Ranged(Longrange),
                None,
                Some(2),
            ),
        ],
        1,
    );

    map.insert(key, val);

    // Thrown
    let key = Thrown;
    let val = Styles::new(
        vec![
            Style::new(
                StyleName::Accurate,
                DT::Ranged,
                Ranged(Accurate),
                None,
                None,
            ),
            Style::new(StyleName::Rapid, DT::Ranged, Ranged(Rapid), Some(-1), None),
            Style::new(
                StyleName::Longrange,
                DT::Ranged,
                Ranged(Longrange),
                None,
                Some(2),
            ),
        ],
        1,
    );

    map.insert(key, val);
}
