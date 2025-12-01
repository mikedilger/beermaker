use beermaker::hydrometer_temp_correction;
use beermaker::prelude::*;
use std::io;

pub fn main() {
    println!("Please enter the reading: ");
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("failed to read input line.");
    let reading: f32 = s.trim().parse().expect("reading not an f32.");

    println!("Please enter the sample temperature (C): ");
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("failed to read input line.");
    let temp: f32 = s.trim().parse().expect("reading not an f32.");

    let specific_gravity = hydrometer_temp_correction(reading, Celsius(temp), Celsius(20.0));

    println!("S.G. = {:.3}", specific_gravity);
}
