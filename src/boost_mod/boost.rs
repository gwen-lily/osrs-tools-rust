use crate::{data::Skill, levels::Levels};
use std::collections::HashMap;

/// Type alias for HashMap<Skill, i32>
pub type BoostMap = HashMap<Skill, Box<dyn Fn(&i32) -> i32>>;
pub type Boost = HashMap<Skill, i32>;

pub trait Boostable {
    fn boost(&self, levels: &Levels) -> Levels;
}

impl Boostable for BoostMap {
    fn boost(&self, levels: &Levels) -> Levels {
        let mut boost: Boost = Boost::new();

        for (skill, fun) in self.iter() {
            if let Some(lvl) = levels.get(skill) {
                let bst: i32 = fun(lvl);
                if let Some(val) = boost.insert(*skill, bst) {
                    panic!("Multiple boosts apply to the same level. {}", val);
                }
            }
        }

        boost
    }
}
