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
    pub id: String,
    pub name: String,
    pub mode_name: String,
    pub disruptions: Vec<Value>,
    pub created: String,
    pub modified: String,
    pub line_statuses: Vec<Value>,
    pub route_sections: Vec<RouteSection>,
    pub service_types: Vec<ServiceType>,
    pub crowding: Crowding,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteSection {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub name: String,
    pub direction: String,
    pub origination_name: String,
    pub destination_name: String,
    pub originator: String,
    pub destination: String,
    pub service_type: String,
    pub valid_to: String,
    pub valid_from: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceType {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub name: String,
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
    pub category: String,
    #[serde(rename = "type")]
    pub type_field2: String,
    pub category_description: String,
    pub description: String,
    pub affected_routes: Vec<Value>,
    pub affected_stops: Vec<Value>,
    pub closure_text: String,
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

pub enum LineID {
    Bakerloo,
    Central,
    Circle,
    District,
    //Hammersmith \& City,
    Jubilee,
    Metropolitan,
    Northern,
    Piccadilly,
    Victoria,
    //Waterloo & City,
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
        }
        
    }
}

#[derive(Debug, Default, Serialize)]
pub struct Parameters {
    #[serde(flatten)]
    
    pub line_id: String,
    pub service_type: String,
}