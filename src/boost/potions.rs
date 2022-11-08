use super::BoostMap;

mod imbued_heart;
mod overloads;
mod ranging_potion;
mod super_combat_potion;

pub struct StandardPotion {
    boosts: BoostMap,
}

impl Potion for StandardPotion {
    fn get_boosts(&self) -> &BoostMap {
        &self.boosts
    }
}

pub trait Potion {
    fn get_boosts(&self) -> &BoostMap;
}

pub trait DivinePotion: Potion {
    fn get_damage(&self) -> u32 {
        10
    }

    fn get_refresh(&self) -> u32 {
        25
    }

    fn get_duration(&self) -> u32 {
        500
    }
}

pub trait OverloadPotion: Potion {
    fn get_damage(&self) -> u32 {
        50
    }

    fn get_refresh(&self) -> u32 {
        25
    }

    fn get_duration(&self) -> u32 {
        500
    }

    fn get_run_energy_restore(&self) -> u32 {
        0
    }
}
