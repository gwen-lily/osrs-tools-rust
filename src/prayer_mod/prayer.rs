use std::{collections::HashMap, fmt::Error};

use crate::data::{
    Skill::{self, Attack, Defence, Strength},
    DT::{self, Typeless},
};

/// A Prayer defines the drain effect and optional stat modifiers
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Prayer {
    drain_effect: u32,
    prayer_stats: Option<PrayerStats>,
}

/** A PrayerCollection contains a collection of prayers. During creation, additional
 *  information aggregate information is collected and stored as part of the struct
 *  which is made available by the PrayerLike trait
 */
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PrayerCollection {
    pub prayers: Vec<Prayer>,
    drain_effect: u32,
    prayer_stats: Option<PrayerStats>,
}

/// A PrayerStats map matches (DT, Skill) keys to integer representations of modifiers
pub type PrayerStats = HashMap<(DT, Skill), i32>;

/// The PrayerLike trait provides access to associated prayer fields
pub trait PrayerLike {
    fn get_drain_effect(&self) -> u32;
    fn get_prayer_stats(&self) -> &Option<PrayerStats>;
}

/// Simple getters for Prayer
impl PrayerLike for Prayer {
    fn get_drain_effect(&self) -> u32 {
        self.drain_effect
    }

    fn get_prayer_stats(&self) -> &Option<PrayerStats> {
        &self.prayer_stats
    }
}

/// Simple getters for PrayerCollection
impl PrayerLike for PrayerCollection {
    fn get_drain_effect(&self) -> u32 {
        self.drain_effect
    }

    fn get_prayer_stats(&self) -> &Option<PrayerStats> {
        &self.prayer_stats
    }
}

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

/// Implements creation method for PrayerCollection and validates input under the hood
impl PrayerCollection {
    #[allow(dead_code)]
    pub fn new(prayers: Vec<Prayer>) -> Self {
        let mut prayer_stats = PrayerStats::new();

        PrayerCollection::aggregate_prayer_stats(&prayers, &mut prayer_stats);
        let drain_effect = PrayerCollection::calculate_prayer_drain(&prayers);

        Self {
            prayers,
            drain_effect,
            prayer_stats: Some(prayer_stats),
        }
    }

    /// The aggregate of prayer stats should be unique from prayers[stat] -> map[stat]
    fn aggregate_prayer_stats(prayers: &[Prayer], map: &mut PrayerStats) {
        // Iterate through the prayers in the collection
        for prayer in prayers.iter().filter(|p| p.prayer_stats != None) {
            // For each prayer that has prayer_stats
            // iterate through { (DT, Skill): prayer_stats } pairs

            for ((dt, skill), ps) in prayer.get_prayer_stats().as_ref().unwrap().iter() {
                // Panic if typeless attribute, else continue
                if dt == &Typeless {
                    panic!("bad prayer!")
                }
                // Panic if unrelated to existing prayer attributes
                match skill {
                    Attack => {}
                    Strength => {}
                    Defence => {}
                    _ => panic!("bad prayer"),
                }
                // If a previous prayer already inserted this key, panic
                if map.insert((*dt, *skill), *ps).is_some() {
                    panic!("bad prayer!")
                }
            }
        }

        // Check for compatability
        if PrayerCollection::check_compatability(map).is_err() {
            panic!("bad prayer collection, incompatible");
        }
    }

    /** Iterate again through the finished collection.  If we have more than one DT with an attack
     *  or strength modifier, panic! This does not apply to defence, as magic and normal defence
     *  can coexist.
     */
    fn check_compatability(map: &PrayerStats) -> Result<(), Error> {
        let no_dupes = [Attack, Strength];

        for skill in no_dupes.into_iter() {
            let skill_mods: usize = map.keys().filter(|(_, sk)| *sk == skill).count();

            if skill_mods > 1 {
                return Err(std::fmt::Error);
            }
        }
        Ok(())
    }

    /// Return the sum of each individual prayer drain effect
    fn calculate_prayer_drain(prayers: &[Prayer]) -> u32 {
        let mut val: u32 = 0;

        for prayer in prayers.iter() {
            val += prayer.get_drain_effect();
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