use crate::units::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::EnumIter;

/// A category of Malt
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MaltCategory {
    /// Base malt with diastatic power
    Base,

    /// Crystal malt or Caramel malt
    Crystal,

    /// Roasted malt
    Roasted,

    /// Special malt
    Special,
}

/// A type of Malt
#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter)]
pub enum Malt {
    /// Briess Victory
    BriessVictory,

    /// Dingemans Special-B
    DingemansSpecialB,

    /// Thomas Fawcett Pale Chocolate
    FawcettPaleChocolate,

    /// Gladfield Ale Malt
    GladfieldAle,

    /// Gladfield American Ale Malt
    GladfieldAmericanAle,

    /// Gladfield Big-O Malted Oats
    GladfieldBigOMaltedOats,

    /// Gladfield Biscuit Malt
    GladfieldBiscuit,

    /// Gladfield Brown Malt
    GladfieldBrown,

    /// Gladfield Crystal Dark
    GladfieldCrystalDark,

    /// Gladfield Crystal Light
    GladfieldCrystalLight,

    /// Gladfield Crystal Medium
    GladfieldCrystalMedium,

    /// Gladfield base malt, German Pilsner Malt
    GladfieldGermanPilsner,

    /// Gladfield Light Lager Malt
    GladfieldLagerLight,

    /// Gladfield Munich
    GladfieldMunich,

    /// Gladfield Pilsner Malt
    GladfieldPilsner,

    /// Gladfield Shepherds Delight
    GladfieldShepherdsDelight,

    /// Gladfield Vienna
    GladfieldVienna,

    /// Gladfield Wheat
    GladfieldWheat,

    /// Oat Hulls
    OatHulls,

    /// Rice Hulls
    RiceHulls,

    /// Maris Otter Pale
    SimpsonsMarisOtterPale,

    /// Weyermann Acidulated
    WeyermannAcidulated,

    /// Weyermann Bohemian Pilsner
    WeyermannBohemianPilsner,

    /// Weyermann Carafa Special II
    WeyermannCarafaSpecial2,

    /// Weyermann Carafa Special III
    WeyermannCarafaSpecial3,

    /// Weyermann Caramunich Type II
    WeyermannCaramunich2,

    /// Weyermann Melanoidin
    WeyermannMelanoidin,

    /// Weyermann Munich Type I
    WeyermannMunich1,

    /// Weyermann Munich Type II (Dark)
    WeyermannMunich2,

    /// Weyermann Pilsner
    WeyermannPilsner,

    /// Weyermann Vienna
    WeyermannVienna,

    /// Weyermann Wheat Malt (Pale)
    WeyermannWheatPale,
}

impl Malt {
    /// Category of malt
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn category(&self) -> MaltCategory {
        match *self {
            Malt::BriessVictory => MaltCategory::Roasted,
            Malt::DingemansSpecialB => MaltCategory::Crystal,
            Malt::FawcettPaleChocolate => MaltCategory::Roasted,
            Malt::GladfieldAle => MaltCategory::Base,
            Malt::GladfieldAmericanAle => MaltCategory::Base,
            Malt::GladfieldBigOMaltedOats => MaltCategory::Base,
            Malt::GladfieldBiscuit => MaltCategory::Roasted,
            Malt::GladfieldBrown => MaltCategory::Roasted,
            Malt::GladfieldCrystalDark => MaltCategory::Crystal,
            Malt::GladfieldCrystalLight => MaltCategory::Crystal,
            Malt::GladfieldCrystalMedium => MaltCategory::Crystal,
            Malt::GladfieldGermanPilsner => MaltCategory::Base,
            Malt::GladfieldLagerLight => MaltCategory::Base,
            Malt::GladfieldMunich => MaltCategory::Base,
            Malt::GladfieldPilsner => MaltCategory::Base,
            Malt::GladfieldShepherdsDelight => MaltCategory::Special,
            Malt::GladfieldVienna => MaltCategory::Base,
            Malt::GladfieldWheat => MaltCategory::Base,
            Malt::OatHulls => MaltCategory::Special,
            Malt::RiceHulls => MaltCategory::Special,
            Malt::SimpsonsMarisOtterPale => MaltCategory::Base,
            Malt::WeyermannAcidulated => MaltCategory::Special,
            Malt::WeyermannBohemianPilsner => MaltCategory::Base,
            Malt::WeyermannCarafaSpecial2 => MaltCategory::Roasted,
            Malt::WeyermannCarafaSpecial3 => MaltCategory::Roasted,
            Malt::WeyermannCaramunich2 => MaltCategory::Crystal,
            Malt::WeyermannMelanoidin => MaltCategory::Crystal,
            Malt::WeyermannMunich1 => MaltCategory::Base,
            Malt::WeyermannMunich2 => MaltCategory::Base,
            Malt::WeyermannPilsner => MaltCategory::Base,
            Malt::WeyermannVienna => MaltCategory::Base,
            Malt::WeyermannWheatPale => MaltCategory::Base,
        }
    }

    /// Distilled water mash pH
    #[must_use]
    pub fn distilled_water_mash_ph(&self) -> Option<Ph> {
        match *self {
            Malt::BriessVictory => None,
            Malt::DingemansSpecialB => None,
            Malt::FawcettPaleChocolate => None,
            Malt::GladfieldAle => Some(Ph(f32::midpoint(5.7, 6.0))), // [2]
            Malt::GladfieldAmericanAle => Some(Ph(f32::midpoint(5.7, 6.0))), // [2]
            Malt::GladfieldBigOMaltedOats => Some(Ph(f32::midpoint(5.7, 6.1))),
            Malt::GladfieldBiscuit => Some(Ph(5.15)), // [2]
            Malt::GladfieldBrown => Some(Ph(4.81)),   // [2]
            Malt::GladfieldCrystalDark => Some(Ph(4.7)), // [2]
            Malt::GladfieldCrystalLight => Some(Ph(5.15)), // [2]
            Malt::GladfieldCrystalMedium => Some(Ph(4.84)), // [2]
            Malt::GladfieldGermanPilsner => Some(Ph(f32::midpoint(5.7, 6.0))), // [2]
            Malt::GladfieldLagerLight => Some(Ph(f32::midpoint(5.8, 6.1))), // [2]
            Malt::GladfieldMunich => Some(Ph(f32::midpoint(5.6, 5.9))), // [2]
            Malt::GladfieldPilsner => Some(Ph(f32::midpoint(5.7, 6.0))), // [2]
            Malt::GladfieldShepherdsDelight => Some(Ph(4.53)), // [2]
            Malt::GladfieldVienna => Some(Ph(f32::midpoint(5.6, 5.9))), // [2]
            Malt::GladfieldWheat => Some(Ph(f32::midpoint(5.7, 6.2))), // [2]
            Malt::OatHulls => None,
            Malt::RiceHulls => None,
            Malt::SimpsonsMarisOtterPale => Some(Ph(5.77)), // [1] diff src
            Malt::WeyermannAcidulated => None,
            Malt::WeyermannBohemianPilsner => Some(Ph(5.76)), // [1] presumed pilsner
            Malt::WeyermannCarafaSpecial2 => None,
            Malt::WeyermannCarafaSpecial3 => None,
            Malt::WeyermannCaramunich2 => None,
            Malt::WeyermannMelanoidin => None,
            Malt::WeyermannMunich1 => Some(Ph(5.3)),  // [1]
            Malt::WeyermannMunich2 => Some(Ph(5.43)), // [1]
            Malt::WeyermannPilsner => Some(Ph(5.76)), // [1]
            Malt::WeyermannVienna => Some(Ph(5.56)),  // [1]
            Malt::WeyermannWheatPale => Some(Ph(6.04)), // [1]
        }

        // [1] http://braukaiser.com/documents/effect_of_water_and_grist_on_mash_pH.pdf
        // [2] https://www.gladfieldmalt.co.nz/
    }

    /// Malt acidity in mEq/kg
    #[must_use]
    pub fn acidity(&self) -> f32 {
        match *self {
            // Tested malts
            Malt::WeyermannCarafaSpecial2 => 45.0,
            Malt::WeyermannCarafaSpecial3 => 45.0,
            Malt::WeyermannCaramunich2 => 49.0,
            Malt::WeyermannMunich1 => 8.4,
            Malt::WeyermannMunich2 => 5.6,
            Malt::WeyermannVienna => 1.6,
            Malt::WeyermannAcidulated => f32::midpoint(315.2, 358.2),
            _ => {
                // Formula for crystal malts
                if self.category() == MaltCategory::Crystal {
                    14.0 + 0.13 * self.ebc().0 // [1] formula for crystal malts
                }
                // Formula for malts with a known distilled water mash pH
                else if let Some(ph) = self.distilled_water_mash_ph() {
                    814_984.25 * 0.12_f32.powf(ph.0)
                } else {
                    match self.category() {
                        MaltCategory::Base => 2.5,
                        MaltCategory::Crystal => unreachable!(),
                        MaltCategory::Roasted => 42.0,
                        MaltCategory::Special => 0.0, // unknown
                    }
                }
            }
        }

        // [1] http://braukaiser.com/documents/effect_of_water_and_grist_on_mash_pH.pdf
        // note: all roasted malts are about 40.
        // formula to estimate for crystals:  acidity = 14 + 0.13 EBC
        // formula to estimate from distilled water mash ph:  814984.25 * 0.12^x
    }

    /// Range of wort color provided
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn ebc_range(&self) -> (Ebc, Ebc) {
        match *self {
            Malt::BriessVictory => (Ebc(55.0), Ebc(55.0)),
            Malt::DingemansSpecialB => (Ebc(300.0), Ebc(300.0)),
            Malt::FawcettPaleChocolate => (Ebc(560.0), Ebc(690.0)),
            Malt::GladfieldAle => (Ebc(5.0), Ebc(6.0)),
            Malt::GladfieldAmericanAle => (Ebc(4.5), Ebc(5.5)),
            Malt::GladfieldBigOMaltedOats => (Ebc(2.1), Ebc(5.0)),
            Malt::GladfieldBiscuit => (Ebc(40.0), Ebc(80.0)),
            Malt::GladfieldBrown => (Ebc(150.0), Ebc(200.0)),
            Malt::GladfieldCrystalDark => (Ebc(175.0), Ebc(225.0)),
            Malt::GladfieldCrystalLight => (Ebc(40.0), Ebc(70.0)),
            Malt::GladfieldCrystalMedium => (Ebc(90.0), Ebc(130.0)),
            Malt::GladfieldGermanPilsner => (Ebc(3.0), Ebc(4.5)), // 2.03
            Malt::GladfieldLagerLight => (Ebc(2.5), Ebc(2.9)),
            Malt::GladfieldMunich => (Ebc(14.0), Ebc(17.0)),
            Malt::GladfieldPilsner => (Ebc(3.2), Ebc(4.0)),
            Malt::GladfieldShepherdsDelight => (Ebc(320.), Ebc(320.)),
            Malt::GladfieldVienna => (Ebc(6.5), Ebc(8.5)),
            Malt::GladfieldWheat => (Ebc(3.2), Ebc(4.2)),
            Malt::OatHulls => (Ebc(0.0), Ebc(0.0)),
            Malt::RiceHulls => (Ebc(0.0), Ebc(0.0)),
            Malt::SimpsonsMarisOtterPale => (Ebc(4.5), Ebc(6.5)),
            Malt::WeyermannAcidulated => (Ebc(2.0), Ebc(5.0)), // 1.2 - 2.3
            Malt::WeyermannBohemianPilsner => (Ebc(3.0), Ebc(5.0)),
            Malt::WeyermannCarafaSpecial2 => (Ebc(1100.0), Ebc(1200.0)),
            Malt::WeyermannCarafaSpecial3 => (Ebc(1300.0), Ebc(1500.0)),
            Malt::WeyermannCaramunich2 => (Ebc(110.0), Ebc(130.0)), // 41.9 - 49.5
            Malt::WeyermannMelanoidin => (Ebc(60.0), Ebc(80.0)),    // 23.1 - 30.6
            Malt::WeyermannMunich1 => (Ebc(12.0), Ebc(18.0)),       // 5.0 - 7.2
            Malt::WeyermannMunich2 => (Ebc(20.0), Ebc(25.0)),       // 8.0 - 9.9
            Malt::WeyermannPilsner => (Ebc(2.5), Ebc(4.5)),
            Malt::WeyermannVienna => (Ebc(6.0), Ebc(9.0)), // 2.7 - 3.8
            Malt::WeyermannWheatPale => (Ebc(3.0), Ebc(5.0)), // 1.6 - 2.3
        }
    }

    /// Range of wort color provided
    #[must_use]
    pub fn ebc(&self) -> Ebc {
        (self.ebc_range().0 + self.ebc_range().1) / 2.0
    }

    /// Maximum amount you should have in a normal beer recipe
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn recommended_max_percent(&self) -> f32 {
        match *self {
            Malt::BriessVictory => 10.0,
            Malt::DingemansSpecialB => 15.0,
            Malt::FawcettPaleChocolate => 5.0,
            Malt::GladfieldAle => 100.0,
            Malt::GladfieldAmericanAle => 100.0,
            Malt::GladfieldBigOMaltedOats => 20.0,
            Malt::GladfieldBiscuit => 15.0,
            Malt::GladfieldBrown => 15.0,
            Malt::GladfieldCrystalDark => 25.0,
            Malt::GladfieldCrystalLight => 25.0,
            Malt::GladfieldCrystalMedium => 25.0,
            Malt::GladfieldGermanPilsner => 100.0,
            Malt::GladfieldLagerLight => 100.0,
            Malt::GladfieldMunich => 100.0,
            Malt::GladfieldPilsner => 100.0,
            Malt::GladfieldShepherdsDelight => 10.0,
            Malt::GladfieldVienna => 100.0,
            Malt::GladfieldWheat => 70.0,
            Malt::OatHulls => 5.0,
            Malt::RiceHulls => 8.0, // 3-8%
            Malt::SimpsonsMarisOtterPale => 100.0,
            Malt::WeyermannAcidulated => 5.0, // 10.0 in sour beers
            Malt::WeyermannBohemianPilsner => 100.0,
            Malt::WeyermannCarafaSpecial2 => 5.0,
            Malt::WeyermannCarafaSpecial3 => 5.0,
            Malt::WeyermannCaramunich2 => 10.0,
            Malt::WeyermannMelanoidin => 20.0,
            Malt::WeyermannMunich1 => 100.0,
            Malt::WeyermannMunich2 => 100.0,
            Malt::WeyermannPilsner => 100.0,
            Malt::WeyermannVienna => 100.0,
            Malt::WeyermannWheatPale => 80.0,
        }
    }

    /// Points per pound per gallon
    // taken from brewersfriend.com
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn ppg(&self) -> f32 {
        match *self {
            Malt::BriessVictory => 34.5,
            Malt::DingemansSpecialB => 33.1,
            Malt::FawcettPaleChocolate => 32.2,
            Malt::GladfieldAle => 37.4,
            Malt::GladfieldAmericanAle => 37.3,
            Malt::GladfieldBigOMaltedOats => 27.5,
            Malt::GladfieldBiscuit => 35.0,
            Malt::GladfieldBrown => 34.0,
            Malt::GladfieldCrystalDark => 35.4,
            Malt::GladfieldCrystalLight => 35.4,
            Malt::GladfieldCrystalMedium => 35.4,
            Malt::GladfieldGermanPilsner => 36.3,
            Malt::GladfieldLagerLight => 35.0,
            Malt::GladfieldMunich => 36.8,
            Malt::GladfieldPilsner => 37.7,
            Malt::GladfieldShepherdsDelight => 35.0,
            Malt::GladfieldVienna => 37.4,
            Malt::GladfieldWheat => 38.8,
            Malt::OatHulls => 0.0,
            Malt::RiceHulls => 0.0,
            Malt::SimpsonsMarisOtterPale => 37.5, // est from other vendor
            Malt::WeyermannAcidulated => 27.0,
            Malt::WeyermannBohemianPilsner => 36.8,
            Malt::WeyermannCarafaSpecial2 => 29.9,
            Malt::WeyermannCarafaSpecial3 => 29.9,
            Malt::WeyermannCaramunich2 => 34.0,
            Malt::WeyermannMelanoidin => 34.5,
            Malt::WeyermannMunich1 => 38.0,
            Malt::WeyermannMunich2 => 37.0,
            Malt::WeyermannPilsner => 36.0,
            Malt::WeyermannVienna => 37.0,
            Malt::WeyermannWheatPale => 36.0,
        }
    }

    /// Percent protein from malt spec
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn percent_protein(&self) -> Option<f32> {
        match *self {
            Malt::BriessVictory => None,
            Malt::DingemansSpecialB => None,
            Malt::FawcettPaleChocolate => Some(11.6), // less than
            Malt::GladfieldAle => None,
            Malt::GladfieldAmericanAle => None,
            Malt::GladfieldBigOMaltedOats => None,
            Malt::GladfieldBiscuit => None,
            Malt::GladfieldBrown => None,
            Malt::GladfieldCrystalDark => None,
            Malt::GladfieldCrystalLight => None,
            Malt::GladfieldCrystalMedium => None,
            Malt::GladfieldGermanPilsner => None,
            Malt::GladfieldLagerLight => None,
            Malt::GladfieldMunich => None,
            Malt::GladfieldPilsner => None,
            Malt::GladfieldShepherdsDelight => None,
            Malt::GladfieldVienna => None,
            Malt::GladfieldWheat => None,
            Malt::OatHulls => Some(0.0),  // presume none
            Malt::RiceHulls => Some(0.0), // presume none
            Malt::SimpsonsMarisOtterPale => Some(f32::midpoint(8.13, 9.69)),
            Malt::WeyermannAcidulated => Some(0.0), // presume none
            Malt::WeyermannBohemianPilsner => Some(f32::midpoint(9.0, 12.0)),
            Malt::WeyermannCarafaSpecial2 => Some(0.0), // presume none
            Malt::WeyermannCarafaSpecial3 => Some(0.0), // presume none
            Malt::WeyermannCaramunich2 => Some(0.0),    // presume none
            Malt::WeyermannMelanoidin => Some(0.0),     // presume none
            Malt::WeyermannMunich1 => Some(f32::midpoint(9.0, 12.5)),
            Malt::WeyermannMunich2 => Some(f32::midpoint(9.0, 12.5)),
            Malt::WeyermannPilsner => Some(f32::midpoint(9.0, 12.0)),
            Malt::WeyermannVienna => Some(f32::midpoint(9.0, 12.5)),
            Malt::WeyermannWheatPale => Some(f32::midpoint(10.0, 13.0)),
        }
    }

    /// Kolbach index (percent of protein that is soluble)
    /// Also called the Soluble Nitrogen Ratio
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn kolbach_index(&self) -> Option<f32> {
        match *self {
            Malt::BriessVictory => None,
            Malt::DingemansSpecialB => None,
            Malt::FawcettPaleChocolate => None,
            Malt::GladfieldAle => Some(38.0), // spec 35 - 41
            Malt::GladfieldAmericanAle => Some(38.0), // spec 35 - 41
            Malt::GladfieldBigOMaltedOats => Some(27.5), // spec 25 - 30
            Malt::GladfieldBiscuit => Some(37.0), // of zero
            Malt::GladfieldBrown => Some(37.0), // of zero
            Malt::GladfieldCrystalDark => None,
            Malt::GladfieldCrystalLight => None,
            Malt::GladfieldCrystalMedium => None,
            Malt::GladfieldGermanPilsner => Some(41.0), // spec 37 - 42
            Malt::GladfieldLagerLight => Some(39.0),    // spec 35 - 41
            Malt::GladfieldMunich => Some(40.0),        // spec 37 - 43
            Malt::GladfieldPilsner => Some(39.0),       // spec 35 - 41
            Malt::GladfieldShepherdsDelight => None,
            Malt::GladfieldVienna => Some(40.0), // spec 37 - 43
            Malt::GladfieldWheat => Some(34.0),  // spec 33 - 40
            Malt::OatHulls => Some(37.0),        // of zero
            Malt::RiceHulls => Some(37.0),       // of zero
            Malt::SimpsonsMarisOtterPale => Some(f32::midpoint(40.0, 44.0)),
            Malt::WeyermannAcidulated => Some(37.0), // of zero
            Malt::WeyermannBohemianPilsner => Some(f32::midpoint(38.0, 42.0)),
            Malt::WeyermannCarafaSpecial2 => None,
            Malt::WeyermannCarafaSpecial3 => None,
            Malt::WeyermannCaramunich2 => Some(37.0), // of zero
            Malt::WeyermannMelanoidin => Some(37.0),  // of zero
            Malt::WeyermannMunich1 => Some(f32::midpoint(37.0, 46.0)),
            Malt::WeyermannMunich2 => Some(f32::midpoint(37.0, 46.0)),
            Malt::WeyermannPilsner => Some(f32::midpoint(36.0, 42.5)),
            Malt::WeyermannVienna => Some(f32::midpoint(37.0, 45.5)),
            Malt::WeyermannWheatPale => Some(f32::midpoint(37.5, 47.0)),
        }
    }

    /// FAN value (mg/L, congress mash at 1.040 or 10 Plato)
    /// from malt specification, if it has one
    #[must_use]
    pub fn fan_from_spec(&self) -> Option<Ppm> {
        match *self {
            Malt::BriessVictory => None,
            Malt::DingemansSpecialB => Some(Ppm(0.0)), // assume 0
            Malt::FawcettPaleChocolate => None,
            Malt::GladfieldAle => Some(Ppm(130.0)), // min spec 120.0
            Malt::GladfieldAmericanAle => Some(Ppm(135.0)), // min spec 120.0
            Malt::GladfieldBigOMaltedOats => Some(Ppm(100.0)), // 80-120
            Malt::GladfieldBiscuit => Some(Ppm(0.0)), // assume 0
            Malt::GladfieldBrown => Some(Ppm(0.0)), // assume 0
            Malt::GladfieldCrystalDark => Some(Ppm(0.0)), // assume 0
            Malt::GladfieldCrystalLight => Some(Ppm(0.0)), // assume 0
            Malt::GladfieldCrystalMedium => Some(Ppm(0.0)), // assume 0
            Malt::GladfieldGermanPilsner => Some(Ppm(140.0)), // min spec 135.0
            Malt::GladfieldLagerLight => Some(Ppm(130.0)), // min spec 120.0
            Malt::GladfieldMunich => Some(Ppm(125.0)), // min spec 120
            Malt::GladfieldPilsner => Some(Ppm(130.0)), // min spec 120.0
            Malt::GladfieldShepherdsDelight => Some(Ppm(0.0)), // assume 0
            Malt::GladfieldVienna => Some(Ppm(140.0)), // min spec 120
            Malt::GladfieldWheat => Some(Ppm(85.0)), // no spec
            Malt::OatHulls => Some(Ppm(0.0)),       // assume 0
            Malt::RiceHulls => Some(Ppm(0.0)),      // assume 0
            Malt::SimpsonsMarisOtterPale => None,
            Malt::WeyermannAcidulated => Some(Ppm(0.0)), // assume 0
            Malt::WeyermannBohemianPilsner => None,
            Malt::WeyermannCarafaSpecial2 => Some(Ppm(0.0)), // assume 0
            Malt::WeyermannCarafaSpecial3 => Some(Ppm(0.0)), // assume 0
            Malt::WeyermannCaramunich2 => Some(Ppm(0.0)),    // assume 0
            Malt::WeyermannMelanoidin => Some(Ppm(0.0)),     // assume 0
            Malt::WeyermannMunich1 => None,
            Malt::WeyermannMunich2 => None,
            Malt::WeyermannPilsner => None,
            Malt::WeyermannVienna => None,
            Malt::WeyermannWheatPale => None,
        }
    }

    /// Free Amino Nitrogen (FAN) contribution in mg/L at 1.040 SG.
    ///
    /// We first try the malt specification, then we estimate based on malt
    /// specified protein and Kolbach index. For non-base malts and wherever
    /// data is missing, this is zero.
    #[must_use]
    pub fn fan(&self) -> Ppm {
        if let Some(fan) = self.fan_from_spec() {
            fan
        } else if matches!(self, Malt::WeyermannWheatPale) {
            Ppm(85.0) // wheat has less
        } else {
            if let Some(pp) = self.percent_protein()
                && let Some(ki) = self.kolbach_index()
            {
                return Ppm(pp * ki * 0.32);
            }

            Ppm(0.0)
        }
    }
}

impl fmt::Display for Malt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Malt::BriessVictory => write!(f, "[Briess Victory]"),
            Malt::DingemansSpecialB => write!(f, "[Dingemans Special B]"),
            Malt::FawcettPaleChocolate => write!(f, "[Thomas Fawcett Pale Chocolate Malt]"),
            Malt::GladfieldAle => write!(f, "[Gladfield Ale Malt]"),
            Malt::GladfieldAmericanAle => write!(f, "[Gladfield American Ale Malt]"),
            Malt::GladfieldBigOMaltedOats => write!(f, "[Gladfield Big-O Malted Oats]"),
            Malt::GladfieldBiscuit => write!(f, "[Gladfield Biscuit Malt]"),
            Malt::GladfieldBrown => write!(f, "[Gladfield Brown Malt]"),
            Malt::GladfieldCrystalDark => write!(f, "[Gladfield Dark Crystal]"),
            Malt::GladfieldCrystalLight => write!(f, "[Gladfield Light Crystal]"),
            Malt::GladfieldCrystalMedium => write!(f, "[Gladfield Medium Crystal]"),
            Malt::GladfieldGermanPilsner => write!(f, "[Gladfield German Pilsner Malt]"),
            Malt::GladfieldLagerLight => write!(f, "[Gladfield Lager Light Malt]"),
            Malt::GladfieldMunich => write!(f, "[Gladfield Munich Malt]"),
            Malt::GladfieldPilsner => write!(f, "[Gladfield Pilsner Malt]"),
            Malt::GladfieldShepherdsDelight => write!(f, "[Gladfield Shepherds Delight]"),
            Malt::GladfieldVienna => write!(f, "[Gladfield Vienna Malt]"),
            Malt::GladfieldWheat => write!(f, "[Gladfield Wheat Malt]"),
            Malt::OatHulls => write!(f, "[Oat Hulls]"),
            Malt::RiceHulls => write!(f, "[Rice Hulls]"),
            Malt::SimpsonsMarisOtterPale => write!(f, "[Simpsons Maris Otter Pale Malt]"),
            Malt::WeyermannAcidulated => write!(f, "[Weyermann Acidulated Malt]"),
            Malt::WeyermannBohemianPilsner => write!(f, "[Weyermann Bohemian Pilsner Malt]"),
            Malt::WeyermannCarafaSpecial2 => write!(f, "[Weyermann Carafa Special II]"),
            Malt::WeyermannCarafaSpecial3 => write!(f, "[Weyermann Carafa Special III]"),
            Malt::WeyermannCaramunich2 => write!(f, "[Weyermann CaraMunich Malt II]"),
            Malt::WeyermannMelanoidin => write!(f, "[Weyermann Melanoidin Malt]"),
            Malt::WeyermannMunich1 => write!(f, "[Weyermann Munich Malt I]"),
            Malt::WeyermannMunich2 => write!(f, "[Weyermann Munich Malt II]"),
            Malt::WeyermannPilsner => write!(f, "[Weyermann Pilsner Malt]"),
            Malt::WeyermannVienna => write!(f, "[Weyermann Vienna Malt]"),
            Malt::WeyermannWheatPale => write!(f, "[Weyermann Wheat Malt Pale]"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::units::color::Lovabond;
    use float_cmp::approx_eq;

    #[test]
    fn test_known_malt_lovabond_values() {
        let (low, high) = Malt::WeyermannMunich2.ebc_range();
        assert!(approx_eq!(
            f32,
            Into::<Lovabond>::into(low).0,
            8.0,
            epsilon = 0.2
        ));
        assert!(approx_eq!(
            f32,
            Into::<Lovabond>::into(high).0,
            9.9,
            epsilon = 0.2
        ));

        let (low, high) = Malt::WeyermannCaramunich2.ebc_range();
        assert!(approx_eq!(
            f32,
            Into::<Lovabond>::into(low).0,
            41.9,
            epsilon = 0.2
        ));
        assert!(approx_eq!(
            f32,
            Into::<Lovabond>::into(high).0,
            49.5,
            epsilon = 0.3
        ));

        let (low, high) = Malt::WeyermannAcidulated.ebc_range();
        assert!(approx_eq!(
            f32,
            Into::<Lovabond>::into(low).0,
            1.2,
            epsilon = 0.2
        ));
        assert!(approx_eq!(
            f32,
            Into::<Lovabond>::into(high).0,
            2.3,
            epsilon = 0.2
        ));

        let (low, high) = Malt::WeyermannMelanoidin.ebc_range();
        assert!(approx_eq!(
            f32,
            Into::<Lovabond>::into(low).0,
            23.1,
            epsilon = 0.2
        ));
        assert!(approx_eq!(
            f32,
            Into::<Lovabond>::into(high).0,
            30.6,
            epsilon = 0.2
        ));
    }

    #[test]
    fn test_malt_fan() {
        use strum::IntoEnumIterator;

        for malt in Malt::iter() {
            println!("{}: FAN={}", malt, malt.fan());
        }
    }
}
