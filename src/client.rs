use crate::{
    line::{
        ArrivalPredictionsByLine, DisruptionByLine, DisruptionByMode, RouteRequest,
        RouteRequestById,
    },
    request::*,
};

// Client for accessing TFL API
pub struct Client {
    pub app_key: String,
    pub req_client: reqwest::Client,
}

impl Client {
    // Create a new client for the TFL API
    pub fn new(app_key: String) -> Self {
        let req_client = reqwest::Client::new();
        Self {
            app_key,
            req_client,
        }
    }

    /// Return API version
    pub fn api_version(&self) -> VersionRequest<'_> {
        VersionRequest::new(self)
    }

    /// Return all the TFL routes
    pub fn routes(&self) -> RouteRequest<'_> {
        RouteRequest::new(self)
    }

    /// Return routes by Line(s)
    pub fn routes_by_line(&self) -> RouteRequestById<'_> {
        RouteRequestById::new(self)
    }

    /// Return disruptions by service mode. For example: Regular, Night
    pub fn disruptions_by_mode(&self) -> DisruptionByMode<'_> {
        DisruptionByMode::new(self)
    }

    /// Return disruptions by specified Line(s)
    pub fn disruptions_by_line(&self) -> DisruptionByLine<'_> {
        DisruptionByLine::new(self)
    }

    /// Return arrival predictions by specified Line(s)
    pub fn arrival_predictions_by_line(&self) -> ArrivalPredictionsByLine<'_> {
        ArrivalPredictionsByLine::new(self)
    }
}
