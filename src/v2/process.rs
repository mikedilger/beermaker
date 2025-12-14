use crate::prelude::*;
use super::{Equipment, Recipe2};
use serde::{Deserialize, Serialize};

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

    /// Warnings
    warnings: Vec<String>,

    /// Errors
    errors: Vec<String>,
}

impl Process2 {
    // NOTE: we order these functions based on dependencies.
    // Each function only depends on things above it.

    /// Create a new process.
    ///
    /// Due to equipment limitations, the actual batch size may compute
    /// differently to the desired one.  If no size is specified, the
    /// maximum batch size will be computed.
    pub fn new(
        equipment: Equipment,
        recipe: Recipe2,
        batch_size: Liters,
    ) -> Process2 {
        Process2 {
            equipment,
            recipe,
            batch_size,
            warnings: vec![],
            errors: vec![],
        }
    }

    /// Water salts to adjust ions
    pub fn water_salts(&mut self) -> Vec<SaltConcentration> {

        let water_profile = self.equipment.water_profile;

        let mut output: Vec<SaltConcentration> = Vec::new();
        // tbd: right now we only push at most one salt to fix
        // the chloride-sulfate concentration.

        if let Some(range) = &self.recipe.chloride_sulfate_ratio_range {
            let current = water_profile.cl.0 / water_profile.so4.0;

            if current < range.start {
                // We need Chloride
                if let Some(salt) = self.equipment.chloride_salt() {

                    let cl_needed = range.start * water_profile.so4.0
                        - water_profile.cl.0;

                    let ion_fraction = salt.ion_fraction(Ion::Chloride);

                    let salt_ppm_needed = cl_needed / ion_fraction;

                    output.push(SaltConcentration {
                        salt,
                        ppm: Ppm(salt_ppm_needed)
                    });
                } else {
                    self.warnings.push(
                        format!("Chloride-Sulfate ratio is too low ({current}), and \
                                 we have no salt to fix it with.")
                    )
                }
            } else if current > range.end {
                // We need sulfate
                if let Some(salt) = self.equipment.sulfate_salt() {
                    let so4_needed = water_profile.cl.0 / range.end
                        - water_profile.so4.0;

                    let ion_fraction = salt.ion_fraction(Ion::Sulfate);

                    let salt_ppm_needed = so4_needed / ion_fraction;

                    output.push(SaltConcentration {
                        salt,
                        ppm: Ppm(salt_ppm_needed)
                    });

                } else {
                    self.warnings.push(
                        format!("Chloride-Sulfate ratio is too high ({current}), and \
                                 we have no salt to fix it with.")
                    )
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
    ///
    /// This is `&mut self` because it may add warnings.
    #[must_use]
    pub fn adjusted_water_profile(&mut self) -> WaterProfile {
        let mut profile = self.equipment.water_profile;

        for salt_conc in &self.water_salts() {
            profile.add_salt(*salt_conc);
        }

        for acid_conc in &self.water_acids() {
            profile.add_acid(*acid_conc);
        }

        profile
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

    /// The weight of the malts in the mash
    #[must_use]
    pub fn grain_weight(&self) -> Kilograms {
        self.malt_doses().iter().map(|dose| dose.weight).sum()
    }

    /// The water absorption of the malts
    #[must_use]
    pub fn water_absorption(&self) -> Liters {
        self.equipment.grain_absorption_per_kg * self.grain_weight().0
    }

    /// The mount of water that evaporates during the boil
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn boil_evaporation(&self) -> Liters {
        self.equipment.boil_evaporation_per_hour / 60.0 * self.recipe.boil_length.0 as f32
    }

    /// How much volume is required in the fermenter for head space?
    pub fn fermentation_head_space(&self) -> Liters {
        // In general 10-20% or 15-25% for aggressive ones
        // TBD: compute how aggressive the fermentation is
        // aggressive: high gravity, high temperature

        self.batch_size * 0.20
    }

    /// Chosen fermenter volume
    //
    /// This is `&mut self` because it may add errors.
    pub fn fermenter_volume(&mut self) -> Option<Liters> {
        let needed = self.batch_size + self.fermentation_head_space();

        // Choose the smallest fermenter that handles this
        let chosen = self.equipment.fermenters.iter()
            .map(|&f| f)
            .filter(|&f| f >= needed)
            .min();

        if chosen.is_none() {
            self.errors.push(
                format!("You don't have a fermenter large enough for this batch size.")
            );
        }

        chosen
    }

    // BUT  we do not know the pre_boil_volume, and can't also calculate it from partial
    //      boil dilution.

    /// Partial boil dilution
    pub fn partial_boil_dilution(&self) -> Liters {
        if self.batch_size > self.equipment.max_kettle_volume && self.recipe.allow_partial_boil_dilution {
            self.batch_size
                + self.equipment.kettle_losses
                + self.boil_evaporation()
                - self.pre_boil_volume()
        } else {
            Liters(0.0)
        }
    }




    // post_boil_volume = f(ferment_volume, partial_boil_dilution)





    /// Post ferment volume
    pub(crate) fn post_ferment_volume(&self) -> Liters {
        self.batch_size * self.recipe.ferment_loss_fraction()
    }

    /// Ferment losses
    pub(crate) fn ferment_losses(&self) -> Liters {
        self.batch_size - self.post_ferment_volume()
    }


    // GIVENS
    //

    //
    // max_kettle_volume <-- specified
    // kettle_losses <-- specified


    // pre_sparge_volume = f(grain_weight, mash_thickness, water_absorption)

    // strike_volume = f(mash_volume, mash_infusions)

    // mash_infusions = f(strike_volume, grain_weight, mash_rests,
    //                    infusion_temperature)

    // mash_volume = f(pre_sparge_volume, water_absorption)


    // water_absorption = f(grain_weight)
    // 0.15 gallons per pound hard coded
    // but also process.grain_absorption_per_kg exists somewhere too.





    // pre_boil_volume = f(post_boil_pre_loss_volume, boil_evaporation)

    // sparge_volume = f(pre_boil_volume, pre_sparge_volume)

    // post_boil_pre_loss_volume = f(post_boil_volume, kettle losses)

    // post_boil_volume = f(ferment_volume, partial_boil_dilution)

    // fermenter_volumes <-- specified

    // post_ferment_volume = f(ferment_volume, ferment_loss_fraction)

    // lagering_volumes <-- specified

    // product_volume = f(post_ferment_volume, post_ferment_dilution)




}
