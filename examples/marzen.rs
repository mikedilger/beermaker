use beermaker::prelude::*;
use beermaker::{Equipment, Process, Recipe, print_process};
use beermaker::{MashRest, Packaging, Style};

/// This is a very small 4.25 L batch experiment that I did.
/// I would not consider this a great recipe, because I made
/// it up and I am a beginning brewer.  But this shows you
/// how to use the software.

fn main() {
    // Here I define my water profile
    pub const PAPAIOEA_PARK_BORE: WaterProfile = WaterProfile {
        ca: Ppm(38.6),
        mg: Ppm(7.0),
        na: Ppm(15.6),
        so4: Ppm(12.5),
        cl: Ppm(21.9),
        alkalinity_caco3: Ppm(120.),
        ph: Ph(8.0), //  not reported, guessed
    };

    // And this is my equipment
    let equipment = Equipment {
        water_profile: PAPAIOEA_PARK_BORE,
        salts_available: vec![
            Salt::CalciumChloride,
            Salt::TableSalt,
            Salt::Epsom,
            Salt::Gypsum,
            Salt::BakingSoda,
        ],
        acids_available: vec![Acid::LacticAcid],

        // I mash in my kettle
        mash_tun_volume: Liters(11.0),

        // I leave 1.5 liters of headroom on the 11L kettle so that it
        // doesn't boil over
        max_kettle_volume: Liters(9.5),

        // This is very small since I tip out the entire kettle
        // except the hops goo at the bottom
        kettle_losses: Liters(0.03),

        // I measured this.  But you can use this formula
        // `pi * (kettle_opening_radius in cm)^2 * 0.00428`
        boil_evaporation_per_hour: Liters(2.27),

        // Standard figures here
        grain_absorption_per_kg: Liters(1.0),
        hops_absorption_per_kg: Liters(5.0),

        // This looks high, but is fairly accurate when you tip the
        // entire mash through a sieve
        mash_efficiency: 0.83,

        // I use boiling water, but within seconds it isn't boiling
        // anymore
        infusion_temperature: Celsius(98.5),

        // It is summer, the house runs a bit warmer
        room_temperature: Celsius(22.0),

        // I don't have a plate chiller, or a pump, or hoses.
        ice_bath: true,

        // I have a number of different vessels I could use to ferment and lager
        fermenters: vec![
            Gallons(1.0).into(), // 1 gallon carboy
            Liters(5.0),         // 5L carboy
            Liters(8.0),         // oxebar 8L keg
            Liters(24.0),        // 24L glass demijohn
            Liters(30.0),        // 30L bucket
        ],
        lagerers: vec![
            Gallons(1.0).into(), // 1 gallon carboy
            Liters(5.0),         // 5L carboy
            Liters(8.0),         // oxebar 8L keg
            Liters(24.0),        // 24L glass demijohn
        ],

        // I use these double-sized big brown bottles that NZ bottle
        // shops charged me a deposit on, but wont take back anymore.
        // That's ok, they are useful.
        packaging: Packaging::Bottle(Liters(0.750), Sugar::Dextrose),

        custom_steps: None,
    };

    // Here is my experimental Märzen recipe
    let recipe = Recipe {
        name: "Example Märzen".to_owned(),
        style: Style::Marzen,

        // The salts aren't that important, but I don't want too much
        // sulfate relative to chloride.
        chloride_sulfate_ratio_range: 3.0..4.5,

        // The malt bill.  The proportions don't have to add up
        // to anything in particular.
        malts: vec![
            MaltProportion {
                malt: Malt::WeyermannMunich2,
                proportion: 60.,
            },
            MaltProportion {
                malt: Malt::GladfieldGermanPilsner,
                proportion: 27.,
            },
            MaltProportion {
                malt: Malt::WeyermannMelanoidin,
                proportion: 4.,
            },
        ],

        // This example has two rests
        mash_rests: vec![
            MashRest {
                target_temperature: Celsius(61.0),
                duration: Minutes(30),
            },
            MashRest {
                target_temperature: Celsius(69.0),
                duration: Minutes(30),
            },
        ],

        // Fairly typical for two-step mash
        // Final mash thickness 3.3 L/kg, but 2.5 L/kg at first rest
        mash_thickness: 3.3,

        // No sugars. If you add DME or maltodextrin you can
        // put that here.
        sugars: vec![],

        // Original graivty
        original_gravity: SpecificGravity(1.056),

        ibu: Ibu(21.0),

        hops: vec![HopsProportion {
            hops: Hops::HallertauMittelfruh, // Alt: Tettnanger
            proportion: 11.0,
            timing: Minutes(60),
        }],

        boil_length: Minutes(80),

        // Yes, lagers should clear
        fining_desired: true,

        // White Labs German X Lager Yeast WLP835
        yeast: Yeast::WLP835,

        // Do not allow partial boils
        max_partial_boil_dilution: 1.0,

        // Just use the optimal temp for that yeast
        ferment_temperature: Yeast::WLP835.temp(),

        // Lets drop the ABV a little bit, just for an example.
        // Do not do this with a real Märzen or you will go straight
        // to hell(es).
        target_abv: Some(Abv(0.05)),
        max_post_ferment_dilution: 1.3,
    };

    let process = Process::new(equipment, recipe, Liters(7.0));

    println!("{}", print_process(&process, None, Some(70)));

    let warnings = process.get_warnings();
    if warnings.is_empty() {
        println!("No warnings. Recipe is good.");
    } else {
        for warning in &warnings {
            if warning.is_error() {
                println!("*ERROR*: {}", warning);
            } else {
                println!("WARNING: {}", warning);
            }
        }
    }
}
