use std::collections::HashMap;
use std::io;


fn main() {
    //getting Userinput

    // the titrant is a standard solution with known concentration
    // the analyte, a solution with a certain Volume but unknown concentration

    println!("Please enter the name of the analyte:");
    let properties_analyte = build_substance_properties();

    println!("Please enter the name of the titrant:");
    let properties_titrant = build_substance_properties();

    //let conc_titrant = 0.5_f64;
    let conc_ions_titrant = properties_titrant.valence * properties_titrant.concentration;

    let pH_analyte = pH_strong(conc_ions_titrant, properties_analyte.volume, properties_titrant.volume, properties_titrant.valence, &properties_titrant.substance);
    let conc_ions_analyte = conc_analyte_ions(properties_titrant.volume, conc_ions_titrant, properties_analyte.volume);
    //println!("The concentration of the {} is {:.2} mol/l", name_analyte, conc_analyte);
    println!("The pH of the {} is {:.2}", properties_analyte.substance, pH_analyte);
    println!("The concentration of the {} is {:.2} mol/l", properties_analyte.substance, conc_ions_analyte);

    println!("{}l of {} molar {} was used.", properties_titrant.volume, properties_titrant.concentration, properties_titrant.substance);
}

struct Properties {

    substance: String,
    concentration: f32,
    volume: f32,
    valence: f32,
}

fn build_substance_properties() -> Properties {

    let mut name = String::new();
    io::stdin().read_line(&mut name);

    println!("Please enter the volume of the substance in l:");
    let mut volume = String::new();
    io::stdin().read_line(&mut volume)

        .expect("Failed to read line");

    let volume: f32 = match volume.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!{"Oops"},
    };

    println!("Please enter the concentration of the substance in mol/l, if known:");
    let mut concentration = String::new();
    io::stdin().read_line(&mut concentration)

        .expect("Failed to read line");

    let concentration: f32 = match concentration.trim().parse() {
        Ok(num) => num,
        Err(_) => 0_f32
    };

    Properties {
        substance: name,
        concentration: concentration,
        volume: volume,
        valence: 1_f32,
    }
}

fn conc_analyte_ions(vol_titrant: f32, conc_ions_titrant: f32, vol_analyte: f32) -> f32 {
    // calculates the conentration of the ions in the analyte
    let amount_ions_titrant = vol_titrant * conc_ions_titrant;
    let amount_ions_analyte = amount_ions_titrant;

    amount_ions_analyte/vol_analyte
}

fn potentia(conc_ion: f32) -> f32 {
    // calculates pH or pOH
    -conc_ion.log10()
}

fn pH_strong(conc_ions_titrant: f32, vol_analyte: f32, vol_titrant: f32, valence_titrant: f32, name_titrant: &str) -> f32 {
    //calculates pH for strong acids and bases
    if valence_titrant == 1_f32 {
        let conc_OH = conc_ions_titrant;
        let conc_H = conc_analyte_ions(vol_titrant, conc_OH, vol_analyte);
        potentia(conc_H)
    } else {
        let conc_H = conc_ions_titrant;
        let conc_OH = conc_analyte_ions(vol_titrant, conc_H, vol_analyte);
        let pOH_analyte = potentia(conc_OH);
        14_f32 - pOH_analyte
    }
}
