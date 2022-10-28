use crate::{
    bonus::MonsterBonus,
    data::{MonsterAttribute, MonsterLocation, Slayer},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Monster {
    pub name: String,
    pub monster_bonus: MonsterBonus,
    pub location: MonsterLocation,
    pub slayer: Option<Slayer>,
    pub attributes: Option<Vec<MonsterAttribute>>,
}
