use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type Line = Vec<Lines>;
pub type Disruption = Vec<Disruptions>;
pub type Arrival = Vec<Arrivals>;
pub type StopPoint = Vec<StopPoints>;
pub type DisruptionCategories = Vec<String>;
pub type Modes = Vec<ValidMode>;
pub type LServiceTypes = Vec<String>;
pub type Serverities = Vec<Severity>;
pub type Route = Vec<Routes>;

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
pub struct Routes {
    #[serde(rename = "$type")]
    pub type_field: String,
    /// Line ID
    pub line_id: String,
    /// Line name
    pub line_name: String,
    /// Line direction
    pub direction: String,
    /// Outbound only line
    pub is_outbound_only: bool,
    /// Mode
    pub mode: String,
    /// Line strings
    pub line_strings: Vec<String>,
    /// Stations on the line
    pub stations: Vec<Station>,
    /// Stop point sequences
    pub stop_point_sequences: Vec<StopPointSequence>,
    /// Ordered line routes
    pub ordered_line_routes: Vec<OrderedLineRoute>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    #[serde(rename = "$type")]
    pub type_field: String,
    /// Station ID
    pub station_id: Option<String>,
    /// ICS ID
    pub ics_id: String,
    /// Top most parent id
    pub top_most_parent_id: Option<String>,
    /// Modes
    pub modes: Vec<String>,
    /// Stop type
    pub stop_type: String,
    /// Station zone
    pub zone: String,
    /// Lines
    pub lines: Vec<RouteLines>,
    /// Status of the station
    pub status: bool,
    /// Station ID
    pub id: String,
    /// Station name
    pub name: String,
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lon: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteLines {
    #[serde(rename = "$type")]
    pub type_field: String,
    /// Line ID
    pub id: String,
    /// Line name
    pub name: String,
    /// URI
    pub uri: String,
    #[serde(rename = "type")]
    pub type_field2: String,
    /// Crowding stat
    pub crowding: Crowding,
    /// Route type
    pub route_type: String,
    /// Line status
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopPointSequence {
    #[serde(rename = "$type")]
    pub type_field: String,
    /// Line ID
    pub line_id: String,
    /// Line name
    pub line_name: String,
    /// Traveling direction
    pub direction: String,
    /// Branch ID
    pub branch_id: i64,
    /// Subsequent branch IDs
    pub next_branch_ids: Vec<Value>,
    /// Previous branch IDs
    pub prev_branch_ids: Vec<Value>,
    /// Stop points
    pub stop_point: Vec<RouteStopPoint>,
    /// Service type
    pub service_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteStopPoint {
    #[serde(rename = "$type")]
    pub type_field: String,
    /// Parent ID
    pub parent_id: Option<String>,
    /// Station ID
    pub station_id: String,
    /// ICS ID
    pub ics_id: String,
    /// Top most parent ID
    pub top_most_parent_id: String,
    /// Modes
    pub modes: Vec<String>,
    /// Stop type
    pub stop_type: String,
    /// Zone
    pub zone: String,
    /// Lines
    pub lines: Vec<Line2>,
    /// Status
    pub status: bool,
    /// Stop point ID
    pub id: String,
    /// Stop point name
    pub name: String,
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lon: f64,
    /// Does it has a reported disruption
    pub has_disruption: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Line2 {
    #[serde(rename = "$type")]
    pub type_field: String,
    /// Line ID
    pub id: String,
    /// Line name
    pub name: String,
    /// Line URI
    pub uri: String,
    #[serde(rename = "type")]
    pub type_field2: String,
    /// Crowding
    pub crowding: Crowding2,
    /// Route type
    pub route_type: String,
    /// Line status
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Crowding2 {
    #[serde(rename = "$type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderedLineRoute {
    #[serde(rename = "$type")]
    pub type_field: String,
    /// Ordered line route name
    pub name: String,
    /// NAPTAN IDs
    pub naptan_ids: Vec<String>,
    /// Service type
    pub service_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Severity {
    #[serde(rename = "$type")]
    pub type_field: String,
    /// Transport mode name
    pub mode_name: String,
    /// Severity level
    pub severity_level: i64,
    /// Severity description
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidMode {
    #[serde(rename = "$type")]
    pub type_field: String,
    /// Is mode type a TFL service
    pub is_tfl_service: bool,
    /// Is Fare paying service
    pub is_fare_paying: bool,
    /// Is it a scheduled service
    pub is_scheduled_service: bool,
    /// Name of the mode
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
    /// NAPTAN ID
    pub naptan_id: String,
    /// List of modes
    pub modes: Vec<String>,
    /// ICS code
    pub ics_code: String,
    /// Stop type
    pub stop_type: String,
    /// Station NAPTAN
    pub station_naptan: String,
    /// List of lines
    pub lines: Vec<StopPointLine>,
    /// Line group
    pub line_group: Vec<LineGroup>,
    /// Line mode groups
    pub line_mode_groups: Vec<LineModeGroup>,
    /// Status
    pub status: bool,
    /// ID
    pub id: String,
    /// Common Name
    pub common_name: String,
    /// Place type
    pub place_type: String,
    /// Additional properties
    pub additional_properties: Vec<AdditionalProperty>,
    /// Children
    pub children: Vec<Children>,
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lon: f64,
    /// Hub NAPTAN code
    pub hub_naptan_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopPointLine {
    #[serde(rename = "$type")]
    pub type_field: String,
    /// Stop point line ID
    pub id: String,
    /// Stop point line name
    pub name: String,
    /// URI
    pub uri: String,
    #[serde(rename = "type")]
    pub type_field2: String,
    /// Crowding details
    pub crowding: Crowding,
    /// Route type
    pub route_type: String,
    /// Status
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineGroup {
    #[serde(rename = "$type")]
    pub type_field: String,
    /// NAPTAN ID reference for line group
    pub naptan_id_reference: Option<String>,
    /// Station atco code for line group
    pub station_atco_code: String,
    /// Line identifier for line group
    pub line_identifier: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineModeGroup {
    #[serde(rename = "$type")]
    pub type_field: String,
    /// Mode name of line mode group
    pub mode_name: String,
    /// Line identifier
    pub line_identifier: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalProperty {
    #[serde(rename = "$type")]
    pub type_field: String,
    /// Additional property category
    pub category: String,
    /// Additional property key
    pub key: String,
    /// Source system key
    pub source_system_key: String,
    /// Porperty value
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children {
    #[serde(rename = "$type")]
    pub type_field: String,
    /// Children NAPTAN ID
    pub naptan_id: String,
    /// Children modes
    pub modes: Vec<Value>,
    /// ICS Code
    pub ics_code: String,
    /// Station NAPTAN
    pub station_naptan: String,
    /// Lines
    pub lines: Vec<Value>,
    /// Line group
    pub line_group: Vec<Value>,
    /// Line mode groups
    pub line_mode_groups: Vec<Value>,
    /// Status
    pub status: bool,
    /// ID
    pub id: String,
    /// Common name
    pub common_name: String,
    /// Place type
    pub place_type: String,
    /// Additional properties
    pub additional_properties: Vec<Value>,
    /// Children
    pub children: Vec<Value>,
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lon: f64,
    /// Hub NAPTAN code
    pub hub_naptan_code: Option<String>,
    /// Indicator
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
    pub created: Option<String>,
    /// Last update time for disruption
    pub last_update: Option<String>,
    /// Affected routes for disruption
    pub affected_routes: Vec<Value>,
    /// Affected stops for disruption
    pub affected_stops: Vec<Value>,
    /// closure text
    pub closure_text: Option<String>,
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

#[derive(Debug)]
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
            Directions::Inbound => "Inbound",
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
    pub line_id: String,
    pub service_type: Option<String>,
    pub mode: String,
    pub stop_point_id: String,
    pub direction: Option<String>,
    pub destination_station_id: Option<String>,
    pub tfl_operated_national_rail_stations_only: Option<bool>,
    pub exclude_crowding: Option<bool>,
}
