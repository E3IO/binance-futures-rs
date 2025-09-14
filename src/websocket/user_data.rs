use crate::client::http::HttpClient;
use crate::error::{BinanceError, Result};
use serde::Deserialize;
use std::time::{Duration, Instant};
use tokio::time::sleep;

/// Listen key response from Binance API
#[derive(Debug, Clone, Deserialize)]
pub struct ListenKeyResponse {
    #[serde(rename = "listenKey")]
    pub listen_key: String,
}

/// User data stream manager
pub struct UserDataStreamManager {
    http_client: HttpClient,
    listen_key: Option<String>,
    last_keepalive: Option<Instant>,
    keepalive_interval: Duration,
}

impl UserDataStreamManager {
    /// Create a new user data stream manager
    pub fn new(http_client: HttpClient) -> Self {
        Self {
            http_client,
            listen_key: None,
            last_keepalive: None,
            keepalive_interval: Duration::from_secs(30 * 60), // 30 minutes
        }
    }

    /// Create a new listen key
    pub async fn create_listen_key(&mut self) -> Result<String> {
        let response: ListenKeyResponse = self
            .http_client
            .post_signed("/fapi/v1/listenKey", None)
            .await?;

        self.listen_key = Some(response.listen_key.clone());
        self.last_keepalive = Some(Instant::now());

        Ok(response.listen_key)
    }

    /// Keep alive the current listen key
    pub async fn keepalive_listen_key(&mut self) -> Result<()> {
        if let Some(ref key) = self.listen_key {
            let mut params = std::collections::HashMap::new();
            params.insert("listenKey".to_string(), key.clone());
            let _: serde_json::Value = self
                .http_client
                .put_signed("/fapi/v1/listenKey", Some(params))
                .await?;

            self.last_keepalive = Some(Instant::now());
            Ok(())
        } else {
            Err(BinanceError::Api {
                code: -1,
                msg: "No listen key available".to_string(),
            })
        }
    }

    /// Close the current listen key
    pub async fn close_listen_key(&mut self) -> Result<()> {
        if let Some(ref key) = self.listen_key {
            let mut params = std::collections::HashMap::new();
            params.insert("listenKey".to_string(), key.clone());
            let _: serde_json::Value = self
                .http_client
                .delete_signed("/fapi/v1/listenKey", Some(params))
                .await?;

            self.listen_key = None;
            self.last_keepalive = None;
            Ok(())
        } else {
            Err(BinanceError::Api {
                code: -1,
                msg: "No listen key to close".to_string(),
            })
        }
    }

    /// Get the current listen key, creating one if necessary
    pub async fn get_listen_key(&mut self) -> Result<String> {
        // Check if we need to send a keepalive first
        if let Some(last_keepalive) = self.last_keepalive {
            if last_keepalive.elapsed() >= self.keepalive_interval && self.listen_key.is_some() {
                self.keepalive_listen_key().await?;
            }
        }

        match &self.listen_key {
            Some(key) => Ok(key.clone()),
            None => self.create_listen_key().await,
        }
    }

    /// Check if keepalive is needed
    pub fn needs_keepalive(&self) -> bool {
        if let Some(last_keepalive) = self.last_keepalive {
            last_keepalive.elapsed() >= self.keepalive_interval
        } else {
            false
        }
    }

    /// Start automatic keepalive task
    pub async fn start_keepalive_task(&mut self) -> Result<()> {
        loop {
            sleep(Duration::from_secs(30 * 60)).await; // Sleep for 30 minutes
            
            if self.listen_key.is_some() {
                if let Err(e) = self.keepalive_listen_key().await {
                    eprintln!("Failed to keepalive listen key: {}", e);
                    // Try to create a new listen key
                    if let Err(e) = self.create_listen_key().await {
                        eprintln!("Failed to create new listen key: {}", e);
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    /// Set custom keepalive interval
    pub fn set_keepalive_interval(&mut self, interval: Duration) {
        self.keepalive_interval = interval;
    }

    /// Get current listen key without creating a new one
    pub fn current_listen_key(&self) -> Option<&String> {
        self.listen_key.as_ref()
    }

    /// Check if listen key is expired (based on last keepalive)
    pub fn is_expired(&self) -> bool {
        if let Some(last_keepalive) = self.last_keepalive {
            // Binance listen keys expire after 60 minutes without keepalive
            last_keepalive.elapsed() >= Duration::from_secs(60 * 60)
        } else {
            true
        }
    }
}

/// User data stream configuration
#[derive(Debug, Clone)]
pub struct UserDataStreamConfig {
    pub auto_keepalive: bool,
    pub keepalive_interval: Duration,
    pub reconnect_on_failure: bool,
    pub max_reconnect_attempts: u32,
}

impl Default for UserDataStreamConfig {
    fn default() -> Self {
        Self {
            auto_keepalive: true,
            keepalive_interval: Duration::from_secs(30 * 60), // 30 minutes
            reconnect_on_failure: true,
            max_reconnect_attempts: 5,
        }
    }
}

/// User data stream handler with automatic management
pub struct UserDataStream {
    manager: UserDataStreamManager,
    #[allow(dead_code)]
    config: UserDataStreamConfig,
}

impl UserDataStream {
    /// Create a new user data stream
    pub fn new(http_client: HttpClient, config: UserDataStreamConfig) -> Self {
        let mut manager = UserDataStreamManager::new(http_client);
        manager.set_keepalive_interval(config.keepalive_interval);

        Self { manager, config }
    }

    /// Start the user data stream and return the listen key
    pub async fn start(&mut self) -> Result<String> {
        self.manager.get_listen_key().await
    }

    /// Stop the user data stream
    pub async fn stop(&mut self) -> Result<()> {
        self.manager.close_listen_key().await
    }

    /// Get the current listen key
    pub async fn listen_key(&mut self) -> Result<String> {
        self.manager.get_listen_key().await
    }

    /// Manual keepalive
    pub async fn keepalive(&mut self) -> Result<()> {
        self.manager.keepalive_listen_key().await
    }

    /// Check if the stream needs maintenance
    pub fn needs_maintenance(&self) -> bool {
        self.manager.needs_keepalive() || self.manager.is_expired()
    }

    /// Perform maintenance (keepalive or recreate)
    pub async fn maintain(&mut self) -> Result<()> {
        if self.manager.is_expired() {
            // Recreate listen key if expired
            self.manager.create_listen_key().await?;
        } else if self.manager.needs_keepalive() {
            // Send keepalive if needed
            self.manager.keepalive_listen_key().await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::auth::Credentials;

    #[tokio::test]
    async fn test_user_data_stream_manager() {
        // This test requires valid API credentials
        // Skip if not available
        if std::env::var("BINANCE_API_KEY").is_err() || std::env::var("BINANCE_SECRET_KEY").is_err() {
            return;
        }

        let credentials = Credentials::new(
            std::env::var("BINANCE_API_KEY").unwrap(),
            std::env::var("BINANCE_SECRET_KEY").unwrap(),
        );

        let http_client = HttpClient::testnet_with_credentials(credentials);
        let mut manager = UserDataStreamManager::new(http_client);

        // Test creating listen key
        let listen_key = manager.create_listen_key().await;
        assert!(listen_key.is_ok());

        // Test keepalive
        let keepalive_result = manager.keepalive_listen_key().await;
        assert!(keepalive_result.is_ok());

        // Test closing listen key
        let close_result = manager.close_listen_key().await;
        assert!(close_result.is_ok());
    }

    #[test]
    fn test_user_data_stream_config() {
        let config = UserDataStreamConfig::default();
        assert!(config.auto_keepalive);
        assert_eq!(config.keepalive_interval, Duration::from_secs(30 * 60));
        assert!(config.reconnect_on_failure);
        assert_eq!(config.max_reconnect_attempts, 5);
    }

    #[test]
    fn test_keepalive_timing() {
        let http_client = HttpClient::new();
        let mut manager = UserDataStreamManager::new(http_client);
        
        // Should not need keepalive initially
        assert!(!manager.needs_keepalive());
        
        // Should be expired initially
        assert!(manager.is_expired());
        
        // Set a short keepalive interval for testing
        manager.set_keepalive_interval(Duration::from_millis(1));
        
        // Simulate having a listen key
        manager.listen_key = Some("test_key".to_string());
        manager.last_keepalive = Some(Instant::now());
        
        // Should not be expired now
        assert!(!manager.is_expired());
    }
}
