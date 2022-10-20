use crate::data::{Stance, StyleName, DT};

/// Style and collection type

pub struct Style {
    pub name: StyleName,
    pub dt: DT,
    pub stance: Stance,
    pub attack_speed_mod: Option<i8>,
    pub attack_range_mod: Option<i8>,
}

pub struct StyleCollection {
    styles: Vec<Style>,
    default: usize,
}

/// Implementation

impl Style {
    fn new(
        name: StyleName,
        dt: DT,
        stance: Stance,
        attack_speed_mod: Option<i8>,
        attack_range_mod: Option<i8>,
    ) -> Self {
        Self {
            name,
            dt,
            stance,
            attack_speed_mod: None,
            attack_range_mod: None,
        }
    }
}

impl StyleCollection {
    fn new(styles: Vec<Style>, default: usize) -> Self {
        Self { styles, default }
    }

    fn default_style(&self) -> &Style {
        &self.styles.get(self.default).unwrap()
    }
}
