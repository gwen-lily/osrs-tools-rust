use config;
use data;
use gear;
use stat;

use data::MeleeDamageType;
use data::DT;
use gear::gear::Gear;

fn main() {
    config::print_config();

    let g: Gear = Gear {
        name: String::from("name"),
        slot: Some(Slot::Head),
        ..Default::default()
    };

    match g.slot {
        Some(_) => println!("slot"),
        None => panic!("must choose a slot"),
    }

    let dt_1: DT = DT::Melee(MeleeDamageType::Stab);
    let dt_2: DT = DT::Ranged;
    let dt_3: DT = DT::Magic;

    let dts = vec![&dt_1, &dt_2, &dt_3];

    for dt in dts.iter() {
        match dt {
            DT::Melee(mdt) => {
                println!("melee");

                match mdt {
                    MeleeDamageType::Stab => println!("stab"),
                    MeleeDamageType::Slash => println!("slash"),
                    MeleeDamageType::Crush => println!("crush"),
                }
            }
            DT::Ranged => println!("ranged"),
            DT::Magic => println!("magic"),
            DT::Typeless => println!("typeless"),
        }
    }
}
