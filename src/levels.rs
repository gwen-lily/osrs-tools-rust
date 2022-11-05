use crate::data::Skill::{self, Attack, Defence, Hitpoints, Magic, Ranged, Strength};
use crate::data::DT;
use crate::{OsrsError, Result, SkillMap};
use strum::IntoEnumIterator;

/// Type alias for HashMap<Skill, i32>
pub type Levels = SkillMap<i32>;

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

        let msg: &str = "&Levels does not contain a value for";
        let mut combat_stats = Levels::new();

        for skill in combat_skills.iter() {
            let value: &i32 = self.get(skill).expect(msg);
            combat_stats.insert(*skill, *value);
        }

        combat_stats
    }
}

impl LevelsBuilder for Levels {
    fn build_fresh() -> Levels {
        let mut map: Levels = Levels::new();

        for skill in Skill::iter() {
            map.insert(skill, 1);
        }
        map.insert(Skill::Hitpoints, 10);

        map
    }

    fn build_maxed() -> Levels {
        let mut map: Levels = Levels::new();

        for skill in Skill::iter() {
            map.insert(skill, 99);
        }

        map
    }
}

/// This only works in the meanwhile where I consider pvm mechanics.
/// Eventually this should be made more generic.
pub(crate) fn combat_skill(dt: DT, skill: Skill) -> Result<Skill> {
    let skl = match dt {
        DT::Melee(_) => match skill {
            Attack => Attack,
            Strength => Strength,
            Defence => Defence,
            _ => return Err(OsrsError::Combat),
        },
        DT::Ranged => match skill {
            Attack => Ranged,
            Strength => Ranged,
            Defence => Defence,
            _ => return Err(OsrsError::Combat),
        },
        DT::Magic => match skill {
            Attack => Magic,
            Defence => Magic,
            _ => return Err(OsrsError::Combat),
        },
        DT::Typeless => return Err(OsrsError::Typeless),
    };

    Ok(skl)
}
