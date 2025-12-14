use super::{Equipment, Recipe2, Warning};
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Write;

/// Process2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Process2 {
    /// The equipment being used
    pub equipment: Equipment,

    /// The recipe to brew
    pub recipe: Recipe2,

    /// The batch size
    // tbd: compute it magically from the max size that fits in
    // the equipment.
    pub batch_size: Liters,
}

impl Process2 {
    // NOTE: we order these functions based on dependencies.
    // Each function only depends on things above it.

    /// Create a new process.
    ///
    /// Due to equipment limitations, the actual batch size may compute
    /// differently to the desired one.  If no size is specified, the
    /// maximum batch size will be computed.
    pub fn new(equipment: Equipment, recipe: Recipe2, batch_size: Liters) -> Process2 {
        Process2 {
            equipment,
            recipe,
            batch_size,
        }
    }

    /// Get warnings
    pub fn get_warnings(&self) -> Vec<Warning> {
        let mut warnings: Vec<Warning> = Vec::new();

        // Check chloride-sulfate ratio
        if let Some(range) = &self.recipe.chloride_sulfate_ratio_range {
            let water_profile = self.adjusted_water_profile();
            let current = water_profile.cl.0 / water_profile.so4.0;
            if current < range.start {
                warnings.push(Warning::ChlorideSulfateRatioLow {
                    current_ratio: current,
                });
            } else if current > range.end {
                warnings.push(Warning::ChlorideSulfateRatioHigh {
                    current_ratio: current,
                });
            }
        }

        // Check fermenter volume
        {
            let needed = self.batch_size + self.fermentation_head_space();

            // Choose the smallest fermenter that handles this
            let chosen = self
                .equipment
                .fermenters
                .iter()
                .map(|&f| f)
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

        warnings
    }

    /// Water salts to adjust ions
    pub fn water_salts(&self) -> Vec<SaltConcentration> {
        let mut output: Vec<SaltConcentration> = Vec::new();
        // tbd: right now we only push at most one salt to fix
        // the chloride-sulfate concentration.

        if let Some(range) = &self.recipe.chloride_sulfate_ratio_range {
            let water_profile = self.equipment.water_profile;

            let current = water_profile.cl.0 / water_profile.so4.0;

            if current < range.start {
                // We need Chloride
                if let Some(salt) = self.equipment.chloride_salt() {
                    let cl_needed = range.start * water_profile.so4.0 - water_profile.cl.0;

                    let ion_fraction = salt.ion_fraction(Ion::Chloride);

                    let salt_ppm_needed = cl_needed / ion_fraction;

                    output.push(SaltConcentration {
                        salt,
                        ppm: Ppm(salt_ppm_needed),
                    });
                }
            } else if current > range.end {
                // We need sulfate
                if let Some(salt) = self.equipment.sulfate_salt() {
                    let so4_needed = water_profile.cl.0 / range.end - water_profile.so4.0;

                    let ion_fraction = salt.ion_fraction(Ion::Sulfate);

                    let salt_ppm_needed = so4_needed / ion_fraction;

                    output.push(SaltConcentration {
                        salt,
                        ppm: Ppm(salt_ppm_needed),
                    });
                }
            }
        }

        output
    }

    /// Water acids to adjust mash pH
    pub fn water_acids(&self) -> Vec<AcidConcentration> {
        // TODO: compute acid additions
        vec![]
    }

    /// The water profile (after salts and acids)
    #[must_use]
    pub fn adjusted_water_profile(&self) -> WaterProfile {
        let mut profile = self.equipment.water_profile;

        for salt_conc in &self.water_salts() {
            profile.add_salt(*salt_conc);
        }

        for acid_conc in &self.water_acids() {
            profile.add_acid(*acid_conc);
        }

        profile
    }

    /// How much volume is required in the fermenter for head space?
    pub fn fermentation_head_space(&self) -> Liters {
        // In general 10-20% or 15-25% for aggressive ones
        // TBD: compute how aggressive the fermentation is
        // aggressive: high gravity, high temperature

        self.batch_size * 0.20
    }

    /// Chosen fermenter volume
    pub fn fermenter_volume(&self) -> Liters {
        let needed = self.batch_size + self.fermentation_head_space();

        // Choose the smallest fermenter that handles this
        let chosen = self
            .equipment
            .fermenters
            .iter()
            .map(|&f| f)
            .filter(|&f| f >= needed)
            .min();

        chosen.unwrap_or(needed)
    }

    /// Post ferment volume
    pub(crate) fn post_ferment_volume(&self) -> Liters {
        self.batch_size * self.recipe.ferment_loss_fraction()
    }

    /// Ferment losses
    pub(crate) fn ferment_losses(&self) -> Liters {
        self.batch_size - self.post_ferment_volume()
    }

    /// The mount of water that evaporates during the boil
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn boil_evaporation(&self) -> Liters {
        self.equipment.boil_evaporation_per_hour / 60.0 * self.recipe.boil_length.0 as f32
    }

    /// Partial boil dilution
    pub fn partial_boil_dilution(&self) -> Liters {
        let volume_needed =
            self.batch_size + self.equipment.kettle_losses + self.boil_evaporation();

        if self.equipment.max_kettle_volume < volume_needed {
            let dilution = volume_needed - self.equipment.max_kettle_volume;
            return dilution;
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
        self.post_boil_volume() + self.equipment.kettle_losses
    }

    /// The pre-boil volume
    #[must_use]
    pub fn pre_boil_volume(&self) -> Liters {
        self.post_boil_pre_loss_volume() + self.boil_evaporation()
    }

    /// Multipler on the grain bill that achieves the original
    /// gravity at the batch size.
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
            self.equipment.mash_efficiency,
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

    /// Hops doses
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn hops_doses(&self) -> Vec<HopsDose> {
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
        self.equipment.grain_absorption_per_kg * self.grain_weight().0
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
                    self.equipment.infusion_temperature,
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
                self.equipment.infusion_temperature,
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

    /*
    /// The product volume at the end
    #[must_use]
    pub fn product_volume(&self) -> Liters {
        self.post_ferment_volume() + self.post_ferment_dilution
    }
     */

    // product_volume = f(post_ferment_volume, post_ferment_dilution)

    /// The total amount of water input during the process
    #[must_use]
    pub fn total_water(&self) -> Liters {
        self.mash_volume() // strike plus infusions
            + self.sparge_volume()
            + self.partial_boil_dilution()
        // FIME plus post_ferment_dilution
    }

    /// Ingredient list
    #[must_use]
    pub fn ingredient_list_string(&self) -> String {
        let mut output: String = String::new();
        // FIXME
        //        writeln!(output, "Total Water: {}", self.total_water()).unwrap();
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
        output
    }
}
