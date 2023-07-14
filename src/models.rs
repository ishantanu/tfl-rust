use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type Line = Vec<Lines>;
pub type Disruption = Vec<Disruptions>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiException {
    /// A string representing the class of exception.
    pub exception: String,
    /// Readable error message
    pub doc: String,
    /// TBD
    pub display: String,
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
    pub line_id: String,
    pub service_type: String,
    pub mode: String,
}
