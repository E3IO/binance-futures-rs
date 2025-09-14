use serde::{Deserialize, Serialize};

/// Order book depth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    pub bids: Vec<[String; 2]>, // [price, quantity]
    pub asks: Vec<[String; 2]>, // [price, quantity]
}

/// 24hr ticker statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker24hr {
    pub symbol: String,
    #[serde(rename = "priceChange")]
    pub price_change: String,
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: String,
    #[serde(rename = "weightedAvgPrice")]
    pub weighted_avg_price: String,
    #[serde(rename = "lastPrice")]
    pub last_price: String,
    #[serde(rename = "lastQty")]
    pub last_qty: String,
    #[serde(rename = "openPrice")]
    pub open_price: String,
    #[serde(rename = "highPrice")]
    pub high_price: String,
    #[serde(rename = "lowPrice")]
    pub low_price: String,
    pub volume: String,
    #[serde(rename = "quoteVolume")]
    pub quote_volume: String,
    #[serde(rename = "openTime")]
    pub open_time: u64,
    #[serde(rename = "closeTime")]
    pub close_time: u64,
    #[serde(rename = "firstId")]
    pub first_id: u64,
    #[serde(rename = "lastId")]
    pub last_id: u64,
    pub count: u64,
}

/// Symbol price ticker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceTicker {
    pub symbol: String,
    pub price: String,
    pub time: u64,
}

/// Kline/Candlestick data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kline {
    pub open_time: u64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
    pub close_time: u64,
    pub quote_asset_volume: String,
    pub number_of_trades: u64,
    pub taker_buy_base_asset_volume: String,
    pub taker_buy_quote_asset_volume: String,
    pub ignore: String,
}

impl From<Vec<serde_json::Value>> for Kline {
    fn from(values: Vec<serde_json::Value>) -> Self {
        Self {
            open_time: values[0].as_u64().unwrap_or(0),
            open: values[1].as_str().unwrap_or("0").to_string(),
            high: values[2].as_str().unwrap_or("0").to_string(),
            low: values[3].as_str().unwrap_or("0").to_string(),
            close: values[4].as_str().unwrap_or("0").to_string(),
            volume: values[5].as_str().unwrap_or("0").to_string(),
            close_time: values[6].as_u64().unwrap_or(0),
            quote_asset_volume: values[7].as_str().unwrap_or("0").to_string(),
            number_of_trades: values[8].as_u64().unwrap_or(0),
            taker_buy_base_asset_volume: values[9].as_str().unwrap_or("0").to_string(),
            taker_buy_quote_asset_volume: values[10].as_str().unwrap_or("0").to_string(),
            ignore: values.get(11).and_then(|v| v.as_str()).unwrap_or("0").to_string(),
        }
    }
}

/// Trade information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub id: u64,
    pub price: String,
    pub qty: String,
    #[serde(rename = "quoteQty")]
    pub quote_qty: String,
    pub time: u64,
    #[serde(rename = "isBuyerMaker")]
    pub is_buyer_maker: bool,
}

/// Aggregate trade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggTrade {
    #[serde(rename = "a")]
    pub agg_trade_id: u64,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "q")]
    pub quantity: String,
    #[serde(rename = "f")]
    pub first_trade_id: u64,
    #[serde(rename = "l")]
    pub last_trade_id: u64,
    #[serde(rename = "T")]
    pub timestamp: u64,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
}

/// Mark price and funding rate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkPrice {
    pub symbol: String,
    #[serde(rename = "markPrice")]
    pub mark_price: String,
    #[serde(rename = "indexPrice")]
    pub index_price: String,
    #[serde(rename = "estimatedSettlePrice")]
    pub estimated_settle_price: String,
    #[serde(rename = "lastFundingRate")]
    pub last_funding_rate: String,
    #[serde(rename = "nextFundingTime")]
    pub next_funding_time: u64,
    #[serde(rename = "interestRate")]
    pub interest_rate: String,
    pub time: u64,
}

/// Exchange information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeInfo {
    pub timezone: String,
    #[serde(rename = "serverTime")]
    pub server_time: u64,
    #[serde(rename = "futuresType")]
    pub futures_type: String,
    #[serde(rename = "rateLimits")]
    pub rate_limits: Vec<RateLimit>,
    pub symbols: Vec<SymbolInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    #[serde(rename = "rateLimitType")]
    pub rate_limit_type: String,
    pub interval: String,
    #[serde(rename = "intervalNum")]
    pub interval_num: u32,
    pub limit: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolInfo {
    pub symbol: String,
    pub status: String,
    #[serde(rename = "baseAsset")]
    pub base_asset: String,
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,
    #[serde(rename = "marginAsset")]
    pub margin_asset: String,
    #[serde(rename = "pricePrecision")]
    pub price_precision: i32,
    #[serde(rename = "quantityPrecision")]
    pub quantity_precision: i32,
    #[serde(rename = "baseAssetPrecision")]
    pub base_asset_precision: i32,
    #[serde(rename = "quotePrecision")]
    pub quote_precision: i32,
    pub filters: Vec<serde_json::Value>,
    #[serde(rename = "orderTypes")]
    pub order_types: Vec<String>,
    #[serde(rename = "timeInForce")]
    pub time_in_force: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kline_from_vec() {
        let values = vec![
            serde_json::Value::Number(1640995200000_u64.into()),
            serde_json::Value::String("50000.0".to_string()),
            serde_json::Value::String("51000.0".to_string()),
            serde_json::Value::String("49000.0".to_string()),
            serde_json::Value::String("50500.0".to_string()),
            serde_json::Value::String("100.0".to_string()),
            serde_json::Value::Number(1640995259999_u64.into()),
            serde_json::Value::String("5050000.0".to_string()),
            serde_json::Value::Number(1000_u64.into()),
            serde_json::Value::String("50.0".to_string()),
            serde_json::Value::String("2525000.0".to_string()),
            serde_json::Value::String("0".to_string()),
        ];

        let kline = Kline::from(values);
        assert_eq!(kline.open_time, 1640995200000);
        assert_eq!(kline.open, "50000.0");
        assert_eq!(kline.high, "51000.0");
    }
}
