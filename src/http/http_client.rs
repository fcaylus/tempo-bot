use crate::http::http_client::Credentials::{Bearer, UsernamePassword};
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Credentials {
    Bearer(String),
    UsernamePassword(String, String),
}

#[derive(Debug)]
pub struct HttpClientConfig {
    pub host: String,
    pub base_url: String,
    pub credentials: Credentials,
}

#[derive(Debug)]
pub struct HttpClient {
    client: Client,
    pub config: HttpClientConfig,
}

impl HttpClientConfig {
    pub fn new(host: &str, base_path: &str, credentials: Credentials) -> Self {
        Self {
            host: host.to_string(),
            credentials,
            base_url: format!("https://{}/{}", host, base_path),
        }
    }
}

impl HttpClient {
    pub fn new(config: HttpClientConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> T {
        let request_builder = self
            .client
            .get(format!("{}/{}", self.config.base_url, path));
        return self.send_request(request_builder).await;
    }

    pub async fn post<T: DeserializeOwned>(
        &self,
        path: &str,
        payload: Option<&HashMap<String, Value>>,
    ) -> T {
        let mut request_builder = self
            .client
            .post(format!("{}/{}", self.config.base_url, path));

        if let Some(p) = payload {
            request_builder = request_builder.json(p);
        }

        return self.send_request(request_builder).await;
    }

    async fn send_request<T: DeserializeOwned>(&self, request_builder: RequestBuilder) -> T {
        let mut builder = request_builder;
        match &self.config.credentials {
            Bearer(bearer) => builder = builder.bearer_auth(bearer),
            UsernamePassword(username, password) => {
                builder = builder.basic_auth(username, Some(password))
            }
        }

        let response = builder.send().await.unwrap();

        let status = response.status();
        let body = response.text().await.unwrap();

        if status.is_server_error() {
            panic!("A server error occurred ({}): {}", status, body);
        } else if status.is_client_error() {
            panic!("A request error occurred: ({}): {}", status, body);
        }

        let json = serde_json::from_str(body.as_str());
        return match json {
            Ok(data) => data,
            Err(e) => panic!(
                "Could not parse response JSON. Reason: {}. Response body: \n{}",
                e, body
            ),
        };
    }
}
