use beermaker::prelude::*;
use strum::IntoEnumIterator;

fn main() {
    for yeast in Yeast::iter() {
        println!("\n{}", yeast);
        println!(
            "  TEMP {}..{}",
            yeast.temp_range().start,
            yeast.temp_range().end
        );
        println!(
            "  ATTENUATION {}..{}",
            yeast.attenuation_range().start,
            yeast.attenuation_range().end
        );
        println!(
            "  ALCOHOL TOLERANCE {}..{}",
            yeast.alcohol_tolerance_range().start,
            yeast.alcohol_tolerance_range().end
        );
        println!("  FLOCCULATION {:?}", yeast.flocculation());

        if let Some(strain) = yeast.strain() {
            println!("  STRAIN={strain:?}");
        }
        if let Some((gallone, conf)) = yeast.gallone_data() {
            println!("  Gallone data (confidence = {}%)", conf * 100.0);
            println!(
                "    Maltose {}/10,  maltotriose {}/10,  Maltose 10C {}/10",
                gallone.get_maltose_use(),
                gallone.get_maltotriose_use(),
                gallone.get_growth_maltose(),
            );
            println!(
                "    Ethanol 12% {}/10,  Sulfite 2.5mM {}/10",
                gallone.get_growth_ethanol(),
                gallone.get_growth_sulfite()
            );
            println!(
                "  Isoamyl acetate {}/10,  Ethyl hexanoate {}/10",
                gallone.get_isoamyl_acetate(),
                gallone.get_ethyl_hexanoate()
            );
            println!(
                "  Ethyl octanoate {}/10  Phenylethyl acetate {}/10",
                gallone.get_ethyl_octanoate(),
                gallone.get_phenylethyl_acetate()
            );
            println!(
                "  POF: {}  FLOC: {}/10, STA1: {:?}, sta10flo8: {}",
                gallone.get_pof(),
                gallone.get_flocculation(),
                gallone.get_sta1(),
                gallone.get_sta10flo8()
            );
        }
    }
}
