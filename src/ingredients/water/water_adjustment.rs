use super::WaterProfile;
use crate::prelude::*;

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
#[derive(Debug, Clone, Copy)]
pub struct WaterAdjustment {
    /// The starting water profile
    pub profile: WaterProfile,

    /// The pH of the (final) saccharification mash step, based on the
    /// malts only (as if the water were distilled).
    pub mash_ph_distilled: Ph,

    /// Target pH. Normally set this to 5.4
    pub target_ph: Ph,

    /// The ideal sulfate/chloride ratio
    pub sulfate_chloride_target: f32,
}

impl WaterAdjustment {
    /// Salts needed to achieve what we want
    pub fn salts_needed(&self) -> Vec<SaltConcentration> {
        // The running profile
        let mut profile = self.profile;

        // These are the salts we will be adding
        let mut salts: Vec<SaltConcentration> = Vec::new();

        let ra_desired: CaCO3 = {
            let ph_shift_desired = self.target_ph.0 - self.mash_ph_distilled.0;
            CaCO3(ph_shift_desired / 0.00168)
        };
        let ra_source_water = profile.residual_alkalinity();

        if ra_desired < ra_source_water {
            // Increase hardness to decrease RA
            // with calcium and magnesium

            // residual_alkalinity =
            //    profile.alkalinity_caco3 - (2.497 * ca / 3.5 + 4.118 * mg / 7.0)
            // but also Ca 50-150, mg 5-40... ratio being about 7x
            // ca = 7 * mg
            // Solving two equasions:
            let mg = (profile.alkalinity_caco3.0 - ra_desired.0) / 5.5822857;
            let ca = mg * 7.0;

            self.compute_cation_salts(&mut profile, &mut salts, Ppm(ca), Ppm(mg));
        } else if ra_desired > ra_source_water {
            // Increase Alkalinity to increase RA
            // with baking soda

            let alkalinity_desired: HCO3 = (ra_desired - ra_source_water).into();

            // we know this is 1:1 in baking soda so
            salts.push(SaltConcentration {
                salt: Salt::BakingSoda,
                ppm: Ppm(alkalinity_desired.0),
            });
        }

        salts
    }

    fn compute_cation_salts(
        &self,
        profile: &mut WaterProfile,
        salts: &mut Vec<SaltConcentration>,
        ca: Ppm,
        mg: Ppm,
    ) {
        // Achieve mg by adding Epsom salt
        while profile.mg < mg {
            let addition = salt_concentration_for_ion(Salt::Epsom, Ion::Magnesium, mg - profile.mg);
            profile.add_salt(addition);
            salts.push(addition);
        }

        // We use a dumb but working method to figure out how much of each salt
        // to add.  We just add 1ppm at a time of either salt depending on which
        // is best based on the current sulfate/chloride ratio.
        let mut gypsum_added: f32 = 0.0;
        let mut cacl_added: f32 = 0.0;
        while profile.ca < ca {
            if profile.sulfate_chloride_ratio() < self.sulfate_chloride_target {
                // add 1ppm gypsum
                let addition = SaltConcentration {
                    salt: Salt::Gypsum,
                    ppm: Ppm(1.0),
                };
                profile.add_salt(addition);
                gypsum_added += 1.0;
            } else {
                // add 1ppm calcium chloride
                let addition = SaltConcentration {
                    salt: Salt::CalciumChloride,
                    ppm: Ppm(1.0),
                };
                profile.add_salt(addition);
                cacl_added += 1.0;
            }
        }
        if gypsum_added > 0.0 {
            salts.push(SaltConcentration {
                salt: Salt::Gypsum,
                ppm: Ppm(gypsum_added),
            });
        }
        if cacl_added > 0.0 {
            salts.push(SaltConcentration {
                salt: Salt::CalciumChloride,
                ppm: Ppm(cacl_added),
            });
        }

        // If we need more chloride we could add
        // table salt, but this is for later TBD. FIXME.
    }
}
