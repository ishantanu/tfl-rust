use crate::request::*;

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
}
