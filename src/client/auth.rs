use crate::error::{BinanceError, Result};
use crate::utils::get_timestamp;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::HashMap;

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone)]
pub struct Credentials {
    pub api_key: String,
    pub secret_key: String,
}

impl Credentials {
    pub fn new(api_key: String, secret_key: String) -> Self {
        Self { api_key, secret_key }
    }
}

#[derive(Clone)]
pub struct Signer {
    credentials: Credentials,
}

impl Signer {
    pub fn new(credentials: Credentials) -> Self {
        Self { credentials }
    }

    /// Generate HMAC-SHA256 signature
    pub fn sign(&self, query_string: &str) -> Result<String> {
        let mut mac = HmacSha256::new_from_slice(self.credentials.secret_key.as_bytes())
            .map_err(|e| BinanceError::Authentication(format!("Invalid signature: {}", e)))?;
        
        mac.update(query_string.as_bytes());
        let signature = mac.finalize().into_bytes();
        Ok(hex::encode(signature))
    }

    /// Sign request parameters
    pub fn sign_request(&self, mut params: HashMap<String, String>) -> Result<HashMap<String, String>> {
        // Add timestamp
        params.insert("timestamp".to_string(), get_timestamp().to_string());
        
        // Build query string
        let mut query_params: Vec<(String, String)> = params.into_iter().collect();
        query_params.sort_by(|a, b| a.0.cmp(&b.0));
        
        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");
        
        // Generate signature
        let signature = self.sign(&query_string)?;
        
        // Convert back to HashMap and add signature
        let mut signed_params: HashMap<String, String> = query_params.into_iter().collect();
        signed_params.insert("signature".to_string(), signature);
        
        Ok(signed_params)
    }

    pub fn get_api_key(&self) -> &str {
        &self.credentials.api_key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign() {
        let credentials = Credentials::new(
            "test_api_key".to_string(),
            "test_secret_key".to_string(),
        );
        let signer = Signer::new(credentials);
        
        let query = "symbol=BTCUSDT&side=BUY&type=LIMIT&quantity=1&price=50000&timestamp=1234567890";
        let signature = signer.sign(query).unwrap();
        
        // Signature should be a 64-character hex string
        assert_eq!(signature.len(), 64);
        assert!(signature.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_sign_request() {
        let credentials = Credentials::new(
            "test_api_key".to_string(),
            "test_secret_key".to_string(),
        );
        let signer = Signer::new(credentials);
        
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), "BTCUSDT".to_string());
        params.insert("side".to_string(), "BUY".to_string());
        
        let signed_params = signer.sign_request(params).unwrap();
        
        assert!(signed_params.contains_key("timestamp"));
        assert!(signed_params.contains_key("signature"));
        assert_eq!(signed_params.get("symbol").unwrap(), "BTCUSDT");
    }
}
