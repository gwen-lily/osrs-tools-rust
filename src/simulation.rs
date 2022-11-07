use crate::character::{Monster, Player};

pub struct Simulation {
    pub players: Vec<Player>,
    pub targets: Vec<Monster>,
}

/// Like a PlayerModifiers class but from the perspective of the Player.
pub struct Strategy<'a> {
    pub target: &'a Monster,
    pub distance: &'a u8,
    pub special_attack: &'a bool,
    pub additional_targets: &'a Option<u8>,
}
