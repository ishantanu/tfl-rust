#[cfg(test)]
mod tests {
    use std::{env, vec};

    use tfl_api_wrapper::models::{self};
    use tfl_api_wrapper::{linemodels, Client, RequestBuilder};

    fn get_client() -> Client {
        Client::new(env::var("APP_KEY").unwrap())
    }

    #[tokio::test]
    async fn it_is_expected_version() {
        let client = get_client();
        let ver = client.api_version().fetch().await.unwrap();
        println!("aaa {:?}", ver);
        assert_eq!(ver.version, "master.5758\r\n");
    }

    #[tokio::test]
    async fn it_fetches_routes() {
        let client = get_client();
        let routes = client
            .routes()
            .service_type(models::ServiceTypes::Regular)
            .fetch()
            .await
            .unwrap();
        assert!(!routes.is_empty());
    }

    #[tokio::test]
    async fn it_fetches_routes_by_line() {
        let client = get_client();

        let lines: Vec<linemodels::LineID> = vec![linemodels::LineID::Bakerloo];
        let route = client.routes_by_line().line(lines).fetch().await.unwrap();
        assert_eq!(route.name, "Bakerloo");
    }

    #[tokio::test]
    async fn it_fetches_disruptions_by_mode() {
        let client = get_client();

        let modes: Vec<models::Mode> = vec![models::Mode::Bus, models::Mode::Tube];
        let disruptions = client
            .disruptions_by_mode()
            .mode(modes)
            .fetch()
            .await
            .unwrap();
        if disruptions.len() == 0 {
            assert!(disruptions.is_empty());
        } else {
            assert!(!disruptions.is_empty());
        }
    }

    #[tokio::test]
    async fn it_fetches_disruptions_by_line() {
        let client = get_client();
        let lines: Vec<linemodels::LineID> =
            vec![linemodels::LineID::Bakerloo, linemodels::LineID::Jubilee];

        let disruptions = client
            .disruptions_by_line()
            .line(lines)
            .fetch()
            .await
            .unwrap();
        if disruptions.len() == 0 {
            assert!(disruptions.is_empty());
        } else {
            assert!(!disruptions.is_empty());
        }
    }

    #[tokio::test]
    async fn it_predicts_arrivals_by_line() {
        let client = get_client();
        let lines: Vec<linemodels::LineID> =
            vec![linemodels::LineID::Bakerloo, linemodels::LineID::Jubilee];

        let arrivals = client
            .arrival_predictions_by_lines()
            .line(lines)
            .fetch()
            .await
            .unwrap();
        if arrivals.len() == 0 {
            assert!(arrivals.is_empty());
        } else {
            assert!(!arrivals.is_empty());
        }
    }

    #[tokio::test]
    async fn it_predicts_arrivals_by_line_with_stoppoint_id() {
        let client = get_client();
        let lines: Vec<linemodels::LineID> = vec![linemodels::LineID::Bakerloo];
        let predicted_arrivals = client
            .arrival_predictions_by_lines_with_stoppoint()
            .line(lines)
            .stop_point("940GZZLUPAH")
            .direction(models::Directions::Inbound)
            .fetch()
            .await
            .unwrap();
        println!("{:?}", predicted_arrivals);
        if predicted_arrivals.len() == 0 {
            assert!(predicted_arrivals.is_empty());
        } else {
            assert!(!predicted_arrivals.is_empty())
        }
    }

    #[tokio::test]
    async fn it_lists_stations_by_lines() {
        let client = get_client();
        let lines: Vec<linemodels::LineID> = vec![linemodels::LineID::Bakerloo];
        let stations = client
            .list_stations_by_lines()
            .line(lines)
            .fetch()
            .await
            .unwrap();
        println!("{:?}", stations);
        assert!(!stations.is_empty())
    }

    #[tokio::test]
    async fn it_lists_disruption_categories() {
        let client = get_client();
        let disruption_categories = client.list_disruption_categories().fetch().await.unwrap();
        assert!(!disruption_categories.is_empty())
    }

    #[tokio::test]
    async fn it_lists_valid_modes() {
        let client = get_client();
        let valid_modes = client.list_modes().fetch().await.unwrap();
        assert!(!valid_modes.is_empty())
    }

    #[tokio::test]
    async fn it_lists_service_types() {
        let client = get_client();
        let service_types = client.list_service_types().fetch().await.unwrap();
        assert!(!service_types.is_empty())
    }

    #[tokio::test]
    async fn it_lists_severity_types() {
        let client = get_client();
        let severity_types = client.list_severity_types().fetch().await.unwrap();
        assert!(!severity_types.is_empty());
    }

    #[tokio::test]
    async fn it_lists_routes_by_modes() {
        let client = get_client();
        let modes: Vec<models::Mode> = vec![models::Mode::Bus, models::Mode::Tube];
        let routes = client
            .list_lines_routes_by_modes()
            .mode(modes)
            .fetch()
            .await
            .unwrap();
        assert!(!routes.is_empty())
    }

    #[tokio::test]
    async fn it_lists_routes_by_line_with_sequence() {
        let client = get_client();
        let routes = client
            .list_routes_by_line_with_sequence()
            .line(linemodels::LineID::Victoria)
            .direction(models::Directions::Inbound)
            .service_type(models::ServiceTypes::Regular)
            .exclude_crowding(true)
            .fetch()
            .await
            .unwrap();
        assert_eq!(routes.line_name, "Victoria")
    }

    #[tokio::test]
    async fn it_lists_lines_by_id() {
        let client = get_client();
        let lines: Vec<linemodels::LineID> = vec![linemodels::LineID::Bakerloo];
        let lines = client.list_lines_by_id().line(lines).fetch().await.unwrap();
        assert!(!lines.is_empty())
    }

    #[tokio::test]
    async fn it_lists_lines_by_modes() {
        let client = get_client();
        let modes: Vec<models::Mode> = vec![models::Mode::Tube, models::Mode::Bus];
        let lines = client
            .list_lines_by_modes()
            .mode(modes)
            .fetch()
            .await
            .unwrap();
        assert!(!lines.is_empty())
    }

    #[tokio::test]
    async fn it_fetches_line_status_by_severity() {
        let client = get_client();
        let line_statuses = client
            .line_status_by_severity()
            .level(0)
            .fetch()
            .await
            .unwrap();
        if line_statuses.len() == 0 {
            assert!(line_statuses.is_empty())
        } else {
            assert!(!line_statuses.is_empty())
        }
    }

    #[tokio::test]
    async fn it_fetches_line_statuses_between_dates() {
        let client = get_client();
        let lines: Vec<linemodels::LineID> = vec![linemodels::LineID::Bakerloo];
        let line_statuses = client
            .line_status_between_dates()
            .line(lines)
            .start_date("2023-07-19T10:11:12")
            .end_date("2023-07-20T10:11:12")
            .detail(true)
            .fetch()
            .await
            .unwrap();
        if line_statuses.len() == 0 {
            assert!(line_statuses.is_empty())
        } else {
            assert!(!line_statuses.is_empty())
        }
    }

    #[tokio::test]
    async fn it_fetches_line_statuses_for_modes() {
        let client = get_client();
        let modes: Vec<models::Mode> = vec![models::Mode::Tube];
        let line_statuses = client
            .line_status_by_modes()
            .mode(modes)
            .detail(true)
            .fetch()
            .await
            .unwrap();
        if line_statuses.len() == 0 {
            assert!(line_statuses.is_empty())
        } else {
            assert!(!line_statuses.is_empty())
        }
    }

    #[tokio::test]
    async fn it_fetches_line_statuses_by_id() {
        let client = get_client();
        let lines: Vec<linemodels::LineID> = vec![linemodels::LineID::Bakerloo];
        let line_statuses = client
            .line_status_by_ids()
            .line(lines)
            .detail(true)
            .fetch()
            .await
            .unwrap();
        if line_statuses.len() == 0 {
            assert!(line_statuses.is_empty())
        } else {
            assert!(!line_statuses.is_empty())
        }
    }

    #[tokio::test]
    async fn it_fetches_timetable_for_station_by_line_id() {
        let client = get_client();
        let timetable = client
            .station_timetable_by_line()
            .line(linemodels::LineID::Bakerloo)
            .from_stop_point_id("940GZZLUERB")
            .fetch()
            .await
            .unwrap();
        assert!(!timetable.disambiguation.disambiguation_options.is_empty())
    }

    #[tokio::test]
    async fn it_fetches_timetable_for_station_with_destination_by_line_id() {
        let client = get_client();
        let timetable = client
            .station_timetable_with_destination_by_line()
            .line(linemodels::LineID::Bakerloo)
            .from_stop_point_id("940GZZLUERB")
            .to_stop_point_id("940GZZLUERB")
            .fetch()
            .await
            .unwrap();
        assert_eq!(timetable.line_name, "Bakerloo")
    }

    #[tokio::test]
    async fn it_fetches_lines_routes_by_query() {
        let client = get_client();
        let modes: Vec<models::Mode> = vec![models::Mode::Tube];
        let results = client
            .search_line_routes_by_query()
            .query(linemodels::LineID::Victoria)
            .mode(modes)
            .service_type(models::ServiceTypes::Regular)
            .fetch()
            .await
            .unwrap();
        assert!(!results.search_matches.is_empty())
    }
}
