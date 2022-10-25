use crate::data::{MeleeDamageType::*, DT};

use crate::stance::{MagicStance::*, MeleeStance, Stance::*};

use super::StylesCategory::*;
use super::StylesMap;
use crate::style::StyleName::{
    self, Bash, DefensiveSpell, Fend, Focus, Jab, Pound, StandardSpell, Swipe,
};
use crate::style::{Style, Styles};

pub(super) fn add_all_magic_styles(map: &mut StylesMap) {
    // BladedStaves
    let key = BladedStaves;
    let val = Styles::new(
        vec![
            Style::new(
                Jab,
                DT::Melee(Stab),
                Melee(MeleeStance::Accurate),
                None,
                None,
            ),
            Style::new(
                Swipe,
                DT::Melee(Slash),
                Melee(MeleeStance::Aggressive),
                None,
                None,
            ),
            Style::new(
                Fend,
                DT::Melee(Crush),
                Melee(MeleeStance::Defensive),
                None,
                None,
            ),
            Style::new(StandardSpell, DT::Magic, Magic(NoStyle), None, None),
            Style::new(DefensiveSpell, DT::Magic, Magic(NoStyle), None, None),
        ],
        1,
    );

    map.insert(key, val);

    // PoweredStaves
    let key = PoweredStaves;
    let val = Styles::new(
        vec![
            Style::new(StyleName::Accurate, DT::Magic, Magic(Accurate), None, None),
            Style::new(
                StyleName::Longrange,
                DT::Magic,
                Magic(Longrange),
                None,
                Some(2),
            ),
        ],
        0,
    );

    map.insert(key, val);

    // Staves
    let key = Staves;
    let val = Styles::new(
        vec![
            Style::new(
                Bash,
                DT::Melee(Crush),
                Melee(MeleeStance::Accurate),
                None,
                None,
            ),
            Style::new(
                Pound,
                DT::Melee(Crush),
                Melee(MeleeStance::Aggressive),
                None,
                None,
            ),
            Style::new(
                Focus,
                DT::Melee(Crush),
                Melee(MeleeStance::Defensive),
                None,
                None,
            ),
            Style::new(StandardSpell, DT::Magic, Magic(NoStyle), None, None),
            Style::new(DefensiveSpell, DT::Magic, Magic(NoStyle), None, None),
        ],
        1,
    );

    map.insert(key, val);
}
