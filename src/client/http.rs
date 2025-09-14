use crate::client::auth::{Credentials, Signer};
use crate::error::{ApiErrorResponse, BinanceError, Result};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::time::Duration;

const BASE_URL: &str = "https://fapi.binance.com";
const TESTNET_URL: &str = "https://testnet.binancefuture.com";

#[derive(Clone)]
pub struct HttpClient {
    client: Client,
    base_url: String,
    signer: Option<Signer>,
}

impl HttpClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: BASE_URL.to_string(),
            signer: None,
        }
    }

    pub fn new_with_credentials(credentials: Credentials) -> Self {
        let mut client = Self::new();
        client.signer = Some(Signer::new(credentials));
        client
    }

    pub fn testnet() -> Self {
        let mut client = Self::new();
        client.base_url = TESTNET_URL.to_string();
        client
    }

    pub fn testnet_with_credentials(credentials: Credentials) -> Self {
        let mut client = Self::testnet();
        client.signer = Some(Signer::new(credentials));
        client
    }

    /// Make a public GET request (no authentication required)
    pub async fn get_public<T>(&self, endpoint: &str, params: Option<HashMap<String, String>>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut request = self.client.get(&url);

        if let Some(params) = params {
            request = request.query(&params);
        }

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Make a signed GET request (authentication required)
    pub async fn get_signed<T>(&self, endpoint: &str, params: Option<HashMap<String, String>>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let signer = self.signer.as_ref().ok_or_else(|| {
            BinanceError::Authentication("No credentials provided for signed request".to_string())
        })?;

        let signed_params = if let Some(params) = params {
            signer.sign_request(params)?
        } else {
            signer.sign_request(HashMap::new())?
        };

        let url = format!("{}{}", self.base_url, endpoint);
        let request = self.client
            .get(&url)
            .query(&signed_params)
            .header("X-MBX-APIKEY", signer.get_api_key());

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Make a signed POST request
    pub async fn post_signed<T>(&self, endpoint: &str, params: Option<HashMap<String, String>>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let signer = self.signer.as_ref().ok_or_else(|| {
            BinanceError::Authentication("No credentials provided for signed request".to_string())
        })?;

        let signed_params = if let Some(params) = params {
            signer.sign_request(params)?
        } else {
            signer.sign_request(HashMap::new())?
        };

        let url = format!("{}{}", self.base_url, endpoint);
        let request = self.client
            .post(&url)
            .form(&signed_params)
            .header("X-MBX-APIKEY", signer.get_api_key());

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Make a signed PUT request
    pub async fn put_signed<T>(&self, endpoint: &str, params: Option<HashMap<String, String>>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let signer = self.signer.as_ref().ok_or(BinanceError::Authentication(
            "No credentials provided for signed request".to_string(),
        ))?;

        let url = format!("{}{}", self.base_url, endpoint);
        let timestamp = crate::utils::get_timestamp();
        
        let mut query_params = params.unwrap_or_default();
        query_params.insert("timestamp".to_string(), timestamp.to_string());

        let query_string = crate::utils::build_query_string_from_map(&query_params);
        let signature = signer.sign(&query_string)?;
        query_params.insert("signature".to_string(), signature);

        let response = self
            .client
            .put(&url)
            .query(&query_params)
            .header("X-MBX-APIKEY", signer.get_api_key())
            .send()
            .await
            .map_err(|e| BinanceError::Http(e))?;

        self.handle_response(response).await
    }

    /// Make a signed DELETE request
    pub async fn delete_signed<T>(&self, endpoint: &str, params: Option<HashMap<String, String>>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let signer = self.signer.as_ref().ok_or(BinanceError::Authentication(
            "No credentials provided for signed request".to_string(),
        ))?;

        let url = format!("{}{}", self.base_url, endpoint);
        let timestamp = crate::utils::get_timestamp();
        
        let mut query_params = params.unwrap_or_default();
        query_params.insert("timestamp".to_string(), timestamp.to_string());

        let query_string = crate::utils::build_query_string_from_map(&query_params);
        let signature = signer.sign(&query_string)?;
        query_params.insert("signature".to_string(), signature);

        let response = self
            .client
            .delete(&url)
            .query(&query_params)
            .header("X-MBX-APIKEY", signer.get_api_key())
            .send()
            .await
            .map_err(|e| BinanceError::Http(e))?;

        self.handle_response(response).await
    }

    async fn handle_response<T>(&self, response: Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let status = response.status();
        let text = response.text().await?;

        if status.is_success() {
            serde_json::from_str(&text).map_err(BinanceError::Json)
        } else {
            // Try to parse as API error response
            if let Ok(error_response) = serde_json::from_str::<ApiErrorResponse>(&text) {
                Err(BinanceError::from(error_response))
            } else {
                Err(BinanceError::Unknown(format!(
                    "HTTP {} - {}",
                    status.as_u16(),
                    text
                )))
            }
        }
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = HttpClient::new();
        assert_eq!(client.base_url, BASE_URL);
        assert!(client.signer.is_none());
    }

    #[test]
    fn test_testnet_client() {
        let client = HttpClient::testnet();
        assert_eq!(client.base_url, TESTNET_URL);
    }

    #[test]
    fn test_client_with_credentials() {
        let credentials = Credentials::new("test_key".to_string(), "test_secret".to_string());
        let client = HttpClient::new_with_credentials(credentials);
        assert!(client.signer.is_some());
    }
}
