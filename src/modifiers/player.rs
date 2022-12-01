pub(crate) mod arclight;
pub(crate) mod berserker;
pub(crate) mod brimstone;
pub(crate) mod chaos_gauntlets;
pub(crate) mod chin;
pub(crate) mod crystal;
pub(crate) mod dharok;
pub(crate) mod dinhs;
pub(crate) mod draconic;
pub(crate) mod guardian;
pub(crate) mod ice_demon;
pub(crate) mod inquisitor;
pub(crate) mod kalphite;
pub(crate) mod leafy;
pub(crate) mod magic_damage;
pub(crate) mod obsidian;
pub(crate) mod powered_staff;
pub(crate) mod salve;
pub(crate) mod slayer;
pub(crate) mod smoke;
pub(crate) mod tome;
pub(crate) mod tumeken;
pub(crate) mod twisted_bow;
pub(crate) mod void;
pub(crate) mod wilderness;

use crate::character::{player::Player, Monster};

use super::{ArMod, DmgMod};

pub struct PlayerModifiers<'a> {
    pub player: &'a Player,
    pub target: &'a Monster,
    pub distance: &'a u8,
    pub special_attack: &'a bool,
    pub additional_targets: &'a Option<u8>,
}

impl<'a> PlayerModifiers<'a> {
    pub(crate) fn get_all_arms(&self) -> Vec<f64> {
        let player = self.player;
        let target = self.target;
        let distance = self.distance;

        //
        // Salve, Smoke, Slayer
        let mut mod_vec: Vec<f64> = vec![];

        let salve_mod = salve::SalveModifier { player, target };
        let slayer_mod = slayer::SlayerModifier { player, target };

        if let Some(md) = salve_mod.accuracy_roll_mod() {
            mod_vec.push(md);
        } else if let Some(md) = slayer_mod.accuracy_roll_mod() {
            mod_vec.push(md);
        };

        // attack rolls
        self.extract_arm(smoke::SmokeModifier { player }, &mut mod_vec);
        self.extract_arm(chin::ChinModifier { player, distance }, &mut mod_vec);
        // attack rolls & damage modifiers
        self.extract_arm(crystal::CrystalModifier { player }, &mut mod_vec);
        self.extract_arm(arclight::ArclightModifier { player, target }, &mut mod_vec);
        self.extract_arm(draconic::DraconicModifier { player, target }, &mut mod_vec);
        self.extract_arm(
            wilderness::WildernessModifier { player, target },
            &mut mod_vec,
        );
        self.extract_arm(
            twisted_bow::TwistedBowModifier { player, target },
            &mut mod_vec,
        );
        self.extract_arm(obsidian::ObsidianModifier { player }, &mut mod_vec);
        self.extract_arm(inquisitor::InquisitorModifier { player }, &mut mod_vec);

        mod_vec
    }

    pub(crate) fn get_all_drms(&self) -> Vec<f64> {
        // let player = self.player;
        // let target = self.target;
        let mut _mod_vec: Vec<f64> = vec![];

        // self.extract_arm(
        //     brimstone::BrimstoneModifier { player, target },
        //     &mut mod_vec,
        // );

        _mod_vec
    }

    pub(crate) fn get_all_dms(&self) -> Vec<f64> {
        let player = self.player;
        let target = self.target;
        let mut dms_vec: Vec<f64> = vec![];

        // Attack roll & damage modifiers
        self.extract_dmg_mod(crystal::CrystalModifier { player }, &mut dms_vec);
        self.extract_dmg_mod(arclight::ArclightModifier { player, target }, &mut dms_vec);
        self.extract_dmg_mod(draconic::DraconicModifier { player, target }, &mut dms_vec);
        self.extract_dmg_mod(
            wilderness::WildernessModifier { player, target },
            &mut dms_vec,
        );
        self.extract_dmg_mod(
            twisted_bow::TwistedBowModifier { player, target },
            &mut dms_vec,
        );
        self.extract_dmg_mod(obsidian::ObsidianModifier { player }, &mut dms_vec);
        self.extract_dmg_mod(inquisitor::InquisitorModifier { player }, &mut dms_vec);

        // Damage modifiers
        self.extract_dmg_mod(magic_damage::MagicDamageModifier { player }, &mut dms_vec);
        self.extract_dmg_mod(
            berserker::BerserkerNecklaceModifier { player },
            &mut dms_vec,
        );
        self.extract_dmg_mod(guardian::GuardianModifier { player, target }, &mut dms_vec);
        self.extract_dmg_mod(ice_demon::IceDemonModifier { player, target }, &mut dms_vec);
        self.extract_dmg_mod(tome::TomeOfFireModifier { player }, &mut dms_vec);

        dms_vec
    }

    fn extract_arm<T: ArMod + 'a>(&self, box_mod: T, mod_vec: &mut Vec<f64>) {
        if let Some(md) = box_mod.accuracy_roll_mod() {
            mod_vec.push(md)
        }
    }

    fn extract_dmg_mod<T: DmgMod + 'a>(&self, box_mod: T, mod_vec: &mut Vec<f64>) {
        if let Some(md) = box_mod.damage_mod() {
            mod_vec.push(md)
        }
    }
}
