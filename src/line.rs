use crate::{
    linemodels::{self},
    models::{self},
    request::create_endpoint,
    Client, RequestBuilder,
};

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
        format!("/Line/Route?{:?}", self.get_parameters().service_type)
    }
}

impl RouteRequest<'_> {
    pub fn service_type(mut self, service_types: models::ServiceTypes) -> Self {
        self.parameters.service_type = Some(service_types.to_type().to_string());
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
            "/Line/{}/Route?{:?}",
            self.get_parameters().lines,
            self.get_parameters().service_type
        )
    }
}

impl RouteRequestById<'_> {
    // filter by line ids
    pub fn line(mut self, line: Vec<linemodels::LineID>) -> Self {
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
        self.parameters.lines = lines;
        self
    }

    pub fn service_type(mut self, service_types: models::ServiceTypes) -> Self {
        self.parameters.service_type = Some(service_types.to_type().to_string());
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
        format!("/Line/Mode/{}/Disruption", self.get_parameters().modes)
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
        self.parameters.modes = modes;
        self
    }
}

create_endpoint!(DisruptionByLines);

impl RequestBuilder for DisruptionByLines<'_> {
    type Response = models::Disruption;

    fn get_client(&self) -> &Client {
        self.client
    }

    fn get_parameters(&self) -> &models::Parameters {
        &self.parameters
    }

    fn get_request_url(&self) -> String {
        format!("/Line/{}/Disruption", self.get_parameters().lines)
    }
}

impl DisruptionByLines<'_> {
    // filter by line
    pub fn line(mut self, line: Vec<linemodels::LineID>) -> Self {
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
        self.parameters.lines = lines;
        self
    }
}

create_endpoint!(ArrivalPredictionsByLines);

impl RequestBuilder for ArrivalPredictionsByLines<'_> {
    type Response = models::Arrival;

    fn get_client(&self) -> &Client {
        self.client
    }

    fn get_parameters(&self) -> &models::Parameters {
        &self.parameters
    }

    fn get_request_url(&self) -> String {
        format!("/Line/{}/Arrivals", self.get_parameters().lines)
    }
}

impl ArrivalPredictionsByLines<'_> {
    // filter by line
    pub fn line(mut self, line: Vec<linemodels::LineID>) -> Self {
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
        self.parameters.lines = lines;
        self
    }
}

create_endpoint!(ArrivalPredictionsByLinesStopPointID);

impl RequestBuilder for ArrivalPredictionsByLinesStopPointID<'_> {
    type Response = models::Arrival;

    fn get_client(&self) -> &Client {
        self.client
    }

    fn get_parameters(&self) -> &models::Parameters {
        &self.parameters
    }

    fn get_request_url(&self) -> String {
        if !self.get_parameters().stop_point_id.is_empty() {
            if self.get_parameters().direction.is_none()
                && self.get_parameters().destination_station_id.is_none()
            {
                format!(
                    "/Line/{}/Arrivals/{}",
                    self.get_parameters().lines,
                    self.get_parameters().stop_point_id
                )
            } else if self.get_parameters().direction.is_some()
                && self.get_parameters().destination_station_id.is_none()
            {
                format!(
                    "/Line/{}/Arrivals/{}?{:?}",
                    self.get_parameters().lines,
                    self.get_parameters().stop_point_id,
                    self.get_parameters().direction
                )
            } else {
                format!(
                    "/Line/{}/Arrivals/{}?{:?}",
                    self.get_parameters().lines,
                    self.get_parameters().stop_point_id,
                    self.get_parameters().direction
                )
            }
        } else {
            format!(
                "/Line/{}/Arrivals/{}",
                self.get_parameters().lines,
                self.get_parameters().stop_point_id
            )
        }
    }
}

impl ArrivalPredictionsByLinesStopPointID<'_> {
    // filter by line
    pub fn line(mut self, line: Vec<linemodels::LineID>) -> Self {
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
        self.parameters.lines = lines;
        self
    }

    pub fn stop_point(mut self, stop_point_id: &str) -> Self {
        self.parameters.stop_point_id = stop_point_id.to_string();
        self
    }

    pub fn direction(mut self, direction: models::Directions) -> Self {
        self.parameters.direction = Some(direction.to_type().to_string());
        self
    }

    pub fn destination_station_id(mut self, destination_station_id: &str) -> Self {
        self.parameters.destination_station_id = Some(destination_station_id.to_string());
        self
    }
}

create_endpoint!(ListStationsByLines);

impl RequestBuilder for ListStationsByLines<'_> {
    type Response = models::StopPoint;

    fn get_client(&self) -> &Client {
        self.client
    }

    fn get_parameters(&self) -> &models::Parameters {
        &self.parameters
    }

    fn get_request_url(&self) -> String {
        format!("/Line/{}/StopPoints?", self.get_parameters().lines)
    }
}

impl ListStationsByLines<'_> {
    // filter by line
    pub fn line(mut self, line: Vec<linemodels::LineID>) -> Self {
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
        self.parameters.lines = lines;
        self
    }

    pub fn tfl_operated_national_rail_stations_only(
        mut self,
        tfl_operated_national_rail_stations_only: bool,
    ) -> Self {
        self.parameters.tfl_operated_national_rail_stations_only =
            Some(tfl_operated_national_rail_stations_only);
        self
    }
}

create_endpoint!(ListDisruptionCategories);

impl RequestBuilder for ListDisruptionCategories<'_> {
    type Response = models::DisruptionCategories;

    fn get_request_url(&self) -> String {
        "/Line/Meta/DisruptionCategories".into()
    }

    fn get_parameters(&self) -> &models::Parameters {
        &self.parameters
    }

    fn get_client(&self) -> &Client {
        self.client
    }
}

create_endpoint!(ListModes);

impl RequestBuilder for ListModes<'_> {
    type Response = models::Modes;

    fn get_request_url(&self) -> String {
        "/Line/Meta/Modes".into()
    }

    fn get_parameters(&self) -> &models::Parameters {
        &self.parameters
    }

    fn get_client(&self) -> &Client {
        self.client
    }
}

create_endpoint!(ListServiceTypes);

impl RequestBuilder for ListServiceTypes<'_> {
    type Response = models::LServiceTypes;

    fn get_request_url(&self) -> String {
        "/Line/Meta/ServiceTypes".into()
    }

    fn get_parameters(&self) -> &models::Parameters {
        &self.parameters
    }

    fn get_client(&self) -> &Client {
        self.client
    }
}

create_endpoint!(ListSeverityTypes);

impl RequestBuilder for ListSeverityTypes<'_> {
    type Response = models::Serverities;

    fn get_request_url(&self) -> String {
        "/Line/Meta/Severity".into()
    }

    fn get_parameters(&self) -> &models::Parameters {
        &self.parameters
    }

    fn get_client(&self) -> &Client {
        self.client
    }
}

create_endpoint!(ListLinesRoutesByModes);

impl RequestBuilder for ListLinesRoutesByModes<'_> {
    type Response = models::Line;

    fn get_request_url(&self) -> String {
        format!(
            "/Line/Mode/{}/Route?{:?}",
            self.get_parameters().modes,
            self.get_parameters().service_type
        )
    }

    fn get_parameters(&self) -> &models::Parameters {
        &self.parameters
    }

    fn get_client(&self) -> &Client {
        self.client
    }
}

impl ListLinesRoutesByModes<'_> {
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
        self.parameters.modes = modes;
        self
    }

    pub fn service_type(mut self, service_types: models::ServiceTypes) -> Self {
        self.parameters.service_type = Some(service_types.to_type().to_string());
        self
    }
}

create_endpoint!(ListRoutesForLineIDWithSequence);

impl RequestBuilder for ListRoutesForLineIDWithSequence<'_> {
    type Response = models::Routes;
    fn get_parameters(&self) -> &models::Parameters {
        &self.parameters
    }

    fn get_request_url(&self) -> String {
        if self.get_parameters().direction.is_some() {
            if self.get_parameters().service_type.is_some()
                && self.get_parameters().exclude_crowding.is_some()
            {
                println!(
                    "{}",
                    format!(
                        "/Line/{}/Route/Sequence/{}?{}&excludeCrowding={}",
                        self.get_parameters().line_id,
                        self.get_parameters().direction.as_ref().unwrap(),
                        self.get_parameters().service_type.as_ref().unwrap(),
                        self.get_parameters().exclude_crowding.as_ref().unwrap()
                    )
                );
                format!(
                    "/Line/{}/Route/Sequence/{}?{}&excludeCrowding={}",
                    self.get_parameters().line_id,
                    self.get_parameters().direction.as_ref().unwrap(),
                    self.get_parameters().service_type.as_ref().unwrap(),
                    self.get_parameters().exclude_crowding.as_ref().unwrap()
                )
            } else if self.get_parameters().service_type.is_some()
                && self.get_parameters().exclude_crowding.is_none()
            {
                format!(
                    "/Line/{}/Route/Sequence/{}/?{}",
                    self.get_parameters().line_id,
                    self.get_parameters().direction.as_ref().unwrap(),
                    self.get_parameters().service_type.as_ref().unwrap()
                )
            } else if self.get_parameters().service_type.is_none()
                && self.get_parameters().exclude_crowding.is_some()
            {
                format!(
                    "/Line/{}/Route/Sequence/{}?excludeCrowding={}",
                    self.get_parameters().line_id,
                    self.get_parameters().direction.as_ref().unwrap(),
                    self.get_parameters().exclude_crowding.as_ref().unwrap()
                )
            } else {
                format!(
                    "/Line/{}/Route/Sequence/{}",
                    self.get_parameters().line_id,
                    self.get_parameters().direction.as_ref().unwrap(),
                )
            }
        } else {
            format!("/Line/{}/Route/Sequence/All", self.get_parameters().line_id)
        }
    }

    fn get_client(&self) -> &Client {
        self.client
    }
}

impl ListRoutesForLineIDWithSequence<'_> {
    // Get line id
    pub fn line(mut self, id: linemodels::LineID) -> Self {
        self.parameters.line_id = id.line().to_string();
        self
    }

    pub fn service_type(mut self, service_type: models::ServiceTypes) -> Self {
        self.parameters.service_type = Some(service_type.to_type().to_string());
        self
    }
    pub fn exclude_crowding(mut self, exclude_crowding: bool) -> Self {
        self.parameters.exclude_crowding = Some(exclude_crowding);
        self
    }

    /// direction of line
    pub fn direction(mut self, direction: models::Directions) -> Self {
        self.parameters.direction = Some(direction.to_type().to_string());
        self
    }
}

create_endpoint!(ListLinesByID);

impl RequestBuilder for ListLinesByID<'_> {
    type Response = models::Line;

    fn get_request_url(&self) -> String {
        format!("/Line/{}", self.get_parameters().lines)
    }

    fn get_parameters(&self) -> &models::Parameters {
        &self.parameters
    }

    fn get_client(&self) -> &Client {
        self.client
    }
}

impl ListLinesByID<'_> {
    // filter by line
    pub fn line(mut self, line: Vec<linemodels::LineID>) -> Self {
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
        self.parameters.lines = lines;
        self
    }
}

create_endpoint!(ListLinesByModes);

impl RequestBuilder for ListLinesByModes<'_> {
    type Response = models::Line;

    fn get_request_url(&self) -> String {
        format!("/Line/Mode/{}", self.get_parameters().modes)
    }

    fn get_parameters(&self) -> &models::Parameters {
        &self.parameters
    }

    fn get_client(&self) -> &Client {
        self.client
    }
}

impl ListLinesByModes<'_> {
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
        self.parameters.modes = modes;
        self
    }
}

create_endpoint!(LineStatusBySeverity);

impl RequestBuilder for LineStatusBySeverity<'_> {
    type Response = models::LineSeverity;

    fn get_request_url(&self) -> String {
        format!("/Line/Status/{}", self.get_parameters().severity)
    }

    fn get_parameters(&self) -> &models::Parameters {
        &self.parameters
    }

    fn get_client(&self) -> &Client {
        self.client
    }
}

impl LineStatusBySeverity<'_> {
    // Get disruption for all lines by mode
    pub fn level(mut self, sev: i8) -> Self {
        self.parameters.severity = sev;
        self
    }
}
