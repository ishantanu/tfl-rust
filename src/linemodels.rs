use strum::EnumProperty;

// LineID
//
// The display name (`to_string()`) matches TFL's display name for the line.
//
// The ID (`line()`) is TFL's ID for the line. This mostly matches the display name, but for more
// complex lines uses kebab-case. We only need to override the ID for lines that don't use their
// display name as the ID. Note: Because we use the display as the default, our IDs are
// capitalised. The TFL API ignores cases for IDs.

#[derive(
    Debug,
    Copy,
    Clone,
    strum::Display,
    strum::IntoStaticStr,
    strum::EnumString,
    strum::EnumProperty,
    strum::EnumIter,
)]
pub enum LineID {
    Bakerloo,
    Central,
    Circle,
    District,
    #[strum(to_string = "Hammersmith & City")]
    #[strum(props(OverrideId = "Hammersmith-City"))]
    HammersmithAndCity,
    Jubilee,
    Metropolitan,
    Northern,
    Piccadilly,
    Victoria,
    #[strum(to_string = "Waterloo & City")]
    #[strum(props(OverrideId = "Waterloo-City"))]
    WaterlooAndCity,
    A10,
    #[strum(to_string = "Avanti West Coast")]
    #[strum(props(OverrideId = "Avanti-West-Coast"))]
    AvantiWestCoast,
    B11,
    B12,
    B13,
    B14,
    B15,
    B16,
    C1,
    C10,
    C11,
    #[strum(to_string = "c2c")]
    C2c,
    C3,
    #[strum(to_string = "Chiltern Railways")]
    #[strum(props(OverrideId = "Chiltern-Railways"))]
    ChilternRailways,
    #[strum(to_string = "Cross Country")]
    #[strum(props(OverrideId = "Cross-Country"))]
    CrossCountry,
    D3,
    D6,
    D7,
    D8,
    DLR,
    E1,
    E10,
    E11,
    E2,
    E3,
    E5,
    E6,
    E7,
    E8,
    E9,
    #[strum(to_string = "East Midlands Railway")]
    #[strum(props(OverrideId = "East-Midlands-Railway"))]
    EastMidlandsRailway,
    EL1,
    EL2,
    EL3,
    #[strum(to_string = "Elizabeth line")]
    #[strum(props(OverrideId = "Elizabeth"))]
    Elizabethline,
    #[strum(to_string = "First Hull Trains")]
    #[strum(props(OverrideId = "First-Hull-Trains"))]
    FirstHullTrains,
    #[strum(to_string = "First TransPennine Express")]
    #[strum(props(OverrideId = "First-Transpennine-Express"))]
    FirstTransPennineExpress,
    G1,
    #[strum(to_string = "Gatwick Express")]
    #[strum(props(OverrideId = "Gatwick-Express"))]
    GatwickExpress,
    #[strum(to_string = "Grand Central")]
    #[strum(props(OverrideId = "Grand-Central"))]
    GrandCentral,
    #[strum(to_string = "Greater Anglia")]
    #[strum(props(OverrideId = "Greater-Anglia"))]
    GreaterAnglia,
    #[strum(to_string = "Great Northern")]
    #[strum(props(OverrideId = "Great-Northern"))]
    GreatNorthern,
    #[strum(to_string = "Great Western Railway")]
    #[strum(props(OverrideId = "Great-Western-Railway"))]
    GreatWesternRailway,
    H10,
    H11,
    H12,
    H13,
    H14,
    H17,
    H18,
    H19,
    H2,
    H20,
    H22,
    H25,
    H26,
    H28,
    H3,
    H32,
    H37,
    H9,
    H91,
    H98,
    #[strum(to_string = "Heathrow Express")]
    #[strum(props(OverrideId = "Heathrow-Express"))]
    HeathrowExpress,
    #[strum(to_string = "Island Line")]
    #[strum(props(OverrideId = "Island-Line"))]
    IslandLine,
    K1,
    K2,
    K3,
    K4,
    K5,
    #[strum(to_string = "IFS Cloud Cable Car")]
    #[strum(props(OverrideId = "London-Cable-Car"))]
    IFSCloudCableCar,
    #[strum(to_string = "London North Eastern Railway")]
    #[strum(props(OverrideId = "London-North-Eastern-Railway"))]
    LondonNorthEasternRailway,
    #[strum(to_string = "London Overground")]
    #[strum(props(OverrideId = "London-Overground"))]
    LondonOverground,
    Lumo,
    Merseyrail,
    N1,
    N109,
    N11,
    N113,
    N133,
    N136,
    N137,
    N140,
    N15,
    N155,
    N171,
    N18,
    N19,
    N199,
    N2,
    N20,
    N205,
    N207,
    N21,
    N22,
    N242,
    N25,
    N250,
    N253,
    N26,
    N266,
    N27,
    N271,
    N277,
    N279,
    N28,
    N29,
    N3,
    N31,
    N32,
    N33,
    N343,
    N38,
    N381,
    N41,
    N44,
    N5,
    N53,
    N55,
    N550,
    N551,
    N63,
    N65,
    N68,
    N7,
    N72,
    N73,
    N74,
    N8,
    N83,
    N86,
    N87,
    N89,
    N9,
    N91,
    N97,
    N98,
    #[strum(to_string = "Northern Rail")]
    #[strum(props(OverrideId = "Northern-Rail"))]
    NorthernRail,
    P12,
    P13,
    P4,
    P5,
    R1,
    R10,
    R11,
    R2,
    R3,
    R4,
    R5,
    R6,
    R68,
    R7,
    R70,
    R8,
    R9,
    RB1,
    RB2,
    RB4,
    RB6,
    S1,
    S3,
    S4,
    #[strum(to_string = "ScotRail")]
    #[strum(props(OverrideId = "ScotRail"))]
    ScotRail,
    SL8,
    Southeastern,
    Southern,
    #[strum(to_string = "South Western Railway")]
    #[strum(props(OverrideId = "South-Western-Railway"))]
    SouthWesternRailway,
    Thameslink,
    #[strum(to_string = "Thames River Services")]
    #[strum(props(OverrideId = "Thames-River-Services"))]
    ThamesRiverServices,
    Tram,
    #[strum(to_string = "Transport for Wales")]
    #[strum(props(OverrideId = "Transport-for-Wales"))]
    TransportforWales,
    U1,
    U10,
    U2,
    U3,
    U4,
    U5,
    U7,
    U9,
    W11,
    W12,
    W13,
    W14,
    W15,
    W16,
    W19,
    W3,
    W4,
    W5,
    W6,
    W7,
    W8,
    W9,
    #[strum(to_string = "West Midlands Trains")]
    #[strum(props(OverrideId = "West-Midlands-Trains"))]
    WestMidlandsTrains,
    #[strum(to_string = "Woolwich Ferry")]
    #[strum(props(OverrideId = "Woolwich-Ferry"))]
    WoolwichFerry,
    X140,
    X26,
    X68,
}

impl LineID {
    pub fn line(&self) -> &'static str {
        if let Some(id) = self.get_str("OverrideId") {
            id
        } else {
            self.into()
        }
    }
}
