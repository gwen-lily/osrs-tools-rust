use crate::SkillMap;

/// Type alias for HashMap<Skill, i32>
pub(crate) type BoostMap = SkillMap<Box<dyn Fn(&u32) -> i32>>;
pub(crate) type Boost = SkillMap<i32>;
