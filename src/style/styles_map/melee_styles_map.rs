use crate::data::{MeleeDamageType::*, DT};

use crate::stance::{MeleeStance::*, Stance::*};

use super::StylesCategory::*;
use super::StylesMap;
use crate::style::StyleName::{
    self, Block, Chop, Deflect, Fend, Flick, Hack, Impale, Jab, Kick, Lash, Lunge, Pound, Pummel,
    Punch, Reap, Smash, Spike, Swipe,
};
use crate::style::{Style, Styles};

pub(super) fn add_all_melee_styles(map: &mut StylesMap) {
    // Two handed swords
    let key = TwoHandedSwords;
    let val = Styles::new(
        vec![
            Style::new(Chop, DT::Melee(Slash), Melee(Aggressive), None, None),
            Style::new(
                StyleName::Slash,
                DT::Melee(Slash),
                Melee(Aggressive),
                None,
                None,
            ),
            Style::new(Smash, DT::Melee(Crush), Melee(Aggressive), None, None),
            Style::new(Block, DT::Melee(Slash), Melee(Defensive), None, None),
        ],
        1,
    );

    map.insert(key, val);

    // Axes
    let key = Axes;
    let val = Styles::new(
        vec![
            Style::new(Chop, DT::Melee(Slash), Melee(Accurate), None, None),
            Style::new(Hack, DT::Melee(Slash), Melee(Aggressive), None, None),
            Style::new(Smash, DT::Melee(Crush), Melee(Aggressive), None, None),
            Style::new(Block, DT::Melee(Slash), Melee(Defensive), None, None),
        ],
        1,
    );

    map.insert(key, val);

    // BluntWeapons
    let key = BluntWeapons;
    let val = Styles::new(
        vec![
            Style::new(Pound, DT::Melee(Crush), Melee(Accurate), None, None),
            Style::new(Pummel, DT::Melee(Crush), Melee(Aggressive), None, None),
            Style::new(Block, DT::Melee(Crush), Melee(Defensive), None, None),
        ],
        0,
    );

    map.insert(key, val);

    // Bludgeons
    let key = Bludgeons;
    let val = Styles::new(
        vec![Style::new(
            Pummel,
            DT::Melee(Crush),
            Melee(Aggressive),
            None,
            None,
        )],
        0,
    );

    map.insert(key, val);

    // Bulwarks
    let key = Bulwarks;
    let val = Styles::new(
        vec![
            Style::new(Pummel, DT::Melee(Crush), Melee(Accurate), None, None),
            Style::new(Block, DT::Melee(Crush), Melee(Defensive), None, None),
        ],
        1,
    );

    map.insert(key, val);

    // Claws
    let key = Claws;
    let val = Styles::new(
        vec![
            Style::new(Chop, DT::Melee(Slash), Melee(Accurate), None, None),
            Style::new(
                StyleName::Slash,
                DT::Melee(Slash),
                Melee(Aggressive),
                None,
                None,
            ),
            Style::new(Lunge, DT::Melee(Stab), Melee(Controlled), None, None),
            Style::new(Block, DT::Melee(Slash), Melee(Defensive), None, None),
        ],
        1,
    );

    map.insert(key, val);

    // Partisans
    let key = Partisans;
    let val = Styles::new(
        vec![
            Style::new(
                StyleName::Stab,
                DT::Melee(Stab),
                Melee(Accurate),
                None,
                None,
            ),
            Style::new(Lunge, DT::Melee(Stab), Melee(Aggressive), None, None),
            Style::new(Pound, DT::Melee(Crush), Melee(Aggressive), None, None),
            Style::new(Block, DT::Melee(Stab), Melee(Defensive), None, None),
        ],
        1,
    );

    map.insert(key, val);

    // Pickaxes
    let key = Pickaxes;
    let val = Styles::new(
        vec![
            Style::new(Spike, DT::Melee(Stab), Melee(Accurate), None, None),
            Style::new(Impale, DT::Melee(Stab), Melee(Aggressive), None, None),
            Style::new(Smash, DT::Melee(Crush), Melee(Aggressive), None, None),
            Style::new(Block, DT::Melee(Stab), Melee(Defensive), None, None),
        ],
        2,
    );

    map.insert(key, val);

    // Polearms
    let key = Polearms;
    let val = Styles::new(
        vec![
            Style::new(Jab, DT::Melee(Stab), Melee(Controlled), None, None),
            Style::new(Swipe, DT::Melee(Slash), Melee(Aggressive), None, None),
            Style::new(Fend, DT::Melee(Stab), Melee(Defensive), None, None),
        ],
        0,
    );
    map.insert(key, val);

    // Polestaves
    let key = Polestaves;
    let val = Styles::new(
        vec![
            Style::new(
                StyleName::Bash,
                DT::Melee(Crush),
                Melee(Accurate),
                None,
                None,
            ),
            Style::new(Pound, DT::Melee(Crush), Melee(Aggressive), None, None),
            Style::new(Block, DT::Melee(Crush), Melee(Defensive), None, None),
        ],
        1,
    );

    map.insert(key, val);

    // Scythes
    let key = Scythes;
    let val = Styles::new(
        vec![
            Style::new(Reap, DT::Melee(Slash), Melee(Accurate), None, None),
            Style::new(Chop, DT::Melee(Slash), Melee(Aggressive), None, None),
            Style::new(Jab, DT::Melee(Crush), Melee(Aggressive), None, None),
            Style::new(Block, DT::Melee(Slash), Melee(Defensive), None, None),
        ],
        1,
    );

    map.insert(key, val);

    // SlashSwords
    let key = SlashSwords;
    let val = Styles::new(
        vec![
            Style::new(Chop, DT::Melee(Slash), Melee(Accurate), None, None),
            Style::new(
                StyleName::Slash,
                DT::Melee(Slash),
                Melee(Aggressive),
                None,
                None,
            ),
            Style::new(Lunge, DT::Melee(Stab), Melee(Controlled), None, None),
            Style::new(Block, DT::Melee(Slash), Melee(Defensive), None, None),
        ],
        1,
    );

    map.insert(key, val);

    // Spears
    let key = Spears;
    let val = Styles::new(
        vec![
            Style::new(Lunge, DT::Melee(Stab), Melee(Controlled), None, None),
            Style::new(Swipe, DT::Melee(Slash), Melee(Controlled), None, None),
            Style::new(Pound, DT::Melee(Crush), Melee(Controlled), None, None),
            Style::new(Block, DT::Melee(Stab), Melee(Defensive), None, None),
        ],
        0,
    );

    map.insert(key, val);

    // SpikedWeapons
    let key = SpikedWeapons;
    let val = Styles::new(
        vec![
            Style::new(Pound, DT::Melee(Crush), Melee(Accurate), None, None),
            Style::new(Pummel, DT::Melee(Crush), Melee(Aggressive), None, None),
            Style::new(Spike, DT::Melee(Stab), Melee(Controlled), None, None),
            Style::new(Block, DT::Melee(Crush), Melee(Defensive), None, None),
        ],
        1,
    );

    map.insert(key, val);

    // StabSwords
    let key = StabSwords;
    let val = Styles::new(
        vec![
            Style::new(
                StyleName::Stab,
                DT::Melee(Stab),
                Melee(Accurate),
                None,
                None,
            ),
            Style::new(Lunge, DT::Melee(Stab), Melee(Aggressive), None, None),
            Style::new(
                StyleName::Slash,
                DT::Melee(Slash),
                Melee(Aggressive),
                None,
                None,
            ),
            Style::new(Block, DT::Melee(Stab), Melee(Defensive), None, None),
        ],
        1,
    );

    map.insert(key, val);

    // UnarmedWeapons
    let key = UnarmedWeapons;
    let val = Styles::new(
        vec![
            Style::new(Punch, DT::Melee(Crush), Melee(Accurate), None, None),
            Style::new(Kick, DT::Melee(Crush), Melee(Aggressive), None, None),
            Style::new(Block, DT::Melee(Crush), Melee(Defensive), None, None),
        ],
        1,
    );

    map.insert(key, val);

    // Whips
    let key = Whips;
    let val = Styles::new(
        vec![
            Style::new(Flick, DT::Melee(Slash), Melee(Accurate), None, None),
            Style::new(Lash, DT::Melee(Slash), Melee(Controlled), None, None),
            Style::new(Deflect, DT::Melee(Slash), Melee(Defensive), None, None),
        ],
        1,
    );

    map.insert(key, val);
}
