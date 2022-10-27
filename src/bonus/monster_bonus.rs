use super::{BonusLike, BonusStats};

/** Gear struct which represents a single Gear item. Gear implements BonusLike. Gear derives
 *  default behavior, which yields a slotless husk with no bonuses or requirements
 */
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct MonsterBonus {
    bonus_stats: BonusStats,
}

impl BonusLike for MonsterBonus {
    fn get_bonus_stats(&self) -> &BonusStats {
        &self.bonus_stats
    }
}
