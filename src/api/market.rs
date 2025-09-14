use crate::client::HttpClient;
use crate::error::Result;
use crate::types::common::KlineInterval;
use crate::types::market::*;
use std::collections::HashMap;

pub struct MarketApi {
    client: HttpClient,
}

impl MarketApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// Get order book depth
    pub async fn depth(&self, symbol: &str, limit: Option<u32>) -> Result<OrderBook> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        
        if let Some(limit) = limit {
            params.insert("limit".to_string(), limit.to_string());
        }

        self.client.get_public("/fapi/v1/depth", Some(params)).await
    }

    /// Get recent trades list
    pub async fn trades(&self, symbol: &str, limit: Option<u32>) -> Result<Vec<Trade>> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        
        if let Some(limit) = limit {
            params.insert("limit".to_string(), limit.to_string());
        }

        self.client.get_public("/fapi/v1/trades", Some(params)).await
    }

    /// Get older market trades
    pub async fn historical_trades(&self, symbol: &str, limit: Option<u32>, from_id: Option<u64>) -> Result<Vec<Trade>> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        
        if let Some(limit) = limit {
            params.insert("limit".to_string(), limit.to_string());
        }
        
        if let Some(from_id) = from_id {
            params.insert("fromId".to_string(), from_id.to_string());
        }

        self.client.get_public("/fapi/v1/historicalTrades", Some(params)).await
    }

    /// Get compressed, aggregate trades
    pub async fn agg_trades(
        &self,
        symbol: &str,
        from_id: Option<u64>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u32>,
    ) -> Result<Vec<AggTrade>> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        
        if let Some(from_id) = from_id {
            params.insert("fromId".to_string(), from_id.to_string());
        }
        
        if let Some(start_time) = start_time {
            params.insert("startTime".to_string(), start_time.to_string());
        }
        
        if let Some(end_time) = end_time {
            params.insert("endTime".to_string(), end_time.to_string());
        }
        
        if let Some(limit) = limit {
            params.insert("limit".to_string(), limit.to_string());
        }

        self.client.get_public("/fapi/v1/aggTrades", Some(params)).await
    }

    /// Get kline/candlestick data
    pub async fn klines(
        &self,
        symbol: &str,
        interval: KlineInterval,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u32>,
    ) -> Result<Vec<Kline>> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("interval".to_string(), interval.to_string());
        
        if let Some(start_time) = start_time {
            params.insert("startTime".to_string(), start_time.to_string());
        }
        
        if let Some(end_time) = end_time {
            params.insert("endTime".to_string(), end_time.to_string());
        }
        
        if let Some(limit) = limit {
            params.insert("limit".to_string(), limit.to_string());
        }

        let response: Vec<Vec<serde_json::Value>> = self.client
            .get_public("/fapi/v1/klines", Some(params))
            .await?;

        Ok(response.into_iter().map(Kline::from).collect())
    }

    /// Get mark price and funding rate
    pub async fn mark_price(&self, symbol: Option<&str>) -> Result<Vec<MarkPrice>> {
        let params = if let Some(symbol) = symbol {
            let mut params = HashMap::new();
            params.insert("symbol".to_string(), symbol.to_string());
            Some(params)
        } else {
            None
        };

        let response = self.client.get_public("/fapi/v1/premiumIndex", params).await?;
        
        // Handle both single object and array responses
        match response {
            serde_json::Value::Array(arr) => {
                Ok(serde_json::from_value(serde_json::Value::Array(arr))?)
            }
            single => {
                let mark_price: MarkPrice = serde_json::from_value(single)?;
                Ok(vec![mark_price])
            }
        }
    }

    /// Get 24hr ticker price change statistics
    pub async fn ticker_24hr(&self, symbol: Option<&str>) -> Result<Vec<Ticker24hr>> {
        let params = if let Some(symbol) = symbol {
            let mut params = HashMap::new();
            params.insert("symbol".to_string(), symbol.to_string());
            Some(params)
        } else {
            None
        };

        let response = self.client.get_public("/fapi/v1/ticker/24hr", params).await?;
        
        // Handle both single object and array responses
        match response {
            serde_json::Value::Array(arr) => {
                Ok(serde_json::from_value(serde_json::Value::Array(arr))?)
            }
            single => {
                let ticker: Ticker24hr = serde_json::from_value(single)?;
                Ok(vec![ticker])
            }
        }
    }

    /// Get symbol price ticker
    pub async fn price_ticker(&self, symbol: Option<&str>) -> Result<Vec<PriceTicker>> {
        let params = if let Some(symbol) = symbol {
            let mut params = HashMap::new();
            params.insert("symbol".to_string(), symbol.to_string());
            Some(params)
        } else {
            None
        };

        let response = self.client.get_public("/fapi/v1/ticker/price", params).await?;
        
        // Handle both single object and array responses
        match response {
            serde_json::Value::Array(arr) => {
                Ok(serde_json::from_value(serde_json::Value::Array(arr))?)
            }
            single => {
                let ticker: PriceTicker = serde_json::from_value(single)?;
                Ok(vec![ticker])
            }
        }
    }

    /// Get exchange information
    pub async fn exchange_info(&self) -> Result<ExchangeInfo> {
        self.client.get_public("/fapi/v1/exchangeInfo", None).await
    }

    /// Test connectivity to the Rest API
    pub async fn ping(&self) -> Result<serde_json::Value> {
        self.client.get_public("/fapi/v1/ping", None).await
    }

    /// Check server time
    pub async fn time(&self) -> Result<serde_json::Value> {
        self.client.get_public("/fapi/v1/time", None).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::HttpClient;

    #[test]
    fn test_market_api_creation() {
        let client = HttpClient::new();
        let market_api = MarketApi::new(client);
        // Just test that we can create the API instance
        assert!(true);
    }

    #[tokio::test]
    async fn test_ping() {
        let client = HttpClient::testnet();
        let market_api = MarketApi::new(client);
        
        // This test requires network access and may fail in CI
        // Uncomment to test manually
        // let result = market_api.ping().await;
        // assert!(result.is_ok());
    }
}
