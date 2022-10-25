/** Roll Modifiers (ArMod) apply directly to maximum rolls for offence and defence, flooring in
 *  between applications.
 */
pub trait ArMod {
    fn roll_mod(&self) -> Option<f64>;
}

/** Damage Modifiers (DmgMod) apply directly to maximum hit calculations, flooring in between
 *  applications.
 */
pub trait DmgMod {
    fn dmg_mod(&self) -> Option<f64>;
}

/** Level Modifiers (LvlMod) apply directly to levels, flooring in between applications.
 */
pub trait LvlMod {
    fn lvl_mod(&self) -> Option<f64>;
}

/** Gear Modifiers (GearMod) apply to gear (or monster gear-like) bonuses (Agg, Def, pry). Gear
 * Modifiers are quite rare and I can only think of the Dinh's bulwark strength bonus offhand.
 */
pub trait GearMod {
    fn gear_mod(&self) -> Option<i32>;
}

/** Damage Buffs (DmgBuff) are directly added to max hits, usually as a final step, after damage
 * modifiers are applied.
 */
pub trait DmgBuff {
    fn dmg_buff(&self) -> Option<i32>;
}
