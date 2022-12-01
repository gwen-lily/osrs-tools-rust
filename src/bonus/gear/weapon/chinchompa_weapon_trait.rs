use crate::{combat::Hitsplats, modifiers::PlayerModifiers};

use super::weapon_trait::WeaponTrait;

pub trait ChinchompaWeaponTrait: WeaponTrait {
    fn damage_distribution(&self, pmods: &PlayerModifiers) -> Hitsplats {
        let mut hitsplats: Hitsplats = WeaponTrait::damage_distribution(self, pmods);
        assert_eq!(hitsplats.len(), 1);

        if let Some(adds) = pmods.additional_targets {
            let hs = &hitsplats[0].clone();

            for _ in 0..*adds {
                hitsplats.push(hs.clone())
            }

            assert_eq!(hitsplats.len(), (1 + adds) as usize);
        }

        hitsplats
    }
}
