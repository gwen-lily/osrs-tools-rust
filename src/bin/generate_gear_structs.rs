use std::{collections::HashMap, env, error::Error, ffi::OsString};

use osrs_tools::{
    bonus::{
        gear::{Gear, GearMap, GearName, Slot, SpecialWeapon, Weapon},
        Agg, AttackMeleeAgg, Def, MagicAgg, MeleeAgg, MeleeDef, RangedAgg,
    },
    data::{MeleeDamageType, Skill, DT},
    levels::Levels,
    style::{
        styles_map::StylesCategory::{self, *},
        Styles,
    },
    STYLES_MAP,
};
use serde::Deserialize;

#[macro_use]
extern crate lazy_static;

/**The Bitterkoekje item container, which is a bit of a tough thing. There are a lot of empty
 * fields in the data and entire blank lines, so we have to be smart about collecting data. A few
 * things to look out for: blank lines, magic spells, proposed gear, etc...
 */
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ItemRecord {
    #[serde(rename = "", deserialize_with = "csv::invalid_option")]
    name: Option<String>,
    // attack secondary bonuses
    #[serde(rename = "Stab attack", deserialize_with = "csv::invalid_option")]
    stab_attack: Option<i32>,
    #[serde(rename = "Slash attack", deserialize_with = "csv::invalid_option")]
    slash_attack: Option<i32>,
    #[serde(rename = "Crush attack", deserialize_with = "csv::invalid_option")]
    crush_attack: Option<i32>,
    #[serde(rename = "Magic attack", deserialize_with = "csv::invalid_option")]
    magic_attack: Option<i32>,
    #[serde(rename = "Ranged attack", deserialize_with = "csv::invalid_option")]
    ranged_attack: Option<i32>,
    // defence secondary bonuses
    #[serde(rename = "Stab defence", deserialize_with = "csv::invalid_option")]
    stab_defence: Option<i32>,
    #[serde(rename = "Slash defence", deserialize_with = "csv::invalid_option")]
    slash_defence: Option<i32>,
    #[serde(rename = "Crush defence", deserialize_with = "csv::invalid_option")]
    crush_defence: Option<i32>,
    #[serde(rename = "Magic defence", deserialize_with = "csv::invalid_option")]
    magic_defence: Option<i32>,
    #[serde(rename = "Ranged defence", deserialize_with = "csv::invalid_option")]
    ranged_defence: Option<i32>,
    // strength bonuses
    #[serde(rename = "Melee strength", deserialize_with = "csv::invalid_option")]
    melee_strength: Option<i32>,
    #[serde(rename = "Ranged strength", deserialize_with = "csv::invalid_option")]
    ranged_strength: Option<i32>,
    #[serde(rename = "Magic damage", deserialize_with = "csv::invalid_option")]
    // prayer
    magic_damage: Option<f64>,
    #[serde(rename = "Prayer", deserialize_with = "csv::invalid_option")]
    prayer: Option<u32>,
    // special weapon fields
    #[serde(rename = "Special accuracy", deserialize_with = "csv::invalid_option")]
    special_accuracy: Option<f64>,
    #[serde(rename = "Special damage 1", deserialize_with = "csv::invalid_option")]
    special_damage_1: Option<f64>,
    #[serde(rename = "Special damage 2", deserialize_with = "csv::invalid_option")]
    special_damage_2: Option<f64>,
    #[serde(
        rename = "Special defence roll",
        deserialize_with = "csv::invalid_option"
    )]
    special_defence_roll_str: Option<String>,
    // ignore
    #[serde(rename = "Spell max hit", deserialize_with = "csv::invalid_option")]
    spell_max_hit: Option<String>,
    // weapon aspects
    #[serde(rename = "Attack speed", deserialize_with = "csv::invalid_option")]
    attack_speed: Option<u8>,
    #[serde(rename = "Combat options", deserialize_with = "csv::invalid_option")]
    combat_options_int: Option<u8>,
    #[serde(rename = "Mining level req", deserialize_with = "csv::invalid_option")]
    mining_lvl_req_int: Option<i32>,
}

pub type CombatOptionsMap = HashMap<u8, StylesCategory>;

lazy_static! {
    static ref COMBAT_OPTIONS_MAP: CombatOptionsMap = get_combat_options_map();
}

fn get_combat_options_map() -> CombatOptionsMap {
    let mut map = HashMap::new();

    map.insert(3, UnarmedWeapons);

    // referred to as "Dagger" by bitter.
    map.insert(4, StabSwords);

    // referred to as "Sword" by bitter.
    map.insert(5, StabSwords);

    // Post-verify, as bitter calls this "long/scim/claws"
    // Likely need to do all claws
    map.insert(6, SlashSwords);

    // Referred to as "Mace/flail" by bitter.
    map.insert(7, SpikedWeapons);

    map.insert(8, Axes);

    // Referred to as "Hammer" by bitter, likely post-verify
    map.insert(9, BluntWeapons);

    map.insert(10, TwoHandedSwords);

    // Referred to as "Halberd" by bitter
    map.insert(11, Polearms);

    map.insert(12, Spears);
    map.insert(13, Whips);
    map.insert(14, Bludgeons);

    // Referred to as "Staff" by bitter
    map.insert(15, Polestaves);

    map.insert(16, Pickaxes);

    // Referred to as "Ranged" by bitter
    // post-verify for sure.
    map.insert(17, Bows);

    map.insert(18, Salamanders);

    // Referred to as "SotD" by bitter
    map.insert(19, BladedStaves);

    // Referred to as "Trident" by bitter
    map.insert(20, PoweredStaves);

    // Referred to as "2h shield" by bitter
    map.insert(21, Bulwarks);

    map.insert(22, Partisans);

    map
}

// #[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
// pub enum StylesCategory {
//     Claws,
//     Scythes,
//     Chinchompas,
//     Crossbows,
//     Thrown,
//     Staves,
// }

/// Go through the provided file and deserialize each record. Return the Vec of results.
fn get_item_records() -> Result<Vec<ItemRecord>, Box<dyn Error>> {
    let file_path = get_nth_file_path(1)?;
    let mut rdr = csv::Reader::from_path(file_path)?;

    let mut items: Vec<ItemRecord> = Vec::new();

    for result in rdr.deserialize() {
        let record: ItemRecord = result?;
        items.push(record);
    }

    Ok(items)
}

fn get_item_record_agg(record: &ItemRecord) -> Agg {
    Agg {
        melee: MeleeAgg {
            attack: AttackMeleeAgg {
                stab: record.stab_attack.unwrap_or(0),
                slash: record.slash_attack.unwrap_or(0),
                crush: record.crush_attack.unwrap_or(0),
            },
            strength: record.melee_strength.unwrap_or(0),
        },
        ranged: RangedAgg {
            attack: record.ranged_attack.unwrap_or(0),
            strength: record.ranged_strength.unwrap_or(0),
        },
        magic: MagicAgg {
            attack: record.magic_attack.unwrap_or(0),
            strength: (record.magic_damage.unwrap_or(0.0) * 100.0) as i32,
        },
    }
}

fn get_item_record_def(record: &ItemRecord) -> Def {
    Def {
        melee: MeleeDef {
            stab: record.stab_defence.unwrap_or(0),
            slash: record.slash_defence.unwrap_or(0),
            crush: record.crush_defence.unwrap_or(0),
        },
        ranged: record.ranged_defence.unwrap_or(0),
        magic: record.magic_defence.unwrap_or(0),
    }
}

/**All weapons have an options integer, which maps to the proper value some of the time.
 * However, bitter was a little fast and loose with the categories so some tinkering will be
 * needed.
 */
fn get_item_record_weapon(record: &ItemRecord) -> Result<Option<Weapon>, Box<dyn Error>> {
    let weapon: Option<Weapon>;

    if record.combat_options_int == None {
        return Ok(None);
    }

    if let Some(options_int) = record.combat_options_int {
        let styles_category: &StylesCategory =
            if let Some(sty_cat) = COMBAT_OPTIONS_MAP.get(&options_int) {
                sty_cat
            } else {
                let s: String = format!("Bad Styles int: {:#?}", record);
                return Err(From::from(s));
            };

        let styles: &Styles = if let Some(sty) = STYLES_MAP.get(styles_category) {
            sty
        } else {
            let s: String = format!("Bad StylesCategory: {:?}", styles_category);
            return Err(From::from(s));
        };

        let base_attack_speed: u8 = if let Some(bas) = record.attack_speed {
            bas
        } else {
            let s: String = format!("Bad attack speed for {:#?}", record);
            return Err(From::from(s));
        };

        weapon = Some(Weapon {
            styles,
            base_attack_speed,
            two_handed: false,
            base_attack_range: 0,
        });

        Ok(weapon)
    } else {
        panic!();
    }
}

/// Get special weapon stats, if they exist.
fn get_item_record_special_weapon(
    record: &ItemRecord,
) -> Result<Option<SpecialWeapon>, Box<dyn Error>> {
    // If no special flags, return None.
    if record.special_accuracy == None
        && record.special_damage_1 == None
        && record.special_damage_2 == None
        && record.special_defence_roll_str == None
    {
        return Ok(None);
    }

    // Accuracy
    let special_arms: Option<Vec<f64>>;

    if let Some(spec_acc_arm) = record.special_accuracy {
        let vec: Vec<f64> = vec![spec_acc_arm];
        special_arms = Some(vec);
    } else {
        special_arms = None;
    }

    let mut vec: Vec<f64> = Vec::new();

    if let Some(spec_dmg_1) = record.special_damage_1 {
        vec.push(spec_dmg_1)
    }
    if let Some(spec_dmg_2) = record.special_damage_2 {
        vec.push(spec_dmg_2)
    }

    let special_dms: Option<Vec<f64>> = if vec.is_empty() { None } else { Some(vec) };

    let special_defence_roll: Option<DT>;
    if let Some(dmg_type_str) = &record.special_defence_roll_str {
        let dt: DT = match &dmg_type_str[..] {
            "Stab" => DT::Melee(MeleeDamageType::Stab),
            "stab" => DT::Melee(MeleeDamageType::Stab),
            "Slash" => DT::Melee(MeleeDamageType::Slash),
            "slash" => DT::Melee(MeleeDamageType::Slash),
            "Crush" => DT::Melee(MeleeDamageType::Crush),
            "crush" => DT::Melee(MeleeDamageType::Crush),
            "Ranged" => DT::Ranged,
            "ranged" => DT::Ranged,
            "Magic" => DT::Magic,
            "magic" => DT::Magic,
            _ => {
                let s = format!(
                    "Bad damage type for {:?}: {:?}",
                    record.name.as_ref(),
                    dmg_type_str
                );
                return Err(From::from(s));
            }
        };
        special_defence_roll = Some(dt);
    } else {
        special_defence_roll = None;
    }

    let special_weapon = Some(SpecialWeapon {
        special_arms,
        special_dms,
        special_defence_roll,
    });

    Ok(special_weapon)
}

fn get_item_record_name(record: &ItemRecord) -> Result<GearName, Box<dyn Error>> {
    if let Some(name) = &record.name {
        #[allow(clippy::match_single_binding)]
        let gear_name: GearName = match name {
            _ => GearName::FooBarBaz,
        };

        Ok(gear_name)
    } else {
        let s = format!("No name field for: {:?}", record);
        Err(From::from(s))
    }
}

/// Transform the provided ItemRecords into their corresponding Gear
fn transform_item_record(record: ItemRecord) -> Result<Gear, Box<dyn Error>> {
    let name: GearName = get_item_record_name(&record)?;
    let slot = Slot::Head;
    let agg = get_item_record_agg(&record);
    let def = get_item_record_def(&record);
    let pry = record.prayer.unwrap_or(0);

    let mut lvl_reqs: Levels = Levels::new();
    if let Some(mining_req) = record.mining_lvl_req_int {
        lvl_reqs.insert(Skill::Mining, mining_req);
    }

    let weapon = get_item_record_weapon(&record)?;
    let special_weapon = get_item_record_special_weapon(&record)?;

    let gear = Gear::new(name, slot, agg, def, pry, lvl_reqs, weapon, special_weapon);
    Ok(gear)
}

/// Get the nth argument from the command line and return the file_path contained.
fn get_nth_file_path(n: usize) -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(n) {
        None => Err(From::from("expected an argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let item_records: Vec<ItemRecord> = get_item_records()?;
    let mut all_gear = GearMap::new();

    for record in item_records.into_iter() {
        match transform_item_record(record) {
            Ok(g) => {
                let k = g.name;
                let v = g;
                all_gear.insert(k, v);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    Ok(())
}
