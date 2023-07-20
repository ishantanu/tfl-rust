pub enum LineID {
    Bakerloo,
    Central,
    Circle,
    District,
    HammersmithAndCity,
    Jubilee,
    Metropolitan,
    Northern,
    Piccadilly,
    Victoria,
    WaterlooAndCity,
    A10,
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
    C2c,
    C3,
    ChilternRailways,
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
    EastMidlandsRailway,
    EL1,
    EL2,
    EL3,
    Elizabethline,
    FirstHullTrains,
    FirstTransPennineExpress,
    G1,
    GatwickExpress,
    GrandCentral,
    GreaterAnglia,
    GreatNorthern,
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
    HeathrowExpress,
    IslandLine,
    K1,
    K2,
    K3,
    K4,
    K5,
    IFSCloudCableCar,
    LondonNorthEasternRailway,
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
    ScotRail,
    SL8,
    Southeastern,
    Southern,
    SouthWesternRailway,
    Thameslink,
    ThamesRiverServices,
    Tram,
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
    WestMidlandsTrains,
    WoolwichFerry,
    X140,
    X26,
    X68,
}

impl LineID {
    pub fn line(&self) -> &'static str {
        match self {
            LineID::Bakerloo => "Bakerloo",
            LineID::Central => "Central",
            LineID::Circle => "Circle",
            LineID::District => "District",
            LineID::HammersmithAndCity => "Hammersmith & City",
            LineID::Jubilee => "Jubilee",
            LineID::Metropolitan => "Metropolitan",
            LineID::Northern => "Northern",
            LineID::Piccadilly => "Piccadilly",
            LineID::Victoria => "Victoria",
            LineID::WaterlooAndCity => "Waterloo & City",
            LineID::A10 => "A10",
            LineID::AvantiWestCoast => "Avanti West Coast",
            LineID::B11 => "B11",
            LineID::B12 => "B12",
            LineID::B13 => "B13",
            LineID::B14 => "B14",
            LineID::B15 => "B15",
            LineID::B16 => "B16",
            LineID::C1 => "C1",
            LineID::C10 => "C10",
            LineID::C11 => "C11",
            LineID::C2c => "c2c",
            LineID::C3 => "C3",
            LineID::ChilternRailways => "Chiltern Railways",
            LineID::CrossCountry => "Cross Country",
            LineID::D3 => "D3",
            LineID::D6 => "D6",
            LineID::D7 => "D7",
            LineID::D8 => "D8",
            LineID::DLR => "DLR",
            LineID::E1 => "E1",
            LineID::E10 => "E10",
            LineID::E11 => "E11",
            LineID::E2 => "E2",
            LineID::E3 => "E3",
            LineID::E5 => "E5",
            LineID::E6 => "E6",
            LineID::E7 => "E7",
            LineID::E8 => "E8",
            LineID::E9 => "E9",
            LineID::EastMidlandsRailway => "East Midlands Railway",
            LineID::EL1 => "EL1",
            LineID::EL2 => "EL2",
            LineID::EL3 => "EL3",
            LineID::Elizabethline => "Elizabeth line",
            LineID::FirstHullTrains => "First Hull Trains",
            LineID::FirstTransPennineExpress => "First Trans Pennine Express",
            LineID::G1 => "G1",
            LineID::GatwickExpress => "Gatwick Express",
            LineID::GrandCentral => "Grand Central",
            LineID::GreaterAnglia => "GreaterAnglia",
            LineID::GreatNorthern => "Great Northern",
            LineID::GreatWesternRailway => "Great Western Railway",
            LineID::H10 => "H10",
            LineID::H11 => "H11",
            LineID::H12 => "H13",
            LineID::H13 => "H13",
            LineID::H14 => "H14",
            LineID::H17 => "H17",
            LineID::H18 => "H18",
            LineID::H19 => "H19",
            LineID::H2 => "H2",
            LineID::H20 => "H20",
            LineID::H22 => "H22",
            LineID::H25 => "H25",
            LineID::H26 => "H26",
            LineID::H28 => "H28",
            LineID::H3 => "H3",
            LineID::H32 => "H32",
            LineID::H37 => "H37",
            LineID::H9 => "H9",
            LineID::H91 => "H91",
            LineID::H98 => "H98",
            LineID::HeathrowExpress => "Heathrow Express",
            LineID::IslandLine => "Island Line",
            LineID::K1 => "K1",
            LineID::K2 => "K2",
            LineID::K3 => "K3",
            LineID::K4 => "K4",
            LineID::K5 => "K5",
            LineID::IFSCloudCableCar => "IFS Cloud Cable Car",
            LineID::LondonNorthEasternRailway => "London North Eastern Railway",
            LineID::LondonOverground => "London Overground",
            LineID::Lumo => "Lumo",
            LineID::Merseyrail => "Merseyrail",
            LineID::N1 => "N1",
            LineID::N109 => "N109",
            LineID::N11 => "N11",
            LineID::N113 => "N113",
            LineID::N133 => "N133",
            LineID::N136 => "N136",
            LineID::N137 => "N137",
            LineID::N140 => "N140",
            LineID::N15 => "N15",
            LineID::N155 => "N155",
            LineID::N171 => "N171",
            LineID::N18 => "N18",
            LineID::N19 => "N19",
            LineID::N199 => "N199",
            LineID::N2 => "N2",
            LineID::N20 => "N20",
            LineID::N205 => "N205",
            LineID::N207 => "N207",
            LineID::N21 => "N21",
            LineID::N22 => "N22",
            LineID::N242 => "N242",
            LineID::N25 => "N25",
            LineID::N250 => "N250",
            LineID::N253 => "N253",
            LineID::N26 => "N26",
            LineID::N266 => "N266",
            LineID::N27 => "N27",
            LineID::N271 => "N271",
            LineID::N277 => "N277",
            LineID::N279 => "N279",
            LineID::N28 => "N28",
            LineID::N29 => "N29",
            LineID::N3 => "N3",
            LineID::N31 => "N31",
            LineID::N32 => "N32",
            LineID::N33 => "N33",
            LineID::N343 => "N343",
            LineID::N38 => "N38",
            LineID::N381 => "N381",
            LineID::N41 => "N41",
            LineID::N44 => "N44",
            LineID::N5 => "N5",
            LineID::N53 => "N53",
            LineID::N55 => "N55",
            LineID::N550 => "N550",
            LineID::N551 => "N551",
            LineID::N63 => "N63",
            LineID::N65 => "N65",
            LineID::N68 => "N68",
            LineID::N7 => "N7",
            LineID::N72 => "N72",
            LineID::N73 => "N73",
            LineID::N74 => "N74",
            LineID::N8 => "N8",
            LineID::N83 => "N83",
            LineID::N86 => "N86",
            LineID::N87 => "N87",
            LineID::N89 => "N89",
            LineID::N9 => "N9",
            LineID::N91 => "N91",
            LineID::N97 => "N97",
            LineID::N98 => "N98",
            LineID::NorthernRail => "Northern Rail",
            LineID::P12 => "P12",
            LineID::P13 => "P13",
            LineID::P4 => "P4",
            LineID::P5 => "P5",
            LineID::R1 => "R1",
            LineID::R10 => "R10",
            LineID::R11 => "R11",
            LineID::R2 => "R2",
            LineID::R3 => "R3",
            LineID::R4 => "R4",
            LineID::R5 => "R5",
            LineID::R6 => "R6",
            LineID::R68 => "R68",
            LineID::R7 => "R7",
            LineID::R70 => "R70",
            LineID::R8 => "R8",
            LineID::R9 => "R9",
            LineID::RB1 => "RB1",
            LineID::RB2 => "RB2",
            LineID::RB4 => "RB4",
            LineID::RB6 => "RB6",
            LineID::S1 => "S1",
            LineID::S3 => "S3",
            LineID::S4 => "S3",
            LineID::ScotRail => "Scot Rail",
            LineID::SL8 => "SL8",
            LineID::Southeastern => "Southeastern",
            LineID::Southern => "Southern",
            LineID::SouthWesternRailway => "South Western Railway",
            LineID::Thameslink => "Thameslink",
            LineID::ThamesRiverServices => "Thames River Services",
            LineID::Tram => "Tram",
            LineID::TransportforWales => "Transport for Wales",
            LineID::U1 => "U1",
            LineID::U10 => "U10",
            LineID::U2 => "U2",
            LineID::U3 => "U3",
            LineID::U4 => "U4",
            LineID::U5 => "U5",
            LineID::U7 => "U7",
            LineID::U9 => "U9",
            LineID::W11 => "W11",
            LineID::W12 => "W12",
            LineID::W13 => "W13",
            LineID::W14 => "W14",
            LineID::W15 => "W15",
            LineID::W16 => "W16",
            LineID::W19 => "W19",
            LineID::W3 => "W3",
            LineID::W4 => "W4",
            LineID::W5 => "W5",
            LineID::W6 => "W6",
            LineID::W7 => "W7",
            LineID::W8 => "W8",
            LineID::W9 => "W9",
            LineID::WestMidlandsTrains => "West Midlands Trains",
            LineID::WoolwichFerry => "Woolwich Ferry",
            LineID::X140 => "X140",
            LineID::X26 => "X26",
            LineID::X68 => "X68",
        }
    }
}
