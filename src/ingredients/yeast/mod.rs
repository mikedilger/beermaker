use crate::units::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Range;
use strum::EnumIter;

mod gallone;
pub use gallone::{Gallone, STA1};

mod provider;
pub use provider::YeastProvider;

/// Flocculation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
pub enum Flocculation {
    /// Low
    Low,

    /// Low to Medium
    LowMedium,

    /// Medium
    Medium,

    /// Medium to High
    MediumHigh,

    /// High
    High,

    /// Very High
    VeryHigh,
}

/// A type of Yesat
#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter)]
#[allow(missing_docs)]
pub enum Yeast {
    SafAleBE134,
    SafAleBE256,
    SafAleF2,
    SafAleK97,
    SafAleS04,
    SafAleS33,
    SafAleT58,
    SafAleUS05,
    SafBrewDA16,
    SafBrewHA18,
    SafBrewLA01,
    SafLagerS189,
    SafLagerS23,
    SafLagerW3470,
    SafSourLP652,

    ImperialBarbarian,
    ImperialBartlby,
    ImperialCablecar,
    ImperialCitrus,
    ImperialDarkness,
    ImperialDieter,
    ImperialDryHop,
    ImperialFlagship,
    ImperialGlobal,
    ImperialGnome,
    ImperialHarvest,
    ImperialHouse,
    ImperialIndependence,
    ImperialJoystick,
    ImperialJuice,
    ImperialKaiser,
    ImperialKveiking,
    ImperialLoki,
    ImperialNapoleon,
    ImperialPOG,
    ImperialPub,
    ImperialRustic,
    ImperialSourBatchKidz,
    ImperialStefon,
    ImperialSuburbanBrett,
    ImperialTartan,
    ImperialTripleDouble,
    ImperialUrkel,
    ImperialWhiteout,

    LalBrewAbbaye,
    LalBrewBelleSaison,
    LalBrewBRY97,
    LalBrewCBC1,
    LalBrewDiamondLager,
    LalBrewKoln,
    LalBrewLondon,
    LalBrewMunichClassic,
    LalBrewNewEngland,
    LalBrewNottingham,
    LalBrewVerdantIPA,
    LalBrewVoss,
    LalBrewWindsor
    LalBrewWit
    LallemandPriseDeMousseWine,
    LallemandSourvisiae
    LallemandWildBrewPhillySour,

    MangroveJackBavarianWheat,
    MangroveJackBelgianAle,
    MangroveJackBelgianWit,
    MangroveJackBohemianLager,
    MangroveJackCaliforniaLager,
    MangroveJackEmpireAle,
    MangroveJackFrenchSaisonAle,
    MangroveJackLibertyBellAle,
    MangroveJackNewWorldStrongAle,
    MangroveJackUSWestCoast,
    MangroveJackWorkhorse,

    MuntonsPremiumGold,
    MuntonsStandardAle,

    WLP001,
    WLP002,
    WLP003,
    WLP004,
    WLP005,
    WLP006,
    WLP007,
    WLP008,
    WLP009,
    WLP011,
    WLP013,
    WLP017,
    WLP019,
    WLP022,
    WLP023,
    WLP025,
    WLP026,
    WLP028,
    WLP029,
    WLP030,
    WLP033,
    WLP036,
    WLP037,
    WLP039,
    WLP041,
    WLP045,
    WLP050,
    WLP051,
    WLP059,
    WLP060,
    WLP064,
    WLP065,
    WLP066,
    WLP067,
    WLP070,
    WLP072,
    WLP073,
    WLP075,
    WLP076,
    WLP077,
    WLP078,
    WLP080,
    WLP085,
    WLP090,
    WLP091,
    SLP095,
    WLP096,
    WLP099,
    WLP101,
    WLP1983,
    WLP300,
    WLP320,
    WLP351,
    WLP380,
    WLP400,
    WLP4000,
    WLP4001,
    WLP4007,
    WLP4015,
    WLP4020,
    WLP4021,
    WLP4022,
    WLP4023,
    WLP4025,
    WLP4027,
    WLP4030,
    WLP4035,
    WLP4040,
    WLP4042,
    WLP4044,
    WLP4045,
    WLP4046,
    WLP4047,
    WLP4050,
    WLP4051,
    WLP4052,
    WLP4053,
    WLP4060,
    WLP4061,
    WLP4062,
    WLP410,
    WLP4605,
    WLP4615,
    WLP4620,
    WLP4626,
    WLP4633,
    WLP4636,
    WLP4637,
    WLP4638,
    WLP4639,
    WLP4640,
    WLP4641,
    WLP4642,
    WLP4643,
    WLP4645,
    WLP4650,
    WLP4651,
    WLP4653,
    WLP4655,
    WLP4656,
    WLP4665,
    WLP4675,
    WLP4681,
    WLP4682,
    WLP4684,
    WLP500,
    WLP510,
    WLP515,
    WLP518,
    WLP519,
    WLP520,
    WLP521,
    WLP530,
    WLP540,
    WLP545,
    WLP546,
    WLP548,
    WLP550,
    WLP561,
    WLP564,
    WLP565,
    WLP566,
    WLP568,
    WLP570,
    WLP575,
    WLP585,
    WLP590,
    WLP600,
    WLP603,
    WLP611,
    WLP616,
    WLP618,
    WLP630,
    WLP631,
    WLP6420,
    WLP644,
    WLP645,
    WLP648,
    WLP650,
    WLP653,
    WLP655,
    WLP661,
    WLP665,
    WLP669,
    WLP670,
    WLP672,
    WLP673,
    WLP675,
    WLP677,
    WLP678,
    WLP685,
    WLP686,
    WLP690,
    WLP692,
    WLP693,
    WLP700,
    WLP705,
    WLP707,
    WLP709,
    WLP715,
    WLP718,
    WLP720,
    WLP727,
    WLP730,
    WLP735,
    WLP740,
    WLP749,
    WLP750,
    WLP760,
    WLP770,
    WLP773,
    WLP775,
    WLP780,
    WLP800,
    WLP802,
    WLP808,
    WLP810,
    WLP815,
    WLP820,
    WLP830,
    WLP833,
    WLP835,
    WLP838,
    WLP840,
    WLP845,
    WLP850,
    WLP860,
    WLP885,
    WLP920,
    WLP925,
    WLP940,

    /// Kveik Voss
    //   - Omega Yeast OYL-061, Lallemand Kveik Ale Yeast,
    //    Imperial Yeast Ale Loki,
    KveikVoss,

    /// Lutra Kveik
    // OYL-071
    // pseudolager, lager-like at 68-72C
    LutraKveik,

    /// Lallemand Munich Classic German Wheat-Style Ale Yeast
    LallemandMunichClassic,

    /// Lallemand Nottingham high performance English Ale yeast
    LallemandNottingham,

    /// Lallemand Windsor British-style Ale yeast
    LallemandWindsor,

    /// Safale S-04 English Ale
    SafaleS04,

    /// Safale S-33
    SafaleS33,

    /// Safale T-58, Phenolic for English/Belgian
    /// high banana, clove, pepper, great with wheat beers
    SafaleT58,

    /// Safale US-05, Neutral ale yeast
    SafaleUS05,

    /// Safale W-68 (german style wheat beers), can't find in NZ
    /// Wyeast Strain 3068 from Weihenstephan Weizen beers,
    /// but behaves differently as it is dry.
    SafaleW68,

    /// Safale WB-06 (wheat)
    SafaleWB06,

    /// Saflager W-34/70
    SaflagerW3470,

    /// White labs Hefeweizen Ale Yeast, WLP300
    /// Wyeast Strain 3068 from Weihenstephan Weizen beers
    WLP300,

    /// White Labs Bavarian Weizen Ale Yeast, WLP351
    WLP351,

    /// White Labs Hefeweizen IV Ale Yeast, WLP380
    WLP380,

    /// White Labs Oktoberfest/Märzen WLP820
    WLP820,

    /// White Labs German Lager Yeast WLP830
    // I've heard this is strain 34/70, as is Wyeast 2206
    WLP830,

    /// White Labs German Bock Lager Yeast WLP833
    WLP833,

    /// White Labs German X Lager Yeast WLP835
    WLP835,

    /// White Labs Southern German Lager Yeast WLP838
    WLP838,
    // EC-1118 dry sparkling wine yeast
    // great for bottle conditioning

    // CBC-1
    // great for bottle conditioning
}

impl Yeast {
    /// Sources say it is between 5b and 20b depending on
    /// viability of the yeast.  Fermentis guarantees a
    /// minimum of 6b, many home brew tests show it is
    /// closer to 20b.  We pick a number in between.
    pub const CELLS_PER_GRAM_DRY: u64 = 13_000_000_000;

    /// The minimum recommended temperature to ferment at
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn temp_range(&self) -> Range<Celsius> {
        match *self {
            Yeast::KveikVoss => Celsius(25.0)..Celsius(40.0),
            Yeast::LutraKveik => Celsius(12.0)..Celsius(35.0),
            Yeast::LallemandMunichClassic => Celsius(17.0)..Celsius(25.0),
            Yeast::LallemandNottingham => Celsius(10.0)..Celsius(25.0),
            Yeast::LallemandWindsor => Celsius(15.0)..Celsius(25.0),
            Yeast::SafaleS04 => Celsius(18.0)..Celsius(26.0),
            Yeast::SafaleS33 => Celsius(18.0)..Celsius(26.0),
            Yeast::SafaleT58 => Celsius(18.0)..Celsius(26.0),
            Yeast::SafaleUS05 => Celsius(18.0)..Celsius(26.0),
            Yeast::SafaleW68 => Celsius(18.0)..Celsius(26.0),
            Yeast::SafaleWB06 => Celsius(18.0)..Celsius(26.0),
            Yeast::SaflagerW3470 => Celsius(12.0)..Celsius(18.0),
            Yeast::WLP300 => Celsius(20.0)..Celsius(22.0),
            Yeast::WLP351 => Celsius(19.0)..Celsius(21.0),
            Yeast::WLP380 => Celsius(19.0)..Celsius(21.0),
            Yeast::WLP820 => Celsius(11.0)..Celsius(14.0),
            Yeast::WLP830 => Celsius(10.0)..Celsius(13.0),
            Yeast::WLP833 => Celsius(9.0)..Celsius(13.0),
            Yeast::WLP835 => Celsius(10.0)..Celsius(12.0),
            Yeast::WLP838 => Celsius(10.0)..Celsius(13.0),
        }
    }

    /// The recommended fermentation temperature
    #[must_use]
    pub fn temp(&self) -> Celsius {
        let range = self.temp_range();
        Celsius(f32::midpoint(range.start.0, range.end.0))
    }

    /// Attenuation (apparent), fraction
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn attenuation_range(&self) -> Range<f32> {
        match *self {
            Yeast::KveikVoss => 0.76..0.82,
            Yeast::LutraKveik => 0.75..0.82,
            Yeast::LallemandMunichClassic => 0.76..0.83,
            Yeast::LallemandNottingham => 0.78..0.84,
            Yeast::LallemandWindsor => 0.65..0.72,
            Yeast::SafaleS04 => 0.74..0.82,
            Yeast::SafaleS33 => 0.68..0.72,
            Yeast::SafaleT58 => 0.72..0.78,
            Yeast::SafaleUS05 => 0.78..0.82,
            Yeast::SafaleW68 => 0.78..0.84,
            Yeast::SafaleWB06 => 0.86..0.90,
            Yeast::SaflagerW3470 => 0.80..0.84,
            Yeast::WLP300 => 0.72..0.76,
            Yeast::WLP351 => 0.75..0.82,
            Yeast::WLP380 => 0.73..0.80,
            Yeast::WLP820 => 0.65..0.73,
            Yeast::WLP830 => 0.74..0.79,
            Yeast::WLP833 => 0.70..0.76,
            Yeast::WLP835 => 0.70..0.76,
            Yeast::WLP838 => 0.68..0.76,
        }
    }

    /// The expected apparent attenuation
    #[must_use]
    pub fn attenuation(&self) -> f32 {
        let range = self.attenuation_range();
        f32::midpoint(range.start, range.end)
    }

    /// Alcohol tolerance, fraction, range
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn alcohol_tolerance_range(&self) -> Range<f32> {
        match *self {
            Yeast::KveikVoss => 0.12..0.12,
            Yeast::LutraKveik => 0.15..0.15,
            Yeast::LallemandMunichClassic => 0.12..0.12,
            Yeast::LallemandNottingham => 0.14..0.14,
            Yeast::LallemandWindsor => 0.12..0.12,
            Yeast::SafaleS04 => 0.09..0.11,
            Yeast::SafaleS33 => 0.09..0.11,
            Yeast::SafaleT58 => 0.09..0.11,
            Yeast::SafaleUS05 => 0.09..0.11,
            Yeast::SafaleW68 => 0.09..0.11, // unlisted on specs
            Yeast::SafaleWB06 => 0.09..0.11,
            Yeast::SaflagerW3470 => 0.09..0.11,
            Yeast::WLP300 => 0.08..0.12,
            Yeast::WLP351 => 0.15..0.15,
            Yeast::WLP380 => 0.05..0.10,
            Yeast::WLP820 => 0.05..0.10,
            Yeast::WLP830 => 0.05..0.10,
            Yeast::WLP833 => 0.05..0.10,
            Yeast::WLP835 => 0.08..0.12,
            Yeast::WLP838 => 0.05..0.10,
        }
    }

    /// Alcohol tolerance, expected
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn alcohol_tolerance(&self) -> f32 {
        let range = self.alcohol_tolerance_range();
        f32::midpoint(range.start, range.end)
    }

    /// Flocculation
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn flocculation(&self) -> Flocculation {
        match *self {
            Yeast::KveikVoss => Flocculation::VeryHigh,
            Yeast::LutraKveik => Flocculation::MediumHigh,
            Yeast::LallemandMunichClassic => Flocculation::Low,
            Yeast::LallemandNottingham => Flocculation::High,
            Yeast::LallemandWindsor => Flocculation::Low,
            Yeast::SafaleS04 => Flocculation::High,
            Yeast::SafaleS33 => Flocculation::Medium,
            Yeast::SafaleT58 => Flocculation::Medium,
            Yeast::SafaleUS05 => Flocculation::Medium,
            Yeast::SafaleW68 => Flocculation::Medium,
            Yeast::SafaleWB06 => Flocculation::Low,
            Yeast::SaflagerW3470 => Flocculation::High,
            Yeast::WLP300 => Flocculation::Low,
            Yeast::WLP351 => Flocculation::Low,
            Yeast::WLP380 => Flocculation::Low,
            Yeast::WLP820 => Flocculation::Medium,
            Yeast::WLP830 => Flocculation::Medium,
            Yeast::WLP833 => Flocculation::Medium,
            Yeast::WLP835 => Flocculation::Medium,
            Yeast::WLP838 => Flocculation::MediumHigh,
        }
    }

    /// Is the yeast dry?
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn is_dry(&self) -> bool {
        match *self {
            Yeast::KveikVoss => true,
            Yeast::LutraKveik => false,
            Yeast::LallemandMunichClassic => true,
            Yeast::LallemandNottingham => true,
            Yeast::LallemandWindsor => true,
            Yeast::SafaleS04 => true,
            Yeast::SafaleS33 => true,
            Yeast::SafaleT58 => true,
            Yeast::SafaleUS05 => true,
            Yeast::SafaleW68 => true,
            Yeast::SafaleWB06 => true,
            Yeast::SaflagerW3470 => true,
            Yeast::WLP300 => false,
            Yeast::WLP351 => false,
            Yeast::WLP380 => false,
            Yeast::WLP820 => false,
            Yeast::WLP830 => false,
            Yeast::WLP833 => false,
            Yeast::WLP835 => false,
            Yeast::WLP838 => false,
        }
    }

    /// Pitching rate, if known
    #[must_use]
    pub fn pitching_rate(&self) -> Option<(Grams, Liters)> {
        match *self {
            // 50-100g/hL
            Yeast::LallemandMunichClassic => Some((Grams(75.0), Liters(100.0))),
            // 50-100g/hL
            Yeast::LallemandNottingham => Some((Grams(75.0), Liters(100.0))),
            _ => None,
        }
    }

    /// FAN requirements, minimum, if known, for standard gravity of 1.040
    /// Worts generally should have (different people say different things):
    ///     180-200 ppm standard
    ///     250-300 ppm high gravity
    #[must_use]
    pub fn fan_requirement(&self) -> Ppm {
        match *self {
            Yeast::KveikVoss => Ppm(180.0),
            Yeast::LutraKveik => Ppm(180.0),
            Yeast::LallemandMunichClassic => Ppm(180.0),
            Yeast::LallemandNottingham => Ppm(150.0), // ale
            Yeast::LallemandWindsor => Ppm(150.0),    // ale
            Yeast::SafaleS04 => Ppm(150.0),           // ale
            Yeast::SafaleS33 => Ppm(150.0),           // ale
            Yeast::SafaleT58 => Ppm(150.0),           // ale
            Yeast::SafaleUS05 => Ppm(150.0),          // ale
            Yeast::SafaleW68 => Ppm(150.0),           // ale
            Yeast::SafaleWB06 => Ppm(150.0),          // ale
            Yeast::SaflagerW3470 => Ppm(100.0),       // lager
            Yeast::WLP300 => Ppm(180.0),
            Yeast::WLP351 => Ppm(100.0), // Low N consumer
            Yeast::WLP380 => Ppm(150.0),
            Yeast::WLP820 => Ppm(100.0), // lager
            Yeast::WLP830 => Ppm(100.0), // lager
            Yeast::WLP833 => Ppm(100.0), // bock lager
            Yeast::WLP835 => Ppm(100.0), // lager
            Yeast::WLP838 => Ppm(100.0), // lager
        }
    }
}

impl fmt::Display for Yeast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Yeast::LutraKveik => write!(f, "[Lutra Kveik OYL-071]"),
            Yeast::KveikVoss => write!(f, "[Kveik Voss]"),
            Yeast::LallemandMunichClassic => {
                write!(f, "[Lallemand Munich Classic German Wheat-Style Ale Yeast]")
            }
            Yeast::LallemandNottingham => write!(f, "[Lallemand Nottingham Ale Yeast]"),
            Yeast::LallemandWindsor => write!(f, "[Lallemand Windsor Ale Yeast]"),
            Yeast::SafaleS04 => write!(f, "[Safale S-04]"),
            Yeast::SafaleS33 => write!(f, "[Safale S-33]"),
            Yeast::SafaleT58 => write!(f, "[Safale T58]"),
            Yeast::SafaleUS05 => write!(f, "[Safale US-05]"),
            Yeast::SafaleW68 => write!(f, "[Safale W68)]"),
            Yeast::SafaleWB06 => write!(f, "[Safale WB-06]"),
            Yeast::SaflagerW3470 => write!(f, "[SafLager W-34/70]"),
            Yeast::WLP300 => write!(f, "[White Labs Hefeweizen Ale Yeast WLP300]"),
            Yeast::WLP351 => write!(f, "[White Labs Bavarian Weizen Ale Yeast WLP351]"),
            Yeast::WLP380 => write!(f, "[White Labs Hefeweizen IV Ale Yeast WLP380]"),
            Yeast::WLP820 => write!(f, "[White Labs Oktoberfest/Märzen WLP820]"),
            Yeast::WLP830 => write!(f, "[White Labs German Lager Yeast WLP830]"),
            Yeast::WLP833 => write!(f, "[White Labs German Bock Lager Yeast WLP833]"),
            Yeast::WLP835 => write!(f, "[White Labs German X Lager Yeast WLP835]"),
            Yeast::WLP838 => write!(f, "[White Labs Southern German Lager Yeast WLP838]"),
        }
    }
}
