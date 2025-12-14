use super::{Equipment, Recipe2, Warning};
use crate::prelude::*;
use crate::Packaging;
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

    /// The length of time before the beer is ready
    #[must_use]
    pub fn time_until_done(&self) -> Days {
        let mut conditioning = self.recipe.style.recommended_conditioning_time();

        if let Packaging::Bottle(_, _) = self.equipment.packaging {
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

    /// The weight of the malts in the mash
    #[must_use]
    pub fn grain_weight(&self) -> Kilograms {
        self.malt_doses().iter().map(|dose| dose.weight).sum()
    }

    /// Estimated mash pH
    #[must_use]
    pub fn mash_ph(&self) -> Ph {
        // FIXME: use Certificate of Analysis of malt to get wort pH
        // (we hard coded 5.4 below), combine different malts somehow.

        let residual_alkalinity = self.equipment.water_profile.residual_alkalinity().0;

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

    /// Strike temperature
    #[must_use]
    pub fn strike_temperature(&self) -> Celsius {
        crate::mash::strike_water_temp(
            self.strike_volume(),
            self.grain_weight(),
            self.equipment.room_temperature,
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

    /// Number of yeast cells needed for a good pitch
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn yeast_cells(&self) -> u64 {
        let ml: Milliliters = self.batch_size.into();
        let plato: Plato = self.recipe.original_gravity.into();
        let pitch_rate: u64 = self.recipe.style.yeast_pitching_rate();
        pitch_rate * (ml.0 * plato.0) as u64
    }

    /// Grams of yeast needed for pitch
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn yeast_grams(&self) -> Option<Grams> {
        if let Some((grams, liters)) = self.recipe.yeast.pitching_rate() {
            Some(grams * (self.batch_size.0 / liters.0))
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
                self.equipment.mash_efficiency * ppg * pounds.0 / gallons.0
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
            let ppm_needed = self.recipe.fan_requirement_of_yeast().0
                - self.fan_from_malt().0;

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

    /// The estimated gravity after fermentation, before any dilution
    // TODO: this doesn't adjust for the presence of unfermentable
    //       sugars
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
                return (natural_abv.0 / target_abv.0)
                    .min(self.recipe.max_post_ferment_dilution);
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
            writeln!(output, "Mash Infusion: +{infusion}     = {total}  mash volume").unwrap();
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

        total = total - self.equipment.kettle_losses;
        writeln!(
            output,
            "Kettle Losses: -{}     = {total}  post boil volume",
            self.equipment.kettle_losses
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
            self.post_fermentation_dilution_fraction()
        )
    }

    /// Final gravity
    #[must_use]
    pub fn final_gravity(&self) -> SpecificGravity {
        let points = self.post_ferment_gravity().0 - 1.0;
        let ratio = self.post_ferment_volume().0 / self.product_volume().0;
        SpecificGravity(1.0 + points * ratio)
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

    /// Compute IBU
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn ibu(&self) -> Ibu {
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

        let gallons: Gallons = self.batch_size.into();
        mcu /= gallons.0;

        // Morey equasion handles the non-linearity
        Srm(1.4922 * mcu.powf(0.6859))
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
        if self.pre_boil_volume() > self.equipment.max_kettle_volume {
            warnings.push(Warning::BoilKettleTooSmall {
                needed: self.pre_boil_volume(),
                available: self.equipment.max_kettle_volume,
            });
        }

        // Verify sparge volume is not negative
        if self.sparge_volume().0 < 0.0 {
            warnings.push(Warning::TooMuchMash {
                overfull: Liters(-self.sparge_volume().0),
                mash_thickness: self.recipe.mash_thickness,
            });
        }

        /*
        if self.process.room_temperature > Celsius(35.0) {
            errors.push(format!(
                "Room temp is unusually high: {}",
                self.process.room_temperature
            ));
        }
        if self.process.room_temperature < Celsius(10.0) {
            errors.push(format!(
                "Room temp is unusually low: {}",
                self.process.room_temperature
            ));
        }
        if self.strike_temperature() > Celsius(100.0) {
            errors.push(format!(
                "Strike temp {} is above boiling!",
                self.strike_temperature()
            ));
        }
        if self.strike_temperature() < Celsius(35.0) {
            errors.push(format!(
                "Strike temp {} is very low",
                self.strike_temperature()
            ));
        }
        if self.ferment_temperature < Celsius(6.0) {
            errors.push(format!(
                "Ferment temp {} is very low",
                self.ferment_temperature
            ));
        }
        if self.ferment_temperature > Celsius(35.0) {
            errors.push(format!(
                "Ferment temp {} is very high",
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

         */



        warnings
    }
}
