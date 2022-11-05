use crate::character::{monster::Monster, player::Player};

#[derive(Builder)]
#[builder(pattern = "owned", build_fn(validate = "Self::validate"))]
pub struct MaxHit {
    pub player: Player,
    pub target: Monster,
    pub special_attack: bool,
    pub distance: u8,
    pub adds: u8,
}

impl MaxHitBuilder {
    /// validation function
    fn validate(&self) -> Result<(), String> {
        if let Some(spec_atk) = self.special_attack {
            match spec_atk {
                true => {
                    if let Some(_spc_wpn) =
                        &self.player.as_ref().unwrap().weapon().special_weapon_info
                    {
                        Ok(())
                    } else {
                        Err(
                            "Attempted to perform special attack with non-special weapon"
                                .to_string(),
                        )
                    }
                }
                false => Ok(()),
            }
        } else {
            Ok(())
        }
    }
}

impl MaxHit {}
