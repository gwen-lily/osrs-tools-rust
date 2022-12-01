pub mod prayers;

use crate::{
    data::{
        CombatAspect::{Attack, Strength},
        DT::{self, Typeless},
    },
    CombatMap, OsrsError, Result,
};

/// A Prayer defines the drain effect and optional stat modifiers
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Prayer {
    pub drain_effect: u32,
    pub prayer_stats: Option<PrayerStats>,
}

/** A Prayers contains a collection of prayers. During creation, additional
 *  information aggregate information is collected and stored as part of the struct
 *  which is made available by the PrayerLike trait
 */
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Prayers {
    pub prayers: Vec<&'static Prayer>,
    pub drain_effect: u32,
    pub prayer_stats: Option<PrayerStats>,
}

/// A PrayerStats map matches (DT, Skill) keys to integer representations of modifiers
pub type PrayerStats = CombatMap<i32>;

/// Implements creation methods for Prayer
impl Prayer {
    /// Use new for Prayer structs with associated prayer stats
    pub fn new(drain_effect: u32, prayer_stats: Option<PrayerStats>) -> Self {
        Self {
            drain_effect,
            prayer_stats,
        }
    }

    /// Use new_statless for Prayer structs with no associated prayer stats
    pub fn new_statless(drain_effect: u32) -> Self {
        Self {
            drain_effect,
            ..Default::default()
        }
    }
}

/// Implements creation method for Prayers and validates input under the hood
impl Prayers {
    pub fn new(prayers: Vec<&'static Prayer>) -> Result<Self> {
        let mut prayer_stats = PrayerStats::new();

        Prayers::aggregate_prayer_stats(&prayers, &mut prayer_stats)?;
        let drain_effect = Prayers::calculate_prayer_drain(&prayers);

        Ok(Self {
            prayers,
            drain_effect,
            prayer_stats: Some(prayer_stats),
        })
    }

    /// The aggregate of prayer stats should be unique from prayers[stat] -> map[stat]
    fn aggregate_prayer_stats(prayers: &[&'static Prayer], map: &mut PrayerStats) -> Result<()> {
        // Iterate through the prayers in the collection
        for prayer in prayers.iter() {
            // For each prayer that has prayer_stats
            if let Some(prayer_stats) = &prayer.prayer_stats {
                // iterate through { (DT, CombatAspect): prayer_stats } pairs
                for ((dt, aspect), ps) in prayer_stats.iter() {
                    // Panic if typeless attribute, else continue
                    if *dt == Typeless {
                        panic!("bad prayer!")
                    }

                    // If a previous prayer already inserted this key, panic
                    if let Some(_prev_ps) = map.insert((*dt, *aspect), *ps) {
                        return Err(OsrsError::CombatMap(Some(vec![*dt]), Some(vec![*aspect])));
                    };
                }
            };
        }

        // Check for compatability
        Prayers::check_compatability(map)?;

        Ok(())
    }

    /** Iterate again through the finished collection.  If we have more than one DT with an attack
     *  or strength modifier, panic! This does not apply to defence, as magic and normal defence
     *  can coexist.
     */
    fn check_compatability(map: &PrayerStats) -> Result<()> {
        let no_dupes = [Attack, Strength];

        for aspect in no_dupes.into_iter() {
            let keys: Vec<DT> = map
                .keys()
                .filter(|(_dt, sk)| *sk == aspect)
                .map(|(dt, _sk)| *dt)
                .collect();

            if keys.len() > 1 {
                return Err(OsrsError::CombatMap(Some(keys), Some(vec![aspect])));
            }
        }
        Ok(())
    }

    /// Return the sum of each individual prayer drain effect
    fn calculate_prayer_drain(prayers: &[&'static Prayer]) -> u32 {
        let mut val: u32 = 0;

        for prayer in prayers.iter() {
            val += prayer.drain_effect;
        }

        val
    }
}

/// Default prayer has no drain and no stat effects
impl Default for Prayer {
    fn default() -> Self {
        Self {
            drain_effect: 0,
            prayer_stats: None,
        }
    }
}
