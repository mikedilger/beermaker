use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// Yeast strain
///
/// Strains are independent of provider, and are each thought to
/// have a common harvested ancestor. We name them after cities or breweries
/// according to what most brewers call them.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
pub enum Strain {
    /// Achouffe
    // WLP550, WY3522, Belgian Ardennes Ale Yeast
    Achouffe,

    /// Adnams, Southwold
    Adnams,

    /// Anchor Steam
    // WLP810
    AnchorSteam,

    /// Augustiner
    // WLP860
    Augustiner,

    /// Ayinger
    // WLP833, WY2487
    Ayinger,

    /// Ballantine, Anchor Liberty
    // ancestor of the Chico strain
    // WLP051, BRY97, WY1272
    Ballantine,

    /// Boddingtons
    // WY1318
    Boddingtons,

    /// Brakspear, Burton Ale
    Brakspear,

    /// Budweiser
    // WLP840, WY2007
    Budweiser,

    /// Carlsberg (copenhagen, 1883)
    Carlsberg,

    /// Charles Wells, Bedford British Ale
    // WLP006
    CharlesWells,

    /// Chico, Sierra Nevada Pale Ale
    // WLP001, BRY 96, WY1056
    Chico,

    /// Chimay
    // WLP500, WY1214
    Chimay,

    /// Coopers, Australian Ale
    // WLP009
    Coopers,

    /// De Koninck
    // WLP515
    DeKoninck,

    /// Brassserie Dupont, Saison
    // WLP565
    Dupont,

    /// Vieille Provision Saison Dupont
    // WLP570
    Dupont2,

    /// Duvel moortgart, McEwan's Brewery
    /// Belgian Strong Golden Ale
    // WLP570
    Duvel,

    /// Fullers ESB, Griffin Brewery, Cheswick, London
    /// (replaced Old Burton Extra in 1971)
    // orange citrus, toffee flavors
    // WLP002, [BE045], WY1968 (London ESB)
    // Imperial Yeast A09 Pub
    Fullers,

    /// Griffin Brewery, Cheswick, London
    // soft fruit flavor, red berry fruits
    Gales,

    /// Guinness, Irish Ale
    // WLP004
    Guinness,

    /// Henlty of Thames, brakspear bitter
    // WY1275
    HenleyOfThames,

    /// Hoegaarden/Celis, Belgian Wit Ale
    // WLP400
    Hoegaarden,

    /// Hürlimann brewery, Switzerland
    // S-189
    Hurlimann,

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

    /// Grupo Modelo
    // WLP940
    Modelo,

    /// Moortgat Brouwerij (via Ommegang), Belgian Wit Ale
    Moortgat,

    /// New Albion, Old Sonoma
    // WLP076
    NewAlbion,

    /// Nottingham, East Midlands, Danstar
    // WLP039
    Nottingham,

    /// Orval, Bastogne Belgian Ale
    // WLP510
    Orval,

    /// PJ Früh, Köln
    PJFruh,

    /// Redhook, Pacific Ale
    // WLP041
    Redhook,

    /// Ridleys, Essex, near Chelmsford
    Ridleys,

    /// Ringwood, British Ale
    // WLP005
    Ringwood,

    /// Rochefort
    // WLP540
    Rochefort,

    /// Sam Adams, East Coast Ale
    // WLP008
    SamAdams,

    /// Samichlaus, Zurich
    // WLP885
    Samichlaus,

    /// Samsons via Budejovicky Mestansky Pivovara
    // WLP802
    Samsons,

    /// Timothy Taylor
    // WY1469
    TimothyTaylor,

    /// Plsen Urquell
    // WLP800
    Urquell,

    /// Voss Kveik
    VossKveik,

    /// Weihenstephanan Weizen, 66
    WeihenstephananWeizen66,

    /// Weihenstephanan Weizen, 3068, 68
    WeihenstephananWeizen68,

    /// Weihenstephanan Weizen, 175
    WeihenstephananWeizen175,

    /// Weihenstephanan Lager, 3470 (Technical University of Munich)
    WeihenstephananLager,

    /// Weihenstephanan Lager, 206 (Technical University of Munich)
    WeihenstephananLager206,

    /// Weisenschaftliche Station #338 Munich, European Ale
    // WLP011, WY1338
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
    // WLP036, WY1007
    ZumUerigeAlt,

    /// Zum Uerige via Widmer, American Hefeweizen Ale
    ZumUerigeHefe,
}
