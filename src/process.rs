use super::{Brewery, Recipe, Warning};
use crate::Packaging;
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Write;

/// Process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Process {
    /// The equipment being used
    pub brewery: Brewery,

    /// The recipe to brew
    pub recipe: Recipe,

    /// The batch size
    // tbd: compute it magically from the max size that fits in
    // the equipment.
    pub batch_size: Liters,
}

impl Process {
    // NOTE: we order these functions based on dependencies.
    // Each function only depends on things above it.

    /// Create a new process.
    ///
    /// Due to equipment limitations, the actual batch size may compute
    /// differently to the desired one.  If no size is specified, the
    /// maximum batch size will be computed.
    #[must_use]
    pub fn new(brewery: Brewery, recipe: Recipe, batch_size: Liters) -> Process {
        Process {
            brewery,
            recipe,
            batch_size,
        }
    }

    /// The length of time before the beer is ready
    #[must_use]
    pub fn time_until_done(&self) -> Days {
        let mut conditioning = self.recipe.style.recommended_conditioning_time();

        if let Packaging::Bottle(_, _) = self.brewery.packaging {
            // At least bottle conditioning time.
            // NOTE: Bottle conditioning counts as conditioning time.
            if conditioning < Days(14) {
                conditioning = Days(14);
            }
        }

        // 2 days of diacetyl rest
        self.recipe.fermentation_time() + Days(2) + conditioning
    }

    /// Water salts to adjust ions
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn water_salts(&self) -> Vec<SaltConcentration> {
        if matches!(self.recipe.ph_method, PhMethod::AdjustWater) {
            let water_adjustment = WaterAdjustment {
                profile: self.brewery.water_profile,
                mash_ph_distilled: self.mash_ph_distilled().pop().unwrap(),
                target_ph: self.recipe.mash_ph_target,
                sulfate_chloride_target: self.recipe.sulfate_chloride_target,
            };

            water_adjustment.salts_needed()
        } else {
            Vec::new()
        }
    }

    /// Water acids to adjust mash pH
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn water_acids(&self) -> Vec<AcidConcentration> {
        if let PhMethod::ComputeAcid(acid) = self.recipe.ph_method {
            let start = self.mash_ph_preacid().pop().unwrap();
            let end = self.recipe.mash_ph_target;

            let shift = start.0 - end.0;

            // FIXME this is only for Lactic Acid 88%
            // FIXME this is not very accurate
            // 25-80 ppm to drop by 0.15 pH
            let ppm = Ppm(300.0 * shift);

            vec![AcidConcentration { acid, ppm }]
        } else {
            Vec::new()
        }
    }

    /// The water profile (after salts, before acids)
    #[must_use]
    pub fn adjusted_water_profile_preacid(&self) -> WaterProfile {
        let mut profile = self.brewery.water_profile;

        for salt_conc in &self.water_salts() {
            profile.add_salt(*salt_conc);
        }

        profile
    }

    /// The water profile (after salts and acids)
    #[must_use]
    pub fn adjusted_water_profile(&self) -> WaterProfile {
        let mut profile = self.brewery.water_profile;

        for salt_conc in &self.water_salts() {
            profile.add_salt(*salt_conc);
        }

        for acid_conc in &self.water_acids() {
            profile.add_acid(*acid_conc);
        }

        profile
    }

    /// How much volume is required in the fermenter for head space?
    #[must_use]
    pub fn fermentation_head_space(&self) -> Liters {
        // In general 10-20% or 15-25% for aggressive ones
        // TBD: compute how aggressive the fermentation is
        // aggressive: high gravity, high temperature

        self.batch_size * 0.20
    }

    /// Chosen fermenter volume
    #[must_use]
    pub fn fermenter_volume(&self) -> Liters {
        let needed = self.batch_size + self.fermentation_head_space();

        // Choose the smallest fermenter that handles this
        let chosen = self
            .brewery
            .fermenters
            .iter()
            .copied()
            .filter(|&f| f >= needed)
            .min();

        chosen.unwrap_or(needed)
    }

    /// Post ferment volume
    pub(crate) fn post_ferment_volume(&self) -> Liters {
        self.batch_size * (1.0 - self.recipe.ferment_loss_fraction())
    }

    /// Ferment losses
    pub(crate) fn ferment_losses(&self) -> Liters {
        self.batch_size - self.post_ferment_volume()
    }

    /// The mount of water that evaporates during the boil
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn boil_evaporation(&self) -> Liters {
        self.brewery.boil_evaporation_per_hour / 60.0 * self.recipe.boil_length.0 as f32
    }

    /// Partial boil dilution
    #[must_use]
    pub fn partial_boil_dilution(&self) -> Liters {
        let volume_needed = self.batch_size + self.brewery.kettle_losses + self.boil_evaporation();

        if self.brewery.max_kettle_volume < volume_needed {
            volume_needed - self.brewery.max_kettle_volume
        } else {
            Liters(0.0)
        }
    }

    /// The volume of undiluted wart after the boil, after kettle losses
    #[must_use]
    pub fn post_boil_volume(&self) -> Liters {
        self.batch_size - self.partial_boil_dilution()
    }

    /// The volume of undiluted wart after the boil, before kettle losses.
    #[must_use]
    pub fn post_boil_pre_loss_volume(&self) -> Liters {
        self.post_boil_volume() + self.brewery.kettle_losses
    }

    /// The pre-boil volume
    #[must_use]
    pub fn pre_boil_volume(&self) -> Liters {
        self.post_boil_pre_loss_volume() + self.boil_evaporation()
    }

    /// Multipler on the grain bill that achieves the original
    /// gravity at the batch size.
    #[must_use]
    pub fn grain_bill_multiplier(&self) -> f32 {
        let malt_doses: Vec<MaltDose> = self
            .recipe
            .malts
            .iter()
            .map(|proportion| MaltDose {
                malt: proportion.malt,
                weight: Kilograms(proportion.proportion), // as if 1.0 is 1.0 kg.
            })
            .collect();

        let sugar_doses: Vec<SugarDose> = self
            .recipe
            .sugars
            .iter()
            .map(|proportion| SugarDose {
                sugar: proportion.sugar,
                weight: Kilograms(proportion.proportion), // as if 1.0 is 1.0 kg.
            })
            .collect();

        let sg = SpecificGravity::from_recipe(
            &malt_doses,
            &sugar_doses,
            self.batch_size.into(),
            self.brewery.mash_efficiency,
        );

        let actual_points = sg.0 - 1.0;

        let ideal_points = self.recipe.original_gravity.0 - 1.0;

        ideal_points / actual_points
    }

    /// Malt doses
    #[must_use]
    pub fn malt_doses(&self) -> Vec<MaltDose> {
        let multiplier = self.grain_bill_multiplier();

        self.recipe
            .malts
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

        self.recipe
            .sugars
            .iter()
            .map(|proportion| SugarDose {
                sugar: proportion.sugar,
                weight: Kilograms(proportion.proportion * multiplier),
            })
            .collect()
    }

    /// The pre-boil original gravity (OG) of the wort
    #[must_use]
    pub fn pre_boil_gravity(&self) -> SpecificGravity {
        let pre_boil_volume: Gallons = self.pre_boil_volume().into();
        SpecificGravity::from_recipe(
            &self.malt_doses(),
            &self.sugar_doses(),
            pre_boil_volume,
            self.brewery.mash_efficiency,
        )
    }

    /// Hops doses
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn hops_doses(&self) -> Vec<HopsDose> {
        // We use Tinseth

        let bigness_factor = 1.65 * (0.000_125_f32).powf(self.recipe.original_gravity.0 - 1.0);
        let gallons: Gallons = self.batch_size.into();

        let mut nominal_ibus: f32 = 0.0;

        for hops_prop in &self.recipe.hops {
            let ounces: Ounces = Grams(hops_prop.proportion).into();
            let boil_time_factor = (1.0 - (-0.04 * hops_prop.timing.0 as f32).exp()) / 4.15;
            let utilization = bigness_factor * boil_time_factor;
            nominal_ibus +=
                utilization * hops_prop.hops.alpha_acid() * ounces.0 * 7490.0 / gallons.0;
        }

        let scaling_factor = self.recipe.ibu.0 / nominal_ibus;

        self.recipe
            .hops
            .iter()
            .map(|prop| HopsDose {
                hops: prop.hops,
                weight: Grams(prop.proportion * scaling_factor),
                timing: prop.timing,
            })
            .collect()
    }

    /// Get a hops additions as a string
    #[must_use]
    pub fn hops_additions_string(&self) -> String {
        let mut output: String = String::new();
        for hopsdose in &self.hops_doses() {
            let after = self.recipe.boil_length - hopsdose.timing;
            writeln!(
                output,
                "\n{} from start {} from end:  Add {} of {}",
                after, hopsdose.timing, hopsdose.weight, hopsdose.hops
            )
            .unwrap();
        }
        output
    }

    /// The amount of whirlfloc tablet to use
    #[must_use]
    pub fn whirlfloc_amount(&self) -> f32 {
        if self.recipe.fining_desired {
            1.0 * (self.batch_size.0 / 19.0)
        } else {
            0.0
        }
    }

    /// The weight of the malts in the mash
    #[must_use]
    pub fn grain_weight(&self) -> Kilograms {
        self.malt_doses().iter().map(|dose| dose.weight).sum()
    }

    /// Mash pHs with distilled water, considering only the malts.
    /// This neither considers RA of water, nor acid additions.
    ///
    /// This is used to help figure out water adjustments and acid additions.
    /// It also feeds into the mash_ph() function
    /// (rather than "beer color" which is a poorer proxy)
    #[must_use]
    pub fn mash_ph_distilled(&self) -> Vec<Ph> {
        // http://braukaiser.com/documents/effect_of_water_and_grist_on_mash_pH.pdf

        let grain_weight = self.grain_weight();

        // base malt term (all steps)
        let base_malt_ph: f32 = {
            let mut ph: f32 = 0.0;
            for dose in self.malt_doses() {
                if dose.malt.category() != MaltCategory::Base {
                    continue;
                }

                let proportion = dose.weight.0 / grain_weight.0;
                let phbi = dose.malt.distilled_water_mash_ph().unwrap_or(Ph(5.75));

                ph += phbi.0 * proportion;
            }

            ph
        };

        // specialty malt term (all steps)
        let specialty_malt_ph_term_one: f32 = {
            let mut ph: f32 = 0.0;
            for dose in self.malt_doses() {
                if dose.malt.category() == MaltCategory::Base {
                    continue;
                }

                let proportion = dose.weight.0 / grain_weight.0;

                ph += 5.7 * proportion;
            }

            ph
        };

        // specialty malt term (per step, thickness-dependent)
        let specialty_malt_ph_term_two: Vec<f32> = {
            let mut acidity: f32 = 0.0;
            for dose in self.malt_doses() {
                if dose.malt.category() == MaltCategory::Base {
                    continue;
                }

                let proportion = dose.weight.0 / grain_weight.0;

                acidity += dose.malt.acidity() * proportion;
            }

            let mut output: Vec<f32> = Vec::new();
            let mash_thicknesses = self.mash_thicknesses();
            for mt in &mash_thicknesses {
                let ph_adjust = -0.14 * acidity / mt;
                output.push(ph_adjust);
            }

            output
        };

        let mut output: Vec<Ph> = Vec::new();

        for t2 in &specialty_malt_ph_term_two {
            output.push(Ph(base_malt_ph + specialty_malt_ph_term_one + t2));
        }

        output
    }

    /// Estimated mash pH prior to acid additions
    #[must_use]
    pub fn mash_ph_preacid(&self) -> Vec<Ph> {
        // Start from the pH effect of the grains only
        let mut output = self.mash_ph_distilled();

        // from https://byo.com/articles/understanding-residual-alkalinity-ph/
        // pH shift = 0.00168 * RA (as CaCO3) or pH shift = 0.084 * RA (as mEq/L)
        let ra = self.adjusted_water_profile_preacid().residual_alkalinity();
        let shift = 0.00168 * ra.0;

        for out in &mut output {
            out.0 += shift;
        }

        output
    }

    /// Estimated mash pH
    /// Considering all effects (grains, water, acids)
    #[must_use]
    pub fn mash_ph(&self) -> Vec<Ph> {
        let mut output = self.mash_ph_preacid();

        let acids = self.water_acids();

        for out in &mut output {
            // FIXME this is only for Lactic Acid 88%
            for acid in &acids {
                // FIXME this is not very accurate
                // 25-80 ppm to drop by 0.15 pH
                let shift = acid.ppm.0 / 300.0;
                out.0 -= shift;
            }
        }

        output
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

        writeln!(output, "Total Malt Weight = {}", self.grain_weight()).unwrap();
        write!(
            output,
            "Total Fermentable Weight = {}",
            self.fermentables_weight()
        )
        .unwrap();

        output
    }

    /// The water absorption of the malts
    #[must_use]
    pub fn water_absorption(&self) -> Liters {
        self.brewery.grain_absorption_per_kg * self.grain_weight().0
    }

    /// The pre-sparge volume
    #[must_use]
    pub fn pre_sparge_volume(&self) -> Liters {
        Liters(self.grain_weight().0 * self.recipe.mash_thickness - self.water_absorption().0)
    }

    /// The volume at the end of the mash, before losses from grain absorption
    #[must_use]
    pub fn mash_volume(&self) -> Liters {
        self.pre_sparge_volume() + self.water_absorption()
    }

    /// The amount of sparge water used
    #[must_use]
    pub fn sparge_volume(&self) -> Liters {
        self.pre_boil_volume() - self.pre_sparge_volume()
    }

    /// Strike volume
    #[must_use]
    pub fn strike_volume(&self) -> Liters {
        // To compute the strike volume, we have to calculate backwards.
        // This is the only place we do this.  Everywhere else, we will
        // calculate forwards from this strike volume to double-check
        // our work.

        // start at the end of the mash with the grains still inside
        let mut current_water = self.mash_volume();
        let mut current_temp: Option<Celsius> = None;

        // Iterate backwards through each mash rest
        for rest in self.recipe.mash_rests.clone().into_iter().rev() {
            if let Some(current_tmp) = current_temp {
                // Compute the mash step backwards
                let infusion = crate::mash::reverse_mash_infusion(
                    self.grain_weight(),
                    current_water,
                    rest.target_temperature,
                    current_tmp,
                    self.brewery.infusion_temperature,
                );

                // Subtract that much water
                current_water = current_water - infusion;
            }

            // Update the current temperature
            current_temp = Some(rest.target_temperature);
        }

        current_water
    }

    /// Strike temperature
    #[must_use]
    pub fn strike_temperature(&self) -> Celsius {
        crate::mash::strike_water_temp(
            self.strike_volume(),
            self.grain_weight(),
            self.brewery.room_temperature,
            self.recipe.mash_rests[0].target_temperature,
        )
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

        for rest in &self.recipe.mash_rests {
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
                self.brewery.infusion_temperature,
            );

            infusions.push(infusion);
            current_water = current_water + infusion;
            current_temp = Some(rest.target_temperature);
        }

        infusions
    }

    /// Thickness of the mash, in liters of water per kilograms of
    /// grain, at each mash step.
    #[must_use]
    pub fn mash_thicknesses(&self) -> Vec<f32> {
        let mut thicknesses: Vec<f32> = Vec::new();

        let mut liters = self.strike_volume();
        thicknesses.push(liters.0 / self.grain_weight().0);

        for infusion in self.mash_infusions() {
            liters = liters + infusion;
            thicknesses.push(liters.0 / self.grain_weight().0);
        }

        thicknesses
    }

    /// Number of yeast cells needed for a good pitch
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn yeast_cells(&self) -> u64 {
        let ml: Milliliters = self.batch_size.into();
        let plato: Plato = self.recipe.original_gravity.into();
        let mut pitch_rate: u64 = self.recipe.yeast.pitching_rate_cmlp();
        if self.recipe.style.is_a_wheat_beer() {
            pitch_rate *= 600;
            pitch_rate /= 750;
        }
        pitch_rate * (ml.0 * plato.0) as u64
    }

    /// Grams of yeast needed for pitch
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn yeast_grams(&self) -> Option<Grams> {
        if let Some(ghl_range) = self.recipe.yeast.pitching_rate_range_ghl() {
            // We assume their low number maps to 1.035
            // We assume their high number maps to 1.070
            let fraction_in: f32 = {
                let low_plato: Plato = SpecificGravity(1.035).into();
                let plato_range: f32 = {
                    let high_plato: Plato = SpecificGravity(1.070).into();
                    high_plato.0 - low_plato.0
                };
                let plato: Plato = self.recipe.original_gravity.into();
                (plato.0 - low_plato.0) / plato_range
            };

            let ghl = ghl_range.start + (ghl_range.end - ghl_range.start) * fraction_in;
            Some(Grams(ghl / 100.0 * self.batch_size.0))
        } else {
            let cells = self.yeast_cells();
            if self.recipe.yeast.is_dry() {
                Some(Grams((cells / Yeast::CELLS_PER_GRAM_DRY) as f32))
            } else {
                None
            }
        }
    }

    /// Estimated FAN amount in the wort from malts, in ppm (mg/L)
    #[must_use]
    pub fn fan_from_malt(&self) -> Ppm {
        let mut total: Ppm = Ppm(0.0);
        let gallons: Gallons = self.batch_size.into();
        for malt_dose in self.malt_doses() {
            let pounds: Pounds = malt_dose.weight.into();
            let points = {
                let ppg = malt_dose.malt.ppg(); // points/(pounds*gallons) at 100% eff.
                self.brewery.mash_efficiency * ppg * pounds.0 / gallons.0
            };
            let malt_fan_per_point: Ppm = malt_dose.malt.fan() / 40.0;
            total = total + malt_fan_per_point * points;
        }

        total
    }

    /// Yeast nutrient needed
    #[must_use]
    pub fn yeast_nutrient_amount(&self) -> Grams {
        if self.recipe.fan_requirement_of_yeast() < self.fan_from_malt() {
            Grams(0.0)
        } else {
            let ppm_needed = self.recipe.fan_requirement_of_yeast().0 - self.fan_from_malt().0;

            // yeast nutrient generally provides about 0.4 ppm
            // per gram in one hectoliter
            // FIXME: ONLY in high FAN nutrients
            let fan_rate = 0.4 * 100.0; // ppm*L/g

            Grams(
                self.batch_size.0         // L
                    * ppm_needed                      // ppm
                    / fan_rate, // g / ppm*L
            )
        }
    }

    /// Total wort FAN from malts and yeast nutrient
    #[must_use]
    pub fn wort_fan(&self) -> Ppm {
        // yeast nutrient generally provides about 0.4 ppm
        // per gram in one hectoliter
        // FIXME: ONLY in high FAN nutrients
        let fan_rate = 0.4 * 100.0; // ppm*L/g

        let nutrient_ppm = Ppm(
            self.yeast_nutrient_amount().0       // g
                * fan_rate                       // ppm*L / g
                / self.batch_size.0, // 1 / L
        );

        self.fan_from_malt() + nutrient_ppm
    }

    /// Zinc drops needed
    #[must_use]
    pub fn zinc_needed(&self) -> Milligrams {
        // Different sourcess quote different Zn ranges:
        //    0.15 to 0.3 mg Zn / L -- minimum required
        //    0.3 to 0.5 mg/L
        //    0.6 (a high-end lallemand yeast nutrient providing it)
        // But recall that some zinc is already provided by the malt.
        // JP says 0.1-0.3 mg/L with 0.5 as a maximum
        //
        // Wyeast packs already has zinc. TODO.

        Milligrams(0.2 * self.batch_size.0)
    }

    /// The estimated gravity after fermentation, before any dilution
    // TODO: this doesn't adjust for the presence of many
    //       unfermentable sugars
    #[must_use]
    pub fn post_ferment_gravity(&self) -> SpecificGravity {
        let og = self.recipe.original_gravity;

        let mut attenuation = self.recipe.yeast.attenuation();

        let mut reduction_percent: f32 = 0.0;

        for malt_dose in &self.malt_doses() {
            if malt_dose.malt.category() == MaltCategory::Crystal {
                let pounds: Pounds = malt_dose.weight.into();
                let lovabond: Lovabond = malt_dose.malt.ebc().into();
                let gallons: Gallons = self.batch_size.into();
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

    /// The post-fermentation dilution fraction to apply to achieve closer to
    /// the target ABV
    #[must_use]
    pub fn post_fermentation_dilution_fraction(&self) -> f32 {
        if let Some(target_abv) = self.recipe.target_abv {
            let natural_abv = Abv::from_gravity(
                self.recipe.original_gravity,
                self.post_ferment_gravity(),
                1.0,
            );
            if natural_abv > target_abv {
                return (natural_abv.0 / target_abv.0).min(self.recipe.max_post_ferment_dilution);
            }
        }

        1.0
    }

    /// The post-fermentation dilution to apply to achieve closer to the
    /// target ABV
    #[must_use]
    pub fn post_fermentation_dilution(&self) -> Liters {
        let end = self.post_ferment_volume() * self.post_fermentation_dilution_fraction();
        let start = self.post_ferment_volume();
        end - start
    }

    /// The product volume at the end
    #[must_use]
    pub fn product_volume(&self) -> Liters {
        self.post_ferment_volume() + self.post_fermentation_dilution()
    }

    /// Volume history
    #[must_use]
    pub fn volume_history_string(&self) -> String {
        let mut output: String = String::new();
        let mut total = self.strike_volume();

        writeln!(
            output,
            "Strike         +{}     = {total}  strike volume",
            self.strike_volume()
        )
        .unwrap();

        for infusion in self.mash_infusions() {
            total = total + infusion;
            writeln!(
                output,
                "Mash Infusion: +{infusion}     = {total}  mash volume"
            )
            .unwrap();
        }

        total = total - self.water_absorption();
        writeln!(
            output,
            "Absorption:    -{}     = {total}  pre sparge volume",
            self.water_absorption()
        )
        .unwrap();

        total = total + self.sparge_volume();
        writeln!(
            output,
            "Sparge:        +{}     = {total}  pre boil volume",
            self.sparge_volume()
        )
        .unwrap();

        total = total - self.boil_evaporation();
        writeln!(
            output,
            "Boil off:      -{}     = {total}  pre boil pre loss volume",
            self.boil_evaporation()
        )
        .unwrap();

        total = total - self.brewery.kettle_losses;
        writeln!(
            output,
            "Kettle Losses: -{}     = {total}  post boil volume",
            self.brewery.kettle_losses
        )
        .unwrap();

        total = total + self.partial_boil_dilution();
        writeln!(
            output,
            "Dilution:      +{}     = {total}  batch size",
            self.partial_boil_dilution()
        )
        .unwrap();

        total = total - self.ferment_losses();
        writeln!(
            output,
            "Ferment Loss:  -{}     = {total}  post ferment volume",
            self.ferment_losses()
        )
        .unwrap();

        total = total + self.post_fermentation_dilution();
        writeln!(
            output,
            "Dilution:      +{}     = {total}  product volume",
            self.post_fermentation_dilution()
        )
        .unwrap();

        output
    }

    /// ABV in %
    #[must_use]
    pub fn abv(&self) -> Abv {
        Abv::from_gravity(
            self.recipe.original_gravity,
            self.post_ferment_gravity(),
            self.post_fermentation_dilution_fraction(),
        )
    }

    /// Final gravity
    #[must_use]
    pub fn final_gravity(&self) -> SpecificGravity {
        let points = self.post_ferment_gravity().0 - 1.0;
        let ratio = self.post_ferment_volume().0 / self.product_volume().0;
        SpecificGravity(1.0 + points * ratio)
    }

    /// Real extract
    ///
    /// This is the true amount of solids (unfermentable sugars) in the
    /// final product.  Final Gravity is 'apparent' on a hydrometer but
    /// not accurate since the product contains alcohol.  This value
    /// adjusts for that.
    #[must_use]
    pub fn real_extract(&self) -> Plato {
        let original_plato: Plato = self.recipe.original_gravity.into();
        let final_plato: Plato = self.final_gravity().into();
        Plato(0.188 * original_plato.0 + 0.8192 * final_plato.0)
    }

    /// The total amount of water input during the process
    #[must_use]
    pub fn total_water(&self) -> Liters {
        self.mash_volume() // strike plus infusions
            + self.sparge_volume()
            + self.partial_boil_dilution()
            + self.post_fermentation_dilution()
    }

    /// Salt doses
    #[must_use]
    pub fn salt_doses(&self, liters: Option<Liters>) -> Vec<SaltDose> {
        let mut output: Vec<SaltDose> = Vec::new();

        let water_liters = match liters {
            Some(l) => l,
            None => self.total_water(),
        };

        for salt_concentration in &self.water_salts() {
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

        for acid_concentration in &self.water_acids() {
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
        if output.is_empty() {
            "No water dosing is required.".to_string()
        } else {
            output
        }
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
        for hops in &self.hops_doses() {
            writeln!(output, "{} of {}", hops.weight, hops.hops).unwrap();
        }
        writeln!(output, "Yeast: {}", self.recipe.yeast).unwrap();
        if self.yeast_nutrient_amount() > Grams(0.0) {
            writeln!(output, "Yeast Nutrient: {}", self.yeast_nutrient_amount()).unwrap();
        }
        output
    }

    /// Compute bitterness in IBU
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn bitterness(&self) -> Ibu {
        // We use Tinseth

        let bigness_factor = 1.65 * (0.000_125_f32).powf(self.recipe.original_gravity.0 - 1.0);
        let gallons: Gallons = self.batch_size.into();

        let mut ibu: f32 = 0.0;

        for dose in &self.hops_doses() {
            let ounces: Ounces = dose.weight.into();
            let boil_time_factor = (1.0 - (-0.04 * dose.timing.0 as f32).exp()) / 4.15;
            let utilization = bigness_factor * boil_time_factor;
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
        for dose in &self.sugar_doses() {
            let pounds: Pounds = dose.weight.into();
            let lovabond: Lovabond = dose.sugar.ebc().into();
            mcu += pounds.0 * lovabond.0;
        }

        let gallons: Gallons = self.batch_size.into();
        mcu /= gallons.0;

        // Morey equasion handles the non-linearity
        Srm(1.4922 * mcu.powf(0.6859))
    }

    /// Get warnings
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn get_warnings(&self) -> Vec<Warning> {
        let mut warnings: Vec<Warning> = Vec::new();

        // Check fermenter volume
        {
            let needed = self.batch_size + self.fermentation_head_space();

            // Choose the smallest fermenter that handles this
            let chosen = self
                .brewery
                .fermenters
                .iter()
                .copied()
                .filter(|&f| f >= needed)
                .min();

            if chosen.is_none() {
                warnings.push(Warning::FermentersTooSmall { needed });
            }
        }

        // Check partial boil dilution amount
        {
            let dilution_ratio =
                (self.batch_size.0 - self.partial_boil_dilution().0) / self.batch_size.0;
            if dilution_ratio > self.recipe.max_partial_boil_dilution {
                warnings.push(Warning::ExcessDilutionRequired {
                    dilution_ratio,
                    maximum: self.recipe.max_partial_boil_dilution,
                });
            }
        }

        // Verify diastatic power of the mash
        {
            // TODO: use degrees Lintner
            let mut diastatic_weight: Kilograms = Kilograms(0.0);
            for malt_dose in &self.malt_doses() {
                if malt_dose.malt.category() == MaltCategory::Base {
                    diastatic_weight = diastatic_weight + malt_dose.weight;
                }
            }

            let fraction_base_malts = diastatic_weight.0 / self.grain_weight().0;

            if fraction_base_malts < 0.7 {
                warnings.push(Warning::LowDiastaticPower {
                    fraction_base_malts,
                });
            }
        }

        // Verify malts are not in excess of recommendations
        for malt_dose in &self.malt_doses() {
            let percent = 100.0 * malt_dose.weight.0 / self.grain_weight().0;
            if percent > malt_dose.malt.recommended_max_percent() {
                warnings.push(Warning::ExcessMalt {
                    malt: malt_dose.malt,
                    percent,
                    max_recommended_percent: malt_dose.malt.recommended_max_percent(),
                });
            }
        }

        // Verify the pre-boil volume fits into the kettle
        if self.pre_boil_volume() > self.brewery.max_kettle_volume {
            warnings.push(Warning::BoilKettleTooSmall {
                needed: self.pre_boil_volume(),
                available: self.brewery.max_kettle_volume,
            });
        }

        // Verify sparge volume is not negative
        if self.sparge_volume().0 < 0.0 {
            warnings.push(Warning::TooMuchMash {
                overfull: Liters(-self.sparge_volume().0),
                mash_thickness: self.recipe.mash_thickness,
            });
        }

        if self.brewery.room_temperature > Celsius(35.0) {
            warnings.push(Warning::UnusualRoomTemperature(
                self.brewery.room_temperature,
            ));
        }
        if self.brewery.room_temperature < Celsius(10.0) {
            warnings.push(Warning::UnusualRoomTemperature(
                self.brewery.room_temperature,
            ));
        }

        if self.brewery.infusion_temperature > Celsius(100.0) {
            warnings.push(Warning::ImpossibleInfusionTemperature(
                self.brewery.infusion_temperature,
            ));
        }
        if self.brewery.infusion_temperature < Celsius(67.0) {
            warnings.push(Warning::UnusualInfusionTemperature(
                self.brewery.infusion_temperature,
            ));
        }

        if self.recipe.ferment_temperature < Celsius(6.0) {
            warnings.push(Warning::UnusualFermentationTemperature(
                self.recipe.ferment_temperature,
            ));
        }
        if self.recipe.ferment_temperature > Celsius(35.0) {
            warnings.push(Warning::UnusualFermentationTemperature(
                self.recipe.ferment_temperature,
            ));
        }

        if self.recipe.ferment_temperature < self.recipe.yeast.temp_range().start {
            warnings.push(Warning::TooCold {
                ferment_temp: self.recipe.ferment_temperature,
                yeast_min: self.recipe.yeast.temp_range().start,
            });
        }

        if self.recipe.ferment_temperature > self.recipe.yeast.temp_range().end {
            warnings.push(Warning::TooHot {
                ferment_temp: self.recipe.ferment_temperature,
                yeast_max: self.recipe.yeast.temp_range().end,
            });
        }

        if self.abv() > Abv(self.recipe.yeast.alcohol_tolerance() * 100.0) {
            warnings.push(Warning::TooMuchAlcohol {
                abv: self.abv(),
                yeast_max: Abv(self.recipe.yeast.alcohol_tolerance() * 100.0),
            });
        }

        // Verify the mash pH
        for (i, ph) in self.mash_ph().iter().enumerate() {
            if !(5.2..5.6).contains(&ph.0) {
                warnings.push(Warning::MashPhOutOfRange(i + 1, *ph));
            }
        }

        // If the recipe calls for acids or acidulated malt, and some of that
        // acidity needed cancelling
        // TBD if in the future we have manual acid additions, check those too
        if self.recipe.malts.iter().any(|m| m.malt.acidity() > 100.0)
            && self
                .water_salts()
                .iter()
                .any(|s| s.salt == Salt::BakingSoda)
        {
            warnings.push(Warning::AcidityNeededCancelling);
        }

        // Verify the style OG
        if !self
            .recipe
            .style
            .original_gravity_range()
            .contains(&self.recipe.original_gravity)
        {
            warnings.push(Warning::OriginalGravityOutOfRange {
                gravity: self.recipe.original_gravity,
                range: self.recipe.style.original_gravity_range(),
            });
        }

        // Verify the style FG
        if !self
            .recipe
            .style
            .final_gravity_range()
            .contains(&self.final_gravity())
        {
            warnings.push(Warning::FinalGravityOutOfRange {
                gravity: self.final_gravity(),
                range: self.recipe.style.final_gravity_range(),
            });
        }

        // Verify the style ABV
        if !self.recipe.style.abv_range().contains(&self.abv()) {
            warnings.push(Warning::AbvOutOfRange {
                abv: self.abv(),
                range: self.recipe.style.abv_range(),
            });
        }

        // Verify the style IBU
        if !self
            .recipe
            .style
            .bitterness_range()
            .contains(&self.bitterness())
        {
            warnings.push(Warning::IbuOutOfRange {
                ibu: self.bitterness(),
                range: self.recipe.style.bitterness_range(),
            });
        }

        // Verify the style SRM
        if !self.recipe.style.color_range().contains(&self.color()) {
            warnings.push(Warning::SrmOutOfRange {
                srm: self.color(),
                range: self.recipe.style.color_range(),
            });
        }

        warnings
    }
}
