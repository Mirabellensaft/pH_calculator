use std::collections::HashMap;
use std::io;


fn main() {
    //getting Userinput
    println!("Please enter the name of the analyte:");
    let mut name_analyte = String::new();
    io::stdin().read_line(&mut name_analyte);

    println!("Please enter the amount to be analysed in l:");
    let mut vol_analyte = String::new();
    io::stdin().read_line(&mut vol_analyte)

        .expect("Failed to read line");

    let vol_analyte: f32 = match vol_analyte.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!{"Oops"},
    };


    println!("Please enter the name of the titrant:");
    let mut name_titrant = String::new();
    io::stdin().read_line(&mut name_titrant);

    //concentration of OH- or H+ Ions in titrant
    println!("Please enter the concentration of the titrant in mol/l:");
    let mut conc_titrant = String::new();
    io::stdin().read_line(&mut conc_titrant)

        .expect("Failed to read line");

    let conc_titrant: f32 = match conc_titrant.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!{"Oops"}
    };

    println!("Please enter the amount in l used of the titrant:");
    let mut vol_titrant = String::new();
    io::stdin().read_line(&mut vol_titrant)

        .expect("Failed to read line");

    let vol_titrant: f32 = match vol_titrant.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!{"Oops"}
    };


    //There needs to be a HashMap!

    // the titrant is a standard solution with known Concentration
    // the analyte, a solution with a certain Volume but unknown concentration

    // is there a way to read valence from formula sting?

    //Number of OH- and H+ Ions per unit
    let valence_titrant = 1_f32;
    let valence_analyte = 1_f32;


    //let conc_titrant = 0.5_f64;
    let conc_ions_titrant = valence_titrant * conc_titrant;

    //Volume of each solution
    //let vol_titrant = 0.3_f64;
    //let vol_analyte = 0.7_f64;

    let pH_analyte = pH_strong(conc_ions_titrant, vol_analyte, vol_titrant, valence_titrant, &name_titrant);
    let conc_ions_analyte = conc_analyte_ions(vol_titrant, conc_ions_titrant, vol_analyte);
    //println!("The concentration of the {} is {:.2} mol/l", name_analyte, conc_analyte);
    println!("The pH of the {} is {:.2}", name_analyte, pH_analyte);
    println!("The concentration of the {} is {:.2} mol/l", name_analyte, conc_ions_analyte);

    println!("{}l of {} molar {} was used.", vol_titrant, conc_titrant, name_titrant);
}

struct Properties {
    substance: String,
    concentration: f32,
    volume_used: f32,
}
fn build_substance_properties(substance: String, concentration: f32, volume_used: f32) -> Properties {
    Properties {
        substance,
        concentration,
        volume_used,
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
