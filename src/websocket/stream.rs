use crate::error::{BinanceError, Result};
use crate::websocket::types::*;
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

const WS_BASE_URL: &str = "wss://fstream.binance.com/ws/";
const WS_TESTNET_URL: &str = "wss://stream.binancefuture.com/ws/";

pub type WebSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// WebSocket client for Binance Futures streams
pub struct WebSocketClient {
    base_url: String,
}

impl WebSocketClient {
    /// Create a new WebSocket client
    pub fn new() -> Self {
        Self {
            base_url: WS_BASE_URL.to_string(),
        }
    }

    /// Create a new testnet WebSocket client
    pub fn testnet() -> Self {
        Self {
            base_url: WS_TESTNET_URL.to_string(),
        }
    }

    /// Connect to a single stream
    pub async fn connect_stream(&self, stream: &str) -> Result<WebSocket> {
        let url = format!("{}{}", self.base_url, stream);
        let (ws_stream, _) = connect_async(&url).await
            .map_err(|e| BinanceError::WebSocket(format!("Failed to connect: {}", e)))?;
        Ok(ws_stream)
    }

    /// Connect to multiple streams
    pub async fn connect_combined_stream(&self, streams: &[String]) -> Result<WebSocket> {
        let combined_streams = streams.join("/");
        let url = format!("{}stream?streams={}", self.base_url, combined_streams);
        let (ws_stream, _) = connect_async(&url).await
            .map_err(|e| BinanceError::WebSocket(format!("Failed to connect: {}", e)))?;
        Ok(ws_stream)
    }

    /// Create depth stream name
    pub fn depth_stream(symbol: &str, levels: Option<u32>) -> String {
        match levels {
            Some(levels) => format!("{}@depth{}@100ms", symbol.to_lowercase(), levels),
            None => format!("{}@depth@100ms", symbol.to_lowercase()),
        }
    }

    /// Create trade stream name
    pub fn trade_stream(symbol: &str) -> String {
        format!("{}@trade", symbol.to_lowercase())
    }

    /// Create kline stream name
    pub fn kline_stream(symbol: &str, interval: &str) -> String {
        format!("{}@kline_{}", symbol.to_lowercase(), interval)
    }

    /// Create 24hr ticker stream name
    pub fn ticker_stream(symbol: &str) -> String {
        format!("{}@ticker", symbol.to_lowercase())
    }

    /// Create all market tickers stream name
    pub fn all_tickers_stream() -> String {
        "!ticker@arr".to_string()
    }

    /// Parse WebSocket message
    pub fn parse_message(msg: &str) -> Result<WebSocketMessage> {
        let value: Value = serde_json::from_str(msg)
            .map_err(|e| BinanceError::Json(e))?;

        // Handle combined stream format
        if let Some(stream_data) = value.get("stream") {
            let stream_name = stream_data.as_str().unwrap_or("");
            let data = value.get("data").unwrap_or(&value);
            return Self::parse_stream_data(stream_name, data);
        }

        // Handle single stream format
        if let Some(event_type) = value.get("e").and_then(|e| e.as_str()) {
            return Self::parse_event_data(event_type, &value);
        }

        // Handle ping/pong
        if value.get("ping").is_some() {
            return Ok(WebSocketMessage::Ping);
        }
        if value.get("pong").is_some() {
            return Ok(WebSocketMessage::Pong);
        }

        Err(BinanceError::WebSocket("Unknown message format".to_string()))
    }

    fn parse_stream_data(stream_name: &str, data: &Value) -> Result<WebSocketMessage> {
        if stream_name.contains("@depth") {
            let depth_update: DepthUpdate = serde_json::from_value(data.clone())?;
            Ok(WebSocketMessage::DepthUpdate(depth_update))
        } else if stream_name.contains("@trade") {
            let trade: TradeStream = serde_json::from_value(data.clone())?;
            Ok(WebSocketMessage::Trade(trade))
        } else if stream_name.contains("@kline") {
            let kline: KlineStream = serde_json::from_value(data.clone())?;
            Ok(WebSocketMessage::Kline(kline))
        } else if stream_name.contains("@ticker") {
            let ticker: TickerStream = serde_json::from_value(data.clone())?;
            Ok(WebSocketMessage::Ticker(ticker))
        } else {
            Err(BinanceError::WebSocket(format!("Unknown stream: {}", stream_name)))
        }
    }

    fn parse_event_data(event_type: &str, data: &Value) -> Result<WebSocketMessage> {
        match event_type {
            "depthUpdate" => {
                let depth_update: DepthUpdate = serde_json::from_value(data.clone())?;
                Ok(WebSocketMessage::DepthUpdate(depth_update))
            }
            "trade" => {
                let trade: TradeStream = serde_json::from_value(data.clone())?;
                Ok(WebSocketMessage::Trade(trade))
            }
            "kline" => {
                let kline: KlineStream = serde_json::from_value(data.clone())?;
                Ok(WebSocketMessage::Kline(kline))
            }
            "24hrTicker" => {
                let ticker: TickerStream = serde_json::from_value(data.clone())?;
                Ok(WebSocketMessage::Ticker(ticker))
            }
            "ACCOUNT_UPDATE" => {
                let account_update: AccountUpdate = serde_json::from_value(data.clone())?;
                Ok(WebSocketMessage::AccountUpdate(account_update))
            }
            "ORDER_TRADE_UPDATE" => {
                let order_update: OrderUpdate = serde_json::from_value(data.clone())?;
                Ok(WebSocketMessage::OrderUpdate(order_update))
            }
            _ => Err(BinanceError::WebSocket(format!("Unknown event type: {}", event_type))),
        }
    }

    /// Handle incoming WebSocket messages
    pub async fn handle_message(
        ws: &mut WebSocket,
        message_handler: impl Fn(WebSocketMessage) -> Result<()>,
    ) -> Result<()> {
        while let Some(msg) = ws.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    match Self::parse_message(&text) {
                        Ok(ws_msg) => {
                            if let Err(e) = message_handler(ws_msg) {
                                eprintln!("Error handling message: {}", e);
                            }
                        }
                        Err(e) => {
                            eprintln!("Error parsing message: {}", e);
                        }
                    }
                }
                Ok(Message::Ping(data)) => {
                    ws.send(Message::Pong(data)).await
                        .map_err(|e| BinanceError::WebSocket(format!("Failed to send pong: {}", e)))?;
                }
                Ok(Message::Close(_)) => {
                    break;
                }
                Err(e) => {
                    return Err(BinanceError::WebSocket(format!("WebSocket error: {}", e)));
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Subscribe to user data stream (requires listen key)
    pub async fn user_data_stream(&self, listen_key: &str) -> Result<WebSocket> {
        let url = format!("{}ws/{}", self.base_url, listen_key);
        let (ws_stream, _) = connect_async(&url).await
            .map_err(|e| BinanceError::WebSocket(format!("Failed to connect to user data stream: {}", e)))?;
        Ok(ws_stream)
    }
}

impl Default for WebSocketClient {
    fn default() -> Self {
        Self::new()
    }
}

/// WebSocket stream builder for easy configuration
pub struct StreamBuilder {
    client: WebSocketClient,
    streams: Vec<String>,
}

impl StreamBuilder {
    pub fn new() -> Self {
        Self {
            client: WebSocketClient::new(),
            streams: Vec::new(),
        }
    }

    pub fn testnet() -> Self {
        Self {
            client: WebSocketClient::testnet(),
            streams: Vec::new(),
        }
    }

    /// Add depth stream
    pub fn depth(mut self, symbol: &str, levels: Option<u32>) -> Self {
        self.streams.push(WebSocketClient::depth_stream(symbol, levels));
        self
    }

    /// Add trade stream
    pub fn trade(mut self, symbol: &str) -> Self {
        self.streams.push(WebSocketClient::trade_stream(symbol));
        self
    }

    /// Add kline stream
    pub fn kline(mut self, symbol: &str, interval: &str) -> Self {
        self.streams.push(WebSocketClient::kline_stream(symbol, interval));
        self
    }

    /// Add ticker stream
    pub fn ticker(mut self, symbol: &str) -> Self {
        self.streams.push(WebSocketClient::ticker_stream(symbol));
        self
    }

    /// Add all tickers stream
    pub fn all_tickers(mut self) -> Self {
        self.streams.push(WebSocketClient::all_tickers_stream());
        self
    }

    /// Connect to the configured streams
    pub async fn connect(self) -> Result<WebSocket> {
        if self.streams.is_empty() {
            return Err(BinanceError::WebSocket("No streams configured".to_string()));
        }

        if self.streams.len() == 1 {
            self.client.connect_stream(&self.streams[0]).await
        } else {
            self.client.connect_combined_stream(&self.streams).await
        }
    }
}

impl Default for StreamBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_names() {
        assert_eq!(WebSocketClient::depth_stream("BTCUSDT", Some(5)), "btcusdt@depth5@100ms");
        assert_eq!(WebSocketClient::trade_stream("BTCUSDT"), "btcusdt@trade");
        assert_eq!(WebSocketClient::kline_stream("BTCUSDT", "1m"), "btcusdt@kline_1m");
        assert_eq!(WebSocketClient::ticker_stream("BTCUSDT"), "btcusdt@ticker");
    }

    #[test]
    fn test_parse_depth_message() {
        let msg = r#"
        {
            "e": "depthUpdate",
            "E": 1640995200000,
            "T": 1640995200000,
            "s": "BTCUSDT",
            "U": 157,
            "u": 160,
            "pu": 156,
            "b": [["50000.0", "1.0"]],
            "a": [["50100.0", "2.0"]]
        }
        "#;

        let result = WebSocketClient::parse_message(msg).unwrap();
        match result {
            WebSocketMessage::DepthUpdate(depth) => {
                assert_eq!(depth.symbol, "BTCUSDT");
            }
            _ => panic!("Expected DepthUpdate"),
        }
    }

    #[test]
    fn test_stream_builder() {
        let builder = StreamBuilder::new()
            .depth("BTCUSDT", Some(5))
            .trade("ETHUSDT")
            .kline("ADAUSDT", "1h");

        assert_eq!(builder.streams.len(), 3);
        assert!(builder.streams.contains(&"btcusdt@depth5@100ms".to_string()));
        assert!(builder.streams.contains(&"ethusdt@trade".to_string()));
        assert!(builder.streams.contains(&"adausdt@kline_1h".to_string()));
    }
}
