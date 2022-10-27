use crate::{
    bonus::{BonusLike, BonusStats},
    levels::Levels,
    CombatMap,
};

pub mod player;

/** Roll Modifiers (ArMod) apply directly to maximum rolls for offence and defence, flooring in
 *  between applications.
 */
pub trait ArMod {
    fn accuracy_roll_mod(&self) -> Option<f64>;
}

/** Damage Modifiers (DmgMod) apply directly to maximum hit calculations, flooring in between
 *  applications.
 */
pub trait DmgMod {
    fn damage_mod(&self) -> Option<f64>;
}

pub trait DmgModFunction {
    fn damage_mod_fn(&self) -> Option<Box<dyn Fn(&u8) -> u8>>;
}

/** Level Modifiers (LvlMod) apply directly to levels, flooring in between applications.
 */
pub trait LvlMod {
    fn levels_mod(&self) -> Option<Levels>;
}

/// Combat Modifiers (CmbMod) are float modifiers applied directly to ()
pub trait CmbMod {
    fn combat_mod(&self) -> Option<CombatMap<f64>>;
}

/** Gear Modifiers (GearMod) apply to gear (or monster gear-like) bonuses (Agg, Def, pry). Gear
 * Modifiers are quite rare and I can only think of the Dinh's bulwark strength bonus offhand.
 */
pub trait BnsMod<T: BonusLike> {
    fn bonus_mod(&self) -> Option<BonusStats>;
}

/** Damage Buffs (DmgBuff) are directly added to max hits, usually as a final step, after damage
 * modifiers are applied.
 */
pub trait DmgBuff {
    fn damage_buff(&self) -> Option<i32>;
}
