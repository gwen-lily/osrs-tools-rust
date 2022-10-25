use crate::data::{
    Skill::{self, Attack, Defence, Strength},
    DT,
};

use std::collections::HashMap;

#[allow(unused_imports)]
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// An enum with the combat options and sub-stance options.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Stance {
    Melee(MeleeStance),
    Ranged(RangedStance),
    Magic(MagicStance),
}

/// All melee stances
#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MeleeStance {
    Accurate,
    Aggressive,
    Controlled,
    Defensive,
}

/// All ranged stances
#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RangedStance {
    Accurate,
    Rapid,
    Longrange,
}

/// All magic stances
#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MagicStance {
    Accurate,
    Longrange,
    NoStyle,
    Defensive,
}

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
fn add_melee_stances(map: &mut StanceMap) {
    use MeleeStance::*;
    use Stance::Melee;

    let key = Melee(Accurate);
    let mut val = StanceStats::new();
    val.insert((DT::Melee(Default::default()), Attack), 3);
    map.insert(key, Some(val));

    let key = Melee(Aggressive);
    let mut val = StanceStats::new();
    val.insert((DT::Melee(Default::default()), Strength), 3);
    map.insert(key, Some(val));

    let key = Melee(Controlled);
    let mut val = StanceStats::new();
    val.insert((DT::Melee(Default::default()), Attack), 1);
    val.insert((DT::Melee(Default::default()), Strength), 1);
    val.insert((DT::Melee(Default::default()), Defence), 1);
    map.insert(key, Some(val));

    let key = Melee(Defensive);
    let mut val = StanceStats::new();
    val.insert((DT::Melee(Default::default()), Defence), 3);
    map.insert(key, Some(val));
}

/// Add all ranged stances to the StanceMap
fn add_ranged_stances(map: &mut StanceMap) {
    use RangedStance::*;
    use Stance::Ranged;

    let key = Ranged(Accurate);
    let mut val = StanceStats::new();
    val.insert((DT::Ranged, Attack), 3);
    val.insert((DT::Ranged, Strength), 3);
    map.insert(key, Some(val));

    let key = Ranged(Rapid);
    map.insert(key, None);

    let key = Ranged(Longrange);
    map.insert(key, None);
}

/// Add all magic stances to the StanceMap
fn add_magic_stances(map: &mut StanceMap) {
    use MagicStance::*;
    use Stance::Magic;

    let key = Magic(Accurate);
    let mut val = StanceStats::new();
    val.insert((DT::Magic, Attack), 3);
    map.insert(key, Some(val));

    let key = Magic(Longrange);
    let mut val = StanceStats::new();
    val.insert((DT::Magic, Attack), 1);
    val.insert((DT::Melee(Default::default()), Defence), 3);
    map.insert(key, Some(val));

    let key = Magic(NoStyle);
    map.insert(key, None);

    let key = Magic(Defensive);
    let mut val = StanceStats::new();
    val.insert((DT::Melee(Default::default()), Defence), 3);
    map.insert(key, Some(val));
}
