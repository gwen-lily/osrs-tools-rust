use crate::data::Skill;
use std::collections::HashMap;

/// Type alias for HashMap<Skill, i32>
pub type BoostMap = HashMap<Skill, Box<dyn Fn(&i32) -> i32>>;
pub type Boost = HashMap<Skill, i32>;
