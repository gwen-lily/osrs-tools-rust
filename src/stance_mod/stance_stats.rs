use crate::data::{
    Skill::{self, Attack, Defence, Strength},
    DT,
};
use crate::stance_mod::stance::{Stance::*, *};
use std::collections::HashMap;

/// A StanceStats map matches (DT, Skill) keys to literal integer modifiers.
pub type StanceStats = HashMap<(DT, Skill), i32>;
pub type StanceMap = HashMap<Stance, Option<StanceStats>>;

/// Return a StanceMap of all the possible Stances
pub fn get_all_stances() -> StanceMap {
    let mut map: StanceMap = StanceMap::new();

    add_melee_stances(&mut map);
    add_ranged_stances(&mut map);
    add_magic_stances(&mut map);

    map
}

/// Add all melee stances to the StanceMap
pub fn add_melee_stances(map: &mut StanceMap) {
    let key = Melee(MeleeStance::Accurate);
    let mut val = StanceStats::new();
    val.insert((DT::Melee(Default::default()), Attack), 3);
    map.insert(key, Some(val));

    let key = Melee(MeleeStance::Aggressive);
    let mut val = StanceStats::new();
    val.insert((DT::Melee(Default::default()), Strength), 3);
    map.insert(key, Some(val));

    let key = Melee(MeleeStance::Controlled);
    let mut val = StanceStats::new();
    val.insert((DT::Melee(Default::default()), Attack), 1);
    val.insert((DT::Melee(Default::default()), Strength), 1);
    val.insert((DT::Melee(Default::default()), Defence), 1);
    map.insert(key, Some(val));

    let key = Melee(MeleeStance::Defensive);
    let mut val = StanceStats::new();
    val.insert((DT::Melee(Default::default()), Defence), 3);
    map.insert(key, Some(val));
}

/// Add all ranged stances to the StanceMap
pub fn add_ranged_stances(map: &mut StanceMap) {
    let key = Ranged(RangedStance::Accurate);
    let mut val = StanceStats::new();
    val.insert((DT::Ranged, Attack), 3);
    val.insert((DT::Ranged, Strength), 3);
    map.insert(key, Some(val));

    let key = Ranged(RangedStance::Rapid);
    map.insert(key, None);

    let key = Ranged(RangedStance::Longrange);
    map.insert(key, None);
}

/// Add all magic stances to the StanceMap
pub fn add_magic_stances(map: &mut StanceMap) {
    let key = Magic(MagicStance::Accurate);
    let mut val = StanceStats::new();
    val.insert((DT::Magic, Attack), 3);
    map.insert(key, Some(val));

    let key = Magic(MagicStance::Longrange);
    let mut val = StanceStats::new();
    val.insert((DT::Magic, Attack), 1);
    val.insert((DT::Melee(Default::default()), Defence), 3);
    map.insert(key, Some(val));

    let key = Magic(MagicStance::NoStyle);
    map.insert(key, None);

    let key = Magic(MagicStance::Defensive);
    let mut val = StanceStats::new();
    val.insert((DT::Melee(Default::default()), Defence), 3);
    map.insert(key, Some(val));
}
