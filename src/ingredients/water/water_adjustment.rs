use super::WaterProfile;
use crate::prelude::*;
use std::ops::Range;

/// Compute how much salt adds the specified ppm of the
/// specified ion
fn salt_concentration_for_ion(salt: Salt, ion: Ion, additional_ppm: Ppm) -> SaltConcentration {
    assert!(
        salt.ions().contains(&ion),
        "{salt} does not contain {ion:?}"
    );
    let ion_fraction = salt.ion_fraction(ion);
    let salt_ppm = additional_ppm / ion_fraction;
    SaltConcentration {
        salt,
        ppm: salt_ppm,
    }
}

/// A WaterAdjustment tool
#[derive(Debug, Clone)]
pub struct WaterAdjustment {
    profile: WaterProfile,
    salts_available: Vec<Salt>,
    sulfate_chloride_ratio_range: Range<f32>,
}

impl WaterAdjustment {
    /// Create a new WaterAdjustment tool with reasonable defaults
    #[must_use]
    pub fn new(
        profile: WaterProfile,
        sulfate_chloride_ratio_range: Range<f32>,
        salts_available: Vec<Salt>,
    ) -> WaterAdjustment {
        WaterAdjustment {
            profile,
            salts_available,
            sulfate_chloride_ratio_range,
        }
    }

    fn cl_target_range(&self) -> Range<Ppm> {
        self.profile.so4 / self.sulfate_chloride_ratio_range.start
            ..self.profile.so4 / self.sulfate_chloride_ratio_range.end
    }

    fn so4_target_range(&self) -> Range<Ppm> {
        self.profile.cl * self.sulfate_chloride_ratio_range.end
            ..self.profile.cl * self.sulfate_chloride_ratio_range.start
    }

    /// Salts needed to achieve what we want
    pub fn salts_needed(&mut self) -> Vec<SaltConcentration> {
        let mut output: Vec<SaltConcentration> = Vec::new();

        // If we need calcium
        if self.profile.ca < Ppm(50.0) {
            let salt = Salt::CalciumChloride;
            if self.salts_available.contains(&salt) {
                // If we can increase chloride, use CalciumChloride
                let cl_range = self.cl_target_range();
                if self.profile.cl < cl_range.end {
                    let amount_for_ca =
                        salt_concentration_for_ion(salt, Ion::Calcium, Ppm(50.0) - self.profile.ca)
                            .ppm
                            .0;

                    let max_amount_for_cl = salt_concentration_for_ion(
                        salt,
                        Ion::Chloride,
                        cl_range.end - self.profile.cl,
                    )
                    .ppm
                    .0;

                    let amt = amount_for_ca.min(max_amount_for_cl);

                    let saltconc = SaltConcentration {
                        salt,
                        ppm: Ppm(amt),
                    };

                    self.profile.add_salt(saltconc);
                    output.push(saltconc);
                }
            }

            let salt = Salt::Gypsum;
            if self.salts_available.contains(&salt) {
                // If we can increase sulfate, use Gypsum
                let so4_range = self.so4_target_range();
                if self.profile.so4 < so4_range.end {
                    let amount_for_ca =
                        salt_concentration_for_ion(salt, Ion::Calcium, Ppm(50.0) - self.profile.ca)
                            .ppm
                            .0;

                    let max_amount_for_so4 = salt_concentration_for_ion(
                        salt,
                        Ion::Sulfate,
                        so4_range.end - self.profile.so4,
                    )
                    .ppm
                    .0;

                    let amt = amount_for_ca.min(max_amount_for_so4);

                    let saltconc = SaltConcentration {
                        salt,
                        ppm: Ppm(amt),
                    };

                    self.profile.add_salt(saltconc);
                    output.push(saltconc);
                }
            }
        }

        let salt = Salt::TableSalt;
        if self.salts_available.contains(&salt) {
            // If we need chloride, use TableSalt
            let cl_range = self.cl_target_range();
            if self.profile.cl < cl_range.start {
                let amt = salt_concentration_for_ion(
                    salt,
                    Ion::Chloride,
                    cl_range.start - self.profile.cl,
                )
                .ppm
                .0;

                let saltconc = SaltConcentration {
                    salt,
                    ppm: Ppm(amt),
                };

                self.profile.add_salt(saltconc);
                output.push(saltconc);
            }
        }

        let salt = Salt::Epsom;
        if self.salts_available.contains(&salt) {
            // If we need sulfate, use Epsom
            let so4_range = self.so4_target_range();
            if self.profile.so4 < so4_range.start {
                let amt = salt_concentration_for_ion(
                    salt,
                    Ion::Sulfate,
                    so4_range.start - self.profile.so4,
                )
                .ppm
                .0;

                let saltconc = SaltConcentration {
                    salt,
                    ppm: Ppm(amt),
                };

                self.profile.add_salt(saltconc);
                output.push(saltconc);
            }
        }

        output
    }
}
