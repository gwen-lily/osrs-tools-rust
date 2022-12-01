// use crate::bonus::Gear;
// use std::error::Error;
// use std::io::Write;
// use std::{ffi::OsString, fs::File};

// #[allow(unused_variables)]
// pub fn generate_gear_text(gear: Gear, out_file: &OsString) -> Result<(), Box<dyn Error>> {
//     let mut output = File::create(out_file)?;
//
//     let s_weapon = if let Some(wpn) = gear.weapon {
//         let weapon_ss = format!(
//             r#"\n
//             let styles: &Styles = {:?};\n
//             let base_attack_speed: u8 = {:?};\n
//             let two_handed: bool = {:?};\n
//             let base_attack_range: u8 = {:?};
//     \
//             let weapon: Option<Weapon> = Some(Weapon {{
//                 styles,
//                 base_attack_speed,
//                 two_handed,
//                 base_attack_range
//             }});
//             "#,
//             wpn.styles, wpn.base_attack_speed, wpn.two_handed, wpn.base_attack_range
//         );
//         weapon_ss
//     } else {
//         "let weapon: Option<Weapon> = None;\n".to_string()
//     };
//
//     let s_special_weapon = if let Some(swpn) = gear.special_weapon {
//         "\n\
//         let foo_spec: bool = True\n\
//         let special_weapon: Option<SpecialWeapon> = None;\n\
//         "
//     } else {
//         "\n\
//         let foo_spec: bool = False\n\
//         let special_weapon: Option<SpecialWeapon> = None;\n\
//         "
//     };
//
//     let base_string = write!(
//         output,
//         "\n\
//         let lvl_reqs: Levels = Levels::new();\n\
//     \n\
//         {:?},\n\
//         {:?},\n\
//     \
//         let g: Gear = Gear::new(\
//             name: {:?},\
//             slot: {:?},\
//             agg: Agg {{\
//                 melee: MeleeAgg {{\
//                     attack: AttackMeleeAgg {{\
//                         stab: {:?},\
//                         slash: {:?},\
//                         crush: {:?},\
//                     }},\
//                     strength: {:?},\
//                 }},\
//                 ranged: RangedAgg {{\
//                     attack: {:?},\
//                     strength: {:?},\
//                 }},\
//                 magic: MagicAgg {{\
//                     attack: {:?},\
//                     strength: {:?},\
//                 }},\
//             }},\
//             def: Def {{\
//                 melee: MeleeDef {{\
//                     stab: {:?},\
//                     slash: {:?},\
//                     crush: {:?},\
//                 }},\
//                 ranged: {:?},\
//                 magic: {:?},\
//             }},\
//             pry: {:?},\
//             lvl_reqs,\
//             weapon,\
//             special_weapon,\
//         );
//         ",
//         s_weapon,
//         s_special_weapon,
//         gear.name,
//         gear.slot,
//         gear.agg.melee.attack.stab,
//         gear.agg.melee.attack.slash,
//         gear.agg.melee.attack.crush,
//         gear.agg.melee.strength,
//         gear.agg.ranged.attack,
//         gear.agg.ranged.strength,
//         gear.agg.magic.attack,
//         gear.agg.magic.strength,
//         gear.def.melee.stab,
//         gear.def.melee.slash,
//         gear.def.melee.crush,
//         gear.def.ranged,
//         gear.def.magic,
//         gear.pry,
//     );
//
//     Ok(())
// }
