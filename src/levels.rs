use crate::data::Skill::{self, Attack, Defence, Hitpoints, Magic, Ranged, Strength};
use crate::data::{CombatAspect, DT};
use crate::SkillMap;
use strum::IntoEnumIterator;

/// Type alias for HashMap<Skill, i32>
pub type Levels = SkillMap<u32>;

/// Trait which certifies that a HashMap has the necessary combat stats
pub trait HasCombatStats {
    fn combat_stats(&self) -> Levels;
}

pub trait LevelsBuilder {
    fn build_fresh() -> Levels;
    fn build_maxed() -> Levels;
}

impl HasCombatStats for Levels {
    fn combat_stats(&self) -> Levels {
        let combat_skills: [Skill; 6] = [Attack, Strength, Defence, Ranged, Magic, Hitpoints];
        let cmb_stats: Levels = combat_skills
            .iter()
            .map(|&sk| (sk, *self.get(&sk).unwrap()))
            .collect();

        cmb_stats
    }
}

impl LevelsBuilder for Levels {
    fn build_fresh() -> Levels {
        let mut map: Levels = Skill::iter().map(|sk| (sk, 1u32)).collect();
        map.insert(Skill::Hitpoints, 10);
        map
    }

    fn build_maxed() -> Levels {
        Skill::iter().map(|sk| (sk, 99_u32)).collect()
    }
}

/// This only works in the meanwhile where I consider pvm mechanics.
/// Eventually this should be made more generic.
pub(crate) fn combat_skill(dt: DT, aspect: CombatAspect) -> Skill {
    match dt {
        DT::Melee(_) => match aspect {
            CombatAspect::Attack => Attack,
            CombatAspect::Strength => Strength,
            CombatAspect::Defence => Defence,
        },
        DT::Ranged => match aspect {
            CombatAspect::Attack => Ranged,
            CombatAspect::Strength => Ranged,
            CombatAspect::Defence => Defence,
        },
        DT::Magic => match aspect {
            CombatAspect::Attack => Magic,
            CombatAspect::Strength => Magic,
            CombatAspect::Defence => Magic,
        },
        DT::Typeless => todo!(),
    }
}
