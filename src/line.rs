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
            self.get_parameters().line,
            self.get_parameters().service_type
        )
    }
}

impl RouteRequestById<'_> {
    // filter by line ids
    pub fn line(mut self, line: Vec<models::LineID>) -> Self {
        let mut lines: String = "".to_owned();
        for k in line {
            //k.line().to_string();
            if lines == "" {
                lines.push_str(k.line().into());
            } else {
                lines.push_str(",");
                lines.push_str(k.line().into());
            }
        }
        self.parameters.line = lines;
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
        format!("/Line/Mode/{}/Disruption", self.get_parameters().mode)
    }
}

impl DisruptionByMode<'_> {
    // Get disruption for all lines by mode
    pub fn mode(mut self, mode: Vec<models::Mode>) -> Self {
        let mut modes: String = "".to_owned();
        for k in mode {
            //k.line().to_string();
            if modes == "" {
                modes.push_str(k.mode().into());
            } else {
                modes.push_str(",");
                modes.push_str(k.mode().into());
            }
        }
        self.parameters.mode = modes;
        self
    }
}

create_endpoint!(DisruptionByLine);

impl RequestBuilder for DisruptionByLine<'_> {
    type Response = models::Disruption;

    fn get_client(&self) -> &Client {
        self.client
    }

    fn get_parameters(&self) -> &models::Parameters {
        &self.parameters
    }

    fn get_request_url(&self) -> String {
        format!("/Line/{}/Disruption", self.get_parameters().line)
    }
}

impl DisruptionByLine<'_> {
    // filter by line
    pub fn line(mut self, line: Vec<models::LineID>) -> Self {
        let mut lines: String = "".to_owned();
        for k in line {
            //k.line().to_string();
            if lines == "" {
                lines.push_str(k.line().into());
            } else {
                lines.push_str(",");
                lines.push_str(k.line().into());
            }
        }
        self.parameters.line = lines;
        self
    }
}

create_endpoint!(ArrivalPredictionsByLine);

impl RequestBuilder for ArrivalPredictionsByLine<'_> {
    type Response = models::Arrival;

    fn get_client(&self) -> &Client {
        self.client
    }

    fn get_parameters(&self) -> &models::Parameters {
        &self.parameters
    }

    fn get_request_url(&self) -> String {
        format!("/Line/{}/Arrivals", self.get_parameters().line)
    }
}

impl ArrivalPredictionsByLine<'_> {
    // filter by line
    pub fn line(mut self, line: Vec<models::LineID>) -> Self {
        let mut lines: String = "".to_owned();
        for k in line {
            //k.line().to_string();
            if lines == "" {
                lines.push_str(k.line().into());
            } else {
                lines.push_str(",");
                lines.push_str(k.line().into());
            }
        }
        self.parameters.line = lines;
        self
    }
}
