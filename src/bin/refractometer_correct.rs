use beermaker::prelude::*;
use beermaker::refractometer_correction;
use std::io;

pub fn main() {
    println!("Please enter the ORIGINAL SGwort reading: ");
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("failed to read input line.");
    let f: f32 = s.trim().parse().expect("reading not an f32.");
    let original_sg_wort = SpecificGravity(f);

    println!("Please enter the CURRENT SGwort reading: ");
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("failed to read input line.");
    let f: f32 = s.trim().parse().expect("reading not an f32.");
    let current_sg_wort = SpecificGravity(f);

    let (corrected_sg, abv) = refractometer_correction(
        original_sg_wort,
        current_sg_wort,
        1.0, // since we use SGwort, we don't need to correct.
    );

    // Print extra data for debugging and comparing to other
    // calculators
    /*
    let original_brix: Brix = original_sg_wort.into();
    let measured_final_brix: Brix = current_sg_wort.into();
    let corrected_final_brix: Brix = corrected_sg.into();
    println!("Original Brix: {:.1}", original_brix);
    println!("Final Brix: {:.1} => {:.1}", measured_final_brix, corrected_final_brix);
     */

    println!("Current:");
    println!("  Original Gravity = {original_sg_wort:.3}");
    println!("  Final Gravity = {corrected_sg:.3} (measured was {current_sg_wort:.3})");
    println!("  ABV = {:.1}%", abv);
}
