use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type Line = Vec<Lines>;
pub type Disruption = Vec<Disruptions>;
pub type Arrival = Vec<Arrivals>;
pub type StopPoint = Vec<StopPoints>;
pub type DisruptionCategories = Vec<String>;
pub type Modes = Vec<ValidMode>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiException {
    /// A string representing the class of exception.
    pub exception: String,
    /// Readable error message
    pub doc: String,
    /// TBD
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidMode {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub is_tfl_service: bool,
    pub is_fare_paying: bool,
    pub is_scheduled_service: bool,
    pub mode_name: String,
}

// Returned by TFl version endpoint
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Version {
    /// Labels returned by the API
    pub label: String,
    /// Timestamp returned from the API
    pub timestamp: String,
    /// Current TFL API version
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopPoints {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub naptan_id: String,
    pub modes: Vec<String>,
    pub ics_code: String,
    pub stop_type: String,
    pub station_naptan: String,
    pub lines: Vec<StopPointLine>,
    pub line_group: Vec<LineGroup>,
    pub line_mode_groups: Vec<LineModeGroup>,
    pub status: bool,
    pub id: String,
    pub common_name: String,
    pub place_type: String,
    pub additional_properties: Vec<AdditionalProperty>,
    pub children: Vec<Children>,
    pub lat: f64,
    pub lon: f64,
    pub hub_naptan_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopPointLine {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub id: String,
    pub name: String,
    pub uri: String,
    #[serde(rename = "type")]
    pub type_field2: String,
    pub crowding: Crowding,
    pub route_type: String,
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineGroup {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub naptan_id_reference: Option<String>,
    pub station_atco_code: String,
    pub line_identifier: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineModeGroup {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub mode_name: String,
    pub line_identifier: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalProperty {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub category: String,
    pub key: String,
    pub source_system_key: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub naptan_id: String,
    pub modes: Vec<Value>,
    pub ics_code: String,
    pub station_naptan: String,
    pub lines: Vec<Value>,
    pub line_group: Vec<Value>,
    pub line_mode_groups: Vec<Value>,
    pub status: bool,
    pub id: String,
    pub common_name: String,
    pub place_type: String,
    pub additional_properties: Vec<Value>,
    pub children: Vec<Value>,
    pub lat: f64,
    pub lon: f64,
    pub hub_naptan_code: Option<String>,
    pub indicator: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lines {
    #[serde(rename = "$type")]
    pub type_field: String,
    /// Line ID
    pub id: String,
    /// Line name. E.g. Bakerloo
    pub name: String,
    /// Mode name of the line. E.g. Tube, DLR, Bus
    pub mode_name: String,
    /// Disruptions for the line
    pub disruptions: Vec<Value>,
    /// Created time
    pub created: String,
    /// Modified time
    pub modified: String,
    /// Line statuses
    pub line_statuses: Vec<Value>,
    /// Route section for the line
    pub route_sections: Vec<RouteSection>,
    /// Type of the service. E.g. Regular, Night
    pub service_types: Vec<ServiceType>,
    /// Crowding details for the line
    pub crowding: Crowding,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteSection {
    #[serde(rename = "$type")]
    pub type_field: String,
    /// Route section name
    pub name: String,
    /// Route direction
    pub direction: String,
    /// Route origin name
    pub origination_name: String,
    /// Route destination name
    pub destination_name: String,
    /// Route originator
    pub originator: String,
    /// Route destination
    pub destination: String,
    /// Service type of the route. E.g. Regular, Night
    pub service_type: String,
    /// Valid until
    pub valid_to: String,
    /// Valid from
    pub valid_from: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceType {
    #[serde(rename = "$type")]
    pub type_field: String,
    /// Service type name
    pub name: String,
    /// Service type URI
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Crowding {
    #[serde(rename = "$type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Disruptions {
    #[serde(rename = "$type")]
    pub type_field: String,
    /// Disruption category
    pub category: String,
    #[serde(rename = "type")]
    pub type_field2: String,
    /// Description for disruption category
    pub category_description: String,
    /// Description
    pub description: String,
    /// Creation time for disruption
    pub created: String,
    /// Last update time for disruption
    pub last_update: String,
    /// Affected routes for disruption
    pub affected_routes: Vec<Value>,
    /// Affected stops for disruption
    pub affected_stops: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arrivals {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub id: String,
    pub operation_type: i64,
    pub vehicle_id: String,
    pub naptan_id: String,
    pub station_name: String,
    pub line_id: String,
    pub line_name: String,
    pub platform_name: String,
    pub direction: Option<String>,
    pub bearing: String,
    pub destination_naptan_id: String,
    pub destination_name: String,
    pub timestamp: String,
    pub time_to_station: i64,
    pub current_location: String,
    pub towards: String,
    pub expected_arrival: String,
    pub time_to_live: String,
    pub mode_name: String,
    pub timing: Timing,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timing {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub countdown_server_adjustment: String,
    pub source: String,
    pub insert: String,
    pub read: String,
    pub sent: String,
    pub received: String,
}

pub enum ServiceTypes {
    Regular,
    Night,
}

impl ServiceTypes {
    pub fn to_type(&self) -> &'static str {
        match self {
            ServiceTypes::Regular => "Regular",
            ServiceTypes::Night => "Night",
        }
    }
}

pub enum Directions {
    Inbound,
    Outbound,
    All,
}

impl Directions {
    pub fn to_type(&self) -> &'static str {
        match self {
            Directions::Inbound => "Inboud",
            Directions::Outbound => "Outbound",
            Directions::All => "All",
        }
    }
}

pub enum Mode {
    Tube,
    DLR,
    Bus,
}

impl Mode {
    pub fn mode(&self) -> &'static str {
        match self {
            Mode::Tube => "Tube",
            Mode::DLR => "DLR",
            Mode::Bus => "Bus",
        }
    }
}

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
}

impl LineID {
    pub fn line(&self) -> &'static str {
        match self {
            LineID::Bakerloo => "Bakerloo",
            LineID::Central => "Central",
            LineID::Circle => "Circle",
            LineID::District => "District",
            LineID::Jubilee => "Jubilee",
            LineID::Metropolitan => "Metropolitan",
            LineID::Northern => "Northern",
            LineID::Piccadilly => "Piccadilly",
            LineID::Victoria => "Victoria",
            LineID::HammersmithAndCity => "Hammersmith & City",
            LineID::WaterlooAndCity => "Waterloo & City",
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct Parameters {
    #[serde(flatten)]
    pub lines: String,
    pub service_type: String,
    pub mode: String,
    pub stop_point_id: String,
    pub direction: Option<String>,
    pub destination_station_id: Option<String>,
    pub tfl_operated_national_rail_stations_only: Option<bool>,
}
