#[cfg(test)]
mod tests {
    use std::env;

    use tfl_api_wrapper::{Client, RequestBuilder};

    fn get_client() -> Client {
        Client::new(env::var("APP_KEY").unwrap())
    }

    #[tokio::test]
    async fn it_is_version_1() {
        let client = get_client();
        let ver = client.api_version().fetch().await.unwrap();
        println!("aaa {:?}", ver);
       assert_eq!(ver.version, "master.5753\r\n");
    }
}