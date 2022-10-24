use crate::{
    bonus::gear_mod::gear::Slot,
    bonus::gear_mod::{equipment::Equipment, gear::Gear},
    boost_mod::boost::Boost,
    combat_mod::combat::multiply_then_trunc,
    data::{Skill, Slayer, DT},
    effect_mod::effect::EffectLike,
    levels::Levels,
    prayer_mod::prayer::{Prayer, Prayers},
    spell_mod::spell::{Spell, Spellbook},
    style_mod::style::Style,
};

/** The Player struct describes a player in OSRS.
 */
pub struct Player {
    equipment: Equipment,
    style: &'static Style,
    spell: Option<&'static Spell>,
    levels: Levels,
    prayers: Option<Prayers>,
    boosts: Boost,
    effects: Vec<Box<dyn EffectLike>>,
    kandarin_hard: bool,
    special_energy: u32,
    run_energy: u32,
    slayer_task: Option<Slayer>,
}

impl Player {
    pub fn weapon(&self) -> &Gear {
        if let Some(gear) = self.equipment.get(&Slot::Weapon) {
            gear
        } else {
            todo!() // &Gear::hands()
        }
    }

    pub fn attack_speed(&self) -> u8 {
        let mut atk_spd: u8 = self.weapon().weapon.as_ref().unwrap().base_attack_speed;
        let stance_modifier: i8 = self.style.attack_speed_mod.unwrap_or(0);
        atk_spd = ((atk_spd as i8) + stance_modifier) as u8;

        if let Some(spl) = self.spell {
            if spl.spellbook == Spellbook::Standard {
                todo!();
                // if self.weapon() == HarmonisedNightmareStaff {
                //     atk_spd -= 1;
                // }
            }
        }

        atk_spd
    }

    pub fn pray(&mut self, prayer: &'static Prayer) {
        match &self.prayers {
            Some(prys) => {
                let mut new_prayers = vec![prayer];
                for pry in prys.prayers.iter() {
                    new_prayers.insert(0, pry);
                }

                self.prayers = Some(Prayers::new(new_prayers))
            }
            None => {
                self.prayers = Some(Prayers::new(vec![prayer]));
            }
        }
    }

    pub fn get_visible_level(&self, skill: Skill) -> i32 {
        let minimum_visible_level: i32 = 1;
        let maximum_visible_level: i32 = 125;

        let mut visible_level: i32 =
            self.levels.get(&skill).unwrap() + self.boosts.get(&skill).unwrap_or(&0);

        visible_level = visible_level
            .max(minimum_visible_level)
            .min(maximum_visible_level);

        visible_level
    }

    pub fn get_invisible_level(&self, dt: DT, skill: Skill) -> i32 {
        let mut invisible_level: i32 = self.get_visible_level(skill);

        if let Some(prys) = &self.prayers {
            if let Some(prayer_stats) = &prys.prayer_stats {
                if let Some(pray_mod) = prayer_stats.get(&(dt, skill)) {
                    let pray_mod_f64 = (100 + pray_mod) as f64 / 100.0;
                    invisible_level = multiply_then_trunc(invisible_level, pray_mod_f64);
                }
            }
        }
        invisible_level
    }
}
