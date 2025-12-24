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
    SafAleBW20,
    SafAleK97,
    SafAleS04,
    SafAleS33,
    SafAleT58,
    SafAleUS05,
    SafAleW68,
    SafAleWB06,
    SafLagerE30,
    SafLagerS189,
    SafLagerS23,
    SafLagerSH45,
    SafLagerW3470,

    /*
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
    ImperialLoki,   // KveikVoss
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
     */

    /*
    LalBrewAbbaye,
    LalBrewBelleSaison,
    LalBrewBRY97,
    LalBrewCBC1,
    LalBrewDiamondLager,
    LalBrewKoln,
    LalBrewLondon,
    */
    LalBrewMunichClassic,
    /*
    LalBrewNewEngland,
    */
    LalBrewNottingham,
    /*
    LalBrewVerdantIPA,
    LalBrewVoss,    // KveikVoss
    */
    LalBrewWindsor,
    /*
    LalBrewWit
    LallemandPriseDeMousseWine,
    LallemandSourvisiae
    LallemandWildBrewPhillySour,
     */

    /*
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
    */

    /*
    MuntonsPremiumGold,
    MuntonsStandardAle,
     */

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
    WLP038,
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

    /*
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
    */
}

impl Yeast {

    /// Provider
    #[must_use]
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

            /*
            Self::ImperialBarbarian |
            Self::ImperialBartlby |
            Self::ImperialCablecar |
            Self::ImperialCitrus |
            Self::ImperialDarkness |
            Self::ImperialDieter |
            Self::ImperialDryHop |
            Self::ImperialFlagship |
            Self::ImperialGlobal |
            Self::ImperialGnome |
            Self::ImperialHarvest |
            Self::ImperialHouse |
            Self::ImperialIndependence |
            Self::ImperialJoystick |
            Self::ImperialJuice |
            Self::ImperialKaiser |
            Self::ImperialKveiking |
            Self::ImperialLoki |
            Self::ImperialNapoleon |
            Self::ImperialPOG |
            Self::ImperialPub |
            Self::ImperialRustic |
            Self::ImperialSourBatchKidz |
            Self::ImperialStefon |
            Self::ImperialSuburbanBrett |
            Self::ImperialTartan |
            Self::ImperialTripleDouble |
            Self::ImperialUrkel |
            Self::ImperialWhiteout => YeastProvider::ImperialYeast,
             */

            /*
            Self::LalBrewAbbaye |
            Self::LalBrewBelleSaison |
            Self::LalBrewBRY97 |
            Self::LalBrewCBC1 |
            Self::LalBrewDiamondLager |
            Self::LalBrewKoln |
            Self::LalBrewLondon |
            */
            Self::LalBrewMunichClassic |
            /*
            Self::LalBrewNewEngland |
            */
            Self::LalBrewNottingham |
            /*
            Self::LalBrewVerdantIPA |
            Self::LalBrewVoss |
            */
            Self::LalBrewWindsor => YeastProvider::Lallemand,
            /*
            Self::LalBrewWit |
            Self::LallemandPriseDeMousseWine |
            Self::LallemandSourvisiae |
            Self::LallemandWildBrewPhillySour => YeastProvider::Lallemand,
             */

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
            Self::WLP019 |
            Self::WLP022 |
            Self::WLP023 |
            Self::WLP025 |
            Self::WLP026 |
            Self::WLP028 |
            Self::WLP029 |
            Self::WLP030 |
            Self::WLP033 |
            Self::WLP036 |
            Self::WLP037 |
            Self::WLP038 |
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
            Self::WLP4605 |
            Self::WLP4615 |
            Self::WLP4620 |
            Self::WLP4626 |
            Self::WLP4633 |
            Self::WLP4636 |
            Self::WLP4637 |
            Self::WLP4638 |
            Self::WLP4639 |
            Self::WLP4640 |
            Self::WLP4641 |
            Self::WLP4642 |
            Self::WLP4643 |
            Self::WLP4645 |
            Self::WLP4650 |
            Self::WLP4651 |
            Self::WLP4653 |
            Self::WLP4655 |
            Self::WLP4656 |
            Self::WLP4665 |
            Self::WLP4675 |
            Self::WLP4681 |
            Self::WLP4682 |
            Self::WLP4684 |
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
            Self::WLP600 |
            Self::WLP603 |
            Self::WLP611 |
            Self::WLP616 |
            Self::WLP618 |
            Self::WLP630 |
            Self::WLP631 |
            Self::WLP6420 |
            Self::WLP644 |
            Self::WLP645 |
            Self::WLP648 |
            Self::WLP650 |
            Self::WLP653 |
            Self::WLP655 |
            Self::WLP661 |
            Self::WLP665 |
            Self::WLP669 |
            Self::WLP670 |
            Self::WLP672 |
            Self::WLP673 |
            Self::WLP675 |
            Self::WLP677 |
            Self::WLP678 |
            Self::WLP685 |
            Self::WLP686 |
            Self::WLP690 |
            Self::WLP692 |
            Self::WLP693 |
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
            Self::WLP775 |
            Self::WLP780 |
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
            /*
            Self::ImperialBarbarian => "",
            Self::ImperialBartlby => "",
            Self::ImperialCablecar => "",
            Self::ImperialCitrus => "",
            Self::ImperialDarkness => "",
            Self::ImperialDieter => "",
            Self::ImperialDryHop => "",
            Self::ImperialFlagship => "",
            Self::ImperialGlobal => "",
            Self::ImperialGnome => "",
            Self::ImperialHarvest => "",
            Self::ImperialHouse => "",
            Self::ImperialIndependence => "",
            Self::ImperialJoystick => "",
            Self::ImperialJuice => "",
            Self::ImperialKaiser => "",
            Self::ImperialKveiking => "",
            Self::ImperialLoki => "",
            Self::ImperialNapoleon => "",
            Self::ImperialPOG => "",
            Self::ImperialPub => "",
            Self::ImperialRustic => "",
            Self::ImperialSourBatchKidz => "",
            Self::ImperialStefon => "",
            Self::ImperialSuburbanBrett => "",
            Self::ImperialTartan => "",
            Self::ImperialTripleDouble => "",
            Self::ImperialUrkel => "",
            Self::ImperialWhiteout => "",
             */

            /*
            Self::LalBrewAbbaye => "",
            Self::LalBrewBelleSaison => "",
            Self::LalBrewBRY97 => "",
            Self::LalBrewCBC1 => "",
            Self::LalBrewDiamondLager => "",
            Self::LalBrewKoln => "",
            Self::LalBrewLondon => "",
            */
            Self::LalBrewMunichClassic => "LalBrew Munich Classic",
            /*
            Self::LalBrewNewEngland => "",
            */
            Self::LalBrewNottingham => "LalBrew Nottingham",
            /*
            Self::LalBrewVerdantIPA => "",
            Self::LalBrewVoss => "",
            */
            Self::LalBrewWindsor => "LalBrew Windsor",
            /*
            Self::LalBrewWit => "",
            Self::LallemandPriseDeMousseWine => "",
            Self::LallemandSourvisiae => "",
            Self::LallemandWildBrewPhillySour => "",
            */
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
            Self::WLP019 => "WLP019 California IV Ale Yeast",
            Self::WLP022 => "WLP022 Essex Ale Yeast",
            Self::WLP023 => "WLP023 Burton Ale Yeast",
            Self::WLP025 => "WLP025 Southwold Ale Yeast",
            Self::WLP026 => "WLP026 Premium Bitter Ale Yeast",
            Self::WLP028 => "WLP028 Edinburgh/Scottish Ale Yeast",
            Self::WLP029 => "WLP029 Kölsch Ale Yeast",
            Self::WLP030 => "WLP030 Thames Valley Ale Yeast",
            Self::WLP033 => "WLP033 Klassic Ale Yeast",
            Self::WLP036 => "WLP036 Dusseldorf Alt Ale Yeast",
            Self::WLP037 => "WLP037 Yorkshire Square Ale Yeast",
            Self::WLP038 => "WLP038 Manchester Ale Yeast",
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
            Self::WLP4605 => "WLP4605 Beersen Brettanomyces",
            Self::WLP4615 => "WLP4615 Brussels Brettanomyces",
            Self::WLP4620 => "WLP4620 Lochristi Brettanomyces",
            Self::WLP4626 => "WLP4626 Saison/Brettanomyces Yeast Blend I",
            Self::WLP4633 => "WLP4633 Mélange Yeast Blend",
            Self::WLP4636 => "WLP4636 Saison/Brettanomyces Yeast Blend II",
            Self::WLP4637 => "WLP4637 Amalgamation I Brettanomyces Blend",
            Self::WLP4638 => "WLP4638 Brettanomyces bruxellensis strain TYB184",
            Self::WLP4639 => "WLP4639 Brettanomyces bruxellensis strain TYB207",
            Self::WLP4640 => "WLP4640 Brettanomyces bruxellensis strain TYB261",
            Self::WLP4641 => "WLP4641 Amalgamation II Brettanomyces Blend",
            Self::WLP4642 => "WLP4642 Oud Vat Brett",
            Self::WLP4643 => "WLP4643 Amalgamation V Brettanomyces Blend",
            Self::WLP4645 => "WLP4645 Transatlantic Berliner Blend",
            Self::WLP4650 => "WLP4650 Metschnikowia reukaufii",
            Self::WLP4651 => "WLP4651 Von Degenberg Hefe Blend",
            Self::WLP4653 => "WLP4653 Dark Belgian Cask Yeast Blend",
            Self::WLP4655 => "WLP4655 Brettanomyces bruxellensis strain TYB307",
            Self::WLP4656 => "WLP4656 Brettanomyces bruxellensis strain TYB415",
            Self::WLP4665 => "WLP4665 Berkeley Hills Sour Yeast",
            Self::WLP4675 => "WLP4675 Farmhouse Sour Ale Yeast Blend",
            Self::WLP4681 => "WLP4681 Lactobacillus brevis Strain TYB282",
            Self::WLP4682 => "WLP4682 Lactobacillus Blend",
            Self::WLP4684 => "WLP4684 The YEast Bay House Sour Blend",
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
            Self::WLP600 => "WLP600 Kombucha Cultures",
            Self::WLP603 => "WLP603 Torulaspora delbrueckii",
            Self::WLP611 => "WLP611 New Nordic Ale Yeast",
            Self::WLP616 => "WLP616 Funky Cider Blend",
            Self::WLP618 => "WLP618 NA All Day",
            Self::WLP630 => "WLP630 Berliner Weisse Blend",
            Self::WLP631 => "WLP631 Appalachian Tart",
            Self::WLP6420 => "WLP6420 Acid Tripp",
            Self::WLP644 => "WLP644 Saccharomyces brux-like Trois",
            Self::WLP645 => "WLP645 Brettanomyces claussenii",
            Self::WLP648 => "WLP648 Brettanomyces bruxellensis Trois Vrai",
            Self::WLP650 => "WLP650 Brettanomyces bruxellensis",
            Self::WLP653 => "WLP653 Brettanomyces lambicus",
            Self::WLP655 => "WLP655 Belgian Sour Mix 1",
            Self::WLP661 => "WLP661 Pediococcus damnosus",
            Self::WLP665 => "WLP665 Flemish Ale Blend",
            Self::WLP669 => "WLP669 Lactobacillus paracollinoides",
            Self::WLP670 => "WLP670 American Farmhouse Blend",
            Self::WLP672 => "WLP672 Lactobacillus brevis",
            Self::WLP673 => "WLP673 Lactobacillus buchneri",
            Self::WLP675 => "WLP675 Malolactic Cultures",
            Self::WLP677 => "WLP677 Lactobacillus delbrueckii",
            Self::WLP678 => "WLP678 Lactobacillus hilgardii",
            Self::WLP685 => "WLP685 Gluconobacter Oxydans",
            Self::WLP686 => "WLP686 Zygosaccharomyces lentus",
            Self::WLP690 => "WLP690 Acetobacter aceti",
            Self::WLP692 => "WLP692 Debaromyces hansenii",
            Self::WLP693 => "WLP693 Lactobacillus plantarum",
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
            Self::WLP775 => "WLP775 English Cider Yeast",
            Self::WLP780 => "WLP780 Thai Rice Chong Yeast",
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
            Self::OYL061 => (25.0, 40.0),
            Self::OYL071 => (12.0, 35.0),
            Self::LalBrewMunichClassic => (17.0, 25.0),
            Self::LalBrewNottingham => (10.0, 25.0),
            Self::LalBrewWindsor => (15.0, 25.0),
            /*

            Self::WLP300 => Celsius(20.0)..Celsius(22.0),
            Self::WLP351 => Celsius(19.0)..Celsius(21.0),
            Self::WLP380 => Celsius(19.0)..Celsius(21.0),
            Self::WLP820 => Celsius(11.0)..Celsius(14.0),
            Self::WLP830 => Celsius(10.0)..Celsius(13.0),
            Self::WLP833 => Celsius(9.0)..Celsius(13.0),
            Self::WLP835 => Celsius(10.0)..Celsius(12.0),
            Self::WLP838 => Celsius(10.0)..Celsius(13.0),
             */
            _ => todo!(),
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
            Self::OYL061 => 0.76..0.82,
            Self::OYL071 => 0.75..0.82,
            Self::LalBrewMunichClassic => 0.76..0.83,
            Self::LalBrewNottingham => 0.78..0.84,
            Self::LalBrewWindsor => 0.65..0.72,
            /*

            Self::WLP300 => 0.72..0.76,
            Self::WLP351 => 0.75..0.82,
            Self::WLP380 => 0.73..0.80,
            Self::WLP820 => 0.65..0.73,
            Self::WLP830 => 0.74..0.79,
            Self::WLP833 => 0.70..0.76,
            Self::WLP835 => 0.70..0.76,
            Self::WLP838 => 0.68..0.76,
            */
            _ => todo!(),
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
            Self::OYL061 => (12, 12),
            Self::OYL071 => (15, 15),
            Self::LalBrewMunichClassic => (12, 12),
            Self::LalBrewNottingham => (14, 14),
            Self::LalBrewWindsor => (12, 12),
            /*
            Self::WLP300 => 0.08..0.12,
            Self::WLP351 => 0.15..0.15,
            Self::WLP380 => 0.05..0.10,
            Self::WLP820 => 0.05..0.10,
            Self::WLP830 => 0.05..0.10,
            Self::WLP833 => 0.05..0.10,
            Self::WLP835 => 0.08..0.12,
            Self::WLP838 => 0.05..0.10,
             */
            _ => todo!(),
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
    pub fn flocculation(&self) -> Flocculation {
        match *self {
            Self::SafAleBE134 => Flocculation::Low, // sed slow
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
            Self::SafLagerS23 => Flocculation::High, // sed fast
            Self::SafLagerSH45 => Flocculation::Medium,
            Self::SafLagerW3470 => Flocculation::High,
            Self::OYL061 => Flocculation::VeryHigh,
            Self::OYL071 => Flocculation::MediumHigh,
            Self::LalBrewMunichClassic => Flocculation::Low,
            Self::LalBrewNottingham => Flocculation::High,
            Self::LalBrewWindsor => Flocculation::Low,
            /*
            Self::WLP300 => Flocculation::Low,
            Self::WLP351 => Flocculation::Low,
            Self::WLP380 => Flocculation::Low,
            Self::WLP820 => Flocculation::Medium,
            Self::WLP830 => Flocculation::Medium,
            Self::WLP833 => Flocculation::Medium,
            Self::WLP835 => Flocculation::Medium,
            Self::WLP838 => Flocculation::MediumHigh,
             */
            _ => todo!(),
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
            Self::LalBrewMunichClassic => true,
            Self::LalBrewNottingham => true,
            Self::LalBrewWindsor => true,
            /*
            Self::WLP300 => false,
            Self::WLP351 => false,
            Self::WLP380 => false,
            Self::WLP820 => false,
            Self::WLP830 => false,
            Self::WLP833 => false,
            Self::WLP835 => false,
            Self::WLP838 => false,
             */
            _ => todo!(),
        }
    }

    /// Is a lager yeast
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn is_lager(&self) -> bool {
        match *self {
            Self::SafLagerE30 => true,
            Self::SafLagerS189 => true,
            Self::SafLagerS23 => true,
            Self::SafLagerSH45 => true,
            Self::SafLagerW3470 => true,
            _ => false,
        }
    }

    /// Pitching rate, if known
    #[must_use]
    pub fn pitching_rate(&self) -> Option<(Grams, Liters)> {
        match *self {
            // 50-100g/hL
            Self::LalBrewMunichClassic => Some((Grams(75.0), Liters(100.0))),
            // 50-100g/hL
            Self::LalBrewNottingham => Some((Grams(75.0), Liters(100.0))),
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
            _ => if self.is_lager() {
                Ppm(100.0)
            } else {
                Ppm(150.0)
            }
        }
    }

    /// Strain
    ///
    /// This data may not be accurate, they are best guesses
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn strain(&self) -> Option<Strain> {
        match *self {
            Self::SafLagerW3470 => Some(Strain::WeihenstephananLager),
            Self::WLP001 => Some(Strain::Chico), // BRY96), WY1056
            Self::WLP002 => Some(Strain::Fullers), // BE045), WY1968
            Self::WLP004 => Some(Strain::Guinness),
            Self::WLP005 => Some(Strain::Ringwood),
            Self::WLP006 => Some(Strain::BedfordBritishAle),
            Self::WLP008 => Some(Strain::SamAdams),
            Self::WLP007 => Some(Strain::WhitbreadDry), // WY1098
            Self::WLP009 => Some(Strain::Coopers),
            Self::WLP011 => Some(Strain::Weisenschaftliche),
            Self::WLP013 => Some(Strain::Worthington),
            Self::WLP026 => Some(Strain::Marstons),
            Self::WLP028 => Some(Strain::McEwan),
            Self::WLP030 => Some(Strain::Mackeson),
            Self::WLP036 => Some(Strain::ZumUerigeAlt),
            Self::WLP037 => Some(Strain::YorkshireSquare),
            Self::WLP038 => Some(Strain::Manchester),
            Self::WLP039 => Some(Strain::Nottingham),
            Self::WLP041 => Some(Strain::Redhook),
            Self::WLP051 => Some(Strain::Ballantine),
            Self::WLP076 => Some(Strain::NewAlbion),
            Self::WLP300 => Some(Strain::WeihenstephananWeizen),
            Self::WLP400 => Some(Strain::Hoegaarden),
            Self::WLP510 => Some(Strain::Orval),
            Self::WLP530 => Some(Strain::Westmalle),
            Self::WLP570 => Some(Strain::Duvel),
            Self::WLP800 => Some(Strain::Urquell),
            Self::WLP810 => Some(Strain::AnchorSteam),
            // Self::WY1316 => Some(Strain::Boddingtons),
            // Self::WY1275 => Some(Strain::HenleyOfThames),
            // Self::WY1469 => Some(Strain::TimothyTaylor),
            // Self::LalBrewBRY97 => Some(Strain::Ballantine),
            _ => None,
        }
    }

    /// Gallone data
    ///
    /// Gives the Gallone paper strain, and a confidence value from 0.0 to 1.0
    #[must_use]
    #[allow(clippy::match_same_arms)]
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

            /*
            Self::ImperialBarbarian => None,
            Self::ImperialBartlby => None,
            Self::ImperialCablecar => None,
            Self::ImperialCitrus => None,
            Self::ImperialDarkness => None,
            Self::ImperialDieter => None,
            Self::ImperialDryHop => None,
            Self::ImperialFlagship => None,
            Self::ImperialGlobal => None,
            Self::ImperialGnome => None,
            Self::ImperialHarvest => None,
            Self::ImperialHouse => None,
            Self::ImperialIndependence => None,
            Self::ImperialJoystick => None,
            Self::ImperialJuice => None,
            Self::ImperialKaiser => None,
            Self::ImperialKveiking => None,
            Self::ImperialLoki => None,
            Self::ImperialNapoleon => None,
            Self::ImperialPOG => None,
            Self::ImperialPub => None,
            Self::ImperialRustic => None,
            Self::ImperialSourBatchKidz => None,
            Self::ImperialStefon => None,
            Self::ImperialSuburbanBrett => None,
            Self::ImperialTartan => None,
            Self::ImperialTripleDouble => None,
            Self::ImperialUrkel => None,
            Self::ImperialWhiteout => None,
             */

            /*
            Self::LalBrewAbbaye => None,
            Self::LalBrewBelleSaison => None,
            Self::LalBrewBRY97 => None,
            Self::LalBrewCBC1 => None,
            Self::LalBrewDiamondLager => None,
            Self::LalBrewKoln => None,
            Self::LalBrewLondon => None,
            */
            Self::LalBrewMunichClassic => None,
            /*
            Self::LalBrewNewEngland => None,
            */
            Self::LalBrewNottingham => None,
            /*
            Self::LalBrewVerdantIPA => None,
            Self::LalBrewVoss => None,
            */
            Self::LalBrewWindsor => None,
            /*
            Self::LalBrewWit => None,
            Self::LallemandPriseDeMousseWine => None,
            Self::LallemandSourvisiae => None,
            Self::LallemandWildBrewPhillySour => None,
             */

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
            Self::WLP019 => Some((Gallone::Be065, 0.2)), // orange guess
            Self::WLP022 => Some((Gallone::Be056, 0.6)), // yellow guess
            Self::WLP023 => Some((Gallone::Be057, 1.0)), // genome sequencing match
            Self::WLP025 => Some((Gallone::Be058, 0.6)), // yellow guess
            Self::WLP026 => Some((Gallone::Be059, 0.6)), // yellow guess
            Self::WLP028 => Some((Gallone::Be060, 1.0)), // genome sequencing match
            Self::WLP029 => Some((Gallone::Be008, 0.2)), // orange guess
            Self::WLP030 => Some((Gallone::Be067, 0.2)), // orange guess
            Self::WLP033 => None,
            Self::WLP036 => Some((Gallone::Be061, 0.6)), // yellow guess
            Self::WLP037 => Some((Gallone::Be062, 0.6)), // yellow guess
            Self::WLP038 => Some((Gallone::Be063, 0.6)), // yellow guess
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
            Self::WLP4605 => None,
            Self::WLP4615 => None,
            Self::WLP4620 => None,
            Self::WLP4626 => None,
            Self::WLP4633 => None,
            Self::WLP4636 => None,
            Self::WLP4637 => None,
            Self::WLP4638 => None,
            Self::WLP4639 => None,
            Self::WLP4640 => None,
            Self::WLP4641 => None,
            Self::WLP4642 => None,
            Self::WLP4643 => None,
            Self::WLP4645 => None,
            Self::WLP4650 => None,
            Self::WLP4651 => None,
            Self::WLP4653 => None,
            Self::WLP4655 => None,
            Self::WLP4656 => None,
            Self::WLP4665 => None,
            Self::WLP4675 => None,
            Self::WLP4681 => None,
            Self::WLP4682 => None,
            Self::WLP4684 => None,
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
            Self::WLP600 => None,
            Self::WLP603 => None,
            Self::WLP611 => Some((Gallone::Wl005, 0.33)), // orange guess; WL 5 6 or 7
            Self::WLP616 => None,
            Self::WLP618 => None,
            Self::WLP630 => None,
            Self::WLP631 => None,
            Self::WLP6420 => None,
            Self::WLP644 => None,
            Self::WLP645 => None,
            Self::WLP648 => None,
            Self::WLP650 => None,
            Self::WLP653 => None,
            Self::WLP655 => None,
            Self::WLP661 => None,
            Self::WLP665 => None,
            Self::WLP669 => None,
            Self::WLP670 => None,
            Self::WLP672 => None,
            Self::WLP673 => None,
            Self::WLP675 => None,
            Self::WLP677 => None,
            Self::WLP678 => None,
            Self::WLP685 => None,
            Self::WLP686 => None,
            Self::WLP690 => None,
            Self::WLP692 => None,
            Self::WLP693 => None,
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
            Self::WLP775 => None,
            Self::WLP780 => None,
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
