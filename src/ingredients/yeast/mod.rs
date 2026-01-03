use crate::units::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Range;
use strum::EnumIter;

mod gallone;
pub use gallone::{Gallone, STA1};

mod provider;
pub use provider::YeastProvider;

mod strain;
pub use strain::Strain;

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

    /// SafAle BW-20
    ///
    /// Belgian wheat, Flanders
    /// Low to moderate phenolic character
    /// Less phenolic character at 23 C. Ferments well at 20C, in 10 days.
    /// Can crop/repitch. Not good for refermentation (bottle conditioning).
    /// Weaker fruit and spice than both WB-06 and W-68... letting the
    /// coriander/orange shine through.
    SafAleBW20,

    SafAleK97,
    SafAleS04,
    SafAleS33,
    SafAleT58,
    SafAleUS05,

    /// SafAle W-68
    ///
    /// Weihenstephanen 3068
    /// German wheat beers, fruity with phenolic character.
    /// Less spicy, more fruity, than WB-06.
    /// Fast fermenter.
    /// At 23C, less pehnolic character, more fruitiness.
    /// Prefers 13.5°P density. 50-80 g/hl pitch rate.
    /// POF+, V-4G.
    /// Can crop/repitch. Not good for refermentation (bottle conditioning).
    SafAleW68,

    /// SafAle WB-06
    ///
    /// Their first wheat beer yeast, all-rounder
    /// High levels of spice and fruit.
    /// Clove-like aromas regardless of fermentation conditions.
    /// High-density increases sensory intensity, solventy, bitterness
    /// and sweetness perception.
    /// Low temps promote fruity armoas, mainly banana notes.
    /// Overpitching at low densities can increase risk of sulfury notes.
    SafAleWB06,

    SafLagerE30,

    /// SafLager S-189
    /// Swiss lager yeast, Hurlimann brewery
    /// Fairly neutral, can yield herbal/floral notes,
    /// Under some conditions can produce pear/apple/berry flavors too
    /// but these can fade out quickly.
    SafLagerS189,

    /// SafLager S-23
    /// Lager yeast from Berlin
    /// Recommended for fruitier or more estery lagers.
    SafLagerS23,

    SafLagerSH45,
    SafLagerW3470,

    // ImperialBarbarian,
    // ImperialBartlby,
    // ImperialCablecar,
    // ImperialCitrus,
    // ImperialDarkness,
    // ImperialDieter,
    // ImperialDryHop,
    // ImperialFlagship,
    // ImperialGlobal,
    // ImperialGnome,
    // ImperialHarvest,
    // ImperialHouse,
    // ImperialIndependence,
    // ImperialJoystick,
    // ImperialJuice,
    // ImperialKaiser,
    // ImperialKveiking,
    // ImperialLoki,   // KveikVoss
    // ImperialNapoleon,
    // ImperialPOG,
    // ImperialPub,
    // ImperialRustic,
    // ImperialSourBatchKidz,
    // ImperialStefon,
    // ImperialSuburbanBrett,
    // ImperialTartan,
    // ImperialTripleDouble,
    // ImperialUrkel,
    // ImperialWhiteout,
    LalBrewAbbaye,
    LalBrewAurora,
    LalBrewBelleSaison,
    LalBrewBRY97,
    LalBrewCBC1,
    LalBrewDiamondLager,
    LalBrewFarmhouse,
    LalBrewHouseAle,
    LalBrewLoNa,
    LalBrewMunichClassic,
    LalBrewNewEngland,
    LalBrewNottingham,
    LalBrewNovaLager,
    LalBrewPomona,
    LalBrewVerdantIPA,
    LalBrewVoss,
    LalBrewWindsor,
    LalBrewWit,

    // MangroveJackBavarianWheat,
    // MangroveJackBelgianAle,
    // MangroveJackBelgianWit,
    // MangroveJackBohemianLager,
    // MangroveJackCaliforniaLager,
    // MangroveJackEmpireAle,  ScottishHeavyAle, ...
    // MangroveJackFrenchSaisonAle,
    // MangroveJackLibertyBellAle,
    // MangroveJackNewWorldStrongAle,
    // MangroveJackUSWestCoast,
    // MangroveJackWorkhorse,

    // MuntonsPremiumGold,
    // MuntonsStandardAle,
    // OYL-052 (Conan strain)
    OYL061, // Voss Kveik
    OYL071, // Lutra Kveik

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
    //WLP019,
    WLP022,
    WLP023,
    WLP025,
    WLP026,
    WLP028,
    WLP029,
    //WLP030,
    WLP033,
    WLP036,
    WLP037,
    //WLP038,
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
    WLP095,
    WLP096,
    WLP099,
    WLP101,
    WLP1983,

    /// White labs Hefeweizen Ale Yeast, WLP300
    /// Wyeast Strain 3068 from Weihenstephan Weizen beers
    WLP300,

    WLP320,

    /// White Labs Bavarian Weizen Ale Yeast, WLP351
    WLP351,

    /// White Labs Hefeweizen IV Ale Yeast, WLP380
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
    // WLP4605,
    // WLP4615,
    // WLP4620,
    // WLP4626,
    // WLP4633,
    // WLP4636,
    // WLP4637,
    // WLP4638,
    // WLP4639,
    // WLP4640,
    // WLP4641,
    // WLP4642,
    // WLP4643,
    // WLP4645,
    // WLP4650,
    // WLP4651,
    // WLP4653,
    // WLP4655,
    // WLP4656,
    // WLP4665,
    // WLP4675,
    // WLP4681,
    // WLP4682,
    // WLP4684,
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
    //WLP600,
    //WLP603,
    WLP611,
    //WLP616,
    //WLP618,
    WLP630,
    // WLP631,
    // WLP6420,
    // WLP644,
    // WLP645,
    // WLP648,
    // WLP650,
    // WLP653,
    // WLP655,
    // WLP661,
    // WLP665,
    // WLP669,
    // WLP670,
    // WLP672,
    // WLP673,
    // WLP675,
    // WLP677,
    // WLP678,
    // WLP685,
    // WLP686,
    // WLP690,
    // WLP692,
    // WLP693,
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
    //WLP775,
    //WLP780,
    WLP800,
    WLP802,
    WLP808,
    WLP810,
    WLP815,

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

    WLP840,
    WLP845,
    WLP850,
    WLP860,
    WLP885,
    WLP920,
    WLP925,
    WLP940,
    // EC-1118 dry sparkling wine yeast
    // great for bottle conditioning

    // CBC-1
    // great for bottle conditioning
}

impl Yeast {
    /// Provider
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn provider(&self) -> YeastProvider {
        match *self {
            Self::SafAleBE134 |
            Self::SafAleBE256 |
            Self::SafAleBW20 |
            Self::SafAleK97 |
            Self::SafAleS04 |
            Self::SafAleS33 |
            Self::SafAleT58 |
            Self::SafAleUS05 |
            Self::SafAleW68 |
            Self::SafAleWB06 |
            Self::SafLagerE30 |
            Self::SafLagerS189 |
            Self::SafLagerS23 |
            Self::SafLagerSH45 |
            Self::SafLagerW3470 => YeastProvider::Fermentis,

            // Self::ImperialBarbarian |
            // Self::ImperialBartlby |
            // Self::ImperialCablecar |
            // Self::ImperialCitrus |
            // Self::ImperialDarkness |
            // Self::ImperialDieter |
            // Self::ImperialDryHop |
            // Self::ImperialFlagship |
            // Self::ImperialGlobal |
            // Self::ImperialGnome |
            // Self::ImperialHarvest |
            // Self::ImperialHouse |
            // Self::ImperialIndependence |
            // Self::ImperialJoystick |
            // Self::ImperialJuice |
            // Self::ImperialKaiser |
            // Self::ImperialKveiking |
            // Self::ImperialLoki |
            // Self::ImperialNapoleon |
            // Self::ImperialPOG |
            // Self::ImperialPub |
            // Self::ImperialRustic |
            // Self::ImperialSourBatchKidz |
            // Self::ImperialStefon |
            // Self::ImperialSuburbanBrett |
            // Self::ImperialTartan |
            // Self::ImperialTripleDouble |
            // Self::ImperialUrkel |
            // Self::ImperialWhiteout => YeastProvider::ImperialYeast,

            Self::LalBrewAbbaye |
            Self::LalBrewAurora |
            Self::LalBrewBelleSaison |
            Self::LalBrewBRY97 |
            Self::LalBrewCBC1 |
            Self::LalBrewDiamondLager |
            Self::LalBrewFarmhouse |
            Self::LalBrewHouseAle |
            Self::LalBrewLoNa |
            Self::LalBrewMunichClassic |
            Self::LalBrewNewEngland |
            Self::LalBrewNottingham |
            Self::LalBrewNovaLager |
            Self::LalBrewPomona |
            Self::LalBrewVerdantIPA |
            Self::LalBrewVoss |
            Self::LalBrewWindsor |
            Self::LalBrewWit => YeastProvider::Lallemand,

            Self::OYL061 |
            Self::OYL071 => YeastProvider::OmegaYeastLabs,

            Self::WLP001 |
            Self::WLP002 |
            Self::WLP003 |
            Self::WLP004 |
            Self::WLP005 |
            Self::WLP006 |
            Self::WLP007 |
            Self::WLP008 |
            Self::WLP009 |
            Self::WLP011 |
            Self::WLP013 |
            Self::WLP017 |
            //Self::WLP019 |
            Self::WLP022 |
            Self::WLP023 |
            Self::WLP025 |
            Self::WLP026 |
            Self::WLP028 |
            Self::WLP029 |
            //Self::WLP030 |
            Self::WLP033 |
            Self::WLP036 |
            Self::WLP037 |
            //Self::WLP038 |
            Self::WLP039 |
            Self::WLP041 |
            Self::WLP045 |
            Self::WLP050 |
            Self::WLP051 |
            Self::WLP059 |
            Self::WLP060 |
            Self::WLP064 |
            Self::WLP065 |
            Self::WLP066 |
            Self::WLP067 |
            Self::WLP070 |
            Self::WLP072 |
            Self::WLP073 |
            Self::WLP075 |
            Self::WLP076 |
            Self::WLP077 |
            Self::WLP078 |
            Self::WLP080 |
            Self::WLP085 |
            Self::WLP090 |
            Self::WLP091 |
            Self::WLP095 |
            Self::WLP096 |
            Self::WLP099 |
            Self::WLP101 |
            Self::WLP1983 |
            Self::WLP300 |
            Self::WLP320 |
            Self::WLP351 |
            Self::WLP380 |
            Self::WLP400 |
            Self::WLP4000 |
            Self::WLP4001 |
            Self::WLP4007 |
            Self::WLP4015 |
            Self::WLP4020 |
            Self::WLP4021 |
            Self::WLP4022 |
            Self::WLP4023 |
            Self::WLP4025 |
            Self::WLP4027 |
            Self::WLP4030 |
            Self::WLP4035 |
            Self::WLP4040 |
            Self::WLP4042 |
            Self::WLP4044 |
            Self::WLP4045 |
            Self::WLP4046 |
            Self::WLP4047 |
            Self::WLP4050 |
            Self::WLP4051 |
            Self::WLP4052 |
            Self::WLP4053 |
            Self::WLP4060 |
            Self::WLP4061 |
            Self::WLP4062 |
            Self::WLP410 |
            // Self::WLP4605 |
            // Self::WLP4615 |
            // Self::WLP4620 |
            // Self::WLP4626 |
            // Self::WLP4633 |
            // Self::WLP4636 |
            // Self::WLP4637 |
            // Self::WLP4638 |
            // Self::WLP4639 |
            // Self::WLP4640 |
            // Self::WLP4641 |
            // Self::WLP4642 |
            // Self::WLP4643 |
            // Self::WLP4645 |
            // Self::WLP4650 |
            // Self::WLP4651 |
            // Self::WLP4653 |
            // Self::WLP4655 |
            // Self::WLP4656 |
            // Self::WLP4665 |
            // Self::WLP4675 |
            // Self::WLP4681 |
            // Self::WLP4682 |
            // Self::WLP4684 |
            Self::WLP500 |
            Self::WLP510 |
            Self::WLP515 |
            Self::WLP518 |
            Self::WLP519 |
            Self::WLP520 |
            Self::WLP521 |
            Self::WLP530 |
            Self::WLP540 |
            Self::WLP545 |
            Self::WLP546 |
            Self::WLP548 |
            Self::WLP550 |
            Self::WLP561 |
            Self::WLP564 |
            Self::WLP565 |
            Self::WLP566 |
            Self::WLP568 |
            Self::WLP570 |
            Self::WLP575 |
            Self::WLP585 |
            Self::WLP590 |
            //Self::WLP600 |
            //Self::WLP603 |
            Self::WLP611 |
            //Self::WLP616 |
            //Self::WLP618 |
            Self::WLP630 |
            // Self::WLP631 |
            // Self::WLP6420 |
            // Self::WLP644 |
            // Self::WLP645 |
            // Self::WLP648 |
            // Self::WLP650 |
            // Self::WLP653 |
            // Self::WLP655 |
            // Self::WLP661 |
            // Self::WLP665 |
            // Self::WLP669 |
            // Self::WLP670 |
            // Self::WLP672 |
            // Self::WLP673 |
            // Self::WLP675 |
            // Self::WLP677 |
            // Self::WLP678 |
            // Self::WLP685 |
            // Self::WLP686 |
            // Self::WLP690 |
            // Self::WLP692 |
            // Self::WLP693 |
            Self::WLP700 |
            Self::WLP705 |
            Self::WLP707 |
            Self::WLP709 |
            Self::WLP715 |
            Self::WLP718 |
            Self::WLP720 |
            Self::WLP727 |
            Self::WLP730 |
            Self::WLP735 |
            Self::WLP740 |
            Self::WLP749 |
            Self::WLP750 |
            Self::WLP760 |
            Self::WLP770 |
            Self::WLP773 |
            //Self::WLP775 |
            //Self::WLP780 |
            Self::WLP800 |
            Self::WLP802 |
            Self::WLP808 |
            Self::WLP810 |
            Self::WLP815 |
            Self::WLP820 |
            Self::WLP830 |
            Self::WLP833 |
            Self::WLP835 |
            Self::WLP838 |
            Self::WLP840 |
            Self::WLP845 |
            Self::WLP850 |
            Self::WLP860 |
            Self::WLP885 |
            Self::WLP920 |
            Self::WLP925 |
            Self::WLP940 => YeastProvider::WhiteLabs,
        }
    }

    /// Description
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn desc(&self) -> &str {
        match *self {
            Self::SafAleBE134 => "SafAle BE-134",
            Self::SafAleBE256 => "SafAle BE-256",
            Self::SafAleBW20 => "SafAle BW-20",
            Self::SafAleK97 => "SafAle K-97",
            Self::SafAleS04 => "SafAle S-04",
            Self::SafAleS33 => "SafAle S-33",
            Self::SafAleT58 => "SafAle T-58",
            Self::SafAleUS05 => "SafAle US-05",
            Self::SafAleW68 => "SafAle W-68",
            Self::SafAleWB06 => "SafAle WB-06",
            Self::SafLagerE30 => "SafLager E-30",
            Self::SafLagerS189 => "SafLager S-189",
            Self::SafLagerS23 => "SafLager S-23",
            Self::SafLagerSH45 => "SafLager SH-45",
            Self::SafLagerW3470 => "SafLager W-34/70",
            // Self::ImperialBarbarian => "",
            // Self::ImperialBartlby => "",
            // Self::ImperialCablecar => "",
            // Self::ImperialCitrus => "",
            // Self::ImperialDarkness => "",
            // Self::ImperialDieter => "",
            // Self::ImperialDryHop => "",
            // Self::ImperialFlagship => "",
            // Self::ImperialGlobal => "",
            // Self::ImperialGnome => "",
            // Self::ImperialHarvest => "",
            // Self::ImperialHouse => "",
            // Self::ImperialIndependence => "",
            // Self::ImperialJoystick => "",
            // Self::ImperialJuice => "",
            // Self::ImperialKaiser => "",
            // Self::ImperialKveiking => "",
            // Self::ImperialLoki => "",
            // Self::ImperialNapoleon => "",
            // Self::ImperialPOG => "",
            // Self::ImperialPub => "",
            // Self::ImperialRustic => "",
            // Self::ImperialSourBatchKidz => "",
            // Self::ImperialStefon => "",
            // Self::ImperialSuburbanBrett => "",
            // Self::ImperialTartan => "",
            // Self::ImperialTripleDouble => "",
            // Self::ImperialUrkel => "",
            // Self::ImperialWhiteout => "",
            Self::LalBrewAbbaye => "LalBrew Abbaye",
            Self::LalBrewAurora => "LalBrew Aurora",
            Self::LalBrewBelleSaison => "LalBrew Belle Saison",
            Self::LalBrewBRY97 => "LalBrew BRY-97",
            Self::LalBrewCBC1 => "LalBrew CBC-1",
            Self::LalBrewDiamondLager => "LalBrew Diamond Lager",
            Self::LalBrewFarmhouse => "LalBrew Farmhouse",
            Self::LalBrewHouseAle => "LalBrew House Ale",
            Self::LalBrewLoNa => "LalBrew LoNa",
            Self::LalBrewMunichClassic => "LalBrew Munich Classic",
            Self::LalBrewNewEngland => "LalBrew New England",
            Self::LalBrewNottingham => "LalBrew Nottingham",
            Self::LalBrewNovaLager => "LalBrew NovaLager",
            Self::LalBrewPomona => "LalBrew Pomona",
            Self::LalBrewVerdantIPA => "LalBrew Verdant IPA",
            Self::LalBrewVoss => "LalBrew Voss",
            Self::LalBrewWindsor => "LalBrew Windsor",
            Self::LalBrewWit => "LalBrew Wit",

            Self::OYL061 => "Voss Kveik",
            Self::OYL071 => "Lutra Kveik",
            Self::WLP001 => "WLP001 California Ale Yeast",
            Self::WLP002 => "WLP002 English Ale Yeast",
            Self::WLP003 => "WLP003 German Ale II Yeast",
            Self::WLP004 => "WLP004 Irish Ale Yeast",
            Self::WLP005 => "WLP005 British Ale Yeast",
            Self::WLP006 => "WLP006 Beddford British Ale Yeast",
            Self::WLP007 => "WLP007 Crisp English Ale Yeast",
            Self::WLP008 => "WLP008 East Coast Ale Yeast",
            Self::WLP009 => "WLP009 Australian Ale Yeast",
            Self::WLP011 => "WLP011 European Ale Yeast",
            Self::WLP013 => "WLP013 London Ale Yeast",
            Self::WLP017 => "WLP017 Whitbread II Ale Yeast",
            //Self::WLP019 => "WLP019 California IV Ale Yeast",
            Self::WLP022 => "WLP022 Essex Ale Yeast",
            Self::WLP023 => "WLP023 Burton Ale Yeast",
            Self::WLP025 => "WLP025 Southwold Ale Yeast",
            Self::WLP026 => "WLP026 Premium Bitter Ale Yeast",
            Self::WLP028 => "WLP028 Edinburgh/Scottish Ale Yeast",
            Self::WLP029 => "WLP029 Kölsch Ale Yeast",
            //Self::WLP030 => "WLP030 Thames Valley Ale Yeast",
            Self::WLP033 => "WLP033 Klassic Ale Yeast",
            Self::WLP036 => "WLP036 Dusseldorf Alt Ale Yeast",
            Self::WLP037 => "WLP037 Yorkshire Square Ale Yeast",
            //Self::WLP038 => "WLP038 Manchester Ale Yeast",
            Self::WLP039 => "WLP039 East Midlands Ale Yeast",
            Self::WLP041 => "WLP041 Pacific Ale Yeast",
            Self::WLP045 => "WLP045 Scotch Whisky Yeast",
            Self::WLP050 => "WLP050 Tennessee Whiskey Yeast",
            Self::WLP051 => "WLP051 California V Ale Yeast",
            Self::WLP059 => "WLP059 Melbourne Ale Yeast",
            Self::WLP060 => "WLP060 American Ale Yeast Blend",
            Self::WLP064 => "WLP064 Buchner Ale Yeast Blend",
            Self::WLP065 => "WLP065 American Whiskey Yeast",
            Self::WLP066 => "WLP066 London Fog Ale Yeast",
            Self::WLP067 => "WLP067 Coastal Haze Ale Yeast Blend",
            Self::WLP070 => "WLP070 Kentucky Bourbon Yeast",
            Self::WLP072 => "WLP072 French Ale Yeast",
            Self::WLP073 => "WLP073 Artisanal Country Ale Yeast",
            Self::WLP075 => "WLP075 Hansen Ale Yeast Blend",
            Self::WLP076 => "WLP076 Old Sonoma Ale Yeast",
            Self::WLP077 => "WLP077 Tropicale Yeast Blend",
            Self::WLP078 => "WLP078 Neutral Grain Yeast",
            Self::WLP080 => "WLP080 Cream Ale Yeast Blend",
            Self::WLP085 => "WLP085 English Ale Yeast Blend",
            Self::WLP090 => "WLP090 San Diego Super Ale Yeast",
            Self::WLP091 => "WLP091 Best Coast Hazy Ale Yeast Blend",
            Self::WLP095 => "WLP095 Burlington Ale Yeast",
            Self::WLP096 => "WLP096 FrankenYeast Blend",
            Self::WLP099 => "WLP099 Super High Gravity Ale Yeast",
            Self::WLP101 => "WLP101 SuperCell Yeast Blend",
            Self::WLP1983 => "WLP1983 Charlie's Fist Bump Yeast",
            Self::WLP300 => "WLP300 Hefeweizen Ale Yeast",
            Self::WLP320 => "WLP320 American Hefeweizen Ale Yeast",
            Self::WLP351 => "WLP351 Bavarian Weizen Ale Yeast",
            Self::WLP380 => "WLP380 Hefeweizen IV Ale Yeast",
            Self::WLP400 => "WLP400 Belgian Wit Ale Yeast",
            Self::WLP4000 => "WLP4000 Vermont Ale Yeast",
            Self::WLP4001 => "WLP4001 Flanders Specialty Ale Yeast",
            Self::WLP4007 => "WLP4007 Saison Ale Yeast Blend I",
            Self::WLP4015 => "WLP4015 Northeastern Abbey Ale Yeast",
            Self::WLP4020 => "WLP4020 Wallonian Farmhouse I Ale Yeast",
            Self::WLP4021 => "WLP4021 Saison Ale Yeast Blend II",
            Self::WLP4022 => "WLP4022 Wallonian Farmhouse II Ale Yeast",
            Self::WLP4023 => "WLP4023 Wallonian Farmhouse III Ale Yeast",
            Self::WLP4025 => "WLP4025 Dry Belgian Ale Yeast",
            Self::WLP4027 => "WLP4027 Funktown Pale Ale Yeast Blend",
            Self::WLP4030 => "WLP4030 Franconian Dark Lager Yeast",
            Self::WLP4035 => "WLP4035 Hessian Pils Lager Yeast",
            Self::WLP4040 => "WLP4040 Midwestern Ale Yeast",
            Self::WLP4042 => "WLP4042 Hazy Daze Yeast Blend I",
            Self::WLP4044 => "WLP4044 Hazy Daze Yeast Blend II",
            Self::WLP4045 => "WLP4045 The Yeast Bay Sigmund's Voss Kveik Ale Yeast",
            Self::WLP4046 => "WLP4046 Simonaitis Lithuanian Farmhouse Ale Yeast",
            Self::WLP4047 => "WLP4047 Pakruojis Lithuanian Farmhouse Ale Yeast",
            Self::WLP4050 => "WLP4050 The Yeast Bay Hornindal Kveik Ale Yeast",
            Self::WLP4051 => "WLP4051 Farmgarden Kveik Ale Yeast",
            Self::WLP4052 => "WLP4052 Lida Kveik Ale Yeast",
            Self::WLP4053 => "WLP4053 Midtbust Kveik Ale Yeast",
            Self::WLP4060 => "WLP4060 Forager Ale Yeast",
            Self::WLP4061 => "WLP4061 Rhine Kölsch Ale Yeast",
            Self::WLP4062 => "WLP4062 LA Fleur Ale Yeast",
            Self::WLP410 => "WLP410 Belgian Wit II Ale Yeast",
            // Self::WLP4605 => "WLP4605 Beersen Brettanomyces",
            // Self::WLP4615 => "WLP4615 Brussels Brettanomyces",
            // Self::WLP4620 => "WLP4620 Lochristi Brettanomyces",
            // Self::WLP4626 => "WLP4626 Saison/Brettanomyces Yeast Blend I",
            // Self::WLP4633 => "WLP4633 Mélange Yeast Blend",
            // Self::WLP4636 => "WLP4636 Saison/Brettanomyces Yeast Blend II",
            // Self::WLP4637 => "WLP4637 Amalgamation I Brettanomyces Blend",
            // Self::WLP4638 => "WLP4638 Brettanomyces bruxellensis strain TYB184",
            // Self::WLP4639 => "WLP4639 Brettanomyces bruxellensis strain TYB207",
            // Self::WLP4640 => "WLP4640 Brettanomyces bruxellensis strain TYB261",
            // Self::WLP4641 => "WLP4641 Amalgamation II Brettanomyces Blend",
            // Self::WLP4642 => "WLP4642 Oud Vat Brett",
            // Self::WLP4643 => "WLP4643 Amalgamation V Brettanomyces Blend",
            // Self::WLP4645 => "WLP4645 Transatlantic Berliner Blend",
            // Self::WLP4650 => "WLP4650 Metschnikowia reukaufii",
            // Self::WLP4651 => "WLP4651 Von Degenberg Hefe Blend",
            // Self::WLP4653 => "WLP4653 Dark Belgian Cask Yeast Blend",
            // Self::WLP4655 => "WLP4655 Brettanomyces bruxellensis strain TYB307",
            // Self::WLP4656 => "WLP4656 Brettanomyces bruxellensis strain TYB415",
            // Self::WLP4665 => "WLP4665 Berkeley Hills Sour Yeast",
            // Self::WLP4675 => "WLP4675 Farmhouse Sour Ale Yeast Blend",
            // Self::WLP4681 => "WLP4681 Lactobacillus brevis Strain TYB282",
            // Self::WLP4682 => "WLP4682 Lactobacillus Blend",
            // Self::WLP4684 => "WLP4684 The YEast Bay House Sour Blend",
            Self::WLP500 => "WLP500 Monastery Ale Yeast",
            Self::WLP510 => "WLP510 Bastogne Belgian Ale Yeast",
            Self::WLP515 => "WLP515 Antwerp Ale Yeast",
            Self::WLP518 => "WLP518 Opshaug Kveik Ale Yeast",
            Self::WLP519 => "WLP519 Stranda Kveik Ale Yeast",
            Self::WLP520 => "WLP520 Sigmund Kveik Ale Yeast",
            Self::WLP521 => "WLP521 Hornindal Kveik Ale Yeast",
            Self::WLP530 => "WLP530 Abbey Ale Yeast",
            Self::WLP540 => "WLP540 Abbey IV Ale Yeast",
            Self::WLP545 => "WLP545 Belgian Strong Ale Yeast",
            Self::WLP546 => "WLP546 Marañón Canyon Wild Cacao Yeast",
            Self::WLP548 => "WLP548 Original Grodziskie Yeast Blend",
            Self::WLP550 => "WLP550 Belgian Ale Yeast",
            Self::WLP561 => "WLP561 Non STA1son Ale Yeast Blend",
            Self::WLP564 => "WLP564 Leeuwenhoek Saison Yeast Blend",
            Self::WLP565 => "WLP565 Belgian Saison I Ale Yeast",
            Self::WLP566 => "WLP566 Belgian Saison II Ale Yeast",
            Self::WLP568 => "WLP568 Belgian Style Saison Ale Yeast Blend",
            Self::WLP570 => "WLP570 Belgian Golden Ale Yeast",
            Self::WLP575 => "WLP575 Belgian Style Ale Yeast Blend",
            Self::WLP585 => "WLP585 Belgian Saison III Ale Yeast",
            Self::WLP590 => "WLP590 French Saison Ale Yeast",
            //Self::WLP600 => "WLP600 Kombucha Cultures",
            //Self::WLP603 => "WLP603 Torulaspora delbrueckii",
            Self::WLP611 => "WLP611 New Nordic Ale Yeast",
            //Self::WLP616 => "WLP616 Funky Cider Blend",
            //Self::WLP618 => "WLP618 NA All Day",
            Self::WLP630 => "WLP630 Berliner Weisse Blend",
            // Self::WLP631 => "WLP631 Appalachian Tart",
            // Self::WLP6420 => "WLP6420 Acid Tripp",
            // Self::WLP644 => "WLP644 Saccharomyces brux-like Trois",
            // Self::WLP645 => "WLP645 Brettanomyces claussenii",
            // Self::WLP648 => "WLP648 Brettanomyces bruxellensis Trois Vrai",
            // Self::WLP650 => "WLP650 Brettanomyces bruxellensis",
            // Self::WLP653 => "WLP653 Brettanomyces lambicus",
            // Self::WLP655 => "WLP655 Belgian Sour Mix 1",
            // Self::WLP661 => "WLP661 Pediococcus damnosus",
            // Self::WLP665 => "WLP665 Flemish Ale Blend",
            // Self::WLP669 => "WLP669 Lactobacillus paracollinoides",
            // Self::WLP670 => "WLP670 American Farmhouse Blend",
            // Self::WLP672 => "WLP672 Lactobacillus brevis",
            // Self::WLP673 => "WLP673 Lactobacillus buchneri",
            // Self::WLP675 => "WLP675 Malolactic Cultures",
            // Self::WLP677 => "WLP677 Lactobacillus delbrueckii",
            // Self::WLP678 => "WLP678 Lactobacillus hilgardii",
            // Self::WLP685 => "WLP685 Gluconobacter Oxydans",
            // Self::WLP686 => "WLP686 Zygosaccharomyces lentus",
            // Self::WLP690 => "WLP690 Acetobacter aceti",
            // Self::WLP692 => "WLP692 Debaromyces hansenii",
            // Self::WLP693 => "WLP693 Lactobacillus plantarum",
            Self::WLP700 => "WLP700 Flor Sherry Yeast",
            Self::WLP705 => "WLP705 Sake Yeast #7",
            Self::WLP707 => "WLP707 California Pinot Noir Wine Yeast",
            Self::WLP709 => "WLP709 Sake #9 Yeast",
            Self::WLP715 => "WLP715 Champagne Yeast",
            Self::WLP718 => "WLP718 Avize Wine Yeast",
            Self::WLP720 => "WLP720 Sweet Mead/Wine Yeast",
            Self::WLP727 => "WLP727 Steinberg-Geisenheim Wine Yesat",
            Self::WLP730 => "WLP730 Chardonnay White Wine Yeast",
            Self::WLP735 => "WLP735 French White Wine Yeast",
            Self::WLP740 => "WLP740 Merlot Red Wine Yeast",
            Self::WLP749 => "WLP749 Assmanshausen Wine Yeast",
            Self::WLP750 => "WLP750 French Red Wine Yeast",
            Self::WLP760 => "WLP760 Carbernet Red Wine Yeast",
            Self::WLP770 => "WLP770 Suremain Burgundy Wine Yeast",
            Self::WLP773 => "WLP773 Scottish Cider Yeast Blend",
            //Self::WLP775 => "WLP775 English Cider Yeast",
            //Self::WLP780 => "WLP780 Thai Rice Chong Yeast",
            Self::WLP800 => "WLP800 Pilsner Lager Yeast",
            Self::WLP802 => "WLP802 Czech Budejovice Lager Yeast",
            Self::WLP808 => "WLP808 Mythical Hammer Lager Yeast Blend",
            Self::WLP810 => "WLP810 San Francisco Lager Yeast",
            Self::WLP815 => "WLP815 Belgian Lager Yeast",
            Self::WLP820 => "WLP820 Oktoberfest/Märzen Lager Yeast",
            Self::WLP830 => "WLP830 German Lager Yeast",
            Self::WLP833 => "WLP833 German Bock Lager Yeast",
            Self::WLP835 => "WLP835 German X Lager Yeast",
            Self::WLP838 => "WLP838 Southern German Lager Yeast",
            Self::WLP840 => "WLP840 American Lager Yeast",
            Self::WLP845 => "WLP845 Fast Lager Yeast",
            Self::WLP850 => "WLP850 Copenhagen Lager Yeast",
            Self::WLP860 => "WLP860 Munich Lager Yeast",
            Self::WLP885 => "WLP885 Zurich Lager Yeast",
            Self::WLP920 => "WLP920 Old Bavarian Lager Yeast",
            Self::WLP925 => "WLP925 High Pressure Lager Yeast",
            Self::WLP940 => "WLP940 Mexican Lager Yeast",
        }
    }

    /// Sources say it is between 5b and 20b depending on
    /// viability of the yeast.  Fermentis guarantees a
    /// minimum of 6b, many home brew tests show it is
    /// closer to 20b.  We pick a number in between.
    pub const CELLS_PER_GRAM_DRY: u64 = 13_000_000_000;

    /// The minimum recommended temperature to ferment at
    #[must_use]
    #[allow(clippy::match_same_arms)]
    #[allow(clippy::too_many_lines)]
    pub fn temp_range(&self) -> Range<Celsius> {
        let (min, max) = match *self {
            Self::SafAleBE134 => (18.0, 26.0),
            Self::SafAleBE256 => (18.0, 26.0),
            Self::SafAleBW20 => (18.0, 26.0),
            Self::SafAleK97 => (18.0, 26.0),
            Self::SafAleS04 => (18.0, 26.0),
            Self::SafAleS33 => (18.0, 26.0),
            Self::SafAleT58 => (18.0, 26.0),
            Self::SafAleUS05 => (18.0, 26.0),
            Self::SafAleW68 => (18.0, 26.0),
            Self::SafAleWB06 => (18.0, 26.0),
            Self::SafLagerE30 => (12.0, 18.0),
            Self::SafLagerS189 => (12.0, 18.0),
            Self::SafLagerS23 => (12.0, 18.0),
            Self::SafLagerSH45 => (12.0, 18.0),
            Self::SafLagerW3470 => (12.0, 18.0),

            // Self::ImperialBarbarian => todo!(),
            // Self::ImperialBartlby => todo!(),
            // Self::ImperialCablecar => todo!(),
            // Self::ImperialCitrus => todo!(),
            // Self::ImperialDarkness => todo!(),
            // Self::ImperialDieter => todo!(),
            // Self::ImperialDryHop => todo!(),
            // Self::ImperialFlagship => todo!(),
            // Self::ImperialGlobal => todo!(),
            // Self::ImperialGnome => todo!(),
            // Self::ImperialHarvest => todo!(),
            // Self::ImperialHouse => todo!(),
            // Self::ImperialIndependence => todo!(),
            // Self::ImperialJoystick => todo!(),
            // Self::ImperialJuice => todo!(),
            // Self::ImperialKaiser => todo!(),
            // Self::ImperialKveiking => todo!(),
            // Self::ImperialLoki => todo!(),
            // Self::ImperialNapoleon => todo!(),
            // Self::ImperialPOG => todo!(),
            // Self::ImperialPub => todo!(),
            // Self::ImperialRustic => todo!(),
            // Self::ImperialSourBatchKidz => todo!(),
            // Self::ImperialStefon => todo!(),
            // Self::ImperialSuburbanBrett => todo!(),
            // Self::ImperialTartan => todo!(),
            // Self::ImperialTripleDouble => todo!(),
            // Self::ImperialUrkel => todo!(),
            // Self::ImperialWhiteout => todo!(),
            Self::LalBrewAbbaye => (17.0, 25.0),
            Self::LalBrewAurora => (12.0, 30.0),
            Self::LalBrewBelleSaison => (20.0, 35.0),
            Self::LalBrewBRY97 => (15.0, 22.0),
            Self::LalBrewCBC1 => (20.0, 30.0),
            Self::LalBrewDiamondLager => (10.0, 15.0),
            Self::LalBrewFarmhouse => (22.0, 30.0),
            Self::LalBrewHouseAle => (16.0, 22.0),
            Self::LalBrewLoNa => (20.0, 25.0),
            Self::LalBrewMunichClassic => (17.0, 25.0),
            Self::LalBrewNewEngland => (18.0, 25.0),
            Self::LalBrewNottingham => (10.0, 25.0),
            Self::LalBrewNovaLager => (10.0, 20.0),
            Self::LalBrewPomona => (18.0, 22.0),
            Self::LalBrewVerdantIPA => (18.0, 25.0),
            Self::LalBrewVoss => (25.0, 40.0),
            Self::LalBrewWindsor => (15.0, 25.0),
            Self::LalBrewWit => (17.0, 25.0),

            Self::OYL061 => (25.0, 40.0),
            Self::OYL071 => (12.0, 35.0),

            Self::WLP001 => (18.0, 23.0),
            Self::WLP002 => (18.0, 20.0),
            Self::WLP003 => (18.0, 21.0),
            Self::WLP004 => (18.0, 20.0),
            Self::WLP005 => (18.0, 21.0),
            Self::WLP006 => (18.0, 21.0),
            Self::WLP007 => (18.0, 21.0),
            Self::WLP008 => (20.0, 23.0),
            Self::WLP009 => (18.0, 21.0),
            Self::WLP011 => (18.0, 21.0),
            Self::WLP013 => (19.0, 22.0),
            Self::WLP017 => (19.0, 21.0),
            //Self::WLP019 => todo!(),
            Self::WLP022 => (19.0, 21.0),
            Self::WLP023 => (20.0, 23.0),
            Self::WLP025 => (19.0, 20.0),
            Self::WLP026 => (19.0, 21.0),
            Self::WLP028 => (18.0, 21.0),
            Self::WLP029 => (18.0, 21.0),
            //Self::WLP030 => todo!(),
            Self::WLP033 => (19.0, 21.0),
            Self::WLP036 => (18.0, 21.0),
            Self::WLP037 => (18.0, 21.0),
            //Self::WLP038 => (18.0, 21.0),
            Self::WLP039 => (19.0, 21.0),
            Self::WLP041 => (18.0, 20.0),
            Self::WLP045 => (22.0, 25.0),
            Self::WLP050 => (24.0, 26.0),
            Self::WLP051 => (19.0, 21.0),
            Self::WLP059 => (23.0, 23.0),
            Self::WLP060 => (20.0, 22.0),
            Self::WLP064 => (19.0, 22.0),
            Self::WLP065 => (24.0, 28.0),
            Self::WLP066 => (18.0, 22.0),
            Self::WLP067 => (20.0, 22.0),
            Self::WLP070 => (22.0, 25.0),
            Self::WLP072 => (17.0, 23.0),
            Self::WLP073 => (21.0, 27.0),
            Self::WLP075 => (19.0, 21.0),
            Self::WLP076 => (19.0, 21.0),
            Self::WLP077 => (17.0, 23.0),
            Self::WLP078 => (24.0, 29.0),
            Self::WLP080 => (18.0, 21.0),
            Self::WLP085 => (20.0, 22.0),
            Self::WLP090 => (18.0, 20.0),
            Self::WLP091 => (20.0, 22.0),
            Self::WLP095 => (19.0, 22.0),
            Self::WLP096 => (19.0, 21.0),
            Self::WLP099 => (18.0, 20.0),
            Self::WLP101 => (25.0, 35.0),
            Self::WLP1983 => (13.0, 23.0),
            Self::WLP300 => (20.0, 22.0),
            Self::WLP320 => (18.0, 21.0),
            Self::WLP351 => (19.0, 21.0),
            Self::WLP380 => (19.0, 21.0),
            Self::WLP400 => (19.0, 23.0),
            Self::WLP4000 => (18.0, 22.0),
            Self::WLP4001 => (20.0, 27.0),
            Self::WLP4007 => (20.0, 27.0),
            Self::WLP4015 => (20.0, 24.0),
            Self::WLP4020 => (22.0, 27.0),
            Self::WLP4021 => (20.0, 27.0),
            Self::WLP4022 => (20.0, 27.0),
            Self::WLP4023 => (20.0, 27.0),
            Self::WLP4025 => (20.0, 26.0),
            Self::WLP4027 => (20.0, 23.0),
            Self::WLP4030 => (8.0, 12.0),
            Self::WLP4035 => (7.0, 12.0),
            Self::WLP4040 => (18.0, 22.0),
            Self::WLP4042 => (18.0, 21.0),
            Self::WLP4044 => (19.0, 22.0),
            Self::WLP4045 => (21.0, 38.0),
            Self::WLP4046 => (24.0, 35.0),
            Self::WLP4047 => (24.0, 35.0),
            Self::WLP4050 => (27.0, 35.0),
            Self::WLP4051 => (27.0, 35.0),
            Self::WLP4052 => (27.0, 32.0),
            Self::WLP4053 => (24.0, 35.0),
            Self::WLP4060 => (20.0, 27.0),
            Self::WLP4061 => (14.0, 20.0),
            Self::WLP4062 => (23.0, 27.0),
            Self::WLP410 => (19.0, 23.0),
            // Self::WLP4605 =>
            // Self::WLP4615 =>
            // Self::WLP4620 =>
            // Self::WLP4626 =>
            // Self::WLP4633 =>
            // Self::WLP4636 =>
            // Self::WLP4637 =>
            // Self::WLP4638 =>
            // Self::WLP4639 =>
            // Self::WLP4640 =>
            // Self::WLP4641 =>
            // Self::WLP4642 =>
            // Self::WLP4643 =>
            // Self::WLP4645 =>
            // Self::WLP4650 =>
            // Self::WLP4651 =>
            // Self::WLP4653 =>
            // Self::WLP4655 =>
            // Self::WLP4656 =>
            // Self::WLP4665 =>
            // Self::WLP4675 =>
            // Self::WLP4681 =>
            // Self::WLP4682 =>
            // Self::WLP4684 =>
            Self::WLP500 => (18.0, 22.0),
            Self::WLP510 => (19.0, 22.0),
            Self::WLP515 => (19.0, 21.0),
            Self::WLP518 => (25.0, 35.0),
            Self::WLP519 => (22.0, 37.0),
            Self::WLP520 => (22.0, 37.0),
            Self::WLP521 => (22.0, 37.0),
            Self::WLP530 => (19.0, 22.0),
            Self::WLP540 => (19.0, 22.0),
            Self::WLP545 => (19.0, 22.0),
            Self::WLP546 => (18.0, 24.0),
            Self::WLP548 => (12.0, 25.0),
            Self::WLP550 => (20.0, 26.0),
            Self::WLP561 => (20.0, 26.0),
            Self::WLP564 => (18.0, 24.0),
            Self::WLP565 => (20.0, 30.0),
            Self::WLP566 => (20.0, 30.0),
            Self::WLP568 => (20.0, 30.0),
            Self::WLP570 => (20.0, 24.0),
            Self::WLP575 => (20.0, 24.0),
            Self::WLP585 => (20.0, 30.0),
            Self::WLP590 => (20.0, 30.0),
            //Self::WLP600 =>
            //Self::WLP603 =>
            Self::WLP611 => (10.0, 30.0),
            //Self::WLP616 =>
            //Self::WLP618 =>
            Self::WLP630 => (20.0, 22.0),
            // Self::WLP631 =>
            // Self::WLP6420 =>
            // Self::WLP644 =>
            // Self::WLP645 =>
            // Self::WLP648 =>
            // Self::WLP650 =>
            // Self::WLP653 =>
            // Self::WLP655 =>
            // Self::WLP661 =>
            // Self::WLP665 =>
            // Self::WLP669 =>
            // Self::WLP670 =>
            // Self::WLP672 =>
            // Self::WLP673 =>
            // Self::WLP675 =>
            // Self::WLP677 =>
            // Self::WLP678 =>
            // Self::WLP685 =>
            // Self::WLP686 =>
            // Self::WLP690 =>
            // Self::WLP692 =>
            // Self::WLP693 =>
            Self::WLP700 => (21.0, 25.0), // top not listed
            Self::WLP705 => (21.0, 32.0),
            Self::WLP707 => (16.0, 32.0),
            Self::WLP709 => (17.0, 20.0),
            Self::WLP715 => (21.0, 24.0),
            Self::WLP718 => (16.0, 32.0),
            Self::WLP720 => (21.0, 24.0),
            Self::WLP727 => (10.0, 32.0),
            Self::WLP730 => (10.0, 32.0),
            Self::WLP735 => (16.0, 32.0),
            Self::WLP740 => (16.0, 32.0),
            Self::WLP749 => (10.0, 32.0),
            Self::WLP750 => (16.0, 32.0),
            Self::WLP760 => (16.0, 32.0),
            Self::WLP770 => (16.0, 32.0),
            Self::WLP773 => (19.0, 22.0),
            //Self::WLP775 => (20.0, 24.0),
            //Self::WLP780 =>
            Self::WLP800 => (10.0, 13.0),
            Self::WLP802 => (10.0, 13.0),
            Self::WLP808 => (10.0, 18.0),
            Self::WLP810 => (14.0, 18.0),
            Self::WLP815 => (10.0, 13.0),
            Self::WLP820 => (11.0, 14.0),
            Self::WLP830 => (10.0, 13.0),
            Self::WLP833 => (9.0, 13.0),
            Self::WLP835 => (10.0, 12.0),
            Self::WLP838 => (10.0, 13.0),
            Self::WLP840 => (10.0, 13.0),
            Self::WLP845 => (10.0, 14.0),
            Self::WLP850 => (10.0, 14.0),
            Self::WLP860 => (9.0, 11.0),
            Self::WLP885 => (10.0, 13.0),
            Self::WLP920 => (10.0, 13.0),
            Self::WLP925 => (17.0, 20.0),
            Self::WLP940 => (10.0, 13.0),
        };

        Celsius(min)..Celsius(max)
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
    #[allow(clippy::too_many_lines)]
    pub fn attenuation_range(&self) -> Range<f32> {
        match *self {
            Self::SafAleBE134 => 0.89..0.93,
            Self::SafAleBE256 => 0.82..0.86,
            Self::SafAleBW20 => 0.77..0.83,
            Self::SafAleK97 => 0.80..0.84,
            Self::SafAleS04 => 0.74..0.82,
            Self::SafAleS33 => 0.68..0.72,
            Self::SafAleT58 => 0.72..0.78,
            Self::SafAleUS05 => 0.78..0.82,
            Self::SafAleW68 => 0.78..0.84,
            Self::SafAleWB06 => 0.86..0.90,
            Self::SafLagerE30 => 0.80..0.84,
            Self::SafLagerS189 => 0.80..0.84,
            Self::SafLagerS23 => 0.80..0.84,
            Self::SafLagerSH45 => 0.77..0.82,
            Self::SafLagerW3470 => 0.80..0.84,

            Self::LalBrewAbbaye => 0.77..0.83,
            Self::LalBrewAurora => 0.74..0.82,
            Self::LalBrewBelleSaison => 0.86..0.94,
            Self::LalBrewBRY97 => 0.78..0.84,
            Self::LalBrewCBC1 => 0.0..0.0, // n/a glucose only
            Self::LalBrewDiamondLager => 0.77..0.83,
            Self::LalBrewFarmhouse => 0.78..0.84,
            Self::LalBrewHouseAle => 0.78..0.86,
            Self::LalBrewLoNa => 0.16..0.20,
            Self::LalBrewMunichClassic => 0.76..0.83,
            Self::LalBrewNewEngland => 0.78..0.83,
            Self::LalBrewNottingham => 0.78..0.84,
            Self::LalBrewNovaLager => 0.78..0.84,
            Self::LalBrewPomona => 0.75..0.84,
            Self::LalBrewVerdantIPA => 0.75..0.82,
            Self::LalBrewVoss => 0.76..0.82,
            Self::LalBrewWindsor => 0.65..0.72,
            Self::LalBrewWit => 0.75..0.82,

            Self::OYL061 => 0.76..0.82,
            Self::OYL071 => 0.75..0.82,

            Self::WLP001 => 0.73..0.85,
            Self::WLP002 => 0.63..0.70,
            Self::WLP003 => 0.73..0.80,
            Self::WLP004 => 0.69..0.74,
            Self::WLP005 => 0.67..0.74,
            Self::WLP006 => 0.72..0.80,
            Self::WLP007 => 0.70..0.80,
            Self::WLP008 => 0.70..0.75,
            Self::WLP009 => 0.70..0.75,
            Self::WLP011 => 0.65..0.70,
            Self::WLP013 => 0.67..0.75,
            Self::WLP017 => 0.67..0.73,
            //Self::WLP019 => todo!(),
            Self::WLP022 => 0.71..0.76,
            Self::WLP023 => 0.72..0.78,
            Self::WLP025 => 0.68..0.75,
            Self::WLP026 => 0.70..0.75,
            Self::WLP028 => 0.70..0.75,
            Self::WLP029 => 0.72..0.78,
            //Self::WLP030 => 0.72..0.78,
            Self::WLP033 => 0.66..0.74,
            Self::WLP036 => 0.65..0.72,
            Self::WLP037 => 0.68..0.72,
            //Self::WLP038 => todo!(),
            Self::WLP039 => 0.73..0.82,
            Self::WLP041 => 0.72..0.78,
            Self::WLP045 => 0.75..0.80,
            Self::WLP050 => 0.75..0.80,
            Self::WLP051 => 0.72..0.78,
            Self::WLP059 => 0.74..0.78,
            Self::WLP060 => 0.72..0.80,
            Self::WLP064 => 0.72..0.78,
            Self::WLP065 => 0.76..0.82,
            Self::WLP066 => 0.75..0.82,
            Self::WLP067 => 0.75..0.82,
            Self::WLP070 => 0.75..0.80,
            Self::WLP072 => 0.68..0.75,
            Self::WLP073 => 0.75..0.80,
            Self::WLP075 => 0.75..0.80,
            Self::WLP076 => 0.70..0.74,
            Self::WLP077 => 0.75..0.82,
            Self::WLP078 => 0.77..0.84,
            Self::WLP080 => 0.75..0.80,
            Self::WLP085 => 0.69..0.76,
            Self::WLP090 => 0.76..0.83,
            Self::WLP091 => 0.70..0.75,
            Self::WLP095 => 0.75..0.80,
            Self::WLP096 => 0.78..0.85,
            Self::WLP099 => 0.80..1.00,
            Self::WLP101 => 0.70..0.80,
            Self::WLP1983 => 0.72..0.78,
            Self::WLP300 => 0.72..0.76,
            Self::WLP320 => 0.70..0.75,
            Self::WLP351 => 0.75..0.82,
            Self::WLP380 => 0.73..0.80,
            Self::WLP400 => 0.74..0.78,
            Self::WLP4000 => 0.78..0.82,
            Self::WLP4001 => 0.78..0.82,
            Self::WLP4007 => 0.78..0.84,
            Self::WLP4015 => 0.77..0.81,
            Self::WLP4020 => 0.81..1.00,
            Self::WLP4021 => 0.85..1.00,
            Self::WLP4022 => 0.80..0.82,
            Self::WLP4023 => 0.86..0.94,
            Self::WLP4025 => 0.85..1.00,
            Self::WLP4027 => 0.78..0.85,
            Self::WLP4030 => 0.74..0.78,
            Self::WLP4035 => 0.73..0.76,
            Self::WLP4040 => 0.76..0.80,
            Self::WLP4042 => 0.79..0.83,
            Self::WLP4044 => 0.81..0.85,
            Self::WLP4045 => 0.78..0.83,
            Self::WLP4046 => 0.76..0.82,
            Self::WLP4047 => 0.90..1.00,
            Self::WLP4050 => 0.77..0.81,
            Self::WLP4051 => 0.78..0.82,
            Self::WLP4052 => 0.75..0.82,
            Self::WLP4053 => 0.76..0.80,
            Self::WLP4060 => 0.85..0.90,
            Self::WLP4061 => 0.75..0.78,
            Self::WLP4062 => 0.75..0.80,
            Self::WLP410 => 0.70..0.75,
            // Self::WLP4605 =>
            // Self::WLP4615 =>
            // Self::WLP4620 =>
            // Self::WLP4626 =>
            // Self::WLP4633 =>
            // Self::WLP4636 =>
            // Self::WLP4637 =>
            // Self::WLP4638 =>
            // Self::WLP4639 =>
            // Self::WLP4640 =>
            // Self::WLP4641 =>
            // Self::WLP4642 =>
            // Self::WLP4643 =>
            // Self::WLP4645 =>
            // Self::WLP4650 =>
            // Self::WLP4651 =>
            // Self::WLP4653 =>
            // Self::WLP4655 =>
            // Self::WLP4656 =>
            // Self::WLP4665 =>
            // Self::WLP4675 =>
            // Self::WLP4681 =>
            // Self::WLP4682 =>
            // Self::WLP4684 =>
            Self::WLP500 => 0.75..0.80,
            Self::WLP510 => 0.74..0.80,
            Self::WLP515 => 0.73..0.80,
            Self::WLP518 => 0.69..0.80,
            Self::WLP519 => 0.75..0.85,
            Self::WLP520 => 0.75..0.83,
            Self::WLP521 => 0.75..0.82,
            Self::WLP530 => 0.75..0.80,
            Self::WLP540 => 0.74..0.82,
            Self::WLP545 => 0.78..0.85,
            Self::WLP546 => 0.65..0.70,
            Self::WLP548 => 0.75..0.80,
            Self::WLP550 => 0.78..0.85,
            Self::WLP561 => 0.78..0.85,
            Self::WLP564 => 0.76..0.82,
            Self::WLP565 => 0.65..0.75,
            Self::WLP566 => 0.78..0.85,
            Self::WLP568 => 0.70..0.80,
            Self::WLP570 => 0.78..0.85,
            Self::WLP575 => 0.74..0.80,
            Self::WLP585 => 0.78..0.85,
            Self::WLP590 => 0.78..0.85,
            //Self::WLP600 =>
            //Self::WLP603 =>
            Self::WLP611 => 0.65..0.75,
            //Self::WLP616 =>
            //Self::WLP618 => x
            Self::WLP630 => 0.73..0.80,
            // Self::WLP631 =>
            // Self::WLP6420 =>
            // Self::WLP644 =>
            // Self::WLP645 =>
            // Self::WLP648 =>
            // Self::WLP650 =>
            // Self::WLP653 =>
            // Self::WLP655 =>
            // Self::WLP661 =>
            // Self::WLP665 =>
            // Self::WLP669 =>
            // Self::WLP670 =>
            // Self::WLP672 =>
            // Self::WLP673 =>
            // Self::WLP675 =>
            // Self::WLP677 =>
            // Self::WLP678 =>
            // Self::WLP685 =>
            // Self::WLP686 =>
            // Self::WLP690 =>
            // Self::WLP692 =>
            // Self::WLP693 =>
            Self::WLP700 => 0.70..0.90,
            Self::WLP705 => 0.70..0.90,
            Self::WLP707 => 0.70..0.90,
            Self::WLP709 => 0.72..0.78,
            Self::WLP715 => 0.70..0.90,
            Self::WLP718 => 0.70..0.90,
            Self::WLP720 => 0.70..0.90,
            Self::WLP727 => 0.70..0.90,
            Self::WLP730 => 0.70..0.90,
            Self::WLP735 => 0.70..0.90,
            Self::WLP740 => 0.70..0.90,
            Self::WLP749 => 0.70..0.90,
            Self::WLP750 => 0.70..0.90,
            Self::WLP760 => 0.70..0.90,
            Self::WLP770 => 0.70..0.90,
            Self::WLP773 => 0.74..0.80,
            //Self::WLP775 => todo!(),
            //Self::WLP780 =>
            Self::WLP800 => 0.72..0.77,
            Self::WLP802 => 0.70..0.75,
            Self::WLP808 => 0.70..0.79,
            Self::WLP810 => 0.70..0.75,
            Self::WLP815 => 0.72..0.78,
            Self::WLP820 => 0.65..0.73,
            Self::WLP830 => 0.74..0.79,
            Self::WLP833 => 0.70..0.76,
            Self::WLP835 => 0.70..0.76,
            Self::WLP838 => 0.68..0.76,
            Self::WLP840 => 0.75..0.80,
            Self::WLP845 => 0.75..0.78,
            Self::WLP850 => 0.72..0.78,
            Self::WLP860 => 0.68..0.72,
            Self::WLP885 => 0.70..0.80,
            Self::WLP920 => 0.66..0.73,
            Self::WLP925 => 0.73..0.82,
            Self::WLP940 => 0.70..0.78,
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
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::cast_precision_loss)]
    pub fn alcohol_tolerance_range(&self) -> Range<f32> {
        let (min, max) = match *self {
            Self::SafAleBE134 => (9, 11),
            Self::SafAleBE256 => (9, 11),
            Self::SafAleBW20 => (9, 11), // unlisted on specs
            Self::SafAleK97 => (9, 11),
            Self::SafAleS04 => (9, 11),
            Self::SafAleS33 => (9, 11),
            Self::SafAleT58 => (9, 11),
            Self::SafAleUS05 => (9, 11),
            Self::SafAleW68 => (9, 11), // unlisted on specs
            Self::SafAleWB06 => (9, 11),
            Self::SafLagerE30 => (9, 11), // unlisted on specs
            Self::SafLagerS189 => (9, 11),
            Self::SafLagerS23 => (9, 11),
            Self::SafLagerSH45 => (9, 11), // unlisted on specs
            Self::SafLagerW3470 => (9, 11),

            Self::LalBrewAbbaye => (14, 14),
            Self::LalBrewAurora => (13, 13),
            Self::LalBrewBelleSaison => (15, 15),
            Self::LalBrewBRY97 => (13, 13),
            Self::LalBrewCBC1 => (12, 14), // for bottle cond; 18% for cider, mead
            Self::LalBrewDiamondLager => (13, 13),
            Self::LalBrewFarmhouse => (13, 13),
            Self::LalBrewHouseAle => (14, 14),
            Self::LalBrewLoNa => (5, 5), // n/a they say
            Self::LalBrewMunichClassic => (12, 12),
            Self::LalBrewNewEngland => (9, 9),
            Self::LalBrewNottingham => (14, 14),
            Self::LalBrewNovaLager => (13, 13),
            Self::LalBrewPomona => (14, 14),
            Self::LalBrewVerdantIPA => (12, 12),
            Self::LalBrewVoss => (12, 12),
            Self::LalBrewWindsor => (12, 12),
            Self::LalBrewWit => (12, 12),

            Self::OYL061 => (12, 12),
            Self::OYL071 => (15, 15),

            Self::WLP001 => (5, 10),
            Self::WLP002 => (5, 10),
            Self::WLP003 => (5, 10),
            Self::WLP004 => (5, 10),
            Self::WLP005 => (5, 10),
            Self::WLP006 => (5, 10),
            Self::WLP007 => (5, 10),
            Self::WLP008 => (5, 10),
            Self::WLP009 => (5, 10),
            Self::WLP011 => (5, 10),
            Self::WLP013 => (8, 12),
            Self::WLP017 => (5, 10),
            //Self::WLP019 => todo!(),
            Self::WLP022 => (5, 10),
            Self::WLP023 => (5, 10),
            Self::WLP025 => (5, 10),
            Self::WLP026 => (5, 10),
            Self::WLP028 => (8, 12),
            Self::WLP029 => (8, 12),
            //Self::WLP030 => (10, 15),
            Self::WLP033 => (5, 10),
            Self::WLP036 => (8, 12),
            Self::WLP037 => (5, 10),
            //Self::WLP038 => todo!(),
            Self::WLP039 => (5, 10),
            Self::WLP041 => (5, 10),
            Self::WLP045 => (10, 15),
            Self::WLP050 => (10, 15),
            Self::WLP051 => (5, 10),
            Self::WLP059 => (10, 15),
            Self::WLP060 => (8, 12),
            Self::WLP064 => (8, 12),
            Self::WLP065 => (10, 15),
            Self::WLP066 => (5, 10),
            Self::WLP067 => (8, 12),
            Self::WLP070 => (10, 15),
            Self::WLP072 => (5, 10),
            Self::WLP073 => (8, 12),
            Self::WLP075 => (10, 15),
            Self::WLP076 => (5, 10),
            Self::WLP077 => (8, 12),
            Self::WLP078 => (10, 15),
            Self::WLP080 => (8, 12),
            Self::WLP085 => (5, 10),
            Self::WLP090 => (5, 10),
            Self::WLP091 => (8, 12),
            Self::WLP095 => (8, 12),
            Self::WLP096 => (10, 15),
            Self::WLP099 => (15, 18),
            Self::WLP101 => (10, 15),
            Self::WLP1983 => (5, 10),
            Self::WLP300 => (8, 12),
            Self::WLP320 => (10, 15),
            Self::WLP351 => (15, 15),
            Self::WLP380 => (5, 10),
            Self::WLP400 => (5, 10),
            Self::WLP4000 => (8, 12),
            Self::WLP4001 => (10, 15),
            Self::WLP4007 => (10, 15),
            Self::WLP4015 => (8, 12),
            Self::WLP4020 => (10, 15),
            Self::WLP4021 => (10, 15),
            Self::WLP4022 => (10, 15),
            Self::WLP4023 => (10, 15),
            Self::WLP4025 => (10, 15),
            Self::WLP4027 => (8, 12),
            Self::WLP4030 => (8, 12),
            Self::WLP4035 => (5, 10),
            Self::WLP4040 => (8, 12),
            Self::WLP4042 => (8, 12),
            Self::WLP4044 => (8, 12),
            Self::WLP4045 => (10, 15),
            Self::WLP4046 => (10, 15),
            Self::WLP4047 => (10, 15),
            Self::WLP4050 => (10, 15),
            Self::WLP4051 => (10, 15),
            Self::WLP4052 => (10, 15),
            Self::WLP4053 => (10, 15),
            Self::WLP4060 => (10, 15),
            Self::WLP4061 => (5, 10),
            Self::WLP4062 => (10, 15),
            Self::WLP410 => (8, 12),
            // Self::WLP4605 =>
            // Self::WLP4615 =>
            // Self::WLP4620 =>
            // Self::WLP4626 =>
            // Self::WLP4633 =>
            // Self::WLP4636 =>
            // Self::WLP4637 =>
            // Self::WLP4638 =>
            // Self::WLP4639 =>
            // Self::WLP4640 =>
            // Self::WLP4641 =>
            // Self::WLP4642 =>
            // Self::WLP4643 =>
            // Self::WLP4645 =>
            // Self::WLP4650 =>
            // Self::WLP4651 =>
            // Self::WLP4653 =>
            // Self::WLP4655 =>
            // Self::WLP4656 =>
            // Self::WLP4665 =>
            // Self::WLP4675 =>
            // Self::WLP4681 =>
            // Self::WLP4682 =>
            // Self::WLP4684 =>
            Self::WLP500 => (10, 15),
            Self::WLP510 => (10, 15),
            Self::WLP515 => (5, 10),
            Self::WLP518 => (15, 18),
            Self::WLP519 => (8, 12),
            Self::WLP520 => (8, 12),
            Self::WLP521 => (8, 12),
            Self::WLP530 => (8, 12),
            Self::WLP540 => (5, 10),
            Self::WLP545 => (15, 18),
            Self::WLP546 => (5, 10),
            Self::WLP548 => (10, 15),
            Self::WLP550 => (10, 15),
            Self::WLP561 => (10, 15),
            Self::WLP564 => (10, 15),
            Self::WLP565 => (15, 18),
            Self::WLP566 => (15, 18),
            Self::WLP568 => (5, 10),
            Self::WLP570 => (15, 18),
            Self::WLP575 => (8, 12),
            Self::WLP585 => (5, 10),
            Self::WLP590 => (15, 18),
            //Self::WLP600 =>
            //Self::WLP603 =>
            Self::WLP611 => (5, 10),
            //Self::WLP616 =>
            //Self::WLP618 =>
            Self::WLP630 => (5, 10),
            // Self::WLP631 =>
            // Self::WLP6420 =>
            // Self::WLP644 =>
            // Self::WLP645 =>
            // Self::WLP648 =>
            // Self::WLP650 =>
            // Self::WLP653 =>
            // Self::WLP655 =>
            // Self::WLP661 =>
            // Self::WLP665 =>
            // Self::WLP669 =>
            // Self::WLP670 =>
            // Self::WLP672 =>
            // Self::WLP673 =>
            // Self::WLP675 =>
            // Self::WLP677 =>
            // Self::WLP678 =>
            // Self::WLP685 =>
            // Self::WLP686 =>
            // Self::WLP690 =>
            // Self::WLP692 =>
            // Self::WLP693 =>
            Self::WLP700 => (15, 18),
            Self::WLP705 => (15, 18),
            Self::WLP707 => (15, 18),
            Self::WLP709 => (15, 18),
            Self::WLP715 => (15, 18),
            Self::WLP718 => (10, 15),
            Self::WLP720 => (10, 15),
            Self::WLP727 => (10, 15),
            Self::WLP730 => (10, 15),
            Self::WLP735 => (10, 15),
            Self::WLP740 => (10, 15),
            Self::WLP749 => (10, 15),
            Self::WLP750 => (15, 18),
            Self::WLP760 => (15, 18),
            Self::WLP770 => (15, 18),
            Self::WLP773 => (5, 10),
            //Self::WLP775 => (15, 18),
            //Self::WLP780 =>
            Self::WLP800 => (15, 18),
            Self::WLP802 => (5, 10),
            Self::WLP808 => (5, 10),
            Self::WLP810 => (5, 10),
            Self::WLP815 => (5, 10),
            Self::WLP820 => (5, 10),
            Self::WLP830 => (5, 10),
            Self::WLP833 => (5, 10),
            Self::WLP835 => (8, 12),
            Self::WLP838 => (5, 10),
            Self::WLP840 => (5, 10),
            Self::WLP845 => (5, 10),
            Self::WLP850 => (5, 10),
            Self::WLP860 => (5, 10),
            Self::WLP885 => (15, 18),
            Self::WLP920 => (5, 10),
            Self::WLP925 => (5, 10),
            Self::WLP940 => (5, 10),
        };

        (min as f32 / 100.0)..(max as f32 / 100.0)
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
    #[allow(clippy::too_many_lines)]
    pub fn flocculation(&self) -> Flocculation {
        match *self {
            Self::SafAleBE134 => Flocculation::Low,  // sed slow
            Self::SafAleBE256 => Flocculation::High, // sed fast
            Self::SafAleBW20 => Flocculation::Low,
            Self::SafAleK97 => Flocculation::Low, // sed slow
            Self::SafAleS04 => Flocculation::High,
            Self::SafAleS33 => Flocculation::Medium,
            Self::SafAleT58 => Flocculation::Medium,
            Self::SafAleUS05 => Flocculation::Medium,
            Self::SafAleW68 => Flocculation::Medium,
            Self::SafAleWB06 => Flocculation::Low,
            Self::SafLagerE30 => Flocculation::Medium,
            Self::SafLagerS189 => Flocculation::High, // sed fast
            Self::SafLagerS23 => Flocculation::High,  // sed fast
            Self::SafLagerSH45 => Flocculation::Medium,
            Self::SafLagerW3470 => Flocculation::High,

            Self::LalBrewAbbaye => Flocculation::MediumHigh,
            Self::LalBrewAurora => Flocculation::MediumHigh,
            Self::LalBrewBelleSaison => Flocculation::Low,
            Self::LalBrewBRY97 => Flocculation::High,
            Self::LalBrewCBC1 => Flocculation::High,
            Self::LalBrewDiamondLager => Flocculation::High,
            Self::LalBrewFarmhouse => Flocculation::Low,
            Self::LalBrewHouseAle => Flocculation::Medium,
            Self::LalBrewLoNa => Flocculation::Medium,
            Self::LalBrewMunichClassic => Flocculation::Low,
            Self::LalBrewNewEngland => Flocculation::Medium,
            Self::LalBrewNottingham => Flocculation::High,
            Self::LalBrewNovaLager => Flocculation::Medium,
            Self::LalBrewPomona => Flocculation::Medium,
            Self::LalBrewVerdantIPA => Flocculation::Medium,
            Self::LalBrewVoss => Flocculation::VeryHigh,
            Self::LalBrewWindsor => Flocculation::Low,
            Self::LalBrewWit => Flocculation::Low,

            Self::OYL061 => Flocculation::VeryHigh,
            Self::OYL071 => Flocculation::MediumHigh,

            Self::WLP001 => Flocculation::Medium,
            Self::WLP002 => Flocculation::VeryHigh,
            Self::WLP003 => Flocculation::Medium,
            Self::WLP004 => Flocculation::MediumHigh,
            Self::WLP005 => Flocculation::High,
            Self::WLP006 => Flocculation::High,
            Self::WLP007 => Flocculation::MediumHigh,
            Self::WLP008 => Flocculation::LowMedium,
            Self::WLP009 => Flocculation::High,
            Self::WLP011 => Flocculation::Medium,
            Self::WLP013 => Flocculation::Medium,
            Self::WLP017 => Flocculation::High,
            //Self::WLP019 => todo!(),
            Self::WLP022 => Flocculation::MediumHigh,
            Self::WLP023 => Flocculation::Medium,
            Self::WLP025 => Flocculation::Medium,
            Self::WLP026 => Flocculation::Medium,
            Self::WLP028 => Flocculation::Medium,
            Self::WLP029 => Flocculation::Medium,
            //Self::WLP030 => Flocculation::High,
            Self::WLP033 => Flocculation::Medium,
            Self::WLP036 => Flocculation::Medium,
            Self::WLP037 => Flocculation::High,
            //Self::WLP038 => todo!(),
            Self::WLP039 => Flocculation::MediumHigh,
            Self::WLP041 => Flocculation::High,
            Self::WLP045 => Flocculation::Medium,
            Self::WLP050 => Flocculation::Medium,
            Self::WLP051 => Flocculation::MediumHigh,
            Self::WLP059 => Flocculation::Medium,
            Self::WLP060 => Flocculation::Medium,
            Self::WLP064 => Flocculation::Medium,
            Self::WLP065 => Flocculation::Medium,
            Self::WLP066 => Flocculation::LowMedium,
            Self::WLP067 => Flocculation::LowMedium,
            Self::WLP070 => Flocculation::Medium,
            Self::WLP072 => Flocculation::MediumHigh,
            Self::WLP073 => Flocculation::LowMedium,
            Self::WLP075 => Flocculation::MediumHigh,
            Self::WLP076 => Flocculation::Medium,
            Self::WLP077 => Flocculation::Low,
            Self::WLP078 => Flocculation::Medium,
            Self::WLP080 => Flocculation::Medium,
            Self::WLP085 => Flocculation::MediumHigh,
            Self::WLP090 => Flocculation::MediumHigh,
            Self::WLP091 => Flocculation::Medium,
            Self::WLP095 => Flocculation::Medium,
            Self::WLP096 => Flocculation::MediumHigh,
            Self::WLP099 => Flocculation::Medium,
            Self::WLP101 => Flocculation::MediumHigh,
            Self::WLP1983 => Flocculation::Low,
            Self::WLP300 => Flocculation::Low,
            Self::WLP320 => Flocculation::Low,
            Self::WLP351 => Flocculation::Low,
            Self::WLP380 => Flocculation::Low,
            Self::WLP400 => Flocculation::LowMedium,
            Self::WLP4000 => Flocculation::LowMedium,
            Self::WLP4001 => Flocculation::Medium,
            Self::WLP4007 => Flocculation::LowMedium,
            Self::WLP4015 => Flocculation::LowMedium,
            Self::WLP4020 => Flocculation::Medium,
            Self::WLP4021 => Flocculation::Medium,
            Self::WLP4022 => Flocculation::LowMedium,
            Self::WLP4023 => Flocculation::LowMedium,
            Self::WLP4025 => Flocculation::MediumHigh,
            Self::WLP4027 => Flocculation::LowMedium,
            Self::WLP4030 => Flocculation::LowMedium,
            Self::WLP4035 => Flocculation::LowMedium,
            Self::WLP4040 => Flocculation::Medium,
            Self::WLP4042 => Flocculation::LowMedium,
            Self::WLP4044 => Flocculation::LowMedium,
            Self::WLP4045 => Flocculation::High,
            Self::WLP4046 => Flocculation::High,
            Self::WLP4047 => Flocculation::Low,
            Self::WLP4050 => Flocculation::LowMedium,
            Self::WLP4051 => Flocculation::Medium,
            Self::WLP4052 => Flocculation::Low,
            Self::WLP4053 => Flocculation::Low,
            Self::WLP4060 => Flocculation::Low,
            Self::WLP4061 => Flocculation::LowMedium,
            Self::WLP4062 => Flocculation::LowMedium,
            Self::WLP410 => Flocculation::LowMedium,
            // Self::WLP4605 => Flocculation::
            // Self::WLP4615 => Flocculation::
            // Self::WLP4620 => Flocculation::
            // Self::WLP4626 => Flocculation::
            // Self::WLP4633 => Flocculation::
            // Self::WLP4636 => Flocculation::
            // Self::WLP4637 => Flocculation::
            // Self::WLP4638 => Flocculation::
            // Self::WLP4639 => Flocculation::
            // Self::WLP4640 => Flocculation::
            // Self::WLP4641 => Flocculation::
            // Self::WLP4642 => Flocculation::
            // Self::WLP4643 => Flocculation::
            // Self::WLP4645 => Flocculation::
            // Self::WLP4650 => Flocculation::
            // Self::WLP4651 => Flocculation::
            // Self::WLP4653 => Flocculation::
            // Self::WLP4655 => Flocculation::
            // Self::WLP4656 => Flocculation::
            // Self::WLP4665 => Flocculation::
            // Self::WLP4675 => Flocculation::
            // Self::WLP4681 => Flocculation::
            // Self::WLP4682 => Flocculation::
            // Self::WLP4684 => Flocculation::
            Self::WLP500 => Flocculation::LowMedium,
            Self::WLP510 => Flocculation::Medium,
            Self::WLP515 => Flocculation::Medium,
            Self::WLP518 => Flocculation::MediumHigh,
            Self::WLP519 => Flocculation::MediumHigh,
            Self::WLP520 => Flocculation::MediumHigh,
            Self::WLP521 => Flocculation::High,
            Self::WLP530 => Flocculation::MediumHigh,
            Self::WLP540 => Flocculation::Medium,
            Self::WLP545 => Flocculation::Medium,
            Self::WLP546 => Flocculation::Low,
            Self::WLP548 => Flocculation::Medium,
            Self::WLP550 => Flocculation::Medium,
            Self::WLP561 => Flocculation::Low,
            Self::WLP564 => Flocculation::Low,
            Self::WLP565 => Flocculation::Medium,
            Self::WLP566 => Flocculation::Medium,
            Self::WLP568 => Flocculation::Medium,
            Self::WLP570 => Flocculation::Low,
            Self::WLP575 => Flocculation::Medium,
            Self::WLP585 => Flocculation::MediumHigh,
            Self::WLP590 => Flocculation::Medium,
            //Self::WLP600 => Flocculation::
            //Self::WLP603 => Flocculation::
            Self::WLP611 => Flocculation::LowMedium,
            //Self::WLP616 => Flocculation::
            //Self::WLP618 => Flocculation::
            Self::WLP630 => Flocculation::Medium,
            // Self::WLP631 => Flocculation::
            // Self::WLP6420 => Flocculation::
            // Self::WLP644 => Flocculation::
            // Self::WLP645 => Flocculation::
            // Self::WLP648 => Flocculation::
            // Self::WLP650 => Flocculation::
            // Self::WLP653 => Flocculation::
            // Self::WLP655 => Flocculation::
            // Self::WLP661 => Flocculation::
            // Self::WLP665 => Flocculation::
            // Self::WLP669 => Flocculation::
            // Self::WLP670 => Flocculation::
            // Self::WLP672 => Flocculation::
            // Self::WLP673 => Flocculation::
            // Self::WLP675 => Flocculation::
            // Self::WLP677 => Flocculation::
            // Self::WLP678 => Flocculation::
            // Self::WLP685 => Flocculation::
            // Self::WLP686 => Flocculation::
            // Self::WLP690 => Flocculation::
            // Self::WLP692 => Flocculation::
            // Self::WLP693 => Flocculation::
            Self::WLP700 => Flocculation::Low,
            Self::WLP705 => Flocculation::Low,
            Self::WLP707 => Flocculation::Low,
            Self::WLP709 => Flocculation::LowMedium,
            Self::WLP715 => Flocculation::Low,
            Self::WLP718 => Flocculation::Low,
            Self::WLP720 => Flocculation::Low,
            Self::WLP727 => Flocculation::Low,
            Self::WLP730 => Flocculation::Low,
            Self::WLP735 => Flocculation::Low,
            Self::WLP740 => Flocculation::Low,
            Self::WLP749 => Flocculation::Low,
            Self::WLP750 => Flocculation::Low,
            Self::WLP760 => Flocculation::Low,
            Self::WLP770 => Flocculation::Low,
            Self::WLP773 => Flocculation::MediumHigh,
            //Self::WLP775 => Flocculation::Medium,
            //Self::WLP780 => Flocculation::
            Self::WLP800 => Flocculation::MediumHigh,
            Self::WLP802 => Flocculation::Medium,
            Self::WLP808 => Flocculation::Medium,
            Self::WLP810 => Flocculation::High,
            Self::WLP815 => Flocculation::Medium,
            Self::WLP820 => Flocculation::Medium,
            Self::WLP830 => Flocculation::Medium,
            Self::WLP833 => Flocculation::Medium,
            Self::WLP835 => Flocculation::Medium,
            Self::WLP838 => Flocculation::MediumHigh,
            Self::WLP840 => Flocculation::Medium,
            Self::WLP845 => Flocculation::Medium,
            Self::WLP850 => Flocculation::Medium,
            Self::WLP860 => Flocculation::Medium,
            Self::WLP885 => Flocculation::Medium,
            Self::WLP920 => Flocculation::Medium,
            Self::WLP925 => Flocculation::Medium,
            Self::WLP940 => Flocculation::Medium,
        }
    }

    /// Is the yeast dry?
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn is_dry(&self) -> bool {
        match *self {
            Self::SafAleBE134 => true,
            Self::SafAleBE256 => true,
            Self::SafAleBW20 => true,
            Self::SafAleK97 => true,
            Self::SafAleS04 => true,
            Self::SafAleS33 => true,
            Self::SafAleT58 => true,
            Self::SafAleUS05 => true,
            Self::SafAleW68 => true,
            Self::SafAleWB06 => true,
            Self::SafLagerE30 => true,
            Self::SafLagerS189 => true,
            Self::SafLagerS23 => true,
            Self::SafLagerSH45 => true,
            Self::SafLagerW3470 => true,
            Self::OYL061 => true,
            Self::OYL071 => false,

            Self::LalBrewAbbaye => true,
            Self::LalBrewAurora => true,
            Self::LalBrewBelleSaison => true,
            Self::LalBrewBRY97 => true,
            Self::LalBrewCBC1 => true,
            Self::LalBrewDiamondLager => true,
            Self::LalBrewFarmhouse => true,
            Self::LalBrewHouseAle => true,
            Self::LalBrewLoNa => true,
            Self::LalBrewMunichClassic => true,
            Self::LalBrewNewEngland => true,
            Self::LalBrewNottingham => true,
            Self::LalBrewNovaLager => true,
            Self::LalBrewPomona => true,
            Self::LalBrewVerdantIPA => true,
            Self::LalBrewVoss => true,
            Self::LalBrewWindsor => true,
            Self::LalBrewWit => true,

            // Self::WLP300 => false,
            // Self::WLP351 => false,
            // Self::WLP380 => false,
            // Self::WLP820 => false,
            // Self::WLP830 => false,
            // Self::WLP833 => false,
            // Self::WLP835 => false,
            // Self::WLP838 => false,
            _ => false,
        }
    }

    /// Is a lager yeast
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn is_lager(&self) -> bool {
        matches!(
            *self,
            Self::SafLagerE30
                | Self::SafLagerS189
                | Self::SafLagerS23
                | Self::SafLagerSH45
                | Self::SafLagerW3470
                | Self::LalBrewDiamondLager
                | Self::LalBrewNovaLager
                | Self::WLP4030
                | Self::WLP4035
                | Self::WLP800
                | Self::WLP802
                | Self::WLP808
                | Self::WLP810
                | Self::WLP815
                | Self::WLP820
                | Self::WLP830
                | Self::WLP833
                | Self::WLP835
                | Self::WLP838
                | Self::WLP840
                | Self::WLP845
                | Self::WLP850
                | Self::WLP860
                | Self::WLP885
                | Self::WLP920
                | Self::WLP925
                | Self::WLP940
        )
    }

    /// Yeast pitching rate, g/hL
    #[must_use]
    pub fn pitching_rate_range_ghl(&self) -> Option<Range<f32>> {
        match *self {
            Self::SafAleBW20 => Some(50.0..80.0),
            Self::SafAleW68 => Some(50.0..80.0),

            Self::LalBrewAbbaye => Some(50.0..100.0),
            Self::LalBrewAurora => Some(50.0..100.0),
            Self::LalBrewBelleSaison => Some(50.0..100.0),
            Self::LalBrewBRY97 => Some(50.0..100.0),
            Self::LalBrewCBC1 => Some(10.0..10.0), // for bottle cond
            Self::LalBrewDiamondLager => Some(100.0..200.0),
            Self::LalBrewFarmhouse => Some(50.0..100.0),
            Self::LalBrewHouseAle => Some(50.0..100.0),
            Self::LalBrewLoNa => Some(50.0..50.0),
            Self::LalBrewMunichClassic => Some(50.0..100.0),
            Self::LalBrewNewEngland => Some(100.0..200.0),
            Self::LalBrewNottingham => Some(50.0..100.0),
            Self::LalBrewNovaLager => Some(50.0..100.0),
            Self::LalBrewPomona => Some(50.0..100.0),
            Self::LalBrewVerdantIPA => Some(50.0..100.0),
            Self::LalBrewVoss => Some(50.0..100.0),
            Self::LalBrewWindsor => Some(50.0..100.0),
            Self::LalBrewWit => Some(50.0..100.0),
            // Self::LallemandPriseDeMousseWine => ,
            // Self::LallemandSourvisiae
            // Self::LallemandWildBrewPhillySour => ,
            _ => None,
        }
    }

    /// Yeast pitching rate (cells per mL per Plato)
    #[must_use]
    pub fn pitching_rate_cmlp(&self) -> u64 {
        if self.is_lager() {
            // 1.5m per ml per plato for lagers
            1_500_000
        } else {
            // 750k to 1m per ml per plato
            750_000
        }
    }

    /// FAN requirements, minimum, if known, for standard gravity of 1.040
    /// Worts generally should have (different people say different things):
    ///     180-200 ppm standard
    ///     250-300 ppm high gravity
    #[must_use]
    pub fn fan_requirement(&self) -> Ppm {
        match *self {
            Self::OYL061 => Ppm(180.0),
            Self::OYL071 => Ppm(180.0),

            Self::LalBrewMunichClassic => Ppm(180.0),
            Self::LalBrewNottingham => Ppm(150.0), // ale
            Self::LalBrewWindsor => Ppm(150.0),    // ale
            /*
            Self::WLP300 => Ppm(180.0),
            Self::WLP351 => Ppm(100.0), // Low N consumer
            Self::WLP380 => Ppm(150.0),
             */
            _ => {
                if self.is_lager() {
                    Ppm(100.0)
                } else {
                    Ppm(150.0)
                }
            }
        }
    }

    /// Strain
    ///
    /// This data may not be accurate, they are best guesses
    // multiple sources including:
    // https://captainbrew.com/white-labs-yeast-strains-chart
    // http://beer.suregork.com/?p=3907
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn strain(&self) -> Option<Strain> {
        match *self {
            Self::LalBrewNewEngland => Some(Strain::Conan),
            Self::LalBrewVoss => Some(Strain::VossKveik),
            Self::SafAleUS05 => Some(Strain::Chico),
            Self::SafAleW68 => Some(Strain::WeihenstephananWeizen68),
            Self::SafLagerS189 => Some(Strain::Hurlimann),
            Self::SafLagerS23 => Some(Strain::BerlinLager),
            Self::SafLagerW3470 => Some(Strain::WeihenstephananLager),
            Self::WLP001 => Some(Strain::Chico), // BRY96), WY1056
            Self::WLP002 => Some(Strain::Fullers), // BE045), WY1968
            Self::WLP003 => None, // unknown german brewery may match Wyeast 2575PC Kolsch II
            Self::WLP004 => Some(Strain::Guinness),
            Self::WLP005 => Some(Strain::Ringwood),
            Self::WLP006 => Some(Strain::CharlesWells),
            Self::WLP007 => Some(Strain::WhitbreadDry), // WY1098
            Self::WLP008 => Some(Strain::SamAdams),
            Self::WLP009 => Some(Strain::Coopers),
            //Self::WLP010 => None, // blend 001/002/004/810
            Self::WLP011 => Some(Strain::Weisenschaftliche),
            Self::WLP013 => Some(Strain::Worthington),
            Self::WLP017 => None, // blend
            Self::WLP022 => Some(Strain::Ridleys),
            Self::WLP023 => Some(Strain::HenleyOfThames),
            Self::WLP025 => Some(Strain::Adnams),
            Self::WLP026 => Some(Strain::Marstons),
            Self::WLP028 => Some(Strain::McEwan),
            Self::WLP029 => Some(Strain::PJFruh),
            //Self::WLP030 => Some(Strain::Mackeson),
            Self::WLP036 => Some(Strain::ZumUerigeAlt),
            Self::WLP037 => Some(Strain::YorkshireSquare),
            //Self::WLP038 => Some(Strain::Manchester),
            Self::WLP039 => Some(Strain::Nottingham),
            Self::WLP041 => Some(Strain::Redhook),
            Self::WLP051 => Some(Strain::Ballantine),
            Self::WLP060 => None, // blend
            Self::WLP064 => None, // blend
            Self::WLP067 => None, // blend
            Self::WLP075 => None, // blend
            Self::WLP076 => Some(Strain::NewAlbion),
            Self::WLP077 => None, // blend
            Self::WLP080 => None, // blend
            Self::WLP085 => None, // blend
            Self::WLP091 => None, // blend
            Self::WLP095 => Some(Strain::Conan),
            Self::WLP096 => None, // blend
            Self::WLP101 => None, // blend
            Self::WLP300 => Some(Strain::WeihenstephananWeizen68),
            Self::WLP320 => Some(Strain::ZumUerigeHefe),
            Self::WLP351 => Some(Strain::WeihenstephananWeizen175),
            Self::WLP380 => Some(Strain::WeihenstephananWeizen66),
            Self::WLP400 => Some(Strain::Hoegaarden),
            Self::WLP4007 => None, // blend
            Self::WLP4021 => None, // blend
            Self::WLP4027 => None, // blend
            Self::WLP4042 => None, // blend
            Self::WLP4044 => None, // blend
            Self::WLP410 => Some(Strain::Moortgat),
            Self::WLP500 => Some(Strain::Chimay),
            Self::WLP510 => Some(Strain::Orval),
            Self::WLP515 => Some(Strain::DeKoninck),
            Self::WLP520 => Some(Strain::VossKveik),
            Self::WLP530 => Some(Strain::Westmalle),
            Self::WLP540 => Some(Strain::Rochefort),
            Self::WLP548 => None, // blend
            Self::WLP550 => Some(Strain::Achouffe),
            Self::WLP561 => None, // blend
            Self::WLP564 => None, // blend
            Self::WLP565 => Some(Strain::Dupont),
            Self::WLP566 => Some(Strain::Dupont2),
            Self::WLP568 => None,                // blend
            Self::WLP570 => Some(Strain::Duvel), // WLP page says East Flanders, check
            Self::WLP575 => None,                // blend
            Self::WLP611 => None,                // blend
            Self::WLP630 => None,                // blend
            Self::WLP773 => None,                // blend
            Self::WLP800 => Some(Strain::Urquell),
            Self::WLP802 => Some(Strain::Samsons),
            Self::WLP808 => None, // blend
            Self::WLP810 => Some(Strain::AnchorSteam),
            Self::WLP820 => Some(Strain::WeihenstephananLager),
            Self::WLP830 => Some(Strain::WeihenstephananLager206),
            Self::WLP833 => Some(Strain::Ayinger),
            Self::WLP840 => Some(Strain::Budweiser),
            Self::WLP860 => Some(Strain::Augustiner),
            Self::WLP885 => Some(Strain::Samichlaus),
            Self::WLP940 => Some(Strain::Modelo),
            // Self::WY1316 or 1318 => Some(Strain::Boddingtons),
            // Self::WY1275 => Some(Strain::HenleyOfThames),
            // Self::WY1469 => Some(Strain::TimothyTaylor),
            Self::LalBrewBRY97 => Some(Strain::Ballantine),
            _ => None,
        }
    }

    /// STA1 positive
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn sta1(&self) -> Option<bool> {
        match *self {
            Self::LalBrewLoNa => Some(false),
            Self::WLP045 => Some(true),
            Self::WLP096 => Some(true),
            Self::WLP099 => Some(true),
            Self::WLP4001 => Some(true),
            Self::WLP4020 => Some(true),
            Self::WLP4021 => Some(true),
            Self::WLP4023 => Some(true),
            Self::WLP4025 => Some(true),
            Self::WLP4027 => Some(true),
            Self::WLP4044 => Some(true),
            Self::WLP4047 => Some(true),
            Self::WLP4060 => Some(true),
            Self::WLP545 => Some(true),
            Self::WLP564 => Some(true),
            Self::WLP565 => Some(true),
            Self::WLP566 => Some(true),
            Self::WLP568 => Some(true),
            Self::WLP570 => Some(true),
            Self::WLP585 => Some(true),
            Self::WLP590 => Some(true),
            Self::WLP630 => Some(true),
            _ => None,
        }
    }

    /// Gallone data
    ///
    /// Gives the Gallone paper strain, and a confidence value from 0.0 to 1.0
    #[must_use]
    #[allow(clippy::match_same_arms)]
    #[allow(clippy::too_many_lines)]
    pub fn gallone_data(&self) -> Option<(Gallone, f32)> {
        match *self {
            Self::SafAleBE134 => None,
            Self::SafAleBE256 => None,
            Self::SafAleBW20 => None,
            Self::SafAleK97 => None,
            Self::SafAleS04 => None,
            Self::SafAleS33 => None,
            Self::SafAleT58 => None,
            Self::SafAleUS05 => None,
            Self::SafAleW68 => None,
            Self::SafAleWB06 => None,
            Self::SafLagerE30 => None,
            Self::SafLagerS189 => None,
            Self::SafLagerS23 => None,
            Self::SafLagerSH45 => None,
            Self::SafLagerW3470 => None,

            // Self::ImperialBarbarian => None, // Conan strain
            // Self::ImperialBartlby => None,
            // Self::ImperialCablecar => None,
            // Self::ImperialCitrus => None,
            // Self::ImperialDarkness => None,
            // Self::ImperialDieter => None,
            // Self::ImperialDryHop => None,
            // Self::ImperialFlagship => None,
            // Self::ImperialGlobal => None,
            // Self::ImperialGnome => None,
            // Self::ImperialHarvest => None,
            // Self::ImperialHouse => None,
            // Self::ImperialIndependence => None,
            // Self::ImperialJoystick => None,
            // Self::ImperialJuice => None,
            // Self::ImperialKaiser => None,
            // Self::ImperialKveiking => None,
            // Self::ImperialLoki => None,
            // Self::ImperialNapoleon => None,
            // Self::ImperialPOG => None,
            // Self::ImperialPub => None,
            // Self::ImperialRustic => None,
            // Self::ImperialSourBatchKidz => None,
            // Self::ImperialStefon => None,
            // Self::ImperialSuburbanBrett => None,
            // Self::ImperialTartan => None,
            // Self::ImperialTripleDouble => None,
            // Self::ImperialUrkel => None,
            // Self::ImperialWhiteout => None,
            Self::LalBrewAbbaye => None,
            Self::LalBrewAurora => None,
            Self::LalBrewBelleSaison => None,
            Self::LalBrewBRY97 => None,
            Self::LalBrewCBC1 => None,
            Self::LalBrewDiamondLager => None,
            Self::LalBrewFarmhouse => None,
            Self::LalBrewHouseAle => None,
            Self::LalBrewLoNa => None,
            Self::LalBrewMunichClassic => None,
            Self::LalBrewNewEngland => None,
            Self::LalBrewNottingham => None,
            Self::LalBrewNovaLager => None,
            Self::LalBrewPomona => None,
            Self::LalBrewVerdantIPA => None,
            Self::LalBrewVoss => None,
            Self::LalBrewWindsor => None,
            Self::LalBrewWit => None,
            // Self::LallemandPriseDeMousseWine => None,
            // Self::LallemandSourvisiae => None,
            // Self::LallemandWildBrewPhillySour => None,
            Self::OYL061 => None,
            Self::OYL071 => None,

            Self::WLP001 => Some((Gallone::Be044, 1.0)), // genome sequencing match
            Self::WLP002 => Some((Gallone::Be050, 0.9)), // genome sequencing match, but 2 close hits
            Self::WLP003 => Some((Gallone::Be046, 0.6)), // yellow guess
            Self::WLP004 => Some((Gallone::Be047, 0.9)), // genome sequencing match, but 2 close hits
            Self::WLP005 => Some((Gallone::Be048, 0.6)), // yellow guess
            Self::WLP006 => Some((Gallone::Be049, 0.6)), // yellow guess
            Self::WLP007 => Some((Gallone::Be050, 0.6)), // yellow guess
            Self::WLP008 => Some((Gallone::Be051, 0.6)), // yellow guess
            Self::WLP009 => Some((Gallone::Be052, 0.6)), // yellow guess
            Self::WLP011 => Some((Gallone::Be053, 0.6)), // yellow guess
            Self::WLP013 => Some((Gallone::Be054, 0.6)), // yellow guess
            Self::WLP017 => Some((Gallone::Be055, 0.6)), // yellow guess
            //Self::WLP019 => Some((Gallone::Be065, 0.2)), // orange guess
            Self::WLP022 => Some((Gallone::Be056, 0.6)), // yellow guess
            Self::WLP023 => Some((Gallone::Be057, 1.0)), // genome sequencing match
            Self::WLP025 => Some((Gallone::Be058, 0.6)), // yellow guess
            Self::WLP026 => Some((Gallone::Be059, 0.6)), // yellow guess
            Self::WLP028 => Some((Gallone::Be060, 1.0)), // genome sequencing match
            Self::WLP029 => Some((Gallone::Be008, 0.2)), // orange guess
            //Self::WLP030 => Some((Gallone::Be067, 0.2)), // orange guess
            Self::WLP033 => None,
            Self::WLP036 => Some((Gallone::Be061, 0.6)), // yellow guess
            Self::WLP037 => Some((Gallone::Be062, 0.6)), // yellow guess
            //Self::WLP038 => Some((Gallone::Be063, 0.6)), // yellow guess
            Self::WLP039 => Some((Gallone::Be064, 0.6)), // yellow guess
            Self::WLP041 => Some((Gallone::Be066, 0.6)), // yellow guess
            Self::WLP045 => Some((Gallone::Sp008, 0.6)), // yellow guess
            Self::WLP050 => Some((Gallone::Sp009, 0.6)), // yellow guess
            Self::WLP051 => Some((Gallone::Be068, 0.2)),
            Self::WLP059 => None,
            Self::WLP060 => None,
            Self::WLP064 => None,
            Self::WLP065 => Some((Gallone::Sp010, 0.2)), // orange guess
            Self::WLP066 => None,
            Self::WLP067 => None,
            Self::WLP070 => None,
            Self::WLP072 => Some((Gallone::Be070, 0.6)), // yellow guess
            Self::WLP073 => None,
            Self::WLP075 => None,
            Self::WLP076 => Some((Gallone::Be069, 0.2)), // orange guess
            Self::WLP077 => None,
            Self::WLP078 => Some((Gallone::Sp011, 0.6)), // yellow guess
            Self::WLP080 => None,
            Self::WLP085 => None,
            Self::WLP090 => Some((Gallone::Be071, 0.6)), // yellow guess
            Self::WLP091 => None,
            Self::WLP095 => None,
            Self::WLP096 => None,
            Self::WLP099 => Some((Gallone::Be033, 1.0)), // genome sequencing match
            Self::WLP101 => None,
            Self::WLP1983 => None,
            Self::WLP300 => Some((Gallone::Be072, 1.0)), // genome sequencing match
            Self::WLP320 => Some((Gallone::Be073, 0.6)), // yellow guess
            Self::WLP351 => Some((Gallone::Be093, 0.2)), // orange guess
            Self::WLP380 => Some((Gallone::Be074, 0.6)), // yellow guess
            Self::WLP400 => Some((Gallone::Be075, 0.6)), // yellow guess
            Self::WLP4000 => None,
            Self::WLP4001 => None,
            Self::WLP4007 => None,
            Self::WLP4015 => None,
            Self::WLP4020 => None,
            Self::WLP4021 => None,
            Self::WLP4022 => None,
            Self::WLP4023 => None,
            Self::WLP4025 => None,
            Self::WLP4027 => None,
            Self::WLP4030 => None,
            Self::WLP4035 => None,
            Self::WLP4040 => None,
            Self::WLP4042 => None,
            Self::WLP4044 => None,
            Self::WLP4045 => None,
            Self::WLP4046 => None,
            Self::WLP4047 => None,
            Self::WLP4050 => None,
            Self::WLP4051 => None,
            Self::WLP4052 => None,
            Self::WLP4053 => None,
            Self::WLP4060 => None,
            Self::WLP4061 => None,
            Self::WLP4062 => None,
            Self::WLP410 => Some((Gallone::Be076, 0.6)), // yellow guess
            // Self::WLP4605 => None,
            // Self::WLP4615 => None,
            // Self::WLP4620 => None,
            // Self::WLP4626 => None,
            // Self::WLP4633 => None,
            // Self::WLP4636 => None,
            // Self::WLP4637 => None,
            // Self::WLP4638 => None,
            // Self::WLP4639 => None,
            // Self::WLP4640 => None,
            // Self::WLP4641 => None,
            // Self::WLP4642 => None,
            // Self::WLP4643 => None,
            // Self::WLP4645 => None,
            // Self::WLP4650 => None,
            // Self::WLP4651 => None,
            // Self::WLP4653 => None,
            // Self::WLP4655 => None,
            // Self::WLP4656 => None,
            // Self::WLP4665 => None,
            // Self::WLP4675 => None,
            // Self::WLP4681 => None,
            // Self::WLP4682 => None,
            // Self::WLP4684 => None,
            Self::WLP500 => None,
            Self::WLP510 => Some((Gallone::Be077, 0.6)),
            Self::WLP515 => Some((Gallone::Be082, 0.2)), // orange guess
            Self::WLP518 => None,
            Self::WLP519 => None,
            Self::WLP520 => None,
            Self::WLP521 => None,
            Self::WLP530 => Some((Gallone::Be078, 0.6)),
            Self::WLP540 => Some((Gallone::Be079, 0.6)),
            Self::WLP545 => Some((Gallone::Be080, 0.6)),
            Self::WLP546 => None,
            Self::WLP548 => None,
            Self::WLP550 => Some((Gallone::Be081, 0.6)),
            Self::WLP561 => None,
            Self::WLP564 => None,
            Self::WLP565 => Some((Gallone::Be083, 1.0)), // genetic sequencing match
            Self::WLP566 => Some((Gallone::Be084, 0.6)), // yellow guess
            Self::WLP568 => None,
            Self::WLP570 => Some((Gallone::Be085, 1.0)), // genome sequencing match
            Self::WLP575 => None,
            Self::WLP585 => Some((Gallone::Be086, 0.6)), // yellow guess
            Self::WLP590 => Some((Gallone::Be092, 0.2)), // orange guess
            //Self::WLP600 => None,
            //Self::WLP603 => None,
            Self::WLP611 => Some((Gallone::Wl005, 0.33)), // orange guess; WL 5 6 or 7
            //Self::WLP616 => None,
            //Self::WLP618 => None,
            Self::WLP630 => None,
            // Self::WLP631 => None,
            // Self::WLP6420 => None,
            // Self::WLP644 => None,
            // Self::WLP645 => None,
            // Self::WLP648 => None,
            // Self::WLP650 => None,
            // Self::WLP653 => None,
            // Self::WLP655 => None,
            // Self::WLP661 => None,
            // Self::WLP665 => None,
            // Self::WLP669 => None,
            // Self::WLP670 => None,
            // Self::WLP672 => None,
            // Self::WLP673 => None,
            // Self::WLP675 => None,
            // Self::WLP677 => None,
            // Self::WLP678 => None,
            // Self::WLP685 => None,
            // Self::WLP686 => None,
            // Self::WLP690 => None,
            // Self::WLP692 => None,
            // Self::WLP693 => None,
            Self::WLP700 => None,
            Self::WLP705 => Some((Gallone::Sa002, 0.8)), // genome sequencing match, but 2 close hits
            Self::WLP707 => None,
            Self::WLP709 => None,
            Self::WLP715 => None,
            Self::WLP718 => None,
            Self::WLP720 => None,
            Self::WLP727 => None,
            Self::WLP730 => None,
            Self::WLP735 => None,
            Self::WLP740 => None,
            Self::WLP749 => None,
            Self::WLP750 => None,
            Self::WLP760 => None,
            Self::WLP770 => None,
            Self::WLP773 => None,
            //Self::WLP775 => None,
            //Self::WLP780 => None,
            Self::WLP800 => Some((Gallone::Be087, 1.0)), // genetic sequencing match
            Self::WLP802 => None,
            Self::WLP808 => None,
            Self::WLP810 => None,
            Self::WLP815 => None,
            Self::WLP820 => None,
            Self::WLP830 => None,
            Self::WLP833 => None,
            Self::WLP835 => None,
            Self::WLP838 => None,
            Self::WLP840 => None,
            Self::WLP845 => None,
            Self::WLP850 => None,
            Self::WLP860 => None,
            Self::WLP885 => None,
            Self::WLP920 => None,
            Self::WLP925 => None,
            Self::WLP940 => None,
        }
    }
}

impl fmt::Display for Yeast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}: {:?} {}]", self.provider(), self, self.desc())
    }
}
