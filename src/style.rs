use crate::units::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Range;

/// Lager style
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LagerStyle {
    /// European
    European,

    /// American
    American,
}

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
    /// Dark Mild
    DarkMild,

    /// Dunkelweizen
    Dunkelweizen,

    /// Märzen is a malty amber lager with medium-body, low esters, and a clean finish.
    /// Märzen is brewed in Germany traditionally in March, lagered through the
    /// summer, and served at Oktoberfest (along with Festbier which is now more
    /// common).
    Marzen,

    /// Hefeweizen, Weissbier
    Hefeweizen,

    /// Leichtesweizen, a light version of Weissbier
    LeichtesWeizen,

    /// Irish Red Ale
    IrishRedAle,

    /// Belgian Qudrupel, Grand Cru, Belgian Dark Strong
    BelgianQuadrupel,
}

impl Style {
    /// BJCP style guideline overall impression
    #[must_use]
    pub fn overall_impression_bjcp(&self) -> &str {
        match *self {
            Style::DarkMild => {
                "A dark, low-gravity, malt-focused British session ale readily \
                 suited to drinking in quantity. Refreshing, yet flavorful for \
                 its strength, with a wide range of dark malt or dark sugar \
                 expression."
            }
            Style::Dunkelweizen => {
                "A moderately dark German wheat beer \
                 with a distinctive banana-and-clove weizen yeast fermentation \
                 profile, supported by a toasted bread or caramel malt flavor. \
                 Highly carbonated and refreshing, with a creamy, fluffy texture \
                 and light finish."
            }
            Style::Marzen => {
                "An amber, malty German lager with a clean, rich, toasty, bready \
                 malt flavor, restrained bitterness, and a well-attenuated finish. \
                 The overall malt impression is soft, elegant, and complex, with a \
                 rich malty aftertaste that is never cloying or heavy."
            }
            Style::Hefeweizen => {
                "A pale, refreshing, lightly-hopped German wheat beer with high \
                 carbonation, dry finish, fluffy mouthfeel, and a distinctive \
                 banana-and-clove weizen yeast fermentation profile."
            }
            Style::LeichtesWeizen => "Light version of Hefeweisen.",
            Style::IrishRedAle => {
                "An easy-drinking pint, often with subtle flavors. Slightly malty \
                 in the balance sometimes with an initial soft toffee or caramel \
                 sweetness, a slightly grainy-biscuity palate, and a touch of \
                 roasted dryness in the finish. Some versions can emphasize the \
                 caramel and sweetness more, while others will favor the grainy \
                 palate and roasted dryness."
            }
            Style::BelgianQuadrupel => "(not listed with BJCP 2021)",
        }
    }

    /// Ranges of original gravity for the style, BJCP then BA
    #[must_use]
    pub fn original_gravity_ranges(&self) -> &[Range<SpecificGravity>] {
        match *self {
            Style::DarkMild => &[
                SpecificGravity(1.030)..SpecificGravity(1.038), // BJCP
            ],
            Style::Dunkelweizen => &[
                SpecificGravity(1.044)..SpecificGravity(1.057), // BJCP
                SpecificGravity(1.048)..SpecificGravity(1.056), // BA
            ],
            Style::Marzen => &[
                SpecificGravity(1.054)..SpecificGravity(1.060), // BJCP
                SpecificGravity(1.052)..SpecificGravity(1.057), // BA
            ],
            Style::Hefeweizen => &[
                SpecificGravity(1.044)..SpecificGravity(1.053), // BJCP
                SpecificGravity(1.047)..SpecificGravity(1.056), // BA
            ],
            Style::LeichtesWeizen => &[
                SpecificGravity(1.028)..SpecificGravity(1.044), // BA
            ],
            Style::IrishRedAle => &[
                SpecificGravity(1.036)..SpecificGravity(1.046), // BJCP
                SpecificGravity(1.040)..SpecificGravity(1.048), // BA
            ],
            Style::BelgianQuadrupel => &[
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
            Style::DarkMild => &[
                SpecificGravity(1.008)..SpecificGravity(1.013), // BJCP
            ],
            Style::Dunkelweizen => &[
                SpecificGravity(1.008)..SpecificGravity(1.014), // BJCP
                SpecificGravity(1.008)..SpecificGravity(1.016), // BA
            ],
            Style::Marzen => &[
                SpecificGravity(1.010)..SpecificGravity(1.014), // BJCP
                SpecificGravity(1.012)..SpecificGravity(1.020), // BA
            ],
            Style::Hefeweizen => &[
                SpecificGravity(1.008)..SpecificGravity(1.014), // BJCP
                SpecificGravity(1.008)..SpecificGravity(1.016), // BA
            ],
            Style::LeichtesWeizen => &[
                SpecificGravity(1.004)..SpecificGravity(1.008), // BA
            ],
            Style::IrishRedAle => &[
                SpecificGravity(1.010)..SpecificGravity(1.014), // BJCP
                SpecificGravity(1.010)..SpecificGravity(1.014), // BA
            ],
            Style::BelgianQuadrupel => &[
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
            Style::DarkMild => vec![3.0..3.8],
            Style::Dunkelweizen => vec![4.3..5.6, 4.8..5.4],
            Style::Marzen => vec![5.6..6.3, 5.1..6.0],
            Style::Hefeweizen => vec![4.3..5.6, 4.9..5.6],
            Style::LeichtesWeizen => vec![2.5..3.5],
            Style::IrishRedAle => vec![3.8..5.0, 4.0..4.8],
            Style::BelgianQuadrupel => vec![10.0..14.2],
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
            Style::DarkMild => &[Ibu(10.0)..Ibu(25.0)],
            Style::Dunkelweizen => &[Ibu(10.0)..Ibu(18.0), Ibu(10.0)..Ibu(15.0)],
            Style::Marzen => &[Ibu(18.0)..Ibu(24.0), Ibu(18.0)..Ibu(25.0)],
            Style::Hefeweizen => &[Ibu(8.0)..Ibu(15.0), Ibu(10.0)..Ibu(15.0)],
            Style::LeichtesWeizen => &[Ibu(10.0)..Ibu(15.0)],
            Style::IrishRedAle => &[Ibu(18.0)..Ibu(28.0), Ibu(20.0)..Ibu(28.0)],
            Style::BelgianQuadrupel => &[Ibu(25.0)..Ibu(50.0)],
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
            Style::DarkMild => &[Srm(14.0)..Srm(25.0)],
            Style::Dunkelweizen => &[Srm(14.0)..Srm(23.0), Srm(10.0)..Srm(25.0)],
            Style::Marzen => &[Srm(8.0)..Srm(17.0), Srm(4.0)..Srm(15.0)],
            Style::Hefeweizen => &[Srm(2.0)..Srm(6.0), Srm(3.0)..Srm(9.0)],
            Style::LeichtesWeizen => &[Srm(3.5)..Srm(15.0)],
            Style::IrishRedAle => &[Srm(9.0)..Srm(14.0), Srm(11.0)..Srm(18.0)],
            Style::BelgianQuadrupel => &[Srm(16.0)..Srm(36.0)],
        }
    }

    /// Range of SRM for the style
    #[must_use]
    pub fn color_range(&self) -> Range<Srm> {
        crate::union_ranges(self.color_ranges())
    }

    /// Yeast pitching rate (cells per mL per Plato)
    #[must_use]
    pub fn yeast_pitching_rate(&self) -> u64 {
        if self.is_a_wheat_beer() {
            // 600k per ml per plato for Weissbier as
            // stressed growth promotes esters
            600_000
        } else if self.is_a_lager() {
            // 1.5m per ml per plato for lagers
            1_500_000
        } else {
            // 750k to 1m per ml per plato
            750_000
        }
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
            Style::DarkMild => 1.8,
            Self::Dunkelweizen => 3.5,
            Self::Marzen => 2.7,
            Self::Hefeweizen => 3.3,
            Self::LeichtesWeizen => 4.0,
            Self::IrishRedAle => 1.9,
            Self::BelgianQuadrupel => 3.0,
        }
    }

    /// Lager style
    #[must_use]
    #[allow(clippy::match_like_matches_macro)]
    pub fn lager_style(&self) -> Option<LagerStyle> {
        match *self {
            Self::Marzen => Some(LagerStyle::European),
            _ => None,
        }
    }

    /// Is a lager
    #[must_use]
    #[allow(clippy::match_like_matches_macro)]
    pub fn is_a_lager(&self) -> bool {
        self.lager_style().is_some()
    }

    /// Is a wheat beer
    #[must_use]
    #[allow(clippy::match_like_matches_macro)]
    pub fn is_a_wheat_beer(&self) -> bool {
        match *self {
            Self::Dunkelweizen => true,
            Self::Hefeweizen => true,
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
        if self.is_a_lager() {
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
        if self.is_a_lager() {
            Days(7 * 7) // 6-8 weeks
        } else {
            Days(14) // 2 weeks
        }
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Style::DarkMild => write!(f, "Dark Mild"),
            Style::Dunkelweizen => write!(f, "Dunkelweizen"),
            Style::Marzen => write!(f, "Märzen"),
            Style::Hefeweizen => write!(f, "Hefeweissen/Weissbier"),
            Style::LeichtesWeizen => write!(f, "Leichtes Weizen"),
            Style::IrishRedAle => write!(f, "Irish Red Ale"),
            Style::BelgianQuadrupel => write!(f, "Belgian Quadrupel"),
        }
    }
}
