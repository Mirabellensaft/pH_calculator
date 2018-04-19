use std::collections::HashMap;


fn main() {

    //There needs to be a HashMap!

    // the titrant is a standard solution with known Concentration
    // the analyte, a solution with a certain Volume but unknown concentration
    let name_titrant = String::from("NaOH");
    let name_titrant = 1;
    let name_analyte = String::from("HCl");
    // is there a way to read valence from formula sting?

    //Number of OH- and H+ Ions per unit
    let valence_titrant = 1_f64;
    let valence_analyte = 1_f64;
    //concentration of OH- or H+ Ions in titrant
    let conc_titrant = 0.5_f64;
    let conc_ions_titrant = valence_titrant * conc_titrant;

    //Volume of each solution
    let vol_titrant = 0.3_f64;
    let vol_analyte = 0.7_f64;

    let pH_analyte = pH_strong(conc_ions_titrant, vol_analyte, vol_titrant, name_titrant);

    //println!("The concentration of the {} is {:.2} mol/l", name_analyte, conc_analyte);
    println!("The pH of the {} is {:.2}", name_analyte, pH_analyte);
}



fn conc_analyte_ions(vol_titrant: f64, conc_ions_titrant: f64, vol_analyte: f64) -> f64 {
    // calculates the conentration of the ions in the analyte
    let amount_ions_titrant = vol_titrant * conc_ions_titrant;
    let amount_ions_analyte = amount_ions_titrant;

    amount_ions_analyte/vol_analyte
}

fn potentia(conc_ion: f64) -> f64 {
    // calculates pH or pOH
    -conc_ion.log10()
}

fn pH_strong(conc_ions_titrant: f64, vol_analyte: f64, vol_titrant: f64, name_titrant: i32) -> f64 {
    //calculates pH for strong acids and bases
    if name_titrant == 1_i32 {
        let conc_OH = conc_ions_titrant;
        let conc_H = conc_analyte_ions(vol_titrant, conc_OH, vol_analyte);
        potentia(conc_H)
    } else {
        let conc_H = conc_ions_titrant;
        let conc_OH = conc_analyte_ions(vol_titrant, conc_H, vol_analyte);
        let pOH_analyte = potentia(conc_OH);
        14_f64 - pOH_analyte
    }
}
