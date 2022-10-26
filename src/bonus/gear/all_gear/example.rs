use crate::bonus::Gear;
use std::ffi::OsString;

#[allow(unused_variables)]
pub fn generate_gear_text(gear: Gear, out_file: &OsString) {
    //     let base_string = "\
    //     let lvl_reqs: Levels = Levels::new();\
    //     let special_weapon: Option<SpecialWeapon>;\
    // \
    // \
    //     let g: Gear = Gear::new(\
    //         name: {:?},\
    //         slot: {:?},\
    //         agg: Agg {\
    //             melee: MeleeAgg {\
    //                 attack: AttackMeleeAgg {\
    //                     stab: {:?},\
    //                     slash: {:?},\
    //                     crush: {:?},\
    //                 },\
    //                 strength: {:?},\
    //             },\
    //             ranged: RangedAgg {\
    //                 attack: {:?},\
    //                 strength: {:?},\
    //             },\
    //             magic: MagicAgg {\
    //                 attack: {:?},\
    //                 strength: {:?},\
    //             },\
    //         },\
    //         def: Def {\
    //             melee: MeleeDef {\
    //                 stab: {:?},\
    //                 slash: {:?},\
    //                 crush: {:?},\
    //             },\
    //             ranged: {:?},\
    //             magic: {:?},\
    //         },\
    //         pry: {:?},\
    //         lvl_reqs,\
    //         weapon,\
    //         special_weapon,\
    //     );
    //     ";
    //
    //     let s_weapon = if let Some(wpn) = gear.weapon {
    //         let ss: &str = "\
    //         let styles: &Styles = {:?};\
    //         let base_attack_speed: u8 = {:?};\
    //         let two_handed: bool = {:?};\
    //         let base_attack_range: u8 = {:?};\
    // \
    //         let weapon: Option<Weapon> = Some(Weapon {\
    //             styles,\
    //             base_attack_speed,\
    //             two_handed,\
    //             base_attack_range\
    //         }).to_string();\
    //         ";
    //         ss.to_string()
    //
    //         // let weapon_ss = format!(
    //         //     ss,
    //         //     wpn.weapon.styles,
    //         //     wpn.weapon.base_attack_speed,
    //         //     wpn.weapon.two_handed,
    //         //     wpn.weapon.base_attack_range
    //         // );
    //         // weapon_ss
    //     } else {
    //         "let weapon: Option<Weapon> = None;".to_string()
    //     };
    //
    //     let s_special_weapon = if let Some(swpn) = gear.special_weapon {
    //         r#"
    //         let foo_spec: bool = True
    //         let special_weapon: Option<SpecialWeapon> = None;
    //         "#
    //     } else {
    //         r#"
    //         let foo_spec: bool = False
    //         let special_weapon: Option<SpecialWeapon> = None;
    //             "#
    //     };
}
