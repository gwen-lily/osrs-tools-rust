use crate::{
    bonus::{BonusLike, BonusStats},
    levels::Levels,
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

/** Level Modifiers (LvlMod) apply directly to levels, flooring in between applications.
 */
pub trait LvlMod {
    fn levels_mod(&self) -> Option<Levels>;
}

/** Gear Modifiers (GearMod) apply to gear (or monster gear-like) bonuses (Agg, Def, pry). Gear
 * Modifiers are quite rare and I can only think of the Dinh's bulwark strength bonus offhand.
 */
pub trait BonusMod<T: BonusLike> {
    fn bonus_mod(&self) -> Option<BonusStats>;
}

/** Damage Buffs (DmgBuff) are directly added to max hits, usually as a final step, after damage
 * modifiers are applied.
 */
pub trait DmgBuff {
    fn damage_buff(&self) -> Option<i32>;
}
