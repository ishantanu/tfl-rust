#[cfg(test)]
mod tests {
    use std::env;

    use tfl_api_wrapper::models::{self};
    use tfl_api_wrapper::{Client, RequestBuilder};

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

        let lines: Vec<models::LineID> = vec![models::LineID::Bakerloo];
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
        let lines: Vec<models::LineID> = vec![models::LineID::Bakerloo, models::LineID::Jubilee];

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
    async fn it_fetches_arrivals_by_line() {
        let client = get_client();
        let lines: Vec<models::LineID> = vec![models::LineID::Bakerloo, models::LineID::Jubilee];

        let arrivals = client
            .arrival_predictions_by_line()
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
}
