use beermaker::refractometer_correction;
use beermaker::prelude::*;
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

    let (sg, abv) = refractometer_correction(
        original_sg_wort,
        current_sg_wort
    );

    println!("S.G. = {:.3}", sg);
    println!("ABV = {:.1}%", abv);

    let ob: Brix = original_sg_wort.into();
    let cb: Brix = current_sg_wort.into();
}
