//! WebSocket module for real-time data streams
//! 
//! This module provides WebSocket connectivity for Binance Futures API,
//! supporting both market data streams and user data streams.

pub mod stream;
pub mod types;
pub mod user_data;

pub use stream::{StreamBuilder, WebSocket, WebSocketClient};
pub use types::*;
pub use user_data::{UserDataStream, UserDataStreamConfig, UserDataStreamManager};
