use crate::bonus::MonsterBonus;

#[derive(Debug, PartialEq, Eq)]
pub struct Monster {
    pub name: String,
    pub monster_bonus: MonsterBonus,
}
