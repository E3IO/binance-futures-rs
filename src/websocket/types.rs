use serde::Deserialize;
use crate::types::common::{OrderSide, OrderStatus, OrderType, PositionSide, TimeInForce};

/// WebSocket stream message wrapper
#[derive(Debug, Clone, Deserialize)]
pub struct StreamMessage<T> {
    pub stream: String,
    pub data: T,
}

/// Depth update stream
#[derive(Debug, Clone, Deserialize)]
pub struct DepthUpdate {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "U")]
    pub first_update_id: u64,
    #[serde(rename = "u")]
    pub final_update_id: u64,
    #[serde(rename = "pu")]
    pub previous_final_update_id: u64,
    #[serde(rename = "b")]
    pub bids: Vec<[String; 2]>, // [price, quantity]
    #[serde(rename = "a")]
    pub asks: Vec<[String; 2]>, // [price, quantity]
}

/// Trade stream
#[derive(Debug, Clone, Deserialize)]
pub struct TradeStream {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "T")]
    pub trade_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "t")]
    pub trade_id: u64,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "q")]
    pub quantity: String,
    #[serde(rename = "X")]
    pub buyer_order_id: u64,
    #[serde(rename = "Y")]
    pub seller_order_id: u64,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
}

/// Kline stream
#[derive(Debug, Clone, Deserialize)]
pub struct KlineStream {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "k")]
    pub kline: KlineData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KlineData {
    #[serde(rename = "t")]
    pub start_time: u64,
    #[serde(rename = "T")]
    pub close_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub interval: String,
    #[serde(rename = "f")]
    pub first_trade_id: u64,
    #[serde(rename = "L")]
    pub last_trade_id: u64,
    #[serde(rename = "o")]
    pub open: String,
    #[serde(rename = "c")]
    pub close: String,
    #[serde(rename = "h")]
    pub high: String,
    #[serde(rename = "l")]
    pub low: String,
    #[serde(rename = "v")]
    pub volume: String,
    #[serde(rename = "n")]
    pub trade_count: u64,
    #[serde(rename = "x")]
    pub is_closed: bool,
    #[serde(rename = "q")]
    pub quote_volume: String,
    #[serde(rename = "V")]
    pub taker_buy_volume: String,
    #[serde(rename = "Q")]
    pub taker_buy_quote_volume: String,
}

/// 24hr ticker stream
#[derive(Debug, Clone, Deserialize)]
pub struct TickerStream {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p")]
    pub price_change: String,
    #[serde(rename = "P")]
    pub price_change_percent: String,
    #[serde(rename = "w")]
    pub weighted_avg_price: String,
    #[serde(rename = "c")]
    pub last_price: String,
    #[serde(rename = "Q")]
    pub last_quantity: String,
    #[serde(rename = "o")]
    pub open_price: String,
    #[serde(rename = "h")]
    pub high_price: String,
    #[serde(rename = "l")]
    pub low_price: String,
    #[serde(rename = "v")]
    pub volume: String,
    #[serde(rename = "q")]
    pub quote_volume: String,
    #[serde(rename = "O")]
    pub open_time: u64,
    #[serde(rename = "C")]
    pub close_time: u64,
    #[serde(rename = "F")]
    pub first_trade_id: u64,
    #[serde(rename = "L")]
    pub last_trade_id: u64,
    #[serde(rename = "n")]
    pub trade_count: u64,
}

/// User data stream - Account update
#[derive(Debug, Clone, Deserialize)]
pub struct AccountUpdate {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "a")]
    pub account_update: AccountUpdateData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccountUpdateData {
    #[serde(rename = "m")]
    pub event_reason: String,
    #[serde(rename = "B")]
    pub balances: Vec<BalanceUpdate>,
    #[serde(rename = "P")]
    pub positions: Vec<PositionUpdate>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BalanceUpdate {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "wb")]
    pub wallet_balance: String,
    #[serde(rename = "cw")]
    pub cross_wallet_balance: String,
    #[serde(rename = "bc")]
    pub balance_change: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PositionUpdate {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "pa")]
    pub position_amount: String,
    #[serde(rename = "ep")]
    pub entry_price: String,
    #[serde(rename = "cr")]
    pub accumulated_realized: String,
    #[serde(rename = "up")]
    pub unrealized_pnl: String,
    #[serde(rename = "mt")]
    pub margin_type: String,
    #[serde(rename = "iw")]
    pub isolated_wallet: String,
    #[serde(rename = "ps")]
    pub position_side: PositionSide,
}

/// User data stream - Order update
#[derive(Debug, Clone, Deserialize)]
pub struct OrderUpdate {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "o")]
    pub order: OrderUpdateData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrderUpdateData {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub client_order_id: String,
    #[serde(rename = "S")]
    pub side: OrderSide,
    #[serde(rename = "o")]
    pub order_type: OrderType,
    #[serde(rename = "f")]
    pub time_in_force: TimeInForce,
    #[serde(rename = "q")]
    pub original_quantity: String,
    #[serde(rename = "p")]
    pub original_price: String,
    #[serde(rename = "ap")]
    pub average_price: String,
    #[serde(rename = "sp")]
    pub stop_price: String,
    #[serde(rename = "x")]
    pub execution_type: String,
    #[serde(rename = "X")]
    pub order_status: OrderStatus,
    #[serde(rename = "i")]
    pub order_id: u64,
    #[serde(rename = "l")]
    pub last_filled_quantity: String,
    #[serde(rename = "z")]
    pub cumulative_filled_quantity: String,
    #[serde(rename = "L")]
    pub last_filled_price: String,
    #[serde(rename = "n")]
    pub commission_amount: String,
    #[serde(rename = "N")]
    pub commission_asset: Option<String>,
    #[serde(rename = "T")]
    pub order_trade_time: u64,
    #[serde(rename = "t")]
    pub trade_id: u64,
    #[serde(rename = "b")]
    pub bids_notional: String,
    #[serde(rename = "a")]
    pub ask_notional: String,
    #[serde(rename = "m")]
    pub is_maker: bool,
    #[serde(rename = "R")]
    pub reduce_only: bool,
    #[serde(rename = "wt")]
    pub working_type: String,
    #[serde(rename = "ot")]
    pub original_order_type: OrderType,
    #[serde(rename = "ps")]
    pub position_side: PositionSide,
    #[serde(rename = "cp")]
    pub close_position: bool,
    #[serde(rename = "AP")]
    pub activation_price: Option<String>,
    #[serde(rename = "cr")]
    pub callback_rate: Option<String>,
    #[serde(rename = "rp")]
    pub realized_profit: String,
}

/// WebSocket message types
#[derive(Debug, Clone)]
pub enum WebSocketMessage {
    DepthUpdate(DepthUpdate),
    Trade(TradeStream),
    Kline(KlineStream),
    Ticker(TickerStream),
    AccountUpdate(AccountUpdate),
    OrderUpdate(OrderUpdate),
    Ping,
    Pong,
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depth_update_deserialization() {
        let json = r#"
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

        let depth_update: DepthUpdate = serde_json::from_str(json).unwrap();
        assert_eq!(depth_update.symbol, "BTCUSDT");
        assert_eq!(depth_update.event_type, "depthUpdate");
        assert_eq!(depth_update.bids.len(), 1);
        assert_eq!(depth_update.asks.len(), 1);
    }
}
