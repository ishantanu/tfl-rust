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
    fn get_parameters(&self) -> &models::Parameters;
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
            parameters: models::Parameters,
        }
        impl<'a> $name<'a> {
            pub(crate) fn new(client: &'a Client) -> Self {
                Self { 
                    client,
                    parameters: Default::default(),
                }
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

    fn get_parameters(&self) ->  &models::Parameters {
        &self.parameters
    }

    fn get_client(&self) -> &Client {
        self.client
    }
}

create_endpoint!(RouteRequest);

impl RequestBuilder for RouteRequest<'_> {
    type Response = models::Line;

    fn get_client(&self) ->  &Client {
        self.client
    }

    fn get_parameters(&self) ->  &models::Parameters {
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

    fn get_client(&self) ->  &Client {
        self.client
    }

    fn get_parameters(&self) -> &models::Parameters {
        &self.parameters
    }

    fn get_request_url(&self) -> String {
        format!("/Line/{}/Route?{}", self.get_parameters().line_id, self.get_parameters().service_type)
    }
}

impl RouteRequestById<'_> {
    // filter by line id
    pub fn line(mut self, line: models::LineID) -> Self {
        self.parameters.line_id = line.line().to_string();
        self
    }

    pub fn service_type(mut self, service_types: models::ServiceTypes) -> Self {
        self.parameters.service_type = service_types.to_type().to_string();
        self
    }
}

