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

        let route = client
            .routes_by_id()
            .line(models::LineID::Bakerloo)
            .fetch()
            .await
            .unwrap();
        assert_eq!(route.name, "Bakerloo");
    }

    #[tokio::test]
    async fn it_fetches_disruptions_by_mode() {
        let client = get_client();

        let disruptions = client
            .disruptions_by_mode()
            .mode(models::Mode::Bus)
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

        let disruptions = client
            .disruptions_by_line()
            .line(models::LineID::Bakerloo)
            .fetch()
            .await
            .unwrap();
        if disruptions.len() == 0 {
            assert!(disruptions.is_empty());
        } else {
            assert!(!disruptions.is_empty());
        }
    }
}
