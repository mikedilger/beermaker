
/// Yeast Family
#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
pub enum Family {
    /// Saccharomyces cerevisiae
    /// Warm-fermenting, top-fermenting, fruity-estery flavors
    Ale,

    /// Saccharomyces pastorianus
    /// Cold-fermenting, bottom-fermenting, clean and crisp
    Lager,
}

/// Yeast strain
///
/// Strains are independent of provider, and are each thought to
/// have a common harvested ancestor. We name them after cities or breweries
/// according to what most brewers call them.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
pub enum Strain {
    /// Adnams, Southwold
    Adnams,

    /// Anchor Steam
    // WLP810
    AnchorSteam,

    /// WLP006
    BedfordBritishAle,

    /// Ballantine, Anchor Liberty
    // WLP051, BRY97
    Ballantine,

    /// Boddingtons
    // WY1318
    Boddingtons,

    /// Brakspear, Burton Ale
    Brakspear,

    /// Charles Wells, Bedford British Ale
    CharlesWells,

    /// Chico, Sierra Nevada Pale Ale
    // WLP001, BRY 96, WY1056
    Chico,

    /// Coopers, Australian Ale
    // WLP009
    Coopers,

    /// Duvel moortgat, McEwan's Brewery
    /// Belgian Strong Golden Ale
    // WLP570
    Duvel,

    /// Fullers ESB, Griffin Brewery, Cheswick, London
    /// (replaced Old Burton Extra in 1971)
    // orange citrus, toffee flavors
    // WLP002, [BE045], WY1968
    Fullers,

    /// Griffin Brewery, Cheswick, London
    // soft fruit flavor, red berry fruits
    Gales,

    /// Guinness, Irish Ale
    // WLP004
    Guinness,

    /// Henlty of Thames
    // WY1275
    HenleyOfThames,

    /// Hoegaarden, Belgian Wit Ale
    // WLP400
    Hoegaarden,

    /// Mackeson, Thames
    // WLP030
    Mackeson,

    /// Manchester
    // WLP038
    Manchester,

    /// Marstons, Premium Bitter
    // WLP026
    Marstons,

    /// McEwan, Edinburgh Ale
    // WLP028
    McEwan,

    /// Moorgat, Belgian Wit Ale
    Moorgat,

    /// New Albion, Old Sonoma
    // WLP076
    NewAlbion,

    /// Nottingham, East Midlands
    // WLP039
    Nottingham,

    /// Orval, Bastogne Belgian Ale
    // WLP510
    Orval,

    /// Redhook, Pacific Ale
    // WLP041
    Redhook,

    /// Ridleys, Essex
    Ridleys,

    /// Ringwood, British Ale
    // WLP005
    Ringwood,

    /// Rochefort
    Rochefort,

    /// Sam Adams, East Coast Ale
    // WLP008
    SamAdams,

    // WY1469
    TimothyTaylor,

    /// Plsen Urquell
    // WLP800
    Urquell,

    /// Weihenstephanan Weizen, 3068, 66
    WeihenstephananWeizen,

    /// Weihenstephanan Lager, 3470
    WeihenstephananLager,

    /// Weisenschaftliche Station #338 Munich, European Ale
    // WLP011
    Weisenschaftliche,

    /// Westmalle, Abbey Ale
    // WLP530
    Westmalle,

    /// Whitbread
    Whitbread,

    /// Whitbread dry, Dry English Ale
    // WLP007, WY1098
    WhitbreadDry,

    /// Worthington White Shield, London
    // WLP013, WY1028
    Worthington,

    /// Yorkshire Square ale
    // WLP037
    YorkshireSquare,

    /// Zum Uerige, Dusseldorf Alt Ale
    // WLP036
    ZumUerigeAlt,

    /// Zum Uerige via Widmer, American Hefeweizen Ale
    ZumUerigeHefe,
}
