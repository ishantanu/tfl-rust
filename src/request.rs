use crate::client::Client;
use crate::models;
use async_trait::async_trait;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

const ROOT_URL: &str = "https://api.tfl.gov.uk";

#[derive(Debug)]
pub struct Error {
    pub api_error: Option<models::ApiException>,
    pub http_status: StatusCode,
}

impl Error {
    pub fn new(api_error: Option<models::ApiException>, http_status: StatusCode) -> Self {
        Self {
            api_error,
            http_status,
        }
    }
    pub fn from_status(http_status: StatusCode) -> Self {
        Self {
            api_error: None,
            http_status,
        }
    }
}

#[async_trait]
pub trait RequestBuilder {
    type Response: DeserializeOwned + std::fmt::Debug;

    fn get_request_url(&self) -> String;
    fn get_client(&self) -> &Client;

    async fn fetch(&self) -> Result<Self::Response, Error> {
        let url = format!("{}{}", ROOT_URL, self.get_request_url());
        let auth_params: Vec<(String, String)> =
            vec![("app_key".into(), self.get_client().app_key.clone())];

        let request = self.get_client().req_client.get(url).query(&auth_params);

        let response = request
            .send()
            .await
            .map_err(|e| Error::from_status(e.status().unwrap_or(StatusCode::BAD_REQUEST)))?;

        let status = response.status();
        // todo: implement an appropriate logging solution
        // println!("{:?}", response);
        if status != StatusCode::OK {
            return Err(Error::new(
                response.json::<models::ApiException>().await.ok(),
                status,
            ));
        }

        response.json::<Self::Response>().await.map_err(|e| {
            println!("{e:#?}");
            Error::from_status(StatusCode::BAD_REQUEST)
        })
    }
}

macro_rules! create_endpoint {
    ($name: ident) => {
        pub struct $name<'a> {
            client: &'a Client,
        }
        impl<'a> $name<'a> {
            pub(crate) fn new(client: &'a Client) -> Self {
                Self { client }
            }
        }
    };
}

create_endpoint!(VersionRequest);

impl RequestBuilder for VersionRequest<'_> {
    type Response = models::Version;

    fn get_request_url(&self) -> String {
        "/version".into()
    }

    fn get_client(&self) -> &Client {
        self.client
    }
}
