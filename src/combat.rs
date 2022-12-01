pub(crate) mod damage_analysis;
pub(crate) mod damage_distribution;
pub(crate) mod max_hit;

use num::integer::Integer;
use num::Bounded;
use num::Num;
use num::NumCast;

pub use damage_distribution::{DamageDistribution, Hitsplats};
pub use max_hit::MaxHit;

use crate::bonus::GearName;
use crate::modifiers::PlayerModifiers;

/** The roll made by a character to determine accuracy calculations, both for offensive and
 * defensive rolls.
 */
pub fn maximum_roll(level: u32, bonus: i32, roll_modifiers: &[f64]) -> u32 {
    let unclamped_roll: i32 = level as i32 * (bonus + 64);
    // Roll must be non-negative. Bonus beyond -64 confers no disadvantage
    let mut roll: u32 = if unclamped_roll < 0 {
        0
    } else {
        unclamped_roll as u32
    };

    for roll_mod in roll_modifiers.iter() {
        roll = multiply_then_trunc(roll, *roll_mod)
    }

    roll
}

/** The probability of a "successful" attack. This is not to be confused with the chance to deal
 * damage, and there are subtleties between the two.
 */
pub fn accuracy(offensive_roll: u32, defensive_roll: u32) -> f64 {
    if offensive_roll > defensive_roll {
        let num: f64 = (defensive_roll as f64) + 2.0;
        let den: f64 = (2 * (offensive_roll + 1)) as f64;
        1.0 - num / den
    } else {
        let num: f64 = offensive_roll as f64;
        let den: f64 = (2 * (defensive_roll + 1)) as f64;
        num / den
    }
}

/** Bitterkoekje damage formula. Source:
 *  <https://docs.google.com/document/d/1hk7FxOAOFT4oxguC8411QQhE4kk-_GzqWcwkaPmaYns/edit>
 */
pub fn base_damage(effective_strength_level: u32, strength_bonus: i32) -> f64 {
    0.5 + (effective_strength_level as f64) * (strength_bonus + 64) as f64 / 640.0
}

/// Multiply a base damage calculation by damage modifiers, flooring in between
pub fn max_hit(base_damage: u32, damage_modifiers: &[f64]) -> u32 {
    let mut max_hit: u32 = base_damage as u32;

    for dmg_mod in damage_modifiers.iter() {
        max_hit = multiply_then_trunc(max_hit, *dmg_mod);
    }

    max_hit
}

/// Effective level is invisible level plus optional stance and void modifiers
pub fn effective_level(
    invisible_level: u32,
    stance_bonus: Option<i32>,
    void_modifier: Option<f64>,
) -> u32 {
    // Invisible 8
    let mut eff_lvl: u32 = invisible_level + 8;

    if let Some(stance) = stance_bonus {
        eff_lvl = ((eff_lvl as i32) + stance) as u32;
    }

    if let Some(void) = void_modifier {
        eff_lvl = multiply_then_trunc(eff_lvl, void);
    }

    eff_lvl
}

/// Return the truncated multiplication of an integer and float as the same type as integer.
pub(crate) fn multiply_then_trunc<T: Integer + Bounded + Num + NumCast + PartialOrd + Clone>(
    value: T,
    modifier: f64,
) -> T
where
    f64: From<T>,
{
    let res_f64: f64 = <f64 as std::convert::From<T>>::from(value) * modifier;
    let res_int = T::from(res_f64.trunc());

    res_int.unwrap()
}

/// Return the proc chance of a crossbow activating its bolt effect
pub(crate) fn crossbow_proc_chance(base_chance: f64, pmods: &PlayerModifiers) -> f64 {
    use GearName::*;
    let weapon_name: GearName = pmods.player.weapon().gear_info.name;

    let mut procc: f64 = base_chance;

    if pmods.player.kandarin_hard {
        procc += procc / 10.0
    };

    if *pmods.special_attack {
        match weapon_name {
            ArmadylCrossbow => procc *= 2.0,
            ZaryteCrossbow => procc = 1.0,
            _ => {}
        }
    };

    procc
}
