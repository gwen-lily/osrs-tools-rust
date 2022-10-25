use crate::prayer::{Prayer, PrayerStats};
use std::collections::HashMap;
use strum_macros::EnumIter;

use crate::data::{
    Skill::{Attack, Defence, Strength},
    DT::{Magic, Melee, Ranged},
};

/// A list of all of the names of prayers in OSRS. These are used as lookup keys for a PrayerMap.
#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PrayerName {
    ThickSkin,
    BurstOfStrength,
    ClarityOfThought,
    SharpEye,
    MysticWill,
    // RockSkin,
    // SuperhumanStrength,
    // ImprovedReflexes,
    // RapidRestore,
    // RapidHeal,
    // ProtectItem,
    // HawkEye,
    // MysticLore,
    // SteelSkin,
    // UltimateStrength,
    // IncredibleReflexes,
    ProtectFromMagic,
    ProtectFromMissiles,
    ProtectFromMelee,
    EagleEye,
    MysticMight,
    Retribution,
    Redemption,
    Smite,
    Preserve,
    Chivalry,
    Piety,
    Rigour,
    Augury,
}

/// A PrayerMap has PrayerName keys which yield the associated Prayer struct.
pub type PrayerMap = HashMap<PrayerName, Prayer>;

/// Return a PrayerMap of every prayer in OSRS.
pub fn get_all_prayers() -> PrayerMap {
    use PrayerName::*;

    let mut prayers = PrayerMap::new();

    // key: PrayerName
    // de: drain_effect
    // ps: prayer_stats
    let key = ThickSkin;
    let de = 3;
    let mut ps = PrayerStats::new();
    ps.insert((Melee(Default::default()), Defence), 5);
    prayers.insert(key, Prayer::new(de, Some(ps)));

    let key = BurstOfStrength;
    let de = 3;
    let mut ps = PrayerStats::new();
    ps.insert((Melee(Default::default()), Strength), 5);
    prayers.insert(key, Prayer::new(de, Some(ps)));

    let key = ClarityOfThought;
    let de = 3;
    let mut ps = PrayerStats::new();
    ps.insert((Melee(Default::default()), Attack), 5);
    prayers.insert(key, Prayer::new(de, Some(ps)));

    let key = SharpEye;
    let de = 3;
    let mut ps = PrayerStats::new();
    ps.insert((Ranged, Attack), 5);
    prayers.insert(key, Prayer::new(de, Some(ps)));

    let key = MysticWill;
    let de = 3;
    let mut ps = PrayerStats::new();
    ps.insert((Magic, Attack), 5);
    prayers.insert(key, Prayer::new(de, Some(ps)));

    // Protection prayers have no PrayerStats
    let key = ProtectFromMagic;
    let de = 12;
    prayers.insert(key, Prayer::new_statless(de));

    //
    let key = ProtectFromMissiles;
    let de = 12;
    prayers.insert(key, Prayer::new_statless(de));

    //
    let key = ProtectFromMelee;
    let de = 12;
    prayers.insert(key, Prayer::new_statless(de));

    let key = EagleEye;
    let de = 12;
    let mut ps = PrayerStats::new();
    ps.insert((Ranged, Attack), 15);
    prayers.insert(key, Prayer::new(de, Some(ps)));

    let key = MysticMight;
    let de = 12;
    let mut ps = PrayerStats::new();
    ps.insert((Magic, Attack), 15);
    ps.insert((Magic, Defence), 15);
    prayers.insert(key, Prayer::new(de, Some(ps)));

    let key = Retribution;
    let de = 3;
    prayers.insert(key, Prayer::new_statless(de));

    let key = Redemption;
    let de = 6;
    prayers.insert(key, Prayer::new_statless(de));

    let key = Smite;
    let de = 18;
    prayers.insert(key, Prayer::new_statless(de));

    let key = Preserve;
    let de = 2;
    prayers.insert(key, Prayer::new_statless(de));

    let key = Chivalry;
    let de = 24;
    let mut ps = PrayerStats::new();
    ps.insert((Melee(Default::default()), Attack), 15);
    ps.insert((Melee(Default::default()), Strength), 18);
    ps.insert((Melee(Default::default()), Defence), 20);
    prayers.insert(key, Prayer::new(de, Some(ps)));

    let key = Piety;
    let de = 24;
    let mut ps = PrayerStats::new();
    ps.insert((Melee(Default::default()), Attack), 20);
    ps.insert((Melee(Default::default()), Strength), 23);
    ps.insert((Melee(Default::default()), Defence), 25);
    prayers.insert(key, Prayer::new(de, Some(ps)));

    let key = Rigour;
    let de = 24;
    let mut ps = PrayerStats::new();
    ps.insert((Ranged, Attack), 20);
    ps.insert((Ranged, Strength), 23);
    ps.insert((Melee(Default::default()), Defence), 25);
    prayers.insert(key, Prayer::new(de, Some(ps)));

    let key = Augury;
    let de = 24;
    let mut ps = PrayerStats::new();
    ps.insert((Magic, Attack), 25);
    ps.insert((Magic, Defence), 25);
    ps.insert((Melee(Default::default()), Defence), 25);
    prayers.insert(key, Prayer::new(de, Some(ps)));

    prayers
}
