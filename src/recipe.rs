use crate::mash::MashRest;
use crate::prelude::*;
use crate::process::{Packaging, Process};
use crate::style::Style;
use serde::{Deserialize, Serialize};
use std::fmt::Write;

/* Notes about water volumes

Mash:
+ Strike Volume                          = strike_volume()
+ Mash Infusion
(+ Mash Infusion)                        = pre_absorption_water()
- Absorption                             = process.pre_sparge_volume()
+ Sparge                                 = process.pre_boil_volume()

Boil:
- Evaporation                            = process.post_boil_volume()
- Kettle Losses                          = process.pre_dilution_volume()

Ferment:
+ Partial Boil Dilution                  = process.ferment_volume
- Ferment Loss Percent                   = process.post_ferment_volume()
 */

/// A recipe for beer
///
/// This struct, along with the process specified within it, does all
/// the calculations for making the beer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    /// The name of the recipe
    pub name: String,

    /// The style of beer
    pub style: Style,

    /// Process specification
    pub process: Process,

    /// The mash rests.
    ///
    /// For single infusion, just list that one.
    pub mash_rests: Vec<MashRest>,

    /// The malts that will be mashed, in proportion to all malts and sugars
    /// by weight. The actual weights are calculated.
    pub malts: Vec<MaltProportion>,

    /// The sugars added after mashing, in proportion to all malts and sugars
    /// by weight. The actual weights are calculated.
    pub sugars: Vec<SugarProportion>,

    /// The hops additions added during the boil
    pub hops: Vec<HopsDose>,

    /// The yeast to ferment with
    pub yeast: Yeast,

    /// Whether or not to use a fining agent
    pub fining_desired: bool,

    /// The temperature to ferment at
    pub ferment_temperature: Celsius,

    /// Force the malt and sugar proportions to be higher or lower
    /// than what would put the O.G. in the middle of the style range.
    /// It will just multiply the amounts of everything by this number.
    pub grain_bill_adjustment: Option<f32>,
}

// Strike water volume can be computed by multipling the weight
// of grain by desired mash thickness (1.0 - 1.5 quarts per pound)

impl Recipe {
    /// Salt doses
    #[must_use]
    pub fn salt_doses(&self, liters: Option<Liters>) -> Vec<SaltDose> {
        let mut output: Vec<SaltDose> = Vec::new();

        let water_liters = match liters {
            Some(l) => l,
            None => self.total_water(),
        };

        for salt_concentration in &self.process.water_salts {
            let mg = Milligrams(water_liters.0 * salt_concentration.ppm.0);
            output.push(SaltDose {
                salt: salt_concentration.salt,
                mg,
            });
        }

        output
    }

    /// Acid doses
    #[must_use]
    pub fn acid_doses(&self, liters: Option<Liters>) -> Vec<AcidDose> {
        let mut output: Vec<AcidDose> = Vec::new();

        let water_liters = match liters {
            Some(l) => l,
            None => self.total_water(),
        };

        for acid_concentration in &self.process.water_acids {
            let mg = Milligrams(water_liters.0 * acid_concentration.ppm.0);
            output.push(AcidDose {
                acid: acid_concentration.acid,
                mg,
            });
        }

        output
    }

    /// Strike water additions string
    #[must_use]
    pub fn water_doses(&self) -> String {
        let salt_doses = self.salt_doses(None);
        let acid_doses = self.acid_doses(None);
        let mut output: String = String::new();
        for salt_dose in &salt_doses {
            writeln!(
                output,
                "Add in {} of {} to total water.",
                salt_dose.mg, salt_dose.salt
            )
            .unwrap();
        }
        for acid_dose in &acid_doses {
            writeln!(
                output,
                "Add in {} of {} to total water.",
                acid_dose.mg, acid_dose.acid
            )
            .unwrap();
        }
        output
    }

    /// The boil time
    #[must_use]
    pub fn boil_time(&self) -> Minutes {
        self.style.recommended_boil_length()
    }

    /// The mount of water that evaporates during the boil
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn boil_evaporation(&self) -> Liters {
        self.process.boil_evaporation_per_hour / 60.0 * self.boil_time().0 as f32
    }

    /// The pre-boil volume
    #[must_use]
    pub fn pre_boil_volume(&self) -> Liters {
        self.process.post_boil_volume() + self.boil_evaporation()
    }

    /// The pre-sparge volume
    #[must_use]
    pub fn pre_sparge_volume(&self) -> Liters {
        self.pre_boil_volume() - self.process.sparge_volume
    }

    /// Strike temperature
    #[must_use]
    pub fn strike_temperature(&self) -> Celsius {
        crate::mash::strike_water_temp(
            self.strike_volume(),
            self.grain_weight(),
            self.process.room_temperature,
            self.mash_rests[0].target_temperature,
        )
    }

    /// Strike volume
    #[must_use]
    pub fn strike_volume(&self) -> Liters {
        // Use a hardcoded correct value until we get this working.
        // Liters(4.627)

        // To compute the strike volume, we have to calculate backwards.
        // This is the only place we do this.  Everywhere else, we will
        // calculate forwards from this strike volume to double-check
        // our work.

        // start at the end of the mash with the grains still inside
        let mut current_water = self.pre_absorption_water();
        let mut current_temp: Option<Celsius> = None;

        // Iterate backwards through each mash rest
        for rest in self.mash_rests.clone().into_iter().rev() {
            if let Some(current_tmp) = current_temp {
                // Compute the mash step backwards
                let infusion = crate::mash::reverse_mash_infusion(
                    self.grain_weight(),
                    current_water,
                    rest.target_temperature,
                    current_tmp,
                    self.process.infusion_temperature,
                );

                // Subtract that much water
                current_water = current_water - infusion;
            }

            // Update the current temperature
            current_temp = Some(rest.target_temperature);
        }

        current_water
    }

    /// Mash steps
    ///
    /// This gives you the volume of infusion water for each step.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn mash_infusions(&self) -> Vec<Liters> {
        let mut infusions: Vec<Liters> = Vec::new();

        let mut current_water = self.strike_volume();
        let mut current_temp: Option<Celsius> = None;

        for rest in &self.mash_rests {
            if current_temp.is_none() {
                current_temp = Some(rest.target_temperature);
                continue;
            }
            let cur_tmp = current_temp.unwrap();

            let infusion = crate::mash::mash_infusion(
                self.grain_weight(),
                current_water,
                cur_tmp,
                rest.target_temperature,
                self.process.infusion_temperature,
            );

            infusions.push(infusion);
            current_water = current_water + infusion;
            current_temp = Some(rest.target_temperature);
        }

        infusions
    }

    /// The water added throughout the mash, before losses from grain
    /// absorption
    #[must_use]
    pub fn pre_absorption_water(&self) -> Liters {
        self.pre_sparge_volume() + self.water_absorption()
    }

    /// The total amount of water input during the process
    #[must_use]
    pub fn total_water(&self) -> Liters {
        self.pre_absorption_water() // strike plus infusions
            + self.process.sparge_volume
            + self.process.partial_boil_dilution
    }

    // compute a multipler from proportions to weights
    // in order to hit the target original gravity of the style.
    fn grain_bill_multiplier(&self) -> f32 {
        let malt_doses: Vec<MaltDose> = self
            .malts
            .iter()
            .map(|proportion| MaltDose {
                malt: proportion.malt,
                weight: Kilograms(proportion.proportion), // as if 1.0 is 1.0 kg.
            })
            .collect();

        let sugar_doses: Vec<SugarDose> = self
            .sugars
            .iter()
            .map(|proportion| SugarDose {
                sugar: proportion.sugar,
                weight: Kilograms(proportion.proportion), // as if 1.0 is 1.0 kg.
            })
            .collect();

        let sg = self.specific_gravity(
            self.process.ferment_volume.into(),
            &malt_doses,
            &sugar_doses,
        );
        let actual_points = sg.0 - 1.0;

        let ogr = self.style.original_gravity_range();
        let ideal_points = f32::midpoint(ogr.start.0, ogr.end.0) - 1.0;

        let adjustment = self.grain_bill_adjustment.unwrap_or(1.0);

        adjustment * ideal_points / actual_points
    }

    /// Malt doses
    #[must_use]
    pub fn malt_doses(&self) -> Vec<MaltDose> {
        let multiplier = self.grain_bill_multiplier();

        self.malts
            .iter()
            .map(|proportion| MaltDose {
                malt: proportion.malt,
                weight: Kilograms(proportion.proportion * multiplier),
            })
            .collect()
    }

    /// Sugar doses
    #[must_use]
    pub fn sugar_doses(&self) -> Vec<SugarDose> {
        let multiplier = self.grain_bill_multiplier();

        self.sugars
            .iter()
            .map(|proportion| SugarDose {
                sugar: proportion.sugar,
                weight: Kilograms(proportion.proportion * multiplier),
            })
            .collect()
    }

    /// The weight of the malts in the mash
    #[must_use]
    pub fn grain_weight(&self) -> Kilograms {
        self.malt_doses().iter().map(|dose| dose.weight).sum()
    }

    /// The weight of all the fermentables
    #[must_use]
    pub fn fermentables_weight(&self) -> Kilograms {
        Kilograms(
            self.malt_doses()
                .iter()
                .map(|dose| dose.weight.0)
                .sum::<f32>()
                + self
                    .sugar_doses()
                    .iter()
                    .map(|dose| dose.weight.0)
                    .sum::<f32>(),
        )
    }

    /// The weight of the malts in the mash
    #[must_use]
    pub fn grain_bill_string(&self) -> String {
        let mut output: String = String::new();

        for malt_dose in self.malt_doses() {
            let percent = 100.0 * malt_dose.weight.0 / self.fermentables_weight().0;
            writeln!(
                output,
                "{}, {:04.1}%  {}",
                malt_dose.weight, percent, malt_dose.malt
            )
            .unwrap();
        }

        for sugar_dose in self.sugar_doses() {
            let percent = 100.0 * sugar_dose.weight.0 / self.fermentables_weight().0;
            writeln!(
                output,
                "{} {:.1}% {}",
                sugar_dose.weight, percent, sugar_dose.sugar
            )
            .unwrap();
        }

        output
    }

    /// The water absorption of the malts
    #[must_use]
    pub fn water_absorption(&self) -> Liters {
        let grain_weight_lbs: Pounds = self.grain_weight().into();

        // We presume 0.15 gallons per pound absorption
        // But we could add a malt property that sets it's absorption
        // from  0.12 to 0.18 gallons per pound.
        Gallons(0.15 * grain_weight_lbs.0).into()
    }

    /// The pre-boil original gravity (OG) of the wort
    #[must_use]
    pub fn pre_boil_gravity(&self) -> SpecificGravity {
        let pre_boil_volume: Gallons = self.pre_boil_volume().into();
        self.specific_gravity(pre_boil_volume, &self.malt_doses(), &self.sugar_doses())
    }

    /// The post-boil original gravity (OG) of the wort
    #[must_use]
    pub fn original_gravity(&self) -> SpecificGravity {
        self.specific_gravity(
            self.process.ferment_volume.into(),
            &self.malt_doses(),
            &self.sugar_doses(),
        )
    }

    fn specific_gravity(
        &self,
        volume: Gallons,
        malts: &[MaltDose],
        sugars: &[SugarDose],
    ) -> SpecificGravity {
        let mut points: f32 = 0.0;

        for malt_dose in malts {
            let pounds: Pounds = malt_dose.weight.into();
            let pts = malt_dose.malt.ppg() * pounds.0 * self.process.mash_efficiency;
            let points_added = pts / volume.0;

            points += points_added;
        }

        for sugar_dose in sugars {
            let pounds: Pounds = sugar_dose.weight.into();
            let pts = sugar_dose.sugar.ppg() * pounds.0;
            let points_added = pts / volume.0;

            points += points_added;
        }

        SpecificGravity(1.0 + points / 1000.0)
    }

    /// The amount of whirlfloc tablet to use
    #[must_use]
    pub fn whirlfloc_amount(&self) -> f32 {
        if self.fining_desired {
            1.0 * (self.process.ferment_volume.0 / 19.0)
        } else {
            0.0
        }
    }

    /// The estimated final gravity
    // TODO: this doesn't adjust for the presence of unfermentable
    //       sugars
    #[must_use]
    pub fn final_gravity(&self) -> SpecificGravity {
        let og = self.original_gravity();

        let mut attenuation = self.yeast.attenuation();

        let mut reduction_percent: f32 = 0.0;

        for malt_dose in &self.malt_doses() {
            if malt_dose.malt.category() == MaltCategory::Crystal {
                let pounds: Pounds = malt_dose.weight.into();
                let lovabond: Lovabond = malt_dose.malt.ebc().into();
                let gallons: Gallons = self.process.ferment_volume.into();
                // for 1 pound in a in a 5 gallon batch.
                let mult = (pounds.0 / gallons.0) / (1.0 / 5.0);

                // 10 lovabond -> 1%, 60 lovabond -> 3%
                reduction_percent += (0.0004 * lovabond.0 + 0.006) * mult;
            }

            // TODO: dextrin / carapils, chocolate roast, etc.
        }

        attenuation *= 1.0 - reduction_percent;

        SpecificGravity(og.0 - (og.0 - 1.0) * attenuation)
    }

    /// Estimated mash pH
    #[must_use]
    pub fn mash_ph(&self) -> Ph {
        // FIXME: use Certificate of Analysis of malt to get wort pH
        // (we hard coded 5.4 below), combine different malts somehow.

        let residual_alkalinity = self.process.water_profile.residual_alkalinity().0;

        let mut light_weight: f32 = 0.0;
        let mut dark_weight: f32 = 0.0;
        let mut crystal_weight: f32 = 0.0;
        let mut acidulated_weight: f32 = 0.0;

        for dose in &self.malt_doses() {
            match dose.malt.acid_category() {
                MaltAcidCategory::Light => light_weight += dose.weight.0,
                MaltAcidCategory::Dark => dark_weight += dose.weight.0,
                MaltAcidCategory::Crystal => crystal_weight += dose.weight.0,
                MaltAcidCategory::Acidulated => acidulated_weight += dose.weight.0,
                MaltAcidCategory::None => (),
            }
        }

        let total = self.grain_weight().0;

        let ph = 5.4 // FIXME, get this from malt Cert of Analysis, combine malts somehow
            + (residual_alkalinity/10.0) * 0.3 // each 10 units of RA add 0.3 pH
            - 100.0 * (light_weight / total) * 0.03
            - 100.0 * (dark_weight / total) * 0.05
            - 100.0 * (crystal_weight / total) * 0.025
            - 100.0 * (acidulated_weight / total) * 0.1;

        // TODO mash thickness (only changes by 0.05 for doubling/halving)
        // https://byo.com/mr-wizard/predicting-mash-ph/

        Ph(ph)
    }

    /// Estimated length of the fermentation
    ///
    /// Warning, this calculation is completely made up and not validated
    /// against anything.  But most calculators, websites, etc, just say
    /// something even less data-driven like "7 days".
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn fermentation_time(&self) -> Days {
        // Higher temperature ferments run faster
        // This fails however if you ferment at or above 35 C.
        let base = (35.0 - self.ferment_temperature.0) / 2.0;

        // If you ferment lower in the temp range of the given
        // yeast, you slow down the fermentation.
        let temp_range = self.yeast.temp_range();
        let temp_range_adjuster = (self.ferment_temperature.0 - temp_range.start.0)
            / (temp_range.end.0 - temp_range.start.0);
        let temp_range_multiplier = 1.2 - 0.4 * temp_range_adjuster;

        // TODO: only include if the style requires clearing?
        let floc_multiplier = match self.yeast.flocculation() {
            Flocculation::Low => 1.2,
            Flocculation::LowMedium => 1.1,
            Flocculation::Medium => 1.0,
            Flocculation::MediumHigh => 0.9,
            Flocculation::High => 0.8,
            Flocculation::VeryHigh => 0.75,
        };

        Days((base * temp_range_multiplier * floc_multiplier) as usize)
    }

    /// The length of time before the beer is ready
    #[must_use]
    pub fn time_until_done(&self) -> Days {
        let mut conditioning = self.style.recommended_conditioning_time();

        if let Packaging::Bottle(_, _) = self.process.packaging {
            // At least bottle conditioning time.
            // NOTE: Bottle conditioning counts as conditioning time.
            if conditioning < Days(14) {
                conditioning = Days(14);
            }
        }

        // 2 days of diacetyl rest
        self.fermentation_time() + Days(2) + conditioning
    }

    /// Volume history
    #[must_use]
    pub fn volume_history_string(&self) -> String {
        let mut output: String = String::new();
        let mut total = self.strike_volume();

        writeln!(
            output,
            "Strike +{}             TOTAL {total}",
            self.strike_volume()
        )
        .unwrap();

        for infusion in self.mash_infusions() {
            total = total + infusion;
            writeln!(output, "Mash Infusion: +{infusion}     TOTAL {total}").unwrap();
        }

        total = total - self.water_absorption();
        writeln!(
            output,
            "Absorption: -{}        TOTAL {total}",
            self.water_absorption()
        )
        .unwrap();

        total = total + self.process.sparge_volume;
        writeln!(
            output,
            "Sparge: +{}            TOTAL {total}",
            self.process.sparge_volume
        )
        .unwrap();

        total = total - self.boil_evaporation();
        writeln!(
            output,
            "Boil off: -{}          TOTAL {total}",
            self.boil_evaporation()
        )
        .unwrap();

        total = total - self.process.kettle_losses;
        writeln!(
            output,
            "Kettle Losses: -{}     TOTAL {total}",
            self.process.kettle_losses
        )
        .unwrap();

        total = total + self.process.partial_boil_dilution;
        writeln!(
            output,
            "Dilution: +{}          TOTAL {total}",
            self.process.partial_boil_dilution
        )
        .unwrap();

        total = total - self.process.ferment_losses();
        writeln!(
            output,
            "Ferment Loss: -{}      TOTAL {total}",
            self.process.ferment_losses()
        )
        .unwrap();

        output
    }

    /// Ingredient list
    #[must_use]
    pub fn ingredient_list_string(&self) -> String {
        let mut output: String = String::new();
        writeln!(output, "Total Water: {}", self.total_water()).unwrap();
        for malt in &self.malt_doses() {
            writeln!(output, "{} of {}", malt.weight, malt.malt).unwrap();
        }
        for sugar in &self.sugar_doses() {
            writeln!(output, "{} of {}", sugar.weight, sugar.sugar).unwrap();
        }
        for hops in &self.hops {
            writeln!(output, "{} of {}", hops.weight, hops.hops).unwrap();
        }
        writeln!(output, "Yeast: {}", self.yeast).unwrap();
        output
    }

    /// Malt bill
    #[must_use]
    pub fn malt_bill_string(&self) -> String {
        let mut output: String = String::new();
        for malt in &self.malt_doses() {
            writeln!(output, "{} of {}", malt.weight, malt.malt).unwrap();
        }
        write!(output, "\nTotal Malt Weight = {}", self.grain_weight()).unwrap();
        output
    }

    /// Get the pre-boil additions as a string
    #[must_use]
    pub fn pre_boil_additions_string(&self) -> String {
        if self.sugars.is_empty() {
            "None.".to_string()
        } else {
            let mut output: String = String::new();
            for sugar in &self.sugar_doses() {
                writeln!(output, "{} of {}", sugar.weight, sugar.sugar).unwrap();
            }
            output
        }
    }

    /// Get a hops additions as a string
    #[must_use]
    pub fn hops_additions_string(&self) -> String {
        let mut output: String = String::new();
        for hopsdose in &self.hops {
            let after = self.boil_time() - hopsdose.timing;
            writeln!(
                output,
                "\n{} from start {} from end:  Add {} of {}",
                after, hopsdose.timing, hopsdose.weight, hopsdose.hops
            )
            .unwrap();
        }
        output
    }

    /// Grams of yeast needed for pitch
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn yeast_grams(&self) -> Option<Grams> {
        if let Some((grams, liters)) = self.yeast.pitching_rate() {
            Some(grams * (self.process.ferment_volume.0 / liters.0))
        } else {
            let cells = self.yeast_cells();
            if self.yeast.is_dry() {
                Some(Grams((cells / Yeast::CELLS_PER_GRAM_DRY) as f32))
            } else {
                None
            }
        }
    }

    /// Number of yeast cells needed for a good pitch
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn yeast_cells(&self) -> u64 {
        let ml: Milliliters = self.process.ferment_volume.into();
        let plato: Plato = self.original_gravity().into();
        let pitch_rate: u64 = self.style.yeast_pitching_rate();
        pitch_rate * (ml.0 * plato.0) as u64
    }

    /// Yeast nutrient needed
    // 1.5 g per liter
    #[must_use]
    pub fn yeast_nutrient_amount(&self) -> Grams {
        Grams(1.5) * self.process.ferment_volume.0
    }

    /// Ice bath ice weight
    #[must_use]
    pub fn ice_weight(&self) -> Kilograms {
        Kilograms(self.process.kettle_volume.0 / 2.0)
    }

    /// Chilled water for ice bath
    #[must_use]
    pub fn chilled_water_volume(&self) -> Liters {
        self.process.kettle_volume
    }

    /// Diacetyl rest temperature
    #[must_use]
    pub fn diacetyl_rest_temperature(&self) -> Celsius {
        Celsius(self.ferment_temperature.0 * (5.0 / 6.0) + (20.0 / 3.0))
    }

    /// IBU (Tinseth formula)
    // FIXME: use the mIBU model which is more accurate
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn ibu_tinseth(&self) -> Ibu {
        let bigness_factor = 1.65 * (0.000_125_f32).powf(self.original_gravity().0 - 1.0);

        let boil_time_factor = (1.0 - (-0.04 * self.boil_time().0 as f32).exp()) / 4.15;

        let utilization = bigness_factor * boil_time_factor;

        let mut ibu: f32 = 0.0;

        let gallons: Gallons = self.process.ferment_volume.into();

        for dose in &self.hops {
            let ounces: Ounces = dose.weight.into();
            ibu += utilization * dose.hops.alpha_acid() * ounces.0 * 7490.0 / gallons.0;
        }

        Ibu(ibu)
    }

    /// Beer color in SRM units (Morey)
    #[must_use]
    pub fn color(&self) -> Srm {
        let mut mcu: f32 = 0.0;

        for dose in &self.malt_doses() {
            let pounds: Pounds = dose.weight.into();
            let lovabond: Lovabond = dose.malt.ebc().into();

            mcu += pounds.0 * lovabond.0;
        }

        let gallons: Gallons = self.process.ferment_volume.into();
        mcu /= gallons.0;

        // Morey equasion handles the non-linearity
        Srm(1.4922 * mcu.powf(0.6859))
    }

    /// ABV in %
    #[must_use]
    pub fn abv(&self) -> f32 {
        let og = self.original_gravity().0;
        let fg = self.final_gravity().0;
        // Simple:
        // (og - fg) * 131.25
        // More precise, esp for higher alcohol drinks:
        (76.08 * (og - fg) / (1.775 - og)) * (fg / 0.794)
    }

    /// Verify that the recipe is sound
    ///
    /// # Errors
    ///
    /// Returns errors if the recipe is not sound.
    #[allow(clippy::too_many_lines)]
    pub fn verify(&self) -> Result<(), Vec<String>> {
        let mut errors: Vec<String> = Vec::new();

        // Verify diastatic power of the mash
        // TODO: use degrees Lintner
        let mut diastatic_weight: Kilograms = Kilograms(0.0);
        for malt_dose in &self.malt_doses() {
            if malt_dose.malt.category() == MaltCategory::Base {
                diastatic_weight = diastatic_weight + malt_dose.weight;
            }
        }
        let fraction_base_malt = diastatic_weight.0 / self.grain_weight().0;
        if fraction_base_malt < 0.7 {
            errors.push(format!("Not enough base malt: {fraction_base_malt} < 0.7"));
        }

        // Verify malts are not in excess of recommendations
        for malt_dose in &self.malt_doses() {
            let percent = 100.0 * malt_dose.weight.0 / self.grain_weight().0;
            if percent > malt_dose.malt.recommended_max_percent() {
                errors.push(format!(
                    "Too much {}: {}% > {}%",
                    malt_dose.malt,
                    percent,
                    malt_dose.malt.recommended_max_percent(),
                ));
            }
        }

        // Verify the pre-boil volume fits in the kettle

        if self.pre_boil_volume() > self.process.kettle_volume {
            errors.push(format!(
                "Kettle will be full or overfull:  {}",
                self.pre_boil_volume()
            ));
        }

        if self.process.room_temperature > Celsius(30.0) {
            errors.push(format!(
                "Room temp is high: {}",
                self.process.room_temperature
            ));
        }
        if self.process.room_temperature < Celsius(10.0) {
            errors.push(format!(
                "Room temp is low: {}",
                self.process.room_temperature
            ));
        }
        if self.strike_temperature() > Celsius(100.0) {
            errors.push(format!(
                "Strike temp {} is above boiling!",
                self.strike_temperature()
            ));
        }
        if self.strike_temperature() < Celsius(20.0) {
            errors.push(format!(
                "Strike temp {} is crazy low",
                self.strike_temperature()
            ));
        }
        if self.ferment_temperature < Celsius(6.0) {
            errors.push(format!(
                "Ferment temp {} is crazy low",
                self.ferment_temperature
            ));
        }
        if self.ferment_temperature > Celsius(35.0) {
            errors.push(format!(
                "Ferment temp {} is crazy high",
                self.ferment_temperature
            ));
        }
        if !self.yeast.temp_range().contains(&self.ferment_temperature) {
            errors.push(format!(
                "Ferment temp {} is out of range {:?} for the yeast {}",
                self.ferment_temperature,
                self.yeast.temp_range(),
                self.yeast,
            ));
        }
        if self.abv() > self.yeast.alcohol_tolerance() * 100.0 {
            errors.push(format!(
                "{} ABV is too high, yeast can only tolerate {}% - {}%.",
                self.abv(),
                self.yeast.alcohol_tolerance_range().start * 100.0,
                self.yeast.alcohol_tolerance_range().end * 100.0,
            ));
        }

        // Verify the mash pH
        if !(5.2..5.6).contains(&self.mash_ph().0) {
            errors.push(format!(
                "Estimated Mash {} is out of range 5.2..5.6",
                self.mash_ph()
            ));
        }

        // Verify the style OG
        if !self
            .style
            .original_gravity_range()
            .contains(&self.original_gravity())
        {
            errors.push(format!(
                "Original Gravity {:.3} out of range {:?} for {}",
                self.original_gravity(),
                self.style.original_gravity_range(),
                self.style
            ));
        }

        // Verify the style FG
        if !self
            .style
            .final_gravity_range()
            .contains(&self.final_gravity())
        {
            errors.push(format!(
                "Final Gravity {:.3} out of range {:?} for {}",
                self.final_gravity(),
                self.style.final_gravity_range(),
                self.style
            ));
        }

        // Verify the style ABV
        if !self.style.abv_range().contains(&self.abv()) {
            errors.push(format!(
                "ABV {:.2} out of range {:?} for {}",
                self.abv(),
                self.style.abv_range(),
                self.style
            ));
        }

        // Verify the style IBU
        if !self.style.ibu_range().contains(&self.ibu_tinseth()) {
            errors.push(format!(
                "IBU {:.1} out of range {:?} for {}",
                self.ibu_tinseth(),
                self.style.ibu_range(),
                self.style
            ));
        }

        // Verify the style SRM
        if !self.style.color_range().contains(&self.color()) {
            errors.push(format!(
                "SRM {:.1} out of range {:?} for {}",
                self.color(),
                self.style.color_range(),
                self.style
            ));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
