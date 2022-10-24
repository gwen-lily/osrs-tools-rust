use crate::style_mod::style::Styles;

/// Weapon struct which represents a Gear item with more fields and implementation.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Weapon {
    pub styles: &'static Styles,
    pub base_attack_speed: u8,
    pub two_handed: bool,
    pub base_attack_range: u8,
}

impl Weapon {
    pub fn new(
        styles: &'static Styles,
        base_attack_speed: u8,
        two_handed: bool,
        base_attack_range: u8,
    ) -> Self {
        Self {
            styles,
            base_attack_speed,
            two_handed,
            base_attack_range,
        }
    }

    // pub fn hands() -> Self {
    //     if let Some(styles) = ALL_STYLES.get(&UnarmedWeapons) {
    //         Self {
    //             styles,
    //             base_attack_speed: 4,
    //             two_handed: false,
    //         }
    //     } else {
    //         panic!("bad map")
    //     }
    // }
}
