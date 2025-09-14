use crate::client::HttpClient;
use crate::error::Result;
use crate::types::account::*;
use std::collections::HashMap;

pub struct AccountApi {
    client: HttpClient,
}

impl AccountApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// Get current account information
    pub async fn account_info(&self) -> Result<AccountInfo> {
        self.client.get_signed("/fapi/v2/account", None).await
    }

    /// Get futures account balance
    pub async fn balance(&self) -> Result<Vec<Balance>> {
        self.client.get_signed("/fapi/v2/balance", None).await
    }

    /// Get position risk
    pub async fn position_risk(&self, symbol: Option<&str>) -> Result<Vec<PositionRisk>> {
        let params = if let Some(symbol) = symbol {
            let mut params = HashMap::new();
            params.insert("symbol".to_string(), symbol.to_string());
            Some(params)
        } else {
            None
        };

        self.client.get_signed("/fapi/v2/positionRisk", params).await
    }

    /// Get income history
    pub async fn income_history(
        &self,
        symbol: Option<&str>,
        income_type: Option<&str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u32>,
    ) -> Result<Vec<Income>> {
        let mut params = HashMap::new();
        
        if let Some(symbol) = symbol {
            params.insert("symbol".to_string(), symbol.to_string());
        }
        
        if let Some(income_type) = income_type {
            params.insert("incomeType".to_string(), income_type.to_string());
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

        let params = if params.is_empty() { None } else { Some(params) };
        self.client.get_signed("/fapi/v1/income", params).await
    }

    /// Get notional and leverage brackets
    pub async fn leverage_bracket(&self, symbol: Option<&str>) -> Result<Vec<LeverageBracket>> {
        let params = if let Some(symbol) = symbol {
            let mut params = HashMap::new();
            params.insert("symbol".to_string(), symbol.to_string());
            Some(params)
        } else {
            None
        };

        self.client.get_signed("/fapi/v1/leverageBracket", params).await
    }

    /// Get position ADL quantile estimation
    pub async fn adl_quantile(&self, symbol: Option<&str>) -> Result<Vec<AdlQuantile>> {
        let params = if let Some(symbol) = symbol {
            let mut params = HashMap::new();
            params.insert("symbol".to_string(), symbol.to_string());
            Some(params)
        } else {
            None
        };

        self.client.get_signed("/fapi/v1/adlQuantile", params).await
    }

    /// Get user's force orders
    pub async fn force_orders(
        &self,
        symbol: Option<&str>,
        auto_close_type: Option<&str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u32>,
    ) -> Result<Vec<ForceOrder>> {
        let mut params = HashMap::new();
        
        if let Some(symbol) = symbol {
            params.insert("symbol".to_string(), symbol.to_string());
        }
        
        if let Some(auto_close_type) = auto_close_type {
            params.insert("autoCloseType".to_string(), auto_close_type.to_string());
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

        let params = if params.is_empty() { None } else { Some(params) };
        self.client.get_signed("/fapi/v1/forceOrders", params).await
    }

    /// Get trading status
    pub async fn api_trading_status(&self) -> Result<serde_json::Value> {
        self.client.get_signed("/fapi/v1/apiTradingStatus", None).await
    }

    /// Get user commission rate
    pub async fn commission_rate(&self, symbol: &str) -> Result<CommissionRate> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), symbol.to_string());

        self.client.get_signed("/fapi/v1/commissionRate", Some(params)).await
    }

    /// Change initial leverage
    pub async fn change_leverage(&self, symbol: &str, leverage: i32) -> Result<serde_json::Value> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("leverage".to_string(), leverage.to_string());

        self.client.post_signed("/fapi/v1/leverage", Some(params)).await
    }

    /// Change margin type
    pub async fn change_margin_type(&self, symbol: &str, margin_type: &str) -> Result<serde_json::Value> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("marginType".to_string(), margin_type.to_string());

        self.client.post_signed("/fapi/v1/marginType", Some(params)).await
    }

    /// Modify isolated position margin
    pub async fn position_margin(
        &self,
        symbol: &str,
        position_side: Option<&str>,
        amount: &str,
        margin_type: i32, // 1: Add position margin, 2: Reduce position margin
    ) -> Result<serde_json::Value> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("amount".to_string(), amount.to_string());
        params.insert("type".to_string(), margin_type.to_string());
        
        if let Some(position_side) = position_side {
            params.insert("positionSide".to_string(), position_side.to_string());
        }

        self.client.post_signed("/fapi/v1/positionMargin", Some(params)).await
    }

    /// Get position margin change history
    pub async fn position_margin_history(
        &self,
        symbol: &str,
        margin_type: Option<i32>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u32>,
    ) -> Result<Vec<serde_json::Value>> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        
        if let Some(margin_type) = margin_type {
            params.insert("type".to_string(), margin_type.to_string());
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

        self.client.get_signed("/fapi/v1/positionMargin/history", Some(params)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::{HttpClient, Credentials};

    #[test]
    fn test_account_api_creation() {
        let credentials = Credentials::new("test_key".to_string(), "test_secret".to_string());
        let client = HttpClient::new_with_credentials(credentials);
        let account_api = AccountApi::new(client);
        // Just test that we can create the API instance
        assert!(true);
    }
}
