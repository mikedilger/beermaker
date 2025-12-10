use beermaker::prelude::*;
use beermaker::*;

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

    // And this is my process
    let process = Process {
        water_profile: PAPAIOEA_PARK_BORE,

        // These are probably wrong.
        // A future version of the beermaker will figure this
        // out for me automatically
        water_salts: vec![
            SaltConcentration {
                salt: Salt::Gypsum,
                ppm: Ppm(60.0),
            },
            SaltConcentration {
                salt: Salt::Epsom,
                ppm: Ppm(20.0),
            },
            SaltConcentration {
                salt: Salt::MagnesiumChloride,
                ppm: Ppm(140.0),
            },
            SaltConcentration {
                salt: Salt::BakingSoda,
                ppm: Ppm(10.0),
            },
        ],

        water_acids: vec![],

        // This is very small since I tip out the entire kettle
        // except the hops goo at the bottom
        kettle_losses: Liters(0.03),

        // I measured this.  But you can use this formula
        // `pi * (kettle_opening_radius in cm)^2 * 0.00428`
        boil_evaporation_per_hour: Liters(2.27),

        // This depends on your method. See the docs.
        grain_absorption_per_kg: Liters(0.66),

        hops_absorption_per_kg: Liters(5.0),

        // The reason the batch size is small is because I only
        // have this 11L kettle that I run on my kitchen hob.
        //
        // I leave a 1.5L buffer for foam and space to prevent
        // boil-over.
        kettle_volume: Liters(9.5),

        // This looks high, but is fairly accurate when you tip the
        // entire mash through a sieve
        mash_efficiency: 0.83,

        sparge_volume: Liters(1.0),

        // I don't have a plate chiller. Somebody send me a plate
        // chiller ;-)
        ice_bath: true,

        // Batch size.  I'm using a 5L glass carboy and I want
        // head space for the Krausen
        ferment_volume: Liters(6.0),

        // Yeast grow and steal this much of my beer!
        ferment_loss_percent: 0.10,

        room_temperature: Celsius(20.0),

        // I use boiling water, but within seconds it isn't boiling
        // anymore
        infusion_temperature: Celsius(98.5),

        // This is for recipes where you dilute the wort after
        // boiling to make the full volume, called a partial boil.
        partial_boil_dilution: Liters(0.0),

        // Post ferment dilution
        post_ferment_dilution: Liters(0.0),

        // I use these double-sized big brown bottles that NZ bottle
        // shops charged me a deposit on, but wont take back anymore.
        // That's ok, they are useful.
        packaging: Packaging::Bottle(Liters(0.750), Sugar::Dextrose),
    };

    // Here is my experimental Märzen recipe
    let recipe = Recipe {
        name: "Example Märzen".to_owned(),

        style: Style::Marzen,

        process,

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

        // No sugars. If you add DME or maltodextrin you can
        // put that here.
        sugars: vec![],

        ibu: Ibu(21.0),

        hops: vec![HopsProportion {
            hops: Hops::HallertauMittelfruh, // Alt: Tettnanger
            proportion: 11.0,
            timing: Minutes(60),
        }],

        // Yes, lagers should clear
        fining_desired: true,

        // White Labs German X Lager Yeast WLP835
        yeast: Yeast::WLP835,

        // Just use the optimal temp for that yeast
        ferment_temperature: Yeast::WLP835.temp(),

        // Original graivty
        original_gravity: SpecificGravity(1.056),

        boil_length_override: None,
    };

    // Finally, instruct the beermaker to print out my
    // recipe in detail (no custom steps)
    println!("{}", print_recipe(&recipe, None, Some(70)));

    // Printout any warnings
    if let Err(warnings) = recipe.verify() {
        for warning in &warnings {
            println!("WARNING: {}", warning);
        }
    }
}
