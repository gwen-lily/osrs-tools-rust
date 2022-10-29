use crate::{
    bonus::{BonusLike, BonusStats},
    levels::Levels,
    CombatMap,
};

pub mod player;

pub(crate) use player::PlayerModifiers;

/** Roll Modifiers (ArMod) apply directly to maximum rolls for offence and defence, flooring in
 *  between applications.
 */
pub trait ArMod {
    fn accuracy_roll_mod(&self) -> Option<f64>;
}

pub trait ArIntMod {
    fn accuracy_roll_int_mod(&self) -> Option<i32>;
}

/** Damage Modifiers (DmgMod) apply directly to maximum hit calculations, flooring in between
 *  applications.
 */
pub trait DmgMod {
    fn damage_mod(&self) -> Option<f64>;
}

/** Level Modifiers (LvlMod) apply directly to levels, flooring in between applications.
 */
pub trait LvlMod {
    fn levels_mod(&self) -> Option<Levels>;
}

/// Combat Modifiers (CmbMod) are float modifiers applied directly to (). Void comes to mind.
pub trait CmbMod {
    fn combat_mod(&self) -> Option<CombatMap<f64>>;
}

/** Gear Modifiers (BnsMod) apply to gear (or monster gear-like) bonuses (Agg, Def, pry). Gear
 * Modifiers are quite rare and I can only think of the Dinh's bulwark strength bonus offhand.
 */
pub trait BnsMod {
    fn bonus_mod(&self) -> Option<BonusStats>;
}

/// Bonus
pub trait TumMod<T: BonusLike> {
    fn bonus_mod(&self) -> Option<CombatMap<f64>>;
}

/** Damage Buffs (DmgBuff) are directly added to max hits, usually as a final step, after damage
 * modifiers are applied. Or as a powered spell!
 */
pub trait DmgBuff {
    fn damage_buff(&self) -> Option<u8>;
}

/// Activation trait for bundling logic shared between different modifier implementations
pub(crate) trait ConditionalModifier {
    /// Returns true when the conditions are met, as needed by the struct
    fn activate(&self) -> bool;
}
