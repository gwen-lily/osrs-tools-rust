pub mod styles_map;

use crate::data::DT;
use crate::stance::{Stance, StanceStats};
use crate::STANCE_MAP;

use strum_macros::EnumIter;

pub use styles_map::StylesCategory;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Style {
    pub name: StyleName,
    pub dt: DT,
    pub stance: Stance,
    pub stance_stats: Option<StanceStats>,
    pub attack_speed_mod: Option<i8>,
    pub attack_range_mod: Option<i8>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Styles {
    pub styles: Vec<Style>,
    pub default: usize,
}

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum StyleName {
    Slash,
    Stab,
    Accurate,
    Rapid,
    Longrange,
    Chop,
    Smash,
    Block,
    Hack,
    Lunge,
    Swipe,
    Pound,
    Pummel,
    Spike,
    Impale,
    Jab,
    Fend,
    Bash,
    Reap,
    Punch,
    Kick,
    Flick,
    Lash,
    Deflect,
    ShortFuse,
    MediumFuse,
    LongFuse,
    Spell,
    Focus,
    StandardSpell,
    DefensiveSpell,
    NpcMelee,
    NpcRanged,
    NpcMagic,
    Scorch,
    Flare,
    Blaze,
}

impl Style {
    pub fn new(
        name: StyleName,
        dt: DT,
        stance: Stance,
        attack_speed_mod: Option<i8>,
        attack_range_mod: Option<i8>,
    ) -> Self {
        if let Some(stance_stats) = STANCE_MAP.get(&stance) {
            Self {
                name,
                dt,
                stance,
                stance_stats: stance_stats.clone(),
                attack_speed_mod,
                attack_range_mod,
            }
        } else {
            panic!("Stance stats gotchu zooted");
        }
    }
}

impl Styles {
    pub fn new(styles: Vec<Style>, default: usize) -> Self {
        match styles.len() > default {
            true => Self { styles, default },
            false => panic!("Default index must point to a valid style in styles"),
        }
    }

    pub fn default_style(&self) -> &Style {
        self.styles.get(self.default).unwrap()
    }
}
