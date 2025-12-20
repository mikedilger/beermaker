use super::WaterProfile;
use crate::prelude::*;
use std::ops::Range;

/// Compute how much salt adds the specified ppm of the
/// specified ion
fn salt_concentration_for_ion(salt: Salt, ion: Ion, additional_ppm: Ppm) -> SaltConcentration {
    if !salt.ions().contains(&ion) {
        panic!("{} does not contain {:?}", salt, ion);
    }
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

    /// The target concentration of Calcium
    pub ca_target: Ppm,

    /// The target concentration of Chloride
    pub cl_target: Ppm,

    /// The target concentration of Sulfate
    pub so4_target: Ppm,
}

impl WaterAdjustment {
    /// Create a new WaterAdjustment tool with reasonable default
    /// Ion desires (you can edit them after).
    pub fn new(
        profile: WaterProfile,
        chloride_sulfate_ratio_range: Option<Range<f32>>,
        salts_available: Vec<Salt>,
    ) -> WaterAdjustment {
        let mut cl_target = profile.cl;
        let mut so4_target = profile.so4;

        if let Some(range) = &chloride_sulfate_ratio_range {
            // Cl and SO4 ratio should achieve the range specified
            let start_ratio = profile.cl.0 / profile.so4.0;
            if start_ratio < range.start {
                cl_target = profile.so4 * range.start;
            } else if start_ratio > range.end {
                so4_target = profile.cl / range.end;
            }
        }

        WaterAdjustment {
            profile,
            salts_available,
            ca_target: Ppm(50.0),
            cl_target,
            so4_target,
        }
    }

    /// Salts needed to achieve what we want
    pub fn salts_needed(&mut self) -> Vec<SaltConcentration> {
        let mut output: Vec<SaltConcentration> = Vec::new();

        // If we need both Ca and Cl, add CalciumChloride
        {
            let salt = Salt::CalciumChloride;

            if self.ca_target > self.profile.ca
                && self.cl_target > self.profile.cl
                && self.salts_available.contains(&salt)
            {
                let amount_for_ca = salt_concentration_for_ion(
                    salt,
                    Ion::Calcium,
                    self.ca_target - self.profile.ca,
                )
                .ppm
                .0;

                let amount_for_cl = salt_concentration_for_ion(
                    salt,
                    Ion::Chloride,
                    self.cl_target - self.profile.cl,
                )
                .ppm
                .0;

                let min = amount_for_ca.min(amount_for_cl);

                let saltconc = SaltConcentration {
                    salt: salt,
                    ppm: Ppm(min),
                };

                self.profile.add_salt(saltconc);
                output.push(saltconc);
            }
        }

        // If we need Ca and SO4, add Gypsum
        {
            let salt = Salt::Gypsum;

            if self.ca_target > self.profile.ca
                && self.so4_target > self.profile.so4
                && self.salts_available.contains(&salt)
            {
                let amount_for_ca = salt_concentration_for_ion(
                    salt,
                    Ion::Calcium,
                    self.ca_target - self.profile.ca,
                )
                .ppm
                .0;

                let amount_for_so4 = salt_concentration_for_ion(
                    salt,
                    Ion::Sulfate,
                    self.so4_target - self.profile.so4,
                )
                .ppm
                .0;

                let min = amount_for_ca.min(amount_for_so4);

                let saltconc = SaltConcentration {
                    salt: salt,
                    ppm: Ppm(min),
                };

                self.profile.add_salt(saltconc);
                output.push(saltconc);
            }
        }

        // If we still need Cl, add TableSalt
        {
            let salt = Salt::TableSalt;

            if self.cl_target > self.profile.cl && self.salts_available.contains(&salt) {
                let amount_for_cl = salt_concentration_for_ion(
                    salt,
                    Ion::Chloride,
                    self.cl_target - self.profile.cl,
                )
                .ppm;

                let saltconc = SaltConcentration {
                    salt: salt,
                    ppm: amount_for_cl,
                };

                self.profile.add_salt(saltconc);
                output.push(saltconc);
            }
        }

        // If we still need SO4, add Epsom
        {
            let salt = Salt::Epsom;

            if self.so4_target > self.profile.so4 && self.salts_available.contains(&salt) {
                let amount_for_so4 = salt_concentration_for_ion(
                    salt,
                    Ion::Sulfate,
                    self.so4_target - self.profile.so4,
                )
                .ppm;

                let saltconc = SaltConcentration {
                    salt: salt,
                    ppm: amount_for_so4,
                };

                self.profile.add_salt(saltconc);
                output.push(saltconc);
            }
        }

        output
    }
}
