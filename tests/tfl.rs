#[cfg(test)]
mod tests {
    use std::env;

    use tfl_api_wrapper::{Client, RequestBuilder};
    use tfl_api_wrapper::models::ServiceTypes;

    fn get_client() -> Client {
        Client::new(env::var("APP_KEY").unwrap())
    }

    #[tokio::test]
    async fn it_is_version_1() {
        let client = get_client();
        let ver = client.api_version().fetch().await.unwrap();
        println!("aaa {:?}", ver);
       assert_eq!(ver.version, "master.5758\r\n");
    }

    #[tokio::test]
    async fn it_fetches_routes () {
        let client = get_client();
        let routes = client.routes().service_type(ServiceTypes::Regular).fetch().await.unwrap();
        for trains in routes.clone() {
            if trains.mode_name ==  "tube" {
                println!("{}", trains.name)
            }
        }
        assert!(!routes.is_empty());
    }

    #[tokio::test]
    async fn it_fetches_routes_by_id () {
        let client = get_client();

        let route = client.routes_by_id().line(tfl_api_wrapper::models::LineID::Bakerloo).fetch().await.unwrap();
        assert_eq!(route.name, "Bakerloo");
    }


}