use crate::prelude::*;
use crate::{Packaging, Process};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

/// Instructions for each major step of the process.
///
/// These instructions can have values substituted in, see the
/// source code file.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Steps {
    /// Header
    pub header: Vec<String>,

    /// Acquisition
    pub acquire: Vec<String>,

    /// Preparation
    pub prep: Vec<String>,

    /// Mashing
    pub mash: Vec<String>,

    /// The boil
    pub boil: Vec<String>,

    /// Chilling the wort
    pub chill: Vec<String>,

    /// Moving to fermenter and pitching the yeast
    pub pitch: Vec<String>,

    /// Fermenting
    pub ferment: Vec<String>,

    /// Packaging
    pub package: Vec<String>,
}

impl Steps {
    /// Merge other steps in (at the end)
    pub fn merge(&mut self, other: &Steps) {
        self.header.extend_from_slice(&other.header);
        self.acquire.extend_from_slice(&other.acquire);
        self.prep.extend_from_slice(&other.prep);
        self.mash.extend_from_slice(&other.mash);
        self.boil.extend_from_slice(&other.boil);
        self.chill.extend_from_slice(&other.chill);
        self.pitch.extend_from_slice(&other.pitch);
        self.ferment.extend_from_slice(&other.ferment);
        self.package.extend_from_slice(&other.package);
    }

    /// Prefix each step with a string
    pub fn prefix(&mut self, prefix: &str) {
        for step in &mut self.header {
            *step = format!("{prefix}: {step}");
        }
        for step in &mut self.acquire {
            *step = format!("{prefix}: {step}");
        }
        for step in &mut self.prep {
            *step = format!("{prefix}: {step}");
        }
        for step in &mut self.mash {
            *step = format!("{prefix}: {step}");
        }
        for step in &mut self.boil {
            *step = format!("{prefix}: {step}");
        }
        for step in &mut self.chill {
            *step = format!("{prefix}: {step}");
        }
        for step in &mut self.pitch {
            *step = format!("{prefix}: {step}");
        }
        for step in &mut self.ferment {
            *step = format!("{prefix}: {step}");
        }
        for step in &mut self.package {
            *step = format!("{prefix}: {step}");
        }
    }
}

/// Print a process in full detail to a String.
///
/// If you pass in custom_steps, they will be the first steps in each
/// section, followed by the standard steps.
#[must_use]
#[allow(clippy::similar_names)]
#[allow(clippy::too_many_lines)]
pub fn print_process(
    process: &Process,
    custom_steps: Option<Steps>,
    char_width: Option<usize>,
) -> String {
    let char_width = char_width.unwrap_or(78);

    let mut steps = custom_steps.unwrap_or_default();
    steps.prefix("CUSTOM");

    if let Some(more_steps) = &process.recipe.custom_steps {
        let mut more_steps = more_steps.clone();
        more_steps.prefix(&process.recipe.name);
        steps.merge(&more_steps);
    }

    // Local variables for format! substitutions

    let style = process.recipe.style;
    let time_until_done = process.time_until_done();
    let batch_size = process.batch_size;
    let fermenter = process.fermenter_volume();
    let mash_ph = process
        .mash_ph()
        .iter()
        .map(|s| format!("{s}"))
        .collect::<Vec<_>>()
        .join(", ");
    let mut mash_thicknesses = String::new();
    for f in process.mash_thicknesses() {
        let _ = write!(mash_thicknesses, "{f:.1}L/kg, ");
    }
    let wort_fan = process.wort_fan();
    let yeast_amount = if let Some(g) = process.yeast_grams() {
        format!("{g}")
    } else {
        format!("{} billion cells", process.yeast_cells() / 1_000_000_000)
    };
    let yeast_max_temperature = process.recipe.yeast.temp_range().end;
    let ibu = process.bitterness();
    let min_ibu = process.recipe.style.bitterness_range().start.0;
    let max_ibu = process.recipe.style.bitterness_range().end.0;
    let color = process.color();
    let min_color = process.recipe.style.color_range().start.0;
    let max_color = process.recipe.style.color_range().end.0;
    let og = process.recipe.original_gravity;
    let min_og = process.recipe.style.original_gravity_range().start.0;
    let max_og = process.recipe.style.original_gravity_range().end.0;
    let fg = process.final_gravity();
    let min_fg = process.recipe.style.final_gravity_range().start.0;
    let max_fg = process.recipe.style.final_gravity_range().end.0;
    let abv = process.abv();
    let min_abv = process.recipe.style.abv_range().start;
    let max_abv = process.recipe.style.abv_range().end;
    let ice_weight = process.brewery.ice_weight();
    let ice_bath_volume = process.brewery.chilled_water_volume();
    let total_water_volume = process.total_water();
    let water_doses = process.water_doses();
    let adjusted_water_profile = process.adjusted_water_profile();
    let ingredient_list = process.ingredient_list_string();
    let strike_volume = process.strike_volume();
    let strike_temp = process.strike_temperature();
    let infusion_temp = process.brewery.infusion_temperature;
    let sparge_volume = process.sparge_volume();
    let pre_boil_gravity = process.pre_boil_gravity();
    let boil_minutes = process.recipe.boil_length;
    let hops_additions = process.hops_additions_string();
    let whirlfloc = if process.recipe.fining_desired {
        process.whirlfloc_amount()
    } else {
        0.0
    };
    let yeast_nutrient = process.yeast_nutrient_amount();
    let zn = process.zinc_needed();
    let post_boil_pre_loss_volume = process.post_boil_pre_loss_volume();
    let partial_boil_dilution = process.partial_boil_dilution();
    let fermentation_temp = process.recipe.ferment_temperature;
    let yeast = process.recipe.yeast;
    let fermentation_time = process.recipe.fermentation_time();
    let lagering_time = process.recipe.style.recommended_conditioning_time();
    let diacetyl_rest_temp = process.recipe.diacetyl_rest_temperature();
    let post_ferment_dilution = process.post_fermentation_dilution();
    let bottles_nz = (process.product_volume().0 / 0.330).floor();
    let bottles_eu = (process.product_volume().0 / 0.500).floor();
    let bottles_large = (process.product_volume().0 / 0.750).floor();

    // -- header ------------

    let old_header = steps.header;
    steps.header = Vec::new();

    steps.header.push(format!(
        "Recipe for {recipe_name}\n(generated by the beermaker)\n",
        recipe_name = &process.recipe.name,
    ));

    steps.header.push(format!(
        "Specification:\n  \
             Style:            {style}\n  \
             Batch size:       {batch_size}\n  \
             Days:             {time_until_done}\n  \
             Fermenter:        {fermenter}\n  \
             Ferment Temp:     {fermentation_temp}\n  \
             Mash pH:          {mash_ph}\n  \
             Mash Thicknesses: {mash_thicknesses}\n  \
             Wort FAN:         {wort_fan}\n  \
             Yeast Pitch:      {yeast_amount}\n  \
             Bitterness:       {ibu}   [style: {min_ibu:.1} .. {max_ibu:.1}]\n  \
             Color:            {color}    [style: {min_color:.1} .. {max_color:.1}]\n  \
             Original Gravity: {og} [style: {min_og:.3} .. {max_og:.3}]\n  \
             Final Gravity:    {fg} [style: {min_fg:.3} .. {max_fg:.3}]\n  \
             ABV:              {abv}       [style: {min_abv:.1} .. {max_abv:.1}]\n  \
             Bottles:          {bottles_nz}x330ml {bottles_eu}x500ml {bottles_large}x750ml\n",
    ));

    steps.header.push(format!(
        "Volume History:\n{}",
        &indent(&process.volume_history_string(), 2, char_width)
    ));

    steps.header.push(format!(
        "Grain Bill:\n{}",
        &indent(&process.grain_bill_string(), 2, char_width)
    ));

    steps.header.extend(old_header);

    // -- acquire ------------

    steps
        .acquire
        .push(format!("Aquire all ingredients:\n\n{ingredient_list}"));

    let mut bits: String = "Acquire sanitizer, iodine (optional), yeast nutrient".to_string();
    if process.recipe.fining_desired {
        bits.push_str(", whirlfloc, fining agent");
    }
    steps.acquire.push(bits);

    if process.brewery.ice_bath {
        steps.acquire.push(format!(
            "Acquire {ice_weight} of ice. Also place a good part of \
                 {ice_bath_volume} of tap water into refrigerator to chill for \
                 ice bath usage.",
        ));
    }

    steps.acquire.push(format!(
        "Acquire {total_water_volume} of water of the type specified in the \
             recipe."
    ));

    steps
        .acquire
        .push(format!("You will need {yeast_amount} of {yeast}."));

    if !process.recipe.yeast.is_dry() {
        steps
            .acquire
            .push("You will need to start a yeast starter the day before.".to_string());
    }

    steps.acquire.push(
        "Set the temperature on the fermentation chamber so it has time to \
               get there."
            .to_string(),
    );

    // -- prep ------------

    steps.prep.push("Calibrate the pH meter.".to_string());

    steps.prep.push(format!(
        "Dose the full {total_water_volume} of source water as follows:\n\
             \n{water_doses}\n\nThis Yields:\n\n{adjusted_water_profile}"
    ));

    steps.prep.push(
        "Weigh out malts. Assemble all other ingredients and other \
               materials."
            .to_string(),
    );

    steps
        .prep
        .push("Clean up the area, make space and clean it.".to_string());

    steps.prep.push(
        "Assemble all equipment for the mash, boil, and ferment including:\n\
         Sanitizer, spray bottle of sanitizer, bowl for sanitizer, \
         scale, thermomemter, pH meter, graduated cylinder, hydrometer or \
         refractometer, turkey baster or sample pipet, ladel, funnel, timer, \
         fermenter, kettle, kettle lid \
         stirrer, rest for stirrer, mash tun, sparging equipment, \
         boiler for strike/infusion water, etc."
            .to_string(),
    );

    steps
        .prep
        .push("Sanitize equipment now, or during the mash.".to_string());

    steps
        .acquire
        .push("Verify the temperature on the fermentation chamber.".to_string());

    // -- mash ------------

    steps.mash.push(format!(
        "Fill the mash tun with {strike_volume} of {strike_temp} treated source water."
    ));

    if process.recipe.mash_rests.len() > 1 {
        steps
            .mash
            .push("Since we are doing a step mash, boil water for step additions.".to_string());
    }

    steps
        .mash
        .push("Add the mashable malts (see grain bill).".to_string());

    steps.mash.push("Start the timer.".to_string());

    steps
        .mash
        .push("Stir well, then take the temperature and record it.".to_string());

    steps
        .mash
        .push("Remove a sample and let it cool.".to_string());

    steps.mash.push(
        "Start to prepare sparge water. If you boil it now \
                     it might be cooled enough when sparge happens."
            .to_string(),
    );

    let infusions = process.mash_infusions();
    for (i, rest) in process.recipe.mash_rests.iter().enumerate() {
        let temp = rest.target_temperature;
        let dur = rest.duration;

        if i == 0 {
            steps
                .mash
                .push(format!("Hold the mash at {temp} for {dur}."));
        } else {
            steps.mash.push(format!(
                "Infuse {} of {infusion_temp} into the mash.",
                infusions[i - 1]
            ));

            steps
                .mash
                .push(format!("Hold the mash at {temp} for {dur}."));
        }
    }

    steps.mash.push(
        "You can exit the mash early if an iodine test indicates there \
         is no more starch."
            .to_string(),
    );

    steps.mash.push(
        "Mash out by raising the temperature to 77°C and hold for 5 to \
               10 minutes."
            .to_string(),
    );

    steps
        .mash
        .push("Take the pH of the sample that cooled and record it.".to_string());

    steps.mash.push(
        "Vorlauf: Lauter out of the mash tun into a jug, pouring back into the \
               mash tun, until the wort runs clear. The clearer the wort the better. \
               Solids that end up in the fermenter usually taste bad."
            .to_string(),
    );

    steps
        .mash
        .push("Lauter the first runnings into the boil kettle.".to_string());

    steps.mash.push(format!(
        "Batch sparge the mash with {sparge_volume} water of about 77°C, stir it well."
    ));

    steps.mash.push(
        "Vorlauf again: Lauter out of the mash tun into a jug, pouring back into \
               the mash tun, until the wort runs clear. The clearer the wort the better. \
               Solids that end up in the fermenter usually taste bad."
            .to_string(),
    );

    steps
        .mash
        .push("Lauter the second runnings into the boil kettle.".to_string());

    steps.mash.push("Discard the grains.".to_string());

    // -- boil ------------

    if !process.recipe.sugars.is_empty() {
        steps
            .boil
            .push("Mix into the boil kettle the fermentable sugars (see grain bill).".to_string());
    }

    steps.boil.push(format!(
        "Take a sample of the wort into a temperature-safe container \
         and let it cool to below 49°C.  Then measure and record the \
         pre-boil Specific Gravity.  The actual correct gravity \
         can be determined by using the hydrometer_correct binary:\n\
         'cargo run --bin hydrometer_correct'\n\
         The target temp-correct pre-boil gravity is {pre_boil_gravity}"
    ));

    steps
        .boil
        .push("Turn on the ventilation hood, full blast.".to_string());

    steps
        .boil
        .push("Bring the wort up to a boil.  When it boils, start the boil timer.".to_string());

    steps
        .boil
        .push("Optionally at hot-break, skim off and discard the protein foam.".to_string());

    steps
        .boil
        .push("Maintain the boil at a rapid rolling boil for the duration.".to_string());

    steps
        .boil
        .push(format!("We will be boiling for {boil_minutes}."));

    steps.boil.push(format!(
        "At various times, add hops:\n\
             \
             {hops_additions}"
    ));

    if process.recipe.fining_desired {
        steps.boil.push(format!(
            "At 10 minutes before the end of the boil, add \
                 {whirlfloc} whirlfloc tablets."
        ));
    }

    if yeast_nutrient > Grams(0.0) {
        steps.boil.push(format!(
            "At 10 minutes before the end of the boil, add \
             {yeast_nutrient} of yeast nutrient."
        ));
    } else {
        steps.boil.push(format!(
            "Do not add yeast nutrient. Instead, at 10 minutes before \
             the end of the boil, add {zn} of zinc."
        ));
    }

    if process.brewery.ice_bath {
        steps.boil.push(
            "Prepare the ice bath before the boil is complete. \
             Place all the prepared chilled water and ice into the bath."
                .to_string(),
        );
    }

    steps.boil.push(format!(
        "Verify Volume\n\n\
         At this point, make sure the volume is approaching the \
         {post_boil_pre_loss_volume}.\n\n\
         If the volume is too high, you can:\n\
         a) discard some but this will lower the gravity\n\
         b) boil longer, but this will change the hop character\n\n\
         If the volume is too low, you can:\n\
         a) add more boiling water, bring back to a boil briefly.\n\n\
         In any case, write down what happened and so that the recipe can be \
         adjusted for future runs."
    ));

    steps
        .boil
        .push(format!("After {boil_minutes}, turn off the burner."));

    // -- chill ------------

    if partial_boil_dilution > Liters(0.0) {
        steps.chill.push(format!(
            "Dilute the wort with {partial_boil_dilution} of \
                     boiled-then-cooled water"
        ));
    }

    if process.recipe.style.fermentation() == Fermentation::Lager {
        steps.chill.push(
            "Rapid chilling is important for multiple reasons to avoid to \
             off-flavors (including DMS), contamination, and drop haze \
             proteins for clarity."
                .to_string(),
        );
    } else {
        steps.chill.push(
            "Rapid chilling is important for multiple reasons to avoid to \
             off-flavors (including DMS) and contamination."
                .to_string(),
        );
    }

    steps.chill.push(
        "After the wort drops below 62C, it is no longer Pasteurized and can \
               become infected. Sanitization is now important."
            .to_string(),
    );

    if process.brewery.ice_bath {
        steps.chill.push(
            "Remove the kettle from the stove and dunk into the ice bath to rapidly \
             chill in the ice bath."
                .to_string(),
        );

        steps
            .chill
            .push("Stir the wort. Also stir the ice.".to_string());

        steps
            .chill
            .push("Put the lid on when you are not stirring.".to_string());
    } else {
        steps
            .chill
            .push("Chill the wart according to your setup and equipment".to_string());
    }

    steps
        .chill
        .push("Sanitize the fermenter and any equipment used for transfer.".to_string());

    steps.chill.push(
        "Before or after the wort has completely cooled, transfer the wort into the \
               fermenter.  Be careful to transfer as little of the protein break and \
               other trub solids into the fermenter as possible. They usually taste \
               bad."
            .to_string(),
    );

    steps
        .chill
        .push("Chill until the temperature gets to 20°C.".to_string());

    steps.chill.push(format!(
        "Original Gravity Reading\n\n\
             When the temperature is down to 20°C, take an Original Gravity reading. \
             Optionally return the sample after testing. Target is {og}.\n\n\
             If the calculator is needed it is at \n\
             ( 'cargo run --bin hydrometer_correct' )."
    ));

    steps.chill.push(format!(
        "Chill further until fermentation temperature is reached, which \
             is {fermentation_temp}"
    ));

    // -- pitch ------------

    if process.recipe.yeast.is_dry() {
        steps.pitch.push("Wort oxygenation is not required for dry yeast since they have plenty of sterols already.".to_string());
    } else {
        steps.pitch.push("Oxygenate the wort.".to_string());
    }

    steps.pitch.push(format!(
        "Verify the wort temperature is below {yeast_max_temperature}.",
    ));

    steps
        .pitch
        .push(format!("Pitch {yeast_amount} of {yeast}.",));

    // -- ferment ------------

    steps
        .ferment
        .push("Close the fermenter and install or setup airlock.".to_string());

    steps.ferment.push(format!(
        "Place the fermenter under temperature control. We need it to \
         be and remain at {fermentation_temp} for about {fermentation_time}."
    ));

    steps.ferment.push(
        "Keep an eye on fermentation. At some point it will start to \
         slow down."
            .to_string(),
    );

    steps.ferment.push(format!(
        "Diacetyl rest: As soon as it starts to slow, or when gravity is 2-5 \
         points above {fg}, do a 2 day diacetyl rest at {diacetyl_rest_temp}, or \
         just let it ferment on the trub at {fermentation_temp} for 3-5 days \
         after fermentation stops."
    ));

    steps.ferment.push(
        "Forced diacetyl test: Take a sample of beer, heat it to 66 C in \
         a water bath for 20 minutes. Then let it cool back to room temperature. \
         Smell and taste it. If it has diacetyl then let the beer ferment for \
         another day and try again."
            .to_string(),
    );

    steps.ferment.push(format!(
        "Final Gravity Reading: Measure the final gravity. Return sample to carboy. \
         Target is {fg}",
    ));

    if lagering_time > Days(28) {
        steps.ferment.push(
            "With sanitized equipment, rack off the trub from primary \
             fermenter to a secondary fermenter."
                .to_string(),
        );
    }

    if process.recipe.fining_desired {
        steps.ferment.push("Fining: Add fining agent.".to_string());
    }

    if process.recipe.style.conditioning() == Conditioning::Lagered {
        match process.recipe.style.origin() {
            StyleOrigin::American => {
                steps.ferment.push(format!(
                    "Crash the temperature down to  0°C - 1°C, and then hold \
                     at this low temperature for {lagering_time}. Be aware that without \
                     taking some kind of remedial action, the fermenter will suck in \
                     whatever is in your airlock and a bunch of atmosphere (with oxygen) \
                     as the cooling creates a vacuum. \
                     So consider these: Replace sanitizer in the airlock with strong alcohol; \
                     Apply continuous low pressure CO2; use a Co2-filled balloon as the \
                     airlock; use a blow-off tube long enough that the water wont be sucked \
                     all the way into the fermenter."
                ));
            }
            _ => {
                steps.ferment.push(format!(
                    "Slowly lower the temperature by 1°C per day until you get near to \
                     the lagering temperature range of 4°C - 7°C. Hold at \
                     this low temperature for {lagering_time}."
                ));
            }
        }
    }

    if post_ferment_dilution > Liters(0.0) {
        steps.ferment.push(format!(
            "Dilute the fermented beer with {post_ferment_dilution} \
                     of boiled-then-cooled water."
        ));
    }

    // -- package ------------

    if let Packaging::Bottle(bottle_volume, sugar) = process.brewery.packaging {
        steps.package.push(
            "Sanitize all equipment including siphon racking cane and tube, bottles, \
             sampler and measuring devices."
                .to_string(),
        );

        steps.package.push(
            "Take second Final Gravity reading. Return the sample to the secondary fermenter."
                .to_string(),
        );

        // This is pretty nutty IMHO
        // steps.package.push(format!(
        //"Measure the temperature of the beer, and adjust the priming \
        //sugar quantity, or follow the for the priming sugar \
        //amount (and type).",
        //));

        let total_priming_amount = sugar.priming_amount(
            process.recipe.style.carbonation_volume(),
            process.product_volume(),
            process.brewery.room_temperature,
        );

        steps.package.push(format!(
            "If priming the entire batch at once, which you can do if you are \
             now using a secondary fermenter, or if you use a bottling bucket, \
             then mix in {total_priming_amount} of {sugar}. \
             Try not to oxygenate, but do mix in \
             the sugar until fully dissolved and distributed.",
        ));

        let bottle_priming_amount = sugar.priming_amount(
            process.recipe.style.carbonation_volume(),
            bottle_volume,
            process.brewery.room_temperature,
        );

        let num_bottles = (process.product_volume().0 / bottle_volume.0).ceil();

        steps.package.push(format!(
            "If priming each bottle separately, add {bottle_priming_amount} \
             of {sugar} to each bottle. Expect to fill up to \
             {num_bottles} bottles.",
        ));

        // This is a bit nutty IMHO too
        //steps.package.push(format!(
        //"Mix in the priming yeast. This should be the same yeast as used \
        //for fermentation, but a small quantity.",
        //));

        steps.package.push(format!(
            "Rack the beer into up to {num_bottles}x {bottle_volume} bottles, \
             and then cap them.",
        ));

        steps.package.push(
            "Bottle Conditioning: Leave all bottles at room temperature, in a container \
             that can catch spills, and cover with a towel in case any bottle happens \
             to explode or fountain.  Leave for two weeks."
                .to_string(),
        );
    } else {
        let carb_volume = process.recipe.style.carbonation_volume();
        steps.package.push(format!(
            "Kegging instructions are TBD. Carbonation volume target is {carb_volume}"
        ));
    }

    steps.package.push("The beer is done.".to_string());

    // -------------------------------

    let mut output = String::new();

    for block in &steps.header {
        output.push_str(&indent(block, 0, char_width));
        output.push('\n');
    }

    header(&mut output, "ACQUIRE", char_width);
    for (i, block) in steps.acquire.iter().enumerate() {
        label(&mut output, "ACQUIRE", i + 1, block, char_width);
        output.push('\n');
    }
    header(&mut output, "PREP", char_width);
    for (i, block) in steps.prep.iter().enumerate() {
        label(&mut output, "PREP", i + 1, block, char_width);
        output.push('\n');
    }
    header(&mut output, "MASH", char_width);
    for (i, block) in steps.mash.iter().enumerate() {
        label(&mut output, "MASH", i + 1, block, char_width);
        output.push('\n');
    }
    header(&mut output, "BOIL", char_width);
    for (i, block) in steps.boil.iter().enumerate() {
        label(&mut output, "BOIL", i + 1, block, char_width);
        output.push('\n');
    }
    header(&mut output, "CHILL", char_width);
    for (i, block) in steps.chill.iter().enumerate() {
        label(&mut output, "CHILL", i + 1, block, char_width);
        output.push('\n');
    }
    header(&mut output, "PITCH", char_width);
    for (i, block) in steps.pitch.iter().enumerate() {
        label(&mut output, "PITCH", i + 1, block, char_width);
        output.push('\n');
    }
    header(&mut output, "FERMENT", char_width);
    for (i, block) in steps.ferment.iter().enumerate() {
        label(&mut output, "FERMENT", i + 1, block, char_width);
        output.push('\n');
    }
    header(&mut output, "PACKAGE", char_width);
    for (i, block) in steps.package.iter().enumerate() {
        label(&mut output, "PACKAGE", i + 1, block, char_width);
        output.push('\n');
    }

    output
}

fn header(output: &mut String, label: &str, char_width: usize) {
    output.push_str("---");
    output.push_str(label);
    output.push_str(&"-".repeat(char_width - label.len() - 3));
    output.push_str("\n\n");
}

fn label(output: &mut String, label: &str, step: usize, s: &str, char_width: usize) {
    use std::fmt::Write;

    let pre_output_len = output.len();
    output.push_str(label);
    write!(output, "-{step:02}: ").unwrap();
    let prefix_len = output.len() - pre_output_len;

    let sublines = textwrap::wrap(s, char_width - prefix_len);
    let mut virgin = true;
    for subline in sublines {
        if !virgin {
            write!(output, "{}", " ".repeat(prefix_len)).unwrap();
        }
        writeln!(output, "{subline}").unwrap();
        virgin = false;
    }
}

fn indent(s: &str, indent: usize, char_width: usize) -> String {
    use std::fmt::Write;
    let mut output = String::new();

    let sublines = textwrap::wrap(s, char_width - indent);
    for subline in sublines {
        write!(output, "{}", " ".repeat(indent)).unwrap();
        writeln!(output, "{subline}").unwrap();
    }
    output
}

/*
fn indent2(s: &str, indent: usize, char_width: usize) -> String {
    use std::fmt::Write;
    let mut output = String::new();

    let sublines = textwrap::wrap(s, char_width - indent);
    let mut virgin = true;
    for subline in sublines {
        if !virgin {
            write!(output, "{}", " ".repeat(indent)).unwrap();
        }
        writeln!(output, "{subline}").unwrap();
        virgin = false;
    }
    output
}
*/
