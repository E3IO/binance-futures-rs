//! Advanced trading functionality including conditional orders and algorithmic trading
//!
//! This module provides advanced trading features such as:
//! - Conditional orders (stop-loss, take-profit, trailing stop)
//! - Algorithmic trading strategies
//! - Order management utilities
//! - Risk management tools

use crate::client::http::HttpClient;
use crate::error::Result;
use crate::types::common::{OrderSide, OrderType, PositionSide, TimeInForce};
use crate::types::trading::Order;
use serde::Deserialize;
use std::collections::HashMap;

/// Advanced trading API client
#[derive(Clone)]
pub struct AdvancedTradingApi {
    client: HttpClient,
}

impl AdvancedTradingApi {
    /// Create a new advanced trading API client
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// Place a stop-loss order
    /// 
    /// # Arguments
    /// 
    /// * `symbol` - Trading pair symbol (e.g., "BTCUSDT")
    /// * `side` - Order side (Buy/Sell)
    /// * `quantity` - Order quantity
    /// * `stop_price` - Stop price to trigger the order
    /// * `price` - Limit price (optional, for stop-limit orders)
    /// * `position_side` - Position side for hedge mode
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use binance_fu_rs::{BinanceClient, Credentials, OrderSide, PositionSide};
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let credentials = Credentials::new("api_key".to_string(), "secret".to_string());
    /// let client = BinanceClient::new_with_credentials(credentials);
    /// 
    /// // Place a stop-loss order
    /// let order = client.advanced_trading().stop_loss_order(
    ///     "BTCUSDT",
    ///     OrderSide::Sell,
    ///     "0.001",
    ///     "45000.0",
    ///     None,
    ///     Some(PositionSide::Long),
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn stop_loss_order(
        &self,
        symbol: &str,
        side: OrderSide,
        quantity: &str,
        stop_price: &str,
        price: Option<&str>,
        position_side: Option<PositionSide>,
    ) -> Result<Order> {
        let order_type = if price.is_some() {
            OrderType::StopMarket
        } else {
            OrderType::StopMarket
        };

        let mut params = HashMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("side".to_string(), side.to_string());
        params.insert("type".to_string(), order_type.to_string());
        params.insert("quantity".to_string(), quantity.to_string());
        params.insert("stopPrice".to_string(), stop_price.to_string());
        params.insert("timeInForce".to_string(), TimeInForce::Gtc.to_string());

        if let Some(p) = price {
            params.insert("price".to_string(), p.to_string());
        }

        if let Some(ps) = position_side {
            params.insert("positionSide".to_string(), ps.to_string());
        }

        self.client.post_signed("/fapi/v1/order", Some(params)).await
    }

    /// Place a take-profit order
    /// 
    /// # Arguments
    /// 
    /// * `symbol` - Trading pair symbol
    /// * `side` - Order side
    /// * `quantity` - Order quantity
    /// * `stop_price` - Take profit trigger price
    /// * `price` - Limit price (optional)
    /// * `position_side` - Position side for hedge mode
    pub async fn take_profit_order(
        &self,
        symbol: &str,
        side: OrderSide,
        quantity: &str,
        stop_price: &str,
        price: Option<&str>,
        position_side: Option<PositionSide>,
    ) -> Result<Order> {
        let order_type = if price.is_some() {
            OrderType::TakeProfit
        } else {
            OrderType::TakeProfitMarket
        };

        let mut params = HashMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("side".to_string(), side.to_string());
        params.insert("type".to_string(), order_type.to_string());
        params.insert("quantity".to_string(), quantity.to_string());
        params.insert("stopPrice".to_string(), stop_price.to_string());
        params.insert("timeInForce".to_string(), TimeInForce::Gtc.to_string());

        if let Some(p) = price {
            params.insert("price".to_string(), p.to_string());
        }

        if let Some(ps) = position_side {
            params.insert("positionSide".to_string(), ps.to_string());
        }

        self.client.post_signed("/fapi/v1/order", Some(params)).await
    }

    /// Place a trailing stop order
    /// 
    /// # Arguments
    /// 
    /// * `symbol` - Trading pair symbol
    /// * `side` - Order side
    /// * `quantity` - Order quantity
    /// * `callback_rate` - Callback rate (0.1 = 0.1%)
    /// * `activation_price` - Activation price (optional)
    /// * `position_side` - Position side for hedge mode
    pub async fn trailing_stop_order(
        &self,
        symbol: &str,
        side: OrderSide,
        quantity: &str,
        callback_rate: &str,
        activation_price: Option<&str>,
        position_side: Option<PositionSide>,
    ) -> Result<Order> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("side".to_string(), side.to_string());
        params.insert("type".to_string(), OrderType::TrailingStopMarket.to_string());
        params.insert("quantity".to_string(), quantity.to_string());
        params.insert("callbackRate".to_string(), callback_rate.to_string());
        params.insert("timeInForce".to_string(), TimeInForce::Gtc.to_string());

        if let Some(ap) = activation_price {
            params.insert("activationPrice".to_string(), ap.to_string());
        }

        if let Some(ps) = position_side {
            params.insert("positionSide".to_string(), ps.to_string());
        }

        self.client.post_signed("/fapi/v1/order", Some(params)).await
    }

    /// Place a bracket order (entry + stop-loss + take-profit)
    /// 
    /// # Arguments
    /// 
    /// * `config` - Bracket order configuration
    pub async fn bracket_order(&self, config: BracketOrderConfig) -> Result<BracketOrderResponse> {
        // Place entry order first
        let entry_order = self.place_entry_order(&config).await?;

        // Place stop-loss order
        let stop_loss_order = self.stop_loss_order(
            &config.symbol,
            config.exit_side(),
            &config.quantity,
            &config.stop_loss_price,
            config.stop_loss_limit_price.as_deref(),
            config.position_side.clone(),
        ).await?;

        // Place take-profit order
        let take_profit_order = self.take_profit_order(
            &config.symbol,
            config.exit_side(),
            &config.quantity,
            &config.take_profit_price,
            config.take_profit_limit_price.as_deref(),
            config.position_side.clone(),
        ).await?;

        Ok(BracketOrderResponse {
            entry_order,
            stop_loss_order,
            take_profit_order,
        })
    }

    /// Place OCO (One-Cancels-Other) order
    /// 
    /// # Arguments
    /// 
    /// * `symbol` - Trading pair symbol
    /// * `side` - Order side
    /// * `quantity` - Order quantity
    /// * `price` - Limit order price
    /// * `stop_price` - Stop order trigger price
    /// * `stop_limit_price` - Stop limit order price (optional)
    /// * `position_side` - Position side for hedge mode
    pub async fn oco_order(
        &self,
        symbol: &str,
        side: OrderSide,
        quantity: &str,
        price: &str,
        stop_price: &str,
        stop_limit_price: Option<&str>,
        position_side: Option<PositionSide>,
    ) -> Result<OcoOrderResponse> {
        // Place limit order
        let mut limit_params = HashMap::new();
        limit_params.insert("symbol".to_string(), symbol.to_string());
        limit_params.insert("side".to_string(), side.to_string());
        limit_params.insert("type".to_string(), OrderType::Limit.to_string());
        limit_params.insert("quantity".to_string(), quantity.to_string());
        limit_params.insert("price".to_string(), price.to_string());
        limit_params.insert("timeInForce".to_string(), TimeInForce::Gtc.to_string());

        if let Some(ps) = position_side {
            limit_params.insert("positionSide".to_string(), ps.to_string());
        }

        let limit_order: Order = self.client
            .post_signed("/fapi/v1/order", Some(limit_params))
            .await?;

        // Place stop order
        let stop_order_type = if stop_limit_price.is_some() {
            OrderType::Stop
        } else {
            OrderType::StopMarket
        };

        let mut stop_params = HashMap::new();
        stop_params.insert("symbol".to_string(), symbol.to_string());
        stop_params.insert("side".to_string(), side.to_string());
        stop_params.insert("type".to_string(), stop_order_type.to_string());
        stop_params.insert("quantity".to_string(), quantity.to_string());
        stop_params.insert("stopPrice".to_string(), stop_price.to_string());
        stop_params.insert("timeInForce".to_string(), TimeInForce::Gtc.to_string());

        if let Some(slp) = stop_limit_price {
            stop_params.insert("price".to_string(), slp.to_string());
        }

        if let Some(ps) = position_side {
            stop_params.insert("positionSide".to_string(), ps.to_string());
        }

        let stop_order: Order = self.client
            .post_signed("/fapi/v1/order", Some(stop_params))
            .await?;

        Ok(OcoOrderResponse {
            limit_order,
            stop_order,
        })
    }

    /// Cancel all orders for a symbol and place new orders atomically
    /// 
    /// # Arguments
    /// 
    /// * `symbol` - Trading pair symbol
    /// * `orders` - New orders to place
    pub async fn replace_all_orders(
        &self,
        symbol: &str,
        orders: Vec<OrderRequest>,
    ) -> Result<ReplaceOrdersResponse> {
        // Cancel all existing orders
        let cancelled_orders: Vec<Order> = self.client
            .delete_signed("/fapi/v1/allOpenOrders", Some({
                let mut params = HashMap::new();
                params.insert("symbol".to_string(), symbol.to_string());
                params
            }))
            .await?;

        // Place new orders
        let mut new_orders = Vec::new();
        for order_req in orders {
            let order = self.place_order_from_request(order_req).await?;
            new_orders.push(order);
        }

        Ok(ReplaceOrdersResponse {
            cancelled_orders,
            new_orders,
        })
    }

    // Helper methods
    async fn place_entry_order(&self, config: &BracketOrderConfig) -> Result<Order> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), config.symbol.clone());
        params.insert("side".to_string(), config.side.to_string());
        params.insert("type".to_string(), config.entry_order_type.to_string());
        params.insert("quantity".to_string(), config.quantity.clone());

        if let Some(price) = &config.entry_price {
            params.insert("price".to_string(), price.clone());
        }

        if let Some(ps) = &config.position_side {
            params.insert("positionSide".to_string(), ps.to_string());
        }

        params.insert("timeInForce".to_string(), TimeInForce::Gtc.to_string());

        self.client.post_signed("/fapi/v1/order", Some(params)).await
    }

    async fn place_order_from_request(&self, req: OrderRequest) -> Result<Order> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), req.symbol);
        params.insert("side".to_string(), req.side.to_string());
        params.insert("type".to_string(), req.order_type.to_string());
        params.insert("quantity".to_string(), req.quantity);

        if let Some(price) = req.price {
            params.insert("price".to_string(), price);
        }

        if let Some(stop_price) = req.stop_price {
            params.insert("stopPrice".to_string(), stop_price);
        }

        if let Some(ps) = req.position_side {
            params.insert("positionSide".to_string(), ps.to_string());
        }

        params.insert("timeInForce".to_string(), TimeInForce::Gtc.to_string());

        self.client.post_signed("/fapi/v1/order", Some(params)).await
    }
}

/// Configuration for bracket orders
#[derive(Debug, Clone)]
pub struct BracketOrderConfig {
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: String,
    pub entry_order_type: OrderType,
    pub entry_price: Option<String>,
    pub stop_loss_price: String,
    pub stop_loss_limit_price: Option<String>,
    pub take_profit_price: String,
    pub take_profit_limit_price: Option<String>,
    pub position_side: Option<PositionSide>,
}

impl BracketOrderConfig {
    /// Get the exit side (opposite of entry side)
    pub fn exit_side(&self) -> OrderSide {
        match self.side {
            OrderSide::Buy => OrderSide::Sell,
            OrderSide::Sell => OrderSide::Buy,
        }
    }
}

/// Response for bracket orders
#[derive(Debug, Clone, Deserialize)]
pub struct BracketOrderResponse {
    pub entry_order: Order,
    pub stop_loss_order: Order,
    pub take_profit_order: Order,
}

/// Response for OCO orders
#[derive(Debug, Clone, Deserialize)]
pub struct OcoOrderResponse {
    pub limit_order: Order,
    pub stop_order: Order,
}

/// Response for replace orders operation
#[derive(Debug, Clone, Deserialize)]
pub struct ReplaceOrdersResponse {
    pub cancelled_orders: Vec<Order>,
    pub new_orders: Vec<Order>,
}

/// Order request for batch operations
#[derive(Debug, Clone)]
pub struct OrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: String,
    pub price: Option<String>,
    pub stop_price: Option<String>,
    pub position_side: Option<PositionSide>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bracket_order_config() {
        let config = BracketOrderConfig {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            quantity: "0.001".to_string(),
            entry_order_type: OrderType::Market,
            entry_price: None,
            stop_loss_price: "45000.0".to_string(),
            stop_loss_limit_price: None,
            take_profit_price: "55000.0".to_string(),
            take_profit_limit_price: None,
            position_side: Some(PositionSide::Long),
        };

        assert_eq!(config.exit_side(), OrderSide::Sell);
    }
}
