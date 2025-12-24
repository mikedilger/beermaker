
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


/// Yeast strains reported in Gallone 2016 paper, coded, we don't know
/// precisely what each one is (except for the help by suregork
/// http://beer.suregork.com/?p=3907), but we have lots of data in that
/// paper if only we can match them up.
pub enum Gallone2016 {
    BE001,
    BE002,
    BE003,
    BE004,
    BE005,
    BE006,
    BE007,
    BE008,
    BE009,
    BE010,
    BE011,
    BE012,
    BE013,
    BE014,
    BE015,
    BE016,
    BE017,
    BE018,
    BE019,
    BE020,
    BE021,
    BE022,
    BE023,
    BE024,
    BE025,
    BE026,
    BE027,
    BE028,
    BE029,
    BE030,
    BE031,
    BE032,
    BE033,
    BE034,
    BE035,
    BE036,
    BE037,
    BE038,
    BE039,
    BE040,
    BE041,
    BE042,
    BE043,
    BE044,
    BE045,
    BE046,
    BE047,
    BE048,
    BE049,
    BE050,
    BE051,
    BE052,
    BE053,
    BE054,
    BE055,
    BE056,
    BE057,
    BE058,
    BE059,
    BE060,
    BE061,
    BE062,
    BE063,
    BE064,
    BE065,
    BE066,
    BE067,
    BE068,
    BE069,
    BE070,
    BE071,
    BE072,
    BE073,
    BE074,
    BE075,
    BE076,
    BE077,
    BE078,
    BE079,
    BE080,
    BE081,
    BE082,
    BE083,
    BE084,
    BE085,
    BE086,
    BE087,
    BE088,
    BE089,
    BE090,
    BE091,
    BE092,
    BE093,
    BE094,
    BE095,
    BE096,
    BE097,
    BE098,
    BE099,
    BE100,
    BE101,
    BE102,
    BI001,
    BI002,
    BI003,
    BI004,
    BI005,
    BR001,
    BR002,
    BR003,
    BR004,
    LA001,
    LA002,
    SA001,
    SA002,
    SA003,
    SA004,
    SA005,
    SA006,
    SA007,
    SP001,
    SP002,
    SP003,
    SP004,
    SP005,
    SP006,
    SP007,
    SP008,
    SP009,
    SP010,
    SP011,
    WI001,
    WI001,
    WI002,
    WI003,
    WI004,
    WI005,
    WI006,
    WI007,
    WI008,
    WI009,
    WI010,
    WI011,
    WI012,
    WI013,
    WI014,
    WI015,
    WI016,
    WI017,
    WI018,
    WI019,
    WL001,
    WL002,
    WL003,
    WL004,
    WL005,
    WL006,
    WL007,
}



/// Yeast strain
///
/// Strains have a common harvested ancestor.
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

    /// WLP007
    // maybe fullers
    CrispEnglishAle,

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

    /// White Labs German Ale II, source unidentified
    // WLP003
    GermanAle2,

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

    /// GINA
    Rochefort,

    /// Sam Adams, East Coast Ale
    // WLP008
    SamAdams,

    // WY1469
    TimothyTaylor,

    /// Plsen Urquell
    // WLP800
    Urquell,

    /// Weihenstephanan Weizen, 3068
    // WLP300
    WeihenstephananWeizen,

    /// Weihenstephanan Weizen, 66
    // WLP300
    WeihenstephananWeizen2,

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
    // WLP036
    ZumUerigeHefe,
}
