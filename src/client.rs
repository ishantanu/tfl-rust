use crate::{
    line::{
        ArrivalPredictionsByLines, ArrivalPredictionsByLinesStopPointID, DisruptionByLines,
        DisruptionByMode, LineStatusBySeverity, ListDisruptionCategories, ListLinesByID,
        ListLinesByModes, ListLinesRoutesByModes, ListModes, ListRoutesForLineIDWithSequence,
        ListServiceTypes, ListSeverityTypes, ListStationsByLines, RouteRequest, RouteRequestById,
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
    pub fn disruptions_by_line(&self) -> DisruptionByLines<'_> {
        DisruptionByLines::new(self)
    }

    /// Return arrival predictions by specified Line(s)
    pub fn arrival_predictions_by_lines(&self) -> ArrivalPredictionsByLines<'_> {
        ArrivalPredictionsByLines::new(self)
    }

    /// Return arrival predictions by lines with stop point ID and optionally specifying direction, destination station id
    pub fn arrival_predictions_by_lines_with_stoppoint(
        &self,
    ) -> ArrivalPredictionsByLinesStopPointID<'_> {
        ArrivalPredictionsByLinesStopPointID::new(self)
    }

    /// Get a list of station that serve give lines
    pub fn list_stations_by_lines(&self) -> ListStationsByLines<'_> {
        ListStationsByLines::new(self)
    }

    /// Gets a list of valid disruption categories
    pub fn list_disruption_categories(&self) -> ListDisruptionCategories<'_> {
        ListDisruptionCategories::new(self)
    }

    /// Gets a list of valid modes
    pub fn list_modes(&self) -> ListModes<'_> {
        ListModes::new(self)
    }

    /// Gets a list of valid ServiceTypes to filter on
    pub fn list_service_types(&self) -> ListServiceTypes<'_> {
        ListServiceTypes::new(self)
    }

    /// Gets a list of valid severity codes
    pub fn list_severity_types(&self) -> ListSeverityTypes<'_> {
        ListSeverityTypes::new(self)
    }

    /// Gets all lines and their valid routes for given modes, including the name and id of the originating and terminating stops for each route
    pub fn list_lines_routes_by_modes(&self) -> ListLinesRoutesByModes<'_> {
        ListLinesRoutesByModes::new(self)
    }

    /// Gets all valid routes for given line id, including the sequence of stops on each route.
    pub fn list_routes_by_line_with_sequence(&self) -> ListRoutesForLineIDWithSequence<'_> {
        ListRoutesForLineIDWithSequence::new(self)
    }

    /// Gets lines that match the specified line ids.
    pub fn list_lines_by_id(&self) -> ListLinesByID<'_> {
        ListLinesByID::new(self)
    }

    /// Gets lines that serve the given modes.
    pub fn list_lines_by_modes(&self) -> ListLinesByModes<'_> {
        ListLinesByModes::new(self)
    }

    /// Gets the line status for all lines with a given severity
    pub fn line_status_by_severity(&self) -> LineStatusBySeverity<'_> {
        LineStatusBySeverity::new(self)
    }
}
