pub mod stat {

    /// Structs

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    pub struct PlayerLevels {
        attack: Option<i32>,
        strength: Option<i32>,
        defence: Option<i32>,
        ranged: Option<i32>,
        prayer: Option<i32>,
        magic: Option<i32>,
        runecraft: Option<i32>,
        construction: Option<i32>,
        hitpoints: Option<i32>,
        agility: Option<i32>,
        herblore: Option<i32>,
        thieving: Option<i32>,
        crafting: Option<i32>,
        fletching: Option<i32>,
        slayer: Option<i32>,
        hunter: Option<i32>,
        mining: Option<i32>,
        smithing: Option<i32>,
        fishing: Option<i32>,
        cooking: Option<i32>,
        firemaking: Option<i32>,
        woodcutting: Option<i32>,
        farming: Option<i32>,
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MonsterLevels {
        attack: Option<i32>,
        strength: Option<i32>,
        defence: Option<i32>,
        ranged: Option<i32>,
        magic: Option<i32>,
        hitpoints: Option<i32>,
    }

    /// Traits

    pub trait CombatTrait {
        fn get_attack(&self) -> &i32;
        fn get_strength(&self) -> &i32;
        fn get_defence(&self) -> &i32;
        fn get_ranged(&self) -> &i32;
        fn get_magic(&self) -> &i32;
        fn get_hitpoints(&self) -> &i32;
        fn combat_stats(&self) -> (&i32, &i32, &i32, &i32, &i32, &i32) {
            (
                self.get_attack(),
                self.get_strength(),
                self.get_defence(),
                self.get_ranged(),
                self.get_magic(),
                self.get_hitpoints(),
            )
        }
    }

    /// Trait implementation

    impl CombatTrait for PlayerLevels {
        fn get_attack(&self) -> &i32 {
            &self.attack.unwrap()
        }
        fn get_strength(&self) -> &i32 {
            &self.strength.unwrap()
        }
        fn get_defence(&self) -> &i32 {
            &self.defence.unwrap()
        }
        fn get_magic(&self) -> &i32 {
            &self.magic.unwrap()
        }
        fn get_ranged(&self) -> &i32 {
            &self.ranged.unwrap()
        }
        fn get_hitpoints(&self) -> &i32 {
            &self.hitpoints.unwrap()
        }
    }

    impl CombatTrait for MonsterLevels {
        fn get_attack(&self) -> &i32 {
            &self.attack.unwrap()
        }
        fn get_strength(&self) -> &i32 {
            &self.strength.unwrap()
        }
        fn get_defence(&self) -> &i32 {
            &self.defence.unwrap()
        }
        fn get_magic(&self) -> &i32 {
            &self.magic.unwrap()
        }
        fn get_ranged(&self) -> &i32 {
            &self.ranged.unwrap()
        }
        fn get_hitpoints(&self) -> &i32 {
            &self.hitpoints.unwrap()
        }
    }

    /// Implementation

    impl PlayerLevels {
        /// Helper functions for creating PlayerLevels objects.

        fn fresh() -> Self {
            Self {
                attack: Some(1),
                strength: Some(1),
                defence: Some(1),
                ranged: Some(1),
                prayer: Some(1),
                magic: Some(1),
                runecraft: Some(1),
                construction: Some(1),
                hitpoints: Some(10), // 10, not 1
                agility: Some(1),
                herblore: Some(1),
                thieving: Some(1),
                crafting: Some(1),
                fletching: Some(1),
                slayer: Some(1),
                hunter: Some(1),
                mining: Some(1),
                smithing: Some(1),
                fishing: Some(1),
                cooking: Some(1),
                firemaking: Some(1),
                woodcutting: Some(1),
                farming: Some(1),
            }
        }
        fn maxed() -> Self {
            Self {
                attack: Some(99),
                strength: Some(99),
                defence: Some(99),
                ranged: Some(99),
                prayer: Some(99),
                magic: Some(99),
                runecraft: Some(99),
                construction: Some(99),
                hitpoints: Some(99),
                agility: Some(99),
                herblore: Some(99),
                thieving: Some(99),
                crafting: Some(99),
                fletching: Some(99),
                slayer: Some(99),
                hunter: Some(99),
                mining: Some(99),
                smithing: Some(99),
                fishing: Some(99),
                cooking: Some(99),
                firemaking: Some(99),
                woodcutting: Some(99),
                farming: Some(99),
            }
        }
    }

    /// Default implementation

    impl Default for PlayerLevels {
        fn default() -> Self {
            Self {
                attack: None,
                strength: None,
                defence: None,
                ranged: None,
                prayer: None,
                magic: None,
                runecraft: None,
                construction: None,
                hitpoints: None,
                agility: None,
                herblore: None,
                thieving: None,
                crafting: None,
                fletching: None,
                slayer: None,
                hunter: None,
                mining: None,
                smithing: None,
                fishing: None,
                cooking: None,
                firemaking: None,
                woodcutting: None,
                farming: None,
            }
        }
    }

    impl Default for MonsterLevels {
        fn default() -> Self {
            Self {
                attack: None,
                strength: None,
                defence: None,
                ranged: None,
                magic: None,
                hitpoints: None,
            }
        }
    }
}
