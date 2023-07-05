#[cfg(test)]
mod tests {
    use tfl_api_wrapper::{Client, RequestBuilder};

    fn get_client() -> Client {
        Client::new(
            std::env::var("APP_KEY").unwrap()
        )
    }

    #[tokio::test]
    async fn it_is_version_1() {
        const VERSION: &str = "master.5754\r\n";
        let client = get_client();
        let ver = client.api_version().fetch().await.unwrap();
        assert_eq!(ver.version, VERSION, "Version is not {}", VERSION);
    }
}