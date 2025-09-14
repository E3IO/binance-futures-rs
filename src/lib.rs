//! # Binance Futures API Rust Client
//! 
//! A comprehensive Rust client library for the Binance Futures API.
//! This library provides async/await support for all Binance Futures endpoints
//! including market data, trading, account management, and WebSocket streams.
//!
//! ## Features
//!
//! - **Async/Await Support**: Built on tokio for high-performance async operations
//! - **Type Safety**: Strong typing with serde serialization/deserialization
//! - **Market Data**: Access to real-time market data, depth, trades, and klines
//! - **Trading**: Full trading functionality including orders, positions, and account management
//! - **WebSocket Streams**: Real-time data streams for market data and user events
//! - **Error Handling**: Comprehensive error types with detailed error information
//! - **Testnet Support**: Built-in support for Binance testnet environment
//!
//! ## Quick Start
//!
//! ### Public Market Data
//!
//! ```rust,no_run
//! use binance_futures_rs::BinanceClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = BinanceClient::new();
//!     
//!     // Get server time
//!     let time = client.market().server_time().await?;
//!     println!("Server time: {}", time.server_time);
//!     
//!     // Get 24hr ticker
//!     let ticker = client.market().ticker_24hr("BTCUSDT").await?;
//!     println!("BTC price: {}", ticker.last_price);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Authenticated Trading
//!
//! ```rust,no_run
//! use binance_futures_rs::{BinanceClient, Credentials, OrderSide, OrderType, TimeInForce};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let credentials = Credentials::new(
//!         "your_api_key".to_string(),
//!         "your_secret_key".to_string(),
//!     );
//!     let client = BinanceClient::new_with_credentials(credentials);
//!     
//!     // Place a limit order
//!     let order = client.trading().new_order(
//!         "BTCUSDT",
//!         OrderSide::Buy,
//!         OrderType::Limit,
//!         Some(TimeInForce::Gtc),
//!         Some("0.001".to_string()),
//!         Some("50000.0".to_string()),
//!         None, None, None, None,
//!     ).await?;
//!     
//!     println!("Order placed: {}", order.order_id);
//!     Ok(())
//! }
//! ```
//!
//! ### WebSocket Streams
//!
//! ```rust,no_run
//! use binance_futures_rs::websocket::{StreamBuilder, WebSocketClient, WebSocketMessage};
//! use futures_util::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut ws = StreamBuilder::new()
//!         .depth("BTCUSDT", Some(5))
//!         .trade("ETHUSDT")
//!         .connect()
//!         .await?;
//!     
//!     while let Some(msg) = ws.next().await {
//!         // Handle WebSocket messages
//!         if let Ok(text) = msg {
//!             match WebSocketClient::parse_message(&text.to_string()) {
//!                 Ok(WebSocketMessage::DepthUpdate(depth)) => {
//!                     println!("Depth update for {}", depth.symbol);
//!                 }
//!                 Ok(WebSocketMessage::Trade(trade)) => {
//!                     println!("Trade: {} @ {}", trade.quantity, trade.price);
//!                 }
//!                 _ => {}
//!             }
//!         }
//!     }
//!     Ok(())
//! }
//! ```
//! ## Quick Start
//!
//! ```rust,no_run
//! use binance_futures_rs::{BinanceClient, KlineInterval};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create client for public endpoints (no authentication required)
//!     let client = BinanceClient::new();
//!     
//!     // Get current Bitcoin price
//!     let price = client.market().price_ticker(Some("BTCUSDT")).await?;
//!     println!("BTC Price: {}", price[0].price);
//!     
//!     // For authenticated endpoints, provide credentials
//!     let credentials = Credentials::new(
//!         "your_api_key".to_string(),
//!         "your_secret_key".to_string(),
//!     );
//!     let auth_client = BinanceClient::new_with_credentials(credentials);
//!     
//!     // Get account information
//!     let account = auth_client.account().account_info().await?;
//!     println!("Account balance: {}", account.total_wallet_balance);
//!     
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod client;
pub mod error;
pub mod types;
pub mod utils;
pub mod websocket;

/// Main client for interacting with Binance Futures API
pub struct BinanceClient {
    http_client: HttpClient,
}

pub use api::{AccountApi, MarketApi, TradingApi};
pub use client::{Credentials, HttpClient};
pub use error::{BinanceError, Result};
pub use types::*;
pub use websocket::{StreamBuilder, WebSocketClient, WebSocketMessage, UserDataStream, UserDataStreamConfig};


impl BinanceClient {
    /// Create a new client for public endpoints (no authentication)
    pub fn new() -> Self {
        Self {
            http_client: HttpClient::new(),
        }
    }

    /// Create a new client with credentials for authenticated endpoints
    pub fn new_with_credentials(credentials: Credentials) -> Self {
        Self {
            http_client: HttpClient::new_with_credentials(credentials),
        }
    }

    /// Create a new testnet client
    pub fn testnet() -> Self {
        Self {
            http_client: HttpClient::testnet(),
        }
    }

    /// Create a new testnet client with credentials
    pub fn testnet_with_credentials(credentials: Credentials) -> Self {
        Self {
            http_client: HttpClient::testnet_with_credentials(credentials),
        }
    }

    /// Get market data API
    pub fn market(&self) -> MarketApi {
        MarketApi::new(self.http_client.clone())
    }

    /// Get trading API client
    pub fn trading(&self) -> TradingApi {
        TradingApi::new(self.http_client.clone())
    }

    /// Get account API client
    pub fn account(&self) -> AccountApi {
        AccountApi::new(self.http_client.clone())
    }


    /// Get HTTP client (for advanced usage)
    pub fn http_client(&self) -> &HttpClient {
        &self.http_client
    }
}

impl Default for BinanceClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = BinanceClient::new();
        let _market_api = client.market();
        assert!(true);
    }

    #[test]
    fn test_client_with_credentials() {
        let credentials = Credentials::new("test_key".to_string(), "test_secret".to_string());
        let client = BinanceClient::new_with_credentials(credentials);
        let _trading_api = client.trading();
        let _account_api = client.account();
        assert!(true);
    }

    #[test]
    fn test_testnet_client() {
        let client = BinanceClient::testnet();
        let _market_api = client.market();
        assert!(true);
    }

}
