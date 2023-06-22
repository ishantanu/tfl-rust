use serde::{Deserialize, Serialize};

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
