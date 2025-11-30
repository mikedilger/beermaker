use crate::prelude::*;
use serde::{Deserialize, Serialize};

/// A rest in a Mash
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MashRest {
    /// The temperature we are trying to achieve
    pub target_temperature: Celsius,

    /// How long to stay at this mash step temperature
    pub duration: Minutes,
}

/// Calculate an initial infusion
#[must_use]
pub(crate) fn strike_water_temp(
    strike_volume: Liters,
    grain_weight: Kilograms,
    grain_temp: Celsius,
    target_temp: Celsius,
) -> Celsius {
    /* INFUSION:
     *
     * Tw = (0.2 / R) * (T2 - T1) + T2
     *
     * Tw = strike water temp in F
     * R = water-to-grist ratio (quarts per pound)
     * T1 = initial temp of the grain in F
     * T2 = target mash temp in F
     */

    let strike_volume: Quarts = strike_volume.into();
    let grain_weight: Pounds = grain_weight.into();
    let grain_temp: Fahrenheit = grain_temp.into();
    let target_temp: Fahrenheit = target_temp.into();

    let r = strike_volume.0 / grain_weight.0;
    let t2 = target_temp.0;
    let t1 = grain_temp.0;

    let strike_water_temp_f = (0.2 / r) * (t2 - t1) + t2;

    Fahrenheit(strike_water_temp_f).into()
}

/// Calculate a mash infusion
#[must_use]
pub(crate) fn mash_infusion(
    grain_weight: Kilograms,
    current_water: Liters,
    start_temp: Celsius,
    target_temp: Celsius,
    infusion_temp: Celsius,
) -> Liters {
    /* MASH INFUSION
     *
     * Wa = (T2 - T1) * (0.2 G + Wm) / (Tw - T2)
     *
     * Wa = Volume of (near boiling) water added (in quarts)
     * Wm = Total volume of water in the mash (in quarts)
     * T1 = initial temp of mash (F)
     * T2 = target temp of mash (F)
     * Tw = actual temp of infusion water (F)
     * G = total grain weight (lbs)
     */

    let grain_weight: Pounds = grain_weight.into();
    let current_water: Quarts = current_water.into();
    let start_temp: Fahrenheit = start_temp.into();
    let target_temp: Fahrenheit = target_temp.into();
    let infusion_temp: Fahrenheit = infusion_temp.into();

    let t1 = start_temp.0;
    let t2 = target_temp.0;
    let g = grain_weight.0;
    let wm = current_water.0;
    let tw = infusion_temp.0;

    let wa = Quarts((t2 - t1) * (0.2 * g + wm) / (tw - t2));

    wa.into()
}

/// Calculate a reverse mash infusion
#[must_use]
pub(crate) fn reverse_mash_infusion(
    grain_weight: Kilograms,
    final_water: Liters,
    start_temp: Celsius,
    target_temp: Celsius,
    infusion_temp: Celsius,
) -> Liters {
    /* MASH INFUSION
     *
     * W1 = ( W2 * (T2 - Tinf) + 0.2 G * (T2 - T1) ) / (T1 - Tinf)
     *
     * W1 = Volume of mash before infusion (quarts)
     * W2 = Volume of mash after infusion (quarts)
     * Tinf = actual temp of infusion water (F)
     * G = total grain weight (lbs)
     * T1 = initial temp of mash (F)
     * T2 = target temp of mash (F)
     */

    let start_temp: Fahrenheit = start_temp.into();
    let target_temp: Fahrenheit = target_temp.into();
    let grain_weight: Pounds = grain_weight.into();
    let final_water: Quarts = final_water.into();
    let infusion_temp: Fahrenheit = infusion_temp.into();

    let w2 = final_water.0;
    let t1 = start_temp.0;
    let t2 = target_temp.0;
    let g = grain_weight.0;
    let tinf = infusion_temp.0;

    let w1 = Quarts(((w2 * (t2 - tinf)) + (0.2 * g * (t2 - t1))) / (t1 - tinf));
    let infusion_volume = final_water - w1;
    infusion_volume.into()
}
