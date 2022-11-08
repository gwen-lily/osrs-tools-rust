use crate::combat;
use crate::{Skill, SkillMap};

pub(crate) mod potions;

/// Type alias for HashMap<Skill, i32>
pub(crate) type BoostClosure = dyn Fn(u32) -> i32;
pub(crate) type BoostMap = SkillMap<Box<BoostClosure>>;
pub(crate) type Boost = SkillMap<i32>;

/** Potion or Boost callable creator for crate use.
 *  Most runescape boosts have the form
 *
 *  boost(lvl) = +/- (base + floor(ratio*lvl))
 */
pub(crate) fn basic_boost(base: i32, ratio: f64, positive: bool) -> impl Fn(u32) -> i32 {
    move |l: u32| -> i32 {
        let diff_val: i32 = combat::multiply_then_trunc(l, ratio) as i32 + base;
        if positive {
            diff_val
        } else {
            -diff_val
        }
    }
}

/** Further, many boosts have the same exact basic boost closure parameters across different
 * skills, so we can create many of the simplest types with this function, which returns a
 * BoostMap.
 */
pub(crate) fn basic_boost_map(base: i32, ratio: f64, positive: bool, skills: &[Skill]) -> BoostMap {
    skills
        .iter()
        .map(|&s| {
            (
                s,
                Box::new(basic_boost(base, ratio, positive)) as Box<dyn Fn(u32) -> i32>,
            )
        })
        .collect()
}
