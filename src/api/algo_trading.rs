//! Algorithmic trading strategies and automation
//!
//! This module provides algorithmic trading capabilities including:
//! - DCA (Dollar Cost Averaging) strategies
//! - Grid trading strategies
//! - TWAP (Time Weighted Average Price) execution
//! - VWAP (Volume Weighted Average Price) execution
//! - Position sizing and risk management

use crate::client::http::HttpClient;
use crate::error::Result;
use crate::types::common::{OrderSide, OrderType, PositionSide, TimeInForce};
use crate::types::trading::Order;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::interval;

/// Algorithmic trading API client
#[derive(Clone)]
pub struct AlgoTradingApi {
    client: HttpClient,
}

impl AlgoTradingApi {
    /// Create a new algorithmic trading API client
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// Execute a DCA (Dollar Cost Averaging) strategy
    /// 
    /// # Arguments
    /// 
    /// * `config` - DCA strategy configuration
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use binance_futures_rs::{BinanceClient, Credentials};
    /// use binance_futures_rs::api::algo_trading::{DcaConfig, OrderSide};
    /// use std::time::Duration;
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let credentials = Credentials::new("api_key".to_string(), "secret".to_string());
    /// let client = BinanceClient::new_with_credentials(credentials);
    /// 
    /// let dca_config = DcaConfig {
    ///     symbol: "BTCUSDT".to_string(),
    ///     side: OrderSide::Buy,
    ///     total_amount: "1000.0".to_string(),
    ///     order_count: 10,
    ///     interval: Duration::from_secs(3600), // 1 hour
    ///     price_deviation_threshold: Some(0.02), // 2%
    ///     position_side: None,
    /// };
    /// 
    /// let result = client.algo_trading().execute_dca(dca_config).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn execute_dca(&self, config: DcaConfig) -> Result<DcaResult> {
        let order_amount = self.calculate_order_amount(&config.total_amount, config.order_count)?;
        let mut orders = Vec::new();
        let mut interval_timer = interval(config.interval);

        for i in 0..config.order_count {
            interval_timer.tick().await;

            // Check price deviation if threshold is set
            if let Some(threshold) = config.price_deviation_threshold {
                if self.should_skip_order(&config.symbol, threshold).await? {
                    continue;
                }
            }

            let order = self.place_market_order(
                &config.symbol,
                config.side,
                &order_amount,
                config.position_side,
            ).await?;

            orders.push(DcaOrderResult {
                order_id: order.order_id as i64,
                price: order.price,
                quantity: order.orig_qty,
                timestamp: order.update_time as i64,
                order_number: (i + 1) as usize,
            });
        }

        let total_executed = self.calculate_total_executed(&orders);
        Ok(DcaResult {
            total_orders: orders.len(),
            orders,
            total_executed_amount: total_executed,
        })
    }

    /// Execute a grid trading strategy
    /// 
    /// # Arguments
    /// 
    /// * `config` - Grid trading configuration
    pub async fn execute_grid_trading(&self, config: GridTradingConfig) -> Result<GridTradingResult> {
        let grid_levels = self.calculate_grid_levels(&config)?;
        let mut orders = Vec::new();

        // Place initial grid orders
        for level in &grid_levels {
            let buy_order = self.place_limit_order(
                &config.symbol,
                OrderSide::Buy,
                &config.quantity_per_grid,
                &level.buy_price,
                config.position_side.clone(),
            ).await?;

            let sell_order = self.place_limit_order(
                &config.symbol,
                OrderSide::Sell,
                &config.quantity_per_grid,
                &level.sell_price,
                config.position_side.clone(),
            ).await?;

            orders.push(GridOrderPair {
                level: level.level,
                buy_order_id: buy_order.order_id as i64,
                sell_order_id: sell_order.order_id as i64,
                buy_price: level.buy_price.clone(),
                sell_price: level.sell_price.clone(),
            });
        }

        Ok(GridTradingResult {
            grid_levels: grid_levels.len(),
            orders,
            total_capital_used: self.calculate_grid_capital(&config, &grid_levels),
        })
    }

    /// Execute TWAP (Time Weighted Average Price) order
    /// 
    /// # Arguments
    /// 
    /// * `config` - TWAP execution configuration
    pub async fn execute_twap(&self, config: TwapConfig) -> Result<TwapResult> {
        let slice_size = self.calculate_twap_slice_size(&config)?;
        let slice_interval = config.duration / config.slices as u32;
        let mut orders = Vec::new();
        let mut interval_timer = interval(slice_interval);

        for i in 0..config.slices {
            interval_timer.tick().await;

            let order = self.place_market_order(
                &config.symbol,
                config.side,
                &slice_size.to_string(),
                config.position_side,
            ).await?;

            orders.push(TwapSliceResult {
                slice_number: (i + 1) as usize,
                order_id: order.order_id as i64,
                price: order.price,
                quantity: order.orig_qty,
                timestamp: order.update_time as i64,
            });
        }

        let average_price = self.calculate_twap_average_price(&orders);
        let total_executed_quantity = self.calculate_total_quantity(&orders);
        Ok(TwapResult {
            total_slices: orders.len(),
            orders,
            average_price,
            total_executed_quantity,
        })
    }

    /// Execute VWAP (Volume Weighted Average Price) order
    /// 
    /// # Arguments
    /// 
    /// * `config` - VWAP execution configuration
    pub async fn execute_vwap(&self, config: VwapConfig) -> Result<VwapResult> {
        let mut orders = Vec::new();
        let mut remaining_quantity = config.total_quantity.parse::<f64>().unwrap_or(0.0);
        let slice_interval = config.duration / config.max_slices as u32;
        let mut interval_timer = interval(slice_interval);

        for i in 0..config.max_slices {
            if remaining_quantity <= 0.0 {
                break;
            }

            interval_timer.tick().await;

            // Get current market volume to determine slice size
            let volume_data = self.get_recent_volume(&config.symbol).await?;
            let slice_size = self.calculate_vwap_slice_size(
                remaining_quantity,
                &volume_data,
                config.participation_rate,
            );

            if slice_size <= 0.0 {
                continue;
            }

            let order = self.place_market_order(
                &config.symbol,
                config.side,
                &slice_size.to_string(),
                config.position_side,
            ).await?;

            remaining_quantity -= order.orig_qty.parse::<f64>().unwrap_or(0.0);

            orders.push(VwapSliceResult {
                slice_number: (i + 1) as usize,
                order_id: order.order_id as i64,
                price: order.price,
                quantity: order.orig_qty,
                timestamp: order.update_time as i64,
                market_volume: volume_data.volume,
            });
        }

        let vwap_price = self.calculate_vwap_price(&orders);
        let total_executed_quantity = self.calculate_total_quantity(&orders);
        Ok(VwapResult {
            total_slices: orders.len(),
            orders,
            vwap_price,
            total_executed_quantity,
            remaining_quantity: remaining_quantity.to_string(),
        })
    }

    /// Calculate optimal position size based on risk parameters
    /// 
    /// # Arguments
    /// 
    /// * `config` - Position sizing configuration
    pub async fn calculate_position_size(&self, config: PositionSizingConfig) -> Result<PositionSizeResult> {
        // Get current account balance
        let account_info = self.get_account_info().await?;
        let available_balance = account_info.available_balance.parse::<f64>().unwrap_or(0.0);

        // Calculate position size based on risk percentage
        let risk_amount = available_balance * config.risk_percentage;
        
        // Get current price
        let current_price = self.get_current_price(&config.symbol).await?;
        
        // Calculate stop loss distance
        let stop_distance = (current_price - config.stop_loss_price).abs();
        
        // Calculate position size
        let position_size = if stop_distance > 0.0 {
            risk_amount / stop_distance
        } else {
            0.0
        };

        // Apply maximum position size limit
        let final_position_size = position_size.min(config.max_position_size);

        Ok(PositionSizeResult {
            recommended_size: final_position_size.to_string(),
            risk_amount: risk_amount.to_string(),
            current_price: current_price.to_string(),
            stop_distance: stop_distance.to_string(),
            risk_reward_ratio: self.calculate_risk_reward_ratio(
                current_price,
                config.stop_loss_price,
                config.take_profit_price,
            ),
        })
    }

    // Helper methods
    async fn place_market_order(
        &self,
        symbol: &str,
        side: OrderSide,
        quantity: &str,
        position_side: Option<PositionSide>,
    ) -> Result<Order> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("side".to_string(), side.to_string());
        params.insert("type".to_string(), OrderType::Market.to_string());
        params.insert("quantity".to_string(), quantity.to_string());

        if let Some(ps) = position_side {
            params.insert("positionSide".to_string(), ps.to_string());
        }

        self.client.post_signed("/fapi/v1/order", Some(params)).await
    }

    async fn place_limit_order(
        &self,
        symbol: &str,
        side: OrderSide,
        quantity: &str,
        price: &str,
        position_side: Option<PositionSide>,
    ) -> Result<Order> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("side".to_string(), side.to_string());
        params.insert("type".to_string(), OrderType::Limit.to_string());
        params.insert("quantity".to_string(), quantity.to_string());
        params.insert("price".to_string(), price.to_string());
        params.insert("timeInForce".to_string(), TimeInForce::Gtc.to_string());

        if let Some(ps) = position_side {
            params.insert("positionSide".to_string(), ps.to_string());
        }

        self.client.post_signed("/fapi/v1/order", Some(params)).await
    }

    fn calculate_order_amount(&self, total_amount: &str, order_count: u32) -> Result<String> {
        let total = total_amount.parse::<f64>().map_err(|_| {
            crate::error::BinanceError::InvalidParameter("Invalid total amount".to_string())
        })?;
        let amount_per_order = total / order_count as f64;
        Ok(amount_per_order.to_string())
    }

    async fn should_skip_order(&self, _symbol: &str, _threshold: f64) -> Result<bool> {
        // Implementation would check current price against moving average
        // For now, return false (don't skip)
        Ok(false)
    }

    fn calculate_total_executed(&self, orders: &[DcaOrderResult]) -> String {
        let total: f64 = orders.iter()
            .map(|o| o.price.parse::<f64>().unwrap_or(0.0) * o.quantity.parse::<f64>().unwrap_or(0.0))
            .sum();
        total.to_string()
    }

    fn calculate_grid_levels(&self, config: &GridTradingConfig) -> Result<Vec<GridLevel>> {
        let mut levels = Vec::new();
        let price_step = (config.upper_price - config.lower_price) / config.grid_count as f64;

        for i in 0..config.grid_count {
            let level_price = config.lower_price + (i as f64 * price_step);
            levels.push(GridLevel {
                level: i + 1,
                buy_price: (level_price - price_step / 2.0).to_string(),
                sell_price: (level_price + price_step / 2.0).to_string(),
            });
        }

        Ok(levels)
    }

    fn calculate_grid_capital(&self, config: &GridTradingConfig, levels: &[GridLevel]) -> String {
        let quantity_per_grid = config.quantity_per_grid.parse::<f64>().unwrap_or(0.0);
        let total_capital: f64 = levels.iter()
            .map(|level| {
                let buy_price = level.buy_price.parse::<f64>().unwrap_or(0.0);
                buy_price * quantity_per_grid
            })
            .sum();
        total_capital.to_string()
    }

    fn calculate_twap_slice_size(&self, config: &TwapConfig) -> Result<String> {
        let total_quantity = config.total_quantity.parse::<f64>().map_err(|_| {
            crate::error::BinanceError::InvalidParameter("Invalid total quantity".to_string())
        })?;
        let slice_size = total_quantity / config.slices as f64;
        Ok(slice_size.to_string())
    }

    fn calculate_twap_average_price(&self, orders: &[TwapSliceResult]) -> String {
        if orders.is_empty() {
            return "0".to_string();
        }

        let total_value: f64 = orders.iter()
            .map(|o| o.price.parse::<f64>().unwrap_or(0.0) * o.quantity.parse::<f64>().unwrap_or(0.0))
            .sum();
        let total_quantity: f64 = orders.iter()
            .map(|o| o.quantity.parse::<f64>().unwrap_or(0.0))
            .sum();

        if total_quantity > 0.0 {
            (total_value / total_quantity).to_string()
        } else {
            "0".to_string()
        }
    }

    fn calculate_total_quantity(&self, orders: &[impl QuantityProvider]) -> String {
        let total: f64 = orders.iter()
            .map(|o| o.get_quantity().parse::<f64>().unwrap_or(0.0))
            .sum();
        total.to_string()
    }

    async fn get_recent_volume(&self, _symbol: &str) -> Result<VolumeData> {
        // Implementation would fetch recent volume data
        // For now, return mock data
        Ok(VolumeData {
            volume: "1000.0".to_string(),
            timestamp: chrono::Utc::now().timestamp_millis(),
        })
    }

    fn calculate_vwap_slice_size(&self, remaining_quantity: f64, volume_data: &VolumeData, participation_rate: f64) -> f64 {
        let market_volume = volume_data.volume.parse::<f64>().unwrap_or(0.0);
        let max_slice = market_volume * participation_rate;
        remaining_quantity.min(max_slice)
    }

    fn calculate_vwap_price(&self, orders: &[VwapSliceResult]) -> String {
        if orders.is_empty() {
            return "0".to_string();
        }

        let total_value: f64 = orders.iter()
            .map(|o| o.price.parse::<f64>().unwrap_or(0.0) * o.quantity.parse::<f64>().unwrap_or(0.0))
            .sum();
        let total_quantity: f64 = orders.iter()
            .map(|o| o.quantity.parse::<f64>().unwrap_or(0.0))
            .sum();

        if total_quantity > 0.0 {
            (total_value / total_quantity).to_string()
        } else {
            "0".to_string()
        }
    }

    async fn get_account_info(&self) -> Result<AccountInfo> {
        // Implementation would fetch account info
        // For now, return mock data
        Ok(AccountInfo {
            available_balance: "10000.0".to_string(),
        })
    }

    async fn get_current_price(&self, _symbol: &str) -> Result<f64> {
        // Implementation would fetch current price
        // For now, return mock price
        Ok(50000.0)
    }

    fn calculate_risk_reward_ratio(&self, entry_price: f64, stop_loss: f64, take_profit: f64) -> f64 {
        let risk = (entry_price - stop_loss).abs();
        let reward = (take_profit - entry_price).abs();
        
        if risk > 0.0 {
            reward / risk
        } else {
            0.0
        }
    }
}

// Configuration structs
#[derive(Debug, Clone)]
pub struct DcaConfig {
    pub symbol: String,
    pub side: OrderSide,
    pub total_amount: String,
    pub order_count: u32,
    pub interval: Duration,
    pub price_deviation_threshold: Option<f64>,
    pub position_side: Option<PositionSide>,
}

#[derive(Debug, Clone)]
pub struct GridTradingConfig {
    pub symbol: String,
    pub lower_price: f64,
    pub upper_price: f64,
    pub grid_count: u32,
    pub quantity_per_grid: String,
    pub position_side: Option<PositionSide>,
}

#[derive(Debug, Clone)]
pub struct TwapConfig {
    pub symbol: String,
    pub side: OrderSide,
    pub total_quantity: String,
    pub duration: Duration,
    pub slices: u32,
    pub position_side: Option<PositionSide>,
}

#[derive(Debug, Clone)]
pub struct VwapConfig {
    pub symbol: String,
    pub side: OrderSide,
    pub total_quantity: String,
    pub duration: Duration,
    pub max_slices: u32,
    pub participation_rate: f64, // 0.0 to 1.0
    pub position_side: Option<PositionSide>,
}

#[derive(Debug, Clone)]
pub struct PositionSizingConfig {
    pub symbol: String,
    pub risk_percentage: f64, // 0.0 to 1.0
    pub stop_loss_price: f64,
    pub take_profit_price: f64,
    pub max_position_size: f64,
}

// Result structs
#[derive(Debug, Clone, Deserialize)]
pub struct DcaResult {
    pub total_orders: usize,
    pub orders: Vec<DcaOrderResult>,
    pub total_executed_amount: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DcaOrderResult {
    pub order_id: i64,
    pub price: String,
    pub quantity: String,
    pub timestamp: i64,
    pub order_number: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GridTradingResult {
    pub grid_levels: usize,
    pub orders: Vec<GridOrderPair>,
    pub total_capital_used: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GridOrderPair {
    pub level: u32,
    pub buy_order_id: i64,
    pub sell_order_id: i64,
    pub buy_price: String,
    pub sell_price: String,
}

#[derive(Debug, Clone)]
pub struct GridLevel {
    pub level: u32,
    pub buy_price: String,
    pub sell_price: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TwapResult {
    pub total_slices: usize,
    pub orders: Vec<TwapSliceResult>,
    pub average_price: String,
    pub total_executed_quantity: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TwapSliceResult {
    pub slice_number: usize,
    pub order_id: i64,
    pub price: String,
    pub quantity: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VwapResult {
    pub total_slices: usize,
    pub orders: Vec<VwapSliceResult>,
    pub vwap_price: String,
    pub total_executed_quantity: String,
    pub remaining_quantity: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VwapSliceResult {
    pub slice_number: usize,
    pub order_id: i64,
    pub price: String,
    pub quantity: String,
    pub timestamp: i64,
    pub market_volume: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PositionSizeResult {
    pub recommended_size: String,
    pub risk_amount: String,
    pub current_price: String,
    pub stop_distance: String,
    pub risk_reward_ratio: f64,
}

// Helper structs
#[derive(Debug, Clone)]
pub struct VolumeData {
    pub volume: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone)]
pub struct AccountInfo {
    pub available_balance: String,
}

// Trait for quantity providers
pub trait QuantityProvider {
    fn get_quantity(&self) -> &str;
}

impl QuantityProvider for TwapSliceResult {
    fn get_quantity(&self) -> &str {
        &self.quantity
    }
}

impl QuantityProvider for VwapSliceResult {
    fn get_quantity(&self) -> &str {
        &self.quantity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dca_config() {
        let config = DcaConfig {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            total_amount: "1000.0".to_string(),
            order_count: 10,
            interval: Duration::from_secs(3600),
            price_deviation_threshold: Some(0.02),
            position_side: None,
        };

        assert_eq!(config.symbol, "BTCUSDT");
        assert_eq!(config.order_count, 10);
    }

    #[test]
    fn test_grid_trading_config() {
        let config = GridTradingConfig {
            symbol: "BTCUSDT".to_string(),
            lower_price: 45000.0,
            upper_price: 55000.0,
            grid_count: 10,
            quantity_per_grid: "0.001".to_string(),
            position_side: None,
        };

        assert_eq!(config.grid_count, 10);
        assert_eq!(config.upper_price - config.lower_price, 10000.0);
    }
}
