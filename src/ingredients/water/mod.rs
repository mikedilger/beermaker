use crate::prelude::*;
use serde::{Deserialize, Serialize};

mod profile;
pub use profile::WaterProfile;

/// Tool for adjusting water with salt
mod water_adjustment;
pub use water_adjustment::WaterAdjustment;

/// A Volume of Water
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WaterVolume {
    /// How much
    pub volume: Liters,

    /// Water Profile
    pub profile: WaterProfile,

    /// At what temperature
    pub temp: Celsius,
}

impl WaterVolume {
    /// Add a dose of salt to this water
    pub fn add_salt(&mut self, salt_dose: SaltDose) {
        self.profile.add_salt(SaltConcentration {
            salt: salt_dose.salt,
            ppm: Ppm(salt_dose.mg.0 / self.volume.0),
        });
    }
}
