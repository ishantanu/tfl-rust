use crate::{models, request::create_endpoint, Client, RequestBuilder};

create_endpoint!(RouteRequest);

impl RequestBuilder for RouteRequest<'_> {
    type Response = models::Line;

    fn get_client(&self) -> &Client {
        self.client
    }

    fn get_parameters(&self) -> &models::Parameters {
        &self.parameters
    }

    fn get_request_url(&self) -> String {
        format!("/Line/Route?{}", self.get_parameters().service_type)
    }
}

impl RouteRequest<'_> {
    pub fn service_type(mut self, service_types: models::ServiceTypes) -> Self {
        self.parameters.service_type = service_types.to_type().to_string();
        self
    }
}

create_endpoint!(RouteRequestById);

impl RequestBuilder for RouteRequestById<'_> {
    type Response = models::Lines;

    fn get_client(&self) -> &Client {
        self.client
    }

    fn get_parameters(&self) -> &models::Parameters {
        &self.parameters
    }

    fn get_request_url(&self) -> String {
        format!(
            "/Line/{}/Route?{}",
            self.get_parameters().line_id,
            self.get_parameters().service_type
        )
    }
}

impl RouteRequestById<'_> {
    // filter by line id
    pub fn line(mut self, line: models::LineID) -> Self {
        self.parameters.line_id = line.line().to_string();
        self
    }

    pub fn service_type(mut self, service_types: models::ServiceTypes) -> Self {
        self.parameters.service_type = service_types.to_type().to_string();
        self
    }
}

create_endpoint!(DisruptionByMode);

impl RequestBuilder for DisruptionByMode<'_> {
    type Response = models::Disruption;

    fn get_client(&self) -> &Client {
        self.client
    }

    fn get_parameters(&self) -> &models::Parameters {
        &self.parameters
    }

    fn get_request_url(&self) -> String {
        format!("/Line/Mode/{}/Disruption", self.get_parameters().mode,)
    }
}

impl DisruptionByMode<'_> {
    // Get disruption for all lines by mode
    pub fn mode(mut self, modes: models::Mode) -> Self {
        self.parameters.mode = modes.mode().to_string();
        self
    }
}
