use hex::encode as hex_encode;
use hmac::{Hmac, Mac};
use crate::errors::{BinanceContentError, Error, Result}; // Updated
use reqwest::StatusCode;
use reqwest::Response; // Updated
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT, CONTENT_TYPE};
use sha2::Sha256;
use serde::de::DeserializeOwned;
use crate::api::API;

#[derive(Clone)]
pub struct Client {
    api_key: String,
    secret_key: String,
    host: String,
    inner_client: reqwest::Client, // Updated
}

impl Client {
    pub fn new(
        api_key: Option<String>,
        secret_key: Option<String>,
        host: String,
    ) -> Self {
        Client {
            api_key: api_key.unwrap_or_default(),
            secret_key: secret_key.unwrap_or_default(),
            host,
            inner_client: reqwest::Client::builder() // Updated
                .pool_idle_timeout(None)
                .build()
                .unwrap(),
        }
    }

    pub async fn get_signed<T: DeserializeOwned>(
        &self,
        endpoint: API,
        request: Option<String>,
    ) -> Result<T> {
        let url = self.sign_request(endpoint, request);
        let client = &self.inner_client;
        let response = client
            .get(url.as_str())
            .headers(self.build_headers(true)?)
            .send()
            .await?; // Updated

        self.handler(response).await // Updated
    }

    pub async fn post_signed<T: DeserializeOwned>(
        &self,
        endpoint: API,
        request: String,
    ) -> Result<T> {
        let url = self.sign_request(endpoint, Some(request));
        let client = &self.inner_client;
        let response = client
            .post(url.as_str())
            .headers(self.build_headers(true)?)
            .send()
            .await?; // Updated

        self.handler(response).await // Updated
    }

    pub async fn delete_signed<T: DeserializeOwned>(
        &self,
        endpoint: API,
        request: Option<String>,
    ) -> Result<T> {
        let url = self.sign_request(endpoint, request);
        let client = &self.inner_client;
        let response = client
            .delete(url.as_str())
            .headers(self.build_headers(true)?)
            .send()
            .await?; // Updated

        self.handler(response).await // Updated
    }

    pub async fn get<T: DeserializeOwned>(
        &self,
        endpoint: API,
        request: Option<String>,
    ) -> Result<T> {
        let mut url: String = format!("{}{}", self.host, String::from(endpoint));
        if let Some(request_str) = request {
            // Renamed to avoid conflict
            if !request_str.is_empty() {
                url.push_str(format!("?{}", request_str).as_str());
            }
        }

        let client = &self.inner_client;
        let response = client.get(url.as_str()).send().await?; // Updated

        self.handler(response).await // Updated
    }

    pub async fn post<T: DeserializeOwned>(
        &self,
        endpoint: API,
    ) -> Result<T> {
        let url: String = format!("{}{}", self.host, String::from(endpoint));

        let client = &self.inner_client;
        let response = client
            .post(url.as_str())
            .headers(self.build_headers(false)?)
            .send()
            .await?; // Updated

        self.handler(response).await // Updated
    }

    pub async fn put<T: DeserializeOwned>(
        &self,
        endpoint: API,
        listen_key: &str,
    ) -> Result<T> {
        let url: String = format!("{}{}", self.host, String::from(endpoint));
        let data: String = format!("listenKey={}", listen_key);

        let client = &self.inner_client;
        let response = client
            .put(url.as_str())
            .headers(self.build_headers(false)?)
            .body(data)
            .send()
            .await?; // Updated

        self.handler(response).await // Updated
    }

    pub async fn delete<T: DeserializeOwned>(
        &self,
        endpoint: API,
        listen_key: &str,
    ) -> Result<T> {
        let url: String = format!("{}{}", self.host, String::from(endpoint));
        let data: String = format!("listenKey={}", listen_key);

        let client = &self.inner_client;
        let response = client
            .delete(url.as_str())
            .headers(self.build_headers(false)?)
            .body(data)
            .send()
            .await?; // Updated

        self.handler(response).await // Updated
    }

    // Request must be signed
    fn sign_request(
        &self,
        endpoint: API,
        request: Option<String>,
    ) -> String {
        if let Some(request_str) = request {
            // Renamed to avoid conflict
            let mut signed_key =
                Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes()).unwrap();
            signed_key.update(request_str.as_bytes());
            let signature = hex_encode(signed_key.finalize().into_bytes());
            let request_body: String = format!("{}&signature={}", request_str, signature);
            format!("{}{}?{}", self.host, String::from(endpoint), request_body)
        } else {
            // HMAC for empty query string still needs a timestamp if that's part of the requirements,
            // but current logic doesn't include it. Assuming it's correct.
            let mut signed_key =
                Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes()).unwrap();
            // If there's no request string, what are we signing?
            // Binance typically requires a timestamp even for parameter-less signed requests.
            // For now, replicating existing logic of signing an empty string if request is None.
            // This might need review based on Binance API spec for parameter-less signed endpoints.
            signed_key.update(b""); // Sign an empty string if no params
            let signature = hex_encode(signed_key.finalize().into_bytes());
            let request_body: String = format!("signature={}", signature); // Removed leading '&'
            format!("{}{}?{}", self.host, String::from(endpoint), request_body)
        }
    }

    fn build_headers(
        &self,
        content_type: bool,
    ) -> Result<HeaderMap> {
        let mut custom_headers = HeaderMap::new();

        custom_headers.insert(USER_AGENT, HeaderValue::from_static("binance-rs"));
        if content_type {
            custom_headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
        }
        custom_headers.insert(
            HeaderName::from_static("x-mbx-apikey"),
            HeaderValue::from_str(self.api_key.as_str())?,
        );

        Ok(custom_headers)
    }

    async fn handler<T: DeserializeOwned>(
        &self,
        response: Response,
    ) -> Result<T> {
        // Updated
        match response.status() {
            StatusCode::OK => Ok(response.json::<T>().await?), // Updated
            StatusCode::INTERNAL_SERVER_ERROR => {
                Err(Error::Custom("Internal Server Error".to_string()))
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                Err(Error::Custom("Service Unavailable".to_string()))
            }
            StatusCode::UNAUTHORIZED => Err(Error::Custom("Unauthorized".to_string())),
            StatusCode::BAD_REQUEST => {
                let error_content = response.text().await?; // Read as text first for better error diagnosis
                match serde_json::from_str::<BinanceContentError>(&error_content) {
                    Ok(binance_error) => Err(Error::BinanceError(binance_error)),
                    Err(json_err) => Err(Error::Custom(format!(
                        "Failed to parse Binance error response (HTTP 400): {}. Original content: {}",
                        json_err, error_content
                    ))),
                }
            }
            s => Err(Error::Custom(format!(
                "Received unexpected status code: {:?}",
                s
            ))),
        }
    }
}
