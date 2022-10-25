use crate::data::DT;

/// SpecialWeapon struct which represents a Weapon item with more fields and implementation.
#[derive(Debug, PartialEq, Clone)]
pub struct SpecialWeapon {
    pub special_arms: Option<Vec<f64>>,
    pub special_dms: Option<Vec<f64>>,
    pub special_defence_roll: Option<DT>,
}

impl SpecialWeapon {
    pub fn new(
        special_arms: Option<Vec<f64>>,
        special_dms: Option<Vec<f64>>,
        special_defence_roll: Option<DT>,
    ) -> Self {
        Self {
            special_arms,
            special_dms,
            special_defence_roll,
        }
    }
}
