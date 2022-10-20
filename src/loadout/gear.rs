pub mod gear {

    use crate::data::data::Slot;
    use crate::stat::{
        aux_stat::aux_stat::{Agg, Def},
        basic_stat::stat::PlayerLevels,
    };

    /// Structs

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    pub struct Gear {
        pub name: String,
        pub slot: Option<Slot>,
        agg: Agg,
        def: Def,
        pry: u32,
        lvl_reqs: PlayerLevels,
    }

    /// Traits

    pub trait HasGearStats {
        fn get_agg(&self) -> &Agg;
        // fn get_def(&self) -> &Def;
        // fn get_pry(&self) -> &u32;
        // fn get_lvl_reqs(&self) -> &PlayerLevels;
    }

    /// Trait implementation

    impl HasGearStats for Gear {
        fn get_agg(&self) -> &Agg {
            &self.agg
        }
        // fn get_def(&self) -> &Def {
        //     &self.def
        // }
        // fn get_pry(&self) -> &u32 {
        //     &self.pry
        // }
        // fn get_lvl_reqs(&self) -> &PlayerLevels {
        //     &self.lvl_reqs
        // }
    }

    /// Default implementation

    impl Default for Gear {
        fn default() -> Self {
            Self {
                name: Default::default(),
                slot: None, // this should panic
                agg: Default::default(),
                def: Default::default(),
                pry: 0,
                lvl_reqs: Default::default(),
            }
        }
    }
}
