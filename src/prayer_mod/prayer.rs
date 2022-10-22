use std::{collections::HashMap, fmt::Error};

use crate::data::{
    Skill::{self, Attack, Defence, Strength},
    DT::{self, Typeless},
};

/// Structs

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Prayer {
    drain_effect: u32,
    prayer_stats: Option<PrayerStats>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PrayerCollection {
    pub prayers: Vec<Prayer>,
    drain_effect: u32,
    prayer_stats: Option<PrayerStats>,
}

/// Type

pub type PrayerStats = HashMap<(DT, Skill), i32>;

/// trait

pub trait PrayerLike {
    fn get_drain_effect(&self) -> u32;
    fn get_prayer_stats(&self) -> &Option<PrayerStats>;
}

/// trait implementation

impl PrayerLike for Prayer {
    fn get_drain_effect(&self) -> u32 {
        self.drain_effect
    }

    fn get_prayer_stats(&self) -> &Option<PrayerStats> {
        &self.prayer_stats
    }
}

impl PrayerLike for PrayerCollection {
    fn get_drain_effect(&self) -> u32 {
        self.drain_effect
    }

    fn get_prayer_stats(&self) -> &Option<PrayerStats> {
        &self.prayer_stats
    }
}

/// Implementation

impl Prayer {
    pub fn new(drain_effect: u32, prayer_stats: Option<PrayerStats>) -> Self {
        Self {
            drain_effect,
            prayer_stats,
        }
    }

    pub fn new_statless(drain_effect: u32) -> Self {
        Self {
            drain_effect,
            ..Default::default()
        }
    }
}

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

    fn check_compatability(map: &PrayerStats) -> Result<(), Error> {
        // Iterate again through the finished collection
        // If we have more than one DT with an attack or strength modifier, panic!
        // This does not apply to defence, magic and normal defence can coexist

        let no_dupes = [Attack, Strength];

        for skill in no_dupes.into_iter() {
            let skill_mods: usize = map.keys().filter(|(_, sk)| *sk == skill).count();

            if skill_mods > 1 {
                return Err(std::fmt::Error);
            }
        }
        Ok(())
    }

    fn calculate_prayer_drain(prayers: &[Prayer]) -> u32 {
        let mut val: u32 = 0;

        for prayer in prayers.iter() {
            val += prayer.get_drain_effect();
        }

        val
    }
}

/// Default implementation

impl Default for Prayer {
    fn default() -> Self {
        Self {
            drain_effect: 0,
            prayer_stats: None,
        }
    }
}
