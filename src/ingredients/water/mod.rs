use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Add;

/// Tool for adjusting water with salt
mod water_adjustment;
pub use water_adjustment::WaterAdjustment;

/// Water profile
#[allow(clippy::doc_markdown)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WaterProfile {
    /// Calcium Ca+2
    pub ca: Ppm,

    /// Magnesium Mg+2
    pub mg: Ppm,

    /// Sodium Na+
    pub na: Ppm,

    /// Sulfite SO4-2
    pub so4: Ppm,

    /// Chloride Cl-
    pub cl: Ppm,

    /// Alkalinity
    pub alkalinity_caco3: CaCO3,

    /// Acidity in pH
    pub ph: Ph,
}

impl WaterProfile {
    /// Munich (boiled)
    pub const MUNICH_BOILED: WaterProfile = WaterProfile {
        ca: Ppm(12.),
        mg: Ppm(17.),
        na: Ppm(4.),
        so4: Ppm(18.),
        cl: Ppm(8.),
        alkalinity_caco3: CaCO3(100.0 * 1.22),
        ph: Ph(7.5), // guess
                     // cations 2.2, anions 2.2
                     // hardness 100, alkalinity 82,
                     // ra 63, SO4/cl ratio 2.3
    };

    /// Weihenstephan
    pub const WEIHENSTEPHAN: WaterProfile = WaterProfile {
        ca: Ppm(15.),
        mg: Ppm(10.),
        na: Ppm(0.),
        so4: Ppm(17.),
        cl: Ppm(17.),
        alkalinity_caco3: CaCO3(46.0 * 1.22),
        ph: Ph(7.5), // guess
    };

    /// Water hardness on account of Calcium, permanent, in CaCO3 units
    #[must_use]
    pub fn calcium_water_hardness(&self) -> CaCO3 {
        // We multiply by the molecular weight of CaCO3
        // divided by the molecular weight of Ca
        // (40.078 + 12.0096 + 3.0 * 15.999) / 40.078 = 2.497(24..)
        CaCO3(self.ca.0 * 2.497)
    }

    /// Water hardness on account of Magnesium, permanent, in CaCO3 units
    #[must_use]
    pub fn magnesium_water_hardness(&self) -> CaCO3 {
        // We multiply by the molecular weight of CaCO3
        // divided by the molecular weight of Mg
        // (40.078 + 12.0096 + 3.0 * 15.999) / 24.304 = 4.118(03..)
        CaCO3(self.mg.0 * 4.118)
    }

    /// Total water hardness, permanent, in CaCO3
    #[must_use]
    pub fn water_hardness(&self) -> CaCO3 {
        self.calcium_water_hardness() + self.magnesium_water_hardness()
    }

    /// Effective water hardness as `CaCO3` ppm
    #[must_use]
    pub fn effective_water_hardness_caco3(&self) -> CaCO3 {
        // this is sometimes calculated as ca / 1.4 + mg / 1.7
        // but that calc is both converting to CaCO3 units and
        // computing the effective hardness in a single step.

        self.calcium_water_hardness() / 3.5 + self.magnesium_water_hardness() / 7.0
    }

    /// Compute residual alkalinity (RA)
    #[must_use]
    pub fn residual_alkalinity(&self) -> CaCO3 {
        // Defined by Kolbach as KH - ((CH - MH/2) / 3.5)
        //   KH = carbonate hardness, CH = calcium hardness, MH = magnesium hardness
        //   measured in dH units.
        // Every 10 dH of residual alkalinity adds 0.3 pH
        // 50ppm as CaCO3 = 2.81 dH

        self.alkalinity_caco3 - self.effective_water_hardness_caco3()
    }

    /// Hardness Alkalinity ratio
    #[must_use]
    pub fn hardness_alkalinity_ratio(&self) -> f32 {
        self.effective_water_hardness_caco3().0 / self.residual_alkalinity().0
    }

    /// Sulfate/Chloride ratio
    #[must_use]
    pub fn sulfate_chloride_ratio(&self) -> f32 {
        self.so4.0 / self.cl.0
    }

    /// Add salt
    #[allow(clippy::match_same_arms)]
    pub fn add_salt(&mut self, salt_conc: SaltConcentration) {
        let mut distinct_ions: Vec<Ion> = salt_conc.salt.ions().to_owned();
        distinct_ions.sort();
        distinct_ions.dedup();

        for ion in &distinct_ions {
            let ion_fraction = salt_conc.salt.ion_fraction(*ion);
            let ppm = Ppm(salt_conc.ppm.0 * ion_fraction);

            match ion {
                Ion::Hydrogen => {}  // lowers pH (TBD)
                Ion::Hydroxide => {} // raises pH (TBD)
                Ion::Water => {}     // no effect
                Ion::Bicarbonate => {
                    self.alkalinity_caco3 = self.alkalinity_caco3 + HCO3(ppm.0).into();
                }
                Ion::Sodium => self.na = self.na + ppm,
                Ion::Magnesium => self.mg = self.mg + ppm,
                Ion::Sulfate => self.so4 = self.so4 + ppm,
                Ion::Chloride => self.cl = self.cl + ppm,
                Ion::Calcium => self.ca = self.ca + ppm,
            }
        }
    }

    /// Add acid
    /// Not yet implemented, but will not error.
    pub fn add_acid(&mut self, _acid_conc: AcidConcentration) {
        /*
        for ion in acid_conc.acid.ions() {
            let ion_fraction = acid_conc.acid.ion_fraction(ion);
            let ppm = Ppm(acid_conc.ppm.0 * ion_fraction);

            match ion {
                Ion::Hydrogen => {
                    // FIXME
                },
                Ion::Hydroxide => {
                    // FIXME
                },
                Ion::Water => { }, // no effect
                Ion::Bicarbonate => {
                    // Convert into CaCO3 first
                    self.alkalinity_caco3 = self.alkalinity_caco3 + (ppm / 1.22);
                },
                Ion::Sodium => self.na = self.na + ppm,
                Ion::Magnesium => self.mg = self.mg + ppm,
                Ion::Sulfate => self.so4 = self.so4 + ppm,
                Ion::Chloride => self.cl = self.cl + ppm,
                Ion::Calcium => self.ca = self.ca + ppm,
            }
        }
        */
    }
}

impl Add for WaterProfile {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        WaterProfile {
            ca: self.ca + other.ca,
            mg: self.mg + other.mg,
            na: self.na + other.na,
            so4: self.so4 + other.so4,
            cl: self.cl + other.cl,
            alkalinity_caco3: self.alkalinity_caco3 + other.alkalinity_caco3,
            ph: Ph(10.0_f32.powf(-self.ph.0) + 10.0_f32.powf(-other.ph.0)),
        }
    }
}

impl fmt::Display for WaterProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ra = self.residual_alkalinity();
        write!(
            f,
            "Ca {:.0}   Mg {:.0}   Na {:.0}   SO4 {:.0}   Cl {:.0}   CaCO3 {:.0}   RA {:.0}",
            self.ca.0, self.mg.0, self.na.0, self.so4.0, self.cl.0, self.alkalinity_caco3.0, ra.0,
        )
    }
}
