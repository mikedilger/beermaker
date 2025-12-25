use crate::units::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Range;

mod conditioning;
pub use conditioning::Conditioning;

mod fermentation;
pub use fermentation::Fermentation;

mod origin;
pub use origin::StyleOrigin;

/// Style of beer
///
/// Styles are defined by a few different groups. Of course they disagree.
/// We take data from two of them:
///
/// Beer Judge Certification Program
/// <https://www.bjcp.org/bjcp-style-guidelines/>
///
/// Brewer's Association
/// <https://www.brewersassociation.org/edu/brewers-association-beer-style-guidelines/>
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Style {
    /// 1A. American Light Lager
    AmericanLightLager,

    /// 1B. American Lager
    AmericanLager,

    /// 1C. Cream Ale
    CreamAle,

    /// 1D. American Wheat Beer
    AmericanWheatBeer,

    // 2A. International Pale Lager

    // 2B. International Amber Lager

    // 2C. International Dark Lager

    // 3A. Czech Pale Lager

    // 3B. Cezeh Premium Pale Lager

    // 3C. Czech Amber Lager

    // 3D. Czech Dark Lager

    // 4A. Munich Helles

    // 4B. Festbier

    // 4C. Helles Bock

    // 5A. German Leichtbeer

    // 5B. Kölsch

    // 5C. Germen Helles Exportbier

    // 5D. German Pils
    /// 6A. Märzen
    Marzen,

    // 6B. Rauchbier

    // 6C. Dunkles Bock

    // 7A. Vienna Lager

    // 7B. Altbier

    // 8A. Munich Dunkel

    // 8B. Schwarzbier

    // 9A. Doppelbock

    // 9B. Eisbock

    // 9C. Baltic Porter
    /// 10A. Weissbier
    Weissbier,

    /// 10B. Dunkles Weissbier
    DunklesWeissbier,

    // 10C. Weizenbock
    /// 11A. Ordinary Bitter
    OrdinaryBitter,

    /// 11B. Best Bitter
    BestBitter,

    /// 11C. Strong Bitter
    StrongBitter,

    // 12A. British Golden Ale

    // 12B. Australian Sparkling Ale

    // 12C. English IPA
    /// 13A. Dark Mild
    DarkMild,

    // 13B. British Brown Ale

    // 13C. English Porter

    // 14A. Scottish Light

    // 14B. Scottish Heavy

    // 14C. Scottish Export
    /// 15A. Irish Red Ale
    IrishRedAle,

    // 15B. Irish Stout

    // 15C. Irish Extra Stout

    // 16A. Sweet Stout

    // 16B. Oatmeal Stout

    // 16C. Tropical Stout

    // 16D. Foreign Extra Stout

    // 17A. British Strong Ale

    // 17B. Old Ale

    // 17C. Wee Heavy

    // 17D. English Barley Wine

    // 18A. Blonde Ale

    // 18B. American Pale Ale

    // 19A. American Amber Ale

    // 19B. California Common

    // 19C. American Brown Ale

    // 20A. American Porter

    // 20B. American Stout

    // 20C. Imperial Stout

    // 21A. American IPA

    // 21B1. Belgian IPA

    // 21B2. Black IPA

    // 21B3. Brown IPA

    // 21B4. Red IPA

    // 21B5. Rye IPA

    // 21B6. White IPA

    // 21B7. Brut IPA

    // 21C. Hazy IPA

    // 22A. Double IPA

    // 22B. American Strong Ale

    // 22C. American Barleywine

    // 22D. Wheatwine

    // 23A. Berliner Weisse

    // 23B. Flanders Red Ale

    // 23C. Oud Bruin

    // 23D. Lambic

    // 23E. Gueuze

    // 23F. Fruit Lambic

    // 23G. Gose

    // 24A. Witbier

    // 24B. Belgian Pale Ale

    // 24C. Bière de Garde

    // 25A. Belgian Blond Ale

    // 25B. Saison

    // 25C. Belgian Golden Strong Ale

    // 26A. Belgian Single

    // 26B. Belgian Dubbel

    // 26C. Belgian Tripel
    /// 26D. Belgian Dark Strong Ale (Belgian Qudrupel, Grand Cru)
    BelgianDarkStrongAle,

    /// Leichtesweizen, a light version of Weissbier
    LeichtesWeizen,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::AmericanLightLager => write!(f, "American Light Lager"),
            Self::AmericanLager => write!(f, "American Lager"),
            Self::CreamAle => write!(f, "Cream Ale"),
            Self::AmericanWheatBeer => write!(f, "American Wheat Beer"),
            Self::OrdinaryBitter => write!(f, "Ordinary Bitter"),
            Self::BestBitter => write!(f, "Best Bitter"),
            Self::StrongBitter => write!(f, "Strong Bitter"),
            Self::DarkMild => write!(f, "Dark Mild"),
            Self::DunklesWeissbier => write!(f, "Dunkles Weissbier"),
            Self::Marzen => write!(f, "Märzen"),
            Self::Weissbier => write!(f, "Weissbier"),
            Self::LeichtesWeizen => write!(f, "Leichtes Weizen"),
            Self::IrishRedAle => write!(f, "Irish Red Ale"),
            Self::BelgianDarkStrongAle => write!(f, "Belgian Dark Strong Ale"),
        }
    }
}

impl Style {
    /// Origin
    #[must_use]
    pub fn origin(&self) -> StyleOrigin {
        match *self {
            Self::AmericanLightLager => StyleOrigin::American,
            Self::AmericanLager => StyleOrigin::American,
            Self::CreamAle => StyleOrigin::American,
            Self::AmericanWheatBeer => StyleOrigin::American,
            Self::OrdinaryBitter => StyleOrigin::British,
            Self::BestBitter => StyleOrigin::British,
            Self::StrongBitter => StyleOrigin::British,
            Self::DarkMild => StyleOrigin::British,
            Self::DunklesWeissbier => StyleOrigin::German,
            Self::Marzen => StyleOrigin::German,
            Self::Weissbier => StyleOrigin::German,
            Self::LeichtesWeizen => StyleOrigin::German,
            Self::IrishRedAle => StyleOrigin::Irish,
            Self::BelgianDarkStrongAle => StyleOrigin::Belgian,
        }
    }

    /// Fermentation
    #[must_use]
    pub fn fermentation(&self) -> Fermentation {
        match *self {
            Self::AmericanLightLager => Fermentation::Lager,
            Self::AmericanLager => Fermentation::Lager,
            Self::CreamAle => Fermentation::Either,
            Self::AmericanWheatBeer => Fermentation::Either,
            Self::OrdinaryBitter => Fermentation::Ale,
            Self::BestBitter => Fermentation::Ale,
            Self::StrongBitter => Fermentation::Ale,
            Self::DarkMild => Fermentation::Ale,
            Self::DunklesWeissbier => Fermentation::Ale,
            Self::Marzen => Fermentation::Lager,
            Self::Weissbier => Fermentation::Ale,
            Self::LeichtesWeizen => Fermentation::Ale,
            Self::IrishRedAle => Fermentation::Ale,
            Self::BelgianDarkStrongAle => Fermentation::Ale,
        }
    }

    /// Conditioning
    #[must_use]
    pub fn conditioning(&self) -> Conditioning {
        match *self {
            Self::AmericanLightLager => Conditioning::Lagered,
            Self::AmericanLager => Conditioning::Lagered,
            Self::CreamAle => Conditioning::None,
            Self::AmericanWheatBeer => Conditioning::None,
            Self::OrdinaryBitter => Conditioning::None,
            Self::BestBitter => Conditioning::None,
            Self::StrongBitter => Conditioning::None,
            Self::DarkMild => Conditioning::None,
            Self::DunklesWeissbier => Conditioning::None,
            Self::Marzen => Conditioning::Lagered,
            Self::Weissbier => Conditioning::None,
            Self::LeichtesWeizen => Conditioning::None,
            Self::IrishRedAle => Conditioning::None,
            Self::BelgianDarkStrongAle => Conditioning::None,
        }
    }

    /// BJCP style guideline overall impression
    #[must_use]
    pub fn overall_impression_bjcp(&self) -> &str {
        match *self {
            Self::AmericanLightLager => {
                "A highly carbonated, very light-bodied, \
                 nearly flavorless lager designed to be consumed very cold. Very \
                 refreshing and thirst-quenching."
            }
            Self::AmericanLager => {
                "A very pale, highly-carbonated, light-bodied, \
                 well-attenuated lager with a very neutral flavor profile \
                 and low bitterness. Served very cold, it can be a very refreshing \
                 and thirst-quenching drink."
            }
            Self::CreamAle => {
                "A clean, well-attenuated, highly \
                 carbonated, flavorful American “lawnmower” beer. Easily \
                 drinkable, smooth, and refreshing, with more character than\
                 typical American lagers, yet still subtle and restrained."
            }
            Self::AmericanWheatBeer => {
                "A pale, refreshing grainy, doughy, or \
                 bready wheat beer with a clean fermentation profile and a \
                 variable hop character and bitterness. Its lighter body and \
                 higher carbonation contribute to its easy-drinking nature."
            }
            Self::OrdinaryBitter => {
                "Low gravity, alcohol, and carbonation \
                 make this an easy-drinking session beer. The malt profile can \
                 vary in flavor and intensity, but should never override the \
                 overall bitter impression. Drinkability is a critical component \
                 of the style."
            }
            Self::BestBitter => {
                "A flavorful, yet refreshing, session beer. \
                 Some examples can be more malt balanced, but this should not \
                 override the overall bitter impression. Drinkability is a critical \
                 component of the style."
            }
            Self::StrongBitter => {
                "An average-strength to moderately-strong British bitter ale. \
                 The balance may vary between fairly \
                 even between malt and hops to somewhat bitter. Drinkability is \
                 a critical component of the style. A rather broad style that \
                 allows for considerable interpretation by the brewer."
            }
            Self::DarkMild => {
                "A dark, low-gravity, malt-focused British session ale readily \
                 suited to drinking in quantity. Refreshing, yet flavorful for \
                 its strength, with a wide range of dark malt or dark sugar \
                 expression."
            }
            Self::DunklesWeissbier => {
                "A moderately dark German wheat beer \
                 with a distinctive banana-and-clove weizen yeast fermentation \
                 profile, supported by a toasted bread or caramel malt flavor. \
                 Highly carbonated and refreshing, with a creamy, fluffy texture \
                 and light finish."
            }
            Self::Marzen => {
                "An amber, malty German lager with a clean, rich, toasty, bready \
                 malt flavor, restrained bitterness, and a well-attenuated finish. \
                 The overall malt impression is soft, elegant, and complex, with a \
                 rich malty aftertaste that is never cloying or heavy."
            }
            Self::Weissbier => {
                "A pale, refreshing, lightly-hopped German wheat beer with high \
                 carbonation, dry finish, fluffy mouthfeel, and a distinctive \
                 banana-and-clove weizen yeast fermentation profile."
            }
            Self::LeichtesWeizen => "Light version of Hefeweisen.",
            Self::IrishRedAle => {
                "An easy-drinking pint, often with subtle flavors. Slightly malty \
                 in the balance sometimes with an initial soft toffee or caramel \
                 sweetness, a slightly grainy-biscuity palate, and a touch of \
                 roasted dryness in the finish. Some versions can emphasize the \
                 caramel and sweetness more, while others will favor the grainy \
                 palate and roasted dryness."
            }
            Self::BelgianDarkStrongAle => "(not listed with BJCP 2021)",
        }
    }

    /// Ranges of original gravity for the style, BJCP then BA
    #[must_use]
    pub fn original_gravity_ranges(&self) -> &[Range<SpecificGravity>] {
        match *self {
            Self::AmericanLightLager => &[SpecificGravity(1.028)..SpecificGravity(1.040)],
            Self::AmericanLager => &[SpecificGravity(1.040)..SpecificGravity(1.050)],
            Self::CreamAle => &[SpecificGravity(1.042)..SpecificGravity(1.055)],
            Self::AmericanWheatBeer => &[SpecificGravity(1.040)..SpecificGravity(1.055)],
            Self::OrdinaryBitter => &[SpecificGravity(1.030)..SpecificGravity(1.039)],
            Self::BestBitter => &[SpecificGravity(1.040)..SpecificGravity(1.048)],
            Self::StrongBitter => &[SpecificGravity(1.048)..SpecificGravity(1.060)],
            Self::DarkMild => &[
                SpecificGravity(1.030)..SpecificGravity(1.038), // BJCP
            ],
            Self::DunklesWeissbier => &[
                SpecificGravity(1.044)..SpecificGravity(1.057), // BJCP
                SpecificGravity(1.048)..SpecificGravity(1.056), // BA
            ],
            Self::Marzen => &[
                SpecificGravity(1.054)..SpecificGravity(1.060), // BJCP
                SpecificGravity(1.052)..SpecificGravity(1.057), // BA
            ],
            Self::Weissbier => &[
                SpecificGravity(1.044)..SpecificGravity(1.053), // BJCP
                SpecificGravity(1.047)..SpecificGravity(1.056), // BA
            ],
            Self::LeichtesWeizen => &[
                SpecificGravity(1.028)..SpecificGravity(1.044), // BA
            ],
            Self::IrishRedAle => &[
                SpecificGravity(1.036)..SpecificGravity(1.046), // BJCP
                SpecificGravity(1.040)..SpecificGravity(1.048), // BA
            ],
            Self::BelgianDarkStrongAle => &[
                SpecificGravity(1.092)..SpecificGravity(1.120), // BA
            ],
        }
    }

    /// Range of original gravity for the style
    #[must_use]
    pub fn original_gravity_range(&self) -> Range<SpecificGravity> {
        crate::union_ranges(self.original_gravity_ranges())
    }

    /// Ranges of final gravity for the style, BJCP then BA
    #[must_use]
    pub fn final_gravity_ranges(&self) -> &[Range<SpecificGravity>] {
        match *self {
            Self::AmericanLightLager => &[SpecificGravity(0.998)..SpecificGravity(1.008)],
            Self::AmericanLager => &[SpecificGravity(1.004)..SpecificGravity(1.010)],
            Self::CreamAle => &[SpecificGravity(1.006)..SpecificGravity(1.012)],
            Self::AmericanWheatBeer => &[SpecificGravity(1.008)..SpecificGravity(1.013)],
            Self::OrdinaryBitter => &[SpecificGravity(1.007)..SpecificGravity(1.011)],
            Self::BestBitter => &[SpecificGravity(1.008)..SpecificGravity(1.012)],
            Self::StrongBitter => &[SpecificGravity(1.010)..SpecificGravity(1.016)],
            Self::DarkMild => &[
                SpecificGravity(1.008)..SpecificGravity(1.013), // BJCP
            ],
            Self::DunklesWeissbier => &[
                SpecificGravity(1.008)..SpecificGravity(1.014), // BJCP
                SpecificGravity(1.008)..SpecificGravity(1.016), // BA
            ],
            Self::Marzen => &[
                SpecificGravity(1.010)..SpecificGravity(1.014), // BJCP
                SpecificGravity(1.012)..SpecificGravity(1.020), // BA
            ],
            Self::Weissbier => &[
                SpecificGravity(1.008)..SpecificGravity(1.014), // BJCP
                SpecificGravity(1.008)..SpecificGravity(1.016), // BA
            ],
            Self::LeichtesWeizen => &[
                SpecificGravity(1.004)..SpecificGravity(1.008), // BA
            ],
            Self::IrishRedAle => &[
                SpecificGravity(1.010)..SpecificGravity(1.014), // BJCP
                SpecificGravity(1.010)..SpecificGravity(1.014), // BA
            ],
            Self::BelgianDarkStrongAle => &[
                SpecificGravity(1.014)..SpecificGravity(1.020), // BA
            ],
        }
    }

    /// Range of final gravity for the style
    #[must_use]
    pub fn final_gravity_range(&self) -> Range<SpecificGravity> {
        crate::union_ranges(self.final_gravity_ranges())
    }

    /// Ranges of ABV for the style, BJCP then BA
    #[must_use]
    pub fn abv_ranges(&self) -> Vec<Range<Abv>> {
        let ranges = match *self {
            Self::AmericanLightLager => vec![2.8..4.2],
            Self::AmericanLager => vec![4.2..5.3],
            Self::CreamAle => vec![4.2..5.6],
            Self::AmericanWheatBeer => vec![4.0..5.5],
            Self::OrdinaryBitter => vec![3.2..3.8],
            Self::BestBitter => vec![3.8..4.6],
            Self::StrongBitter => vec![4.6..6.2],
            Self::DarkMild => vec![3.0..3.8],
            Self::DunklesWeissbier => vec![4.3..5.6, 4.8..5.4],
            Self::Marzen => vec![5.6..6.3, 5.1..6.0],
            Self::Weissbier => vec![4.3..5.6, 4.9..5.6],
            Self::LeichtesWeizen => vec![2.5..3.5],
            Self::IrishRedAle => vec![3.8..5.0, 4.0..4.8],
            Self::BelgianDarkStrongAle => vec![10.0..14.2],
        };

        ranges
            .iter()
            .map(|range| Percent(range.start).into()..Percent(range.end).into())
            .collect()
    }

    /// Range of ABV for the style
    #[must_use]
    pub fn abv_range(&self) -> Range<Abv> {
        crate::union_ranges(&self.abv_ranges())
    }

    /// Ranges of bitterness (IBU) for the style, BJCP then BA
    #[must_use]
    pub fn bitterness_ranges(&self) -> &[Range<Ibu>] {
        match *self {
            Self::AmericanLightLager => &[Ibu(8.0)..Ibu(12.0)],
            Self::AmericanLager => &[Ibu(8.0)..Ibu(18.0)],
            Self::CreamAle => &[Ibu(8.0)..Ibu(20.0)],
            Self::AmericanWheatBeer => &[Ibu(15.0)..Ibu(30.0)],
            Self::OrdinaryBitter => &[Ibu(25.0)..Ibu(35.0)],
            Self::BestBitter => &[Ibu(25.0)..Ibu(40.0)],
            Self::StrongBitter => &[Ibu(30.0)..Ibu(50.0)],
            Self::DarkMild => &[Ibu(10.0)..Ibu(25.0)],
            Self::DunklesWeissbier => &[Ibu(10.0)..Ibu(18.0), Ibu(10.0)..Ibu(15.0)],
            Self::Marzen => &[Ibu(18.0)..Ibu(24.0), Ibu(18.0)..Ibu(25.0)],
            Self::Weissbier => &[Ibu(8.0)..Ibu(15.0), Ibu(10.0)..Ibu(15.0)],
            Self::LeichtesWeizen => &[Ibu(10.0)..Ibu(15.0)],
            Self::IrishRedAle => &[Ibu(18.0)..Ibu(28.0), Ibu(20.0)..Ibu(28.0)],
            Self::BelgianDarkStrongAle => &[Ibu(25.0)..Ibu(50.0)],
        }
    }

    /// Range of IBU for the style
    #[must_use]
    pub fn bitterness_range(&self) -> Range<Ibu> {
        crate::union_ranges(self.bitterness_ranges())
    }

    /// Ranges of SRM for the style, BJCP then BA
    #[must_use]
    pub fn color_ranges(&self) -> &[Range<Srm>] {
        match *self {
            Self::AmericanLightLager => &[Srm(2.0)..Srm(3.0)],
            Self::AmericanLager => &[Srm(2.0)..Srm(3.5)],
            Self::CreamAle => &[Srm(2.0)..Srm(5.0)],
            Self::AmericanWheatBeer => &[Srm(3.0)..Srm(6.0)],
            Self::OrdinaryBitter => &[Srm(8.0)..Srm(14.0)],
            Self::BestBitter => &[Srm(8.0)..Srm(16.0)],
            Self::StrongBitter => &[Srm(8.0)..Srm(18.0)],
            Self::DarkMild => &[Srm(14.0)..Srm(25.0)],
            Self::DunklesWeissbier => &[Srm(14.0)..Srm(23.0), Srm(10.0)..Srm(25.0)],
            Self::Marzen => &[Srm(8.0)..Srm(17.0), Srm(4.0)..Srm(15.0)],
            Self::Weissbier => &[Srm(2.0)..Srm(6.0), Srm(3.0)..Srm(9.0)],
            Self::LeichtesWeizen => &[Srm(3.5)..Srm(15.0)],
            Self::IrishRedAle => &[Srm(9.0)..Srm(14.0), Srm(11.0)..Srm(18.0)],
            Self::BelgianDarkStrongAle => &[Srm(16.0)..Srm(36.0)],
        }
    }

    /// Range of SRM for the style
    #[must_use]
    pub fn color_range(&self) -> Range<Srm> {
        crate::union_ranges(self.color_ranges())
    }

    /// Carbonation volume
    #[must_use]
    pub fn carbonation_volume(&self) -> f32 {
        // 1.5 - 2.0:  British cask ale, barleywine
        // 2.0 - 2.5:  American and British ales
        // 2.5 - 3.0:  Lagers, wheat beers
        // 3.0 - 4.0:  Belgian ales, champagne-like beers
        // Only use heavy bottles if going >= 3.0.
        //
        // british ale 1.8  1.5-2.0
        // fruit lambic 3.0-4.5
        // porter/stout 2.2  1.7-2.3
        // american ale 2.5
        // lager 2.6
        // euro lager 2.2 - 2.7
        // wheat beer 2.8, or 3.3-4.5
        // lambic 2.4-2.8
        // belgian ale 3.0  1.9-2.4

        match *self {
            Self::AmericanLightLager => 4.0,
            Self::AmericanLager => 4.0,
            Self::CreamAle => 3.2,
            Self::AmericanWheatBeer => 2.8,
            Self::OrdinaryBitter => 1.8,
            Self::BestBitter => 1.8,
            Self::StrongBitter => 1.8,
            Self::DarkMild => 1.8,
            Self::DunklesWeissbier => 3.5,
            Self::Marzen => 2.7,
            Self::Weissbier => 3.3,
            Self::LeichtesWeizen => 4.0,
            Self::IrishRedAle => 1.9,
            Self::BelgianDarkStrongAle => 3.0,
        }
    }

    /// Is a wheat beer
    #[must_use]
    #[allow(clippy::match_like_matches_macro)]
    pub fn is_a_wheat_beer(&self) -> bool {
        match *self {
            Self::AmericanWheatBeer => true,
            Self::Weissbier => true,
            Self::DunklesWeissbier => true,
            Self::LeichtesWeizen => true,
            _ => false,
        }
    }

    /// Recommended boil length (minutes)
    ///
    /// Pilsner malt has more DMS, indicating longer boil to drive it off.
    ///
    /// Clear beers (lagers) benefit from more hot-break of longer boils.
    ///
    /// You can get more bitterness from the same hops from longer boils
    /// but this is easily done with more hops in a shorter boil.
    ///
    /// Longer boils create more melanoidins.
    /// Longer boils ensure protein coagulation.
    #[must_use]
    pub fn recommended_boil_length(&self) -> Minutes {
        if self.fermentation() == Fermentation::Lager {
            // Drive off that DMS, and get more hot-break benefit
            Minutes(80)
        } else if self.is_a_wheat_beer() {
            // Ensure protein coagulation and create more melanoidin
            Minutes(75)
        } else {
            Minutes(50)
        }
    }

    /// Recommended conditioning time
    #[must_use]
    pub fn recommended_conditioning_time(&self) -> Days {
        // TODO: this could be better
        if self.conditioning() == Conditioning::Lagered {
            Days(7 * 7) // 6-8 weeks
        } else {
            Days(14) // 2 weeks
        }
    }
}
