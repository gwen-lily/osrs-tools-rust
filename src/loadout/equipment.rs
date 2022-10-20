pub mod equipment {
    use std::{clone, collections::HashMap};

    use crate::{
        data::data::Slot,
        loadout::gear::gear::{Gear, HasGearStats},
        stat::{
            aux_stat::aux_stat::{Agg, Def},
            basic_stat::stat::PlayerLevels,
        },
    };

    use strum::IntoEnumIterator;

    /// structs

    #[derive(Debug, Eq, PartialEq, Hash, Clone)]
    pub struct Equipment {
        pub head: Option<Gear>,
        pub cape: Option<Gear>,
        pub neck: Option<Gear>,
        pub ammunition: Option<Gear>,
        pub weapon: Option<Gear>,
        pub body: Option<Gear>,
        pub shield: Option<Gear>,
        pub legs: Option<Gear>,
        pub hands: Option<Gear>,
        pub feet: Option<Gear>,
        pub ring: Option<Gear>,
    }

    /// Traits

    trait GearContainer {
        fn get_gear(&self) -> Vec<&Gear>;
    }

    /// Implementation

    impl Equipment {
        fn as_array(&self) -> [&Option<Gear>; 11] {
            [
                &self.head,
                &self.cape,
                &self.neck,
                &self.ammunition,
                &self.weapon,
                &self.body,
                &self.shield,
                &self.legs,
                &self.hands,
                &self.feet,
                &self.ring,
            ]
        }

        fn as_map(&self) -> HashMap<Slot, &Option<Gear>> {
            let mut map: HashMap<Slot, &Option<Gear>> = HashMap::new();

            let vals: [&Option<Gear>; 11] = self.as_array();
            for (k, v) in Slot::iter().zip(vals.iter()) {
                map.insert(k, v);
            }

            map
        }
    }

    /// Trait implementation

    impl GearContainer for Equipment {
        fn get_gear(&self) -> Vec<&Gear> {
            self.as_array().into_iter().flatten().collect()
        }
    }

    impl HasGearStats for Equipment {
        fn get_agg(self) -> Agg {
            let mut agg: Agg = Agg::default();

            for gear in self.get_gear() {
                agg = agg + gear.get_agg();
            }

            let agg = agg;
            agg
        }

        // fn get_def(&self) -> &Def {}
        //
        // fn get_pry(&self) -> &u32 {}
        //
        // fn get_lvl_reqs(&self) -> &PlayerLevels {}
    }

    /// Default implementation

    impl Default for Equipment {
        fn default() -> Self {
            Self {
                head: None,
                cape: None,
                neck: None,
                ammunition: None,
                weapon: None,
                body: None,
                shield: None,
                legs: None,
                hands: None,
                feet: None,
                ring: None,
            }
        }
    }
}
