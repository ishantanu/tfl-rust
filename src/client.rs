use crate::{
    line::{DisruptionByLine, DisruptionByMode, RouteRequest, RouteRequestById},
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

    pub fn api_version(&self) -> VersionRequest<'_> {
        VersionRequest::new(self)
    }

    // APIs relating to Line and similar services
    // pub fn line(&self) -> LineRequest {
    //    LineRequest::new(Self)
    //}

    pub fn routes(&self) -> RouteRequest<'_> {
        RouteRequest::new(self)
    }

    pub fn routes_by_id(&self) -> RouteRequestById<'_> {
        RouteRequestById::new(self)
    }

    pub fn disruptions_by_mode(&self) -> DisruptionByMode<'_> {
        DisruptionByMode::new(self)
    }

    pub fn disruptions_by_line(&self) -> DisruptionByLine<'_> {
        DisruptionByLine::new(self)
    }
}
