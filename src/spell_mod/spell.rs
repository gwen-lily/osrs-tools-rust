use crate::data::SpellName;
use std::cmp::max;

/// structs

#[allow(dead_code)]
struct StandardSpell {
    // God spells are included as standard spells too
    // Charge behavior should be handled as part of modifiers
    name: SpellName,
    base_max: u8,
    attack_speed: u8,
}

#[allow(dead_code)]
pub struct AncientSpell {
    name: SpellName,
    base_max: u8,
    attack_speed: u8,
    max_targets_hit: u8,
}

#[allow(dead_code)]
pub struct PoweredSpell {
    name: SpellName,
    base_max: u8,
    attack_speed: u8,
}

/// traits

trait SpellMax {
    fn get_base_max(&self) -> u8;
    fn max_hit(&self) -> u8;
}

trait AoeSpell {
    fn get_max_targets_hit(&self) -> u8;
}

trait PoweredSpellTrait: SpellMax {
    fn get_visible_level(&self) -> i32;

    fn max_hit(&self) -> u8 {
        let minimum_level: i32 = 75;
        let visible_level: i32 = self.get_visible_level();
        let adj_visible_level: i32 = max(minimum_level, visible_level);
        let level_diff: i32 = adj_visible_level - visible_level;
        let base_max: u8 = self.get_base_max();
        let max: u8 = base_max + (level_diff / 3) as u8;
        max
    }
}

trait TumekensPoweredSpellTrait: PoweredSpellTrait {
    fn max_hit(&self) -> u8 {
        let visible_level: i32 = self.get_visible_level();
        let base_max: u8 = self.get_base_max();
        let max: u8 = base_max + (visible_level / 3) as u8;
        max
    }
}
