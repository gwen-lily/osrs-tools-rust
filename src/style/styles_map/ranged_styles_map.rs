use crate::data::DT;

use super::StylesCategory::*;
use super::StylesMap;
use crate::stance::{RangedStance::*, Stance::*};
use crate::style::StyleName::{self, LongFuse, MediumFuse, ShortFuse};
use crate::style::{Style, Styles};

pub(super) fn add_all_ranged_styles(map: &mut StylesMap) {
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
