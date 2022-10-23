use num::integer::Integer;
use num::Bounded;
use num::Num;
use num::NumCast;

/** The roll made by a character to determine accuracy calculations, both for offensive and
 * defensive rolls.
 */
pub fn maximum_roll(level: &i32, bonus: &i32, roll_modifiers: &[f64]) -> i32 {
    let mut roll: i32 = level * (bonus + 64);

    for roll_mod in roll_modifiers.iter() {
        roll = multiply_then_trunc(roll, *roll_mod)
    }

    roll
}

/** The probability of a "successful" attack. This is not to be confused with the chance to deal
 * damage, and there are subtleties between the two.
 */
pub fn accuracy(offensive_roll: &i32, defensive_roll: &i32) -> f64 {
    if offensive_roll > defensive_roll {
        let num: f64 = (*defensive_roll as f64) + 2.0;
        let den: f64 = (2 * (offensive_roll + 1)) as f64;
        1.0 - num / den
    } else {
        let num: f64 = *offensive_roll as f64;
        let den: f64 = (2 * (defensive_roll + 1)) as f64;
        num / den
    }
}

/** Bitterkoekje damage formula. Source:
 *  https://docs.google.com/document/d/1hk7FxOAOFT4oxguC8411QQhE4kk-_GzqWcwkaPmaYns/edit
 */
pub fn base_damage(effective_strength_level: &i32, strength_bonus: &i32) -> f64 {
    let bd: f64 = 0.5 + (*effective_strength_level as f64) * (strength_bonus + 64) as f64 / 640.0;
    bd
}

/// Multiply a base damage calculation by damage modifiers, flooring in between
pub fn max_hit(base_damage: f64, damage_modifiers: &[f64]) -> u32 {
    let mut max_hit: u32 = base_damage.trunc() as u32;

    for dmg_mod in damage_modifiers.iter() {
        max_hit = multiply_then_trunc(max_hit, *dmg_mod);
    }

    max_hit
}

/// Effective level is invisible level plus optional stance and void modifiers
pub fn effective_level(
    invisible_level: i32,
    stance_bonus: Option<i32>,
    void_modifier: Option<f64>,
) -> i32 {
    // Invisible 8
    let mut eff_lvl: i32 = invisible_level + 8;

    if let Some(stance) = stance_bonus {
        eff_lvl += stance;
    }

    if let Some(void) = void_modifier {
        eff_lvl = multiply_then_trunc(eff_lvl, void);
    }

    eff_lvl
}

/// Return the truncated multiplication of an integer and float as the same type as integer.
pub fn multiply_then_trunc<T: Integer + Bounded + Num + NumCast + PartialOrd + Clone>(
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
