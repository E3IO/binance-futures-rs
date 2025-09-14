use crate::client::HttpClient;
use crate::error::Result;
use crate::types::trading::*;
use std::collections::HashMap;

pub struct TradingApi {
    client: HttpClient,
}

impl TradingApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// Place a new order
    pub async fn new_order(&self, order: NewOrderRequest) -> Result<Order> {
        let params = self.order_to_params(&order)?;
        self.client.post_signed("/fapi/v1/order", Some(params)).await
    }

    /// Cancel an order
    pub async fn cancel_order(&self, cancel_req: CancelOrderRequest) -> Result<Order> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), cancel_req.symbol);
        
        if let Some(order_id) = cancel_req.order_id {
            params.insert("orderId".to_string(), order_id.to_string());
        }
        
        if let Some(client_order_id) = cancel_req.orig_client_order_id {
            params.insert("origClientOrderId".to_string(), client_order_id);
        }

        self.client.delete_signed("/fapi/v1/order", Some(params)).await
    }

    /// Cancel all open orders on a symbol
    pub async fn cancel_all_orders(&self, symbol: &str) -> Result<serde_json::Value> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), symbol.to_string());

        self.client.delete_signed("/fapi/v1/allOpenOrders", Some(params)).await
    }

    /// Query order
    pub async fn query_order(&self, query_req: QueryOrderRequest) -> Result<Order> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), query_req.symbol);
        
        if let Some(order_id) = query_req.order_id {
            params.insert("orderId".to_string(), order_id.to_string());
        }
        
        if let Some(client_order_id) = query_req.orig_client_order_id {
            params.insert("origClientOrderId".to_string(), client_order_id);
        }

        self.client.get_signed("/fapi/v1/order", Some(params)).await
    }

    /// Get all orders (active, canceled, or filled)
    pub async fn all_orders(
        &self,
        symbol: &str,
        order_id: Option<u64>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u32>,
    ) -> Result<Vec<Order>> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        
        if let Some(order_id) = order_id {
            params.insert("orderId".to_string(), order_id.to_string());
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

        self.client.get_signed("/fapi/v1/allOrders", Some(params)).await
    }

    /// Get current open orders
    pub async fn open_orders(&self, symbol: Option<&str>) -> Result<Vec<Order>> {
        let params = if let Some(symbol) = symbol {
            let mut params = HashMap::new();
            params.insert("symbol".to_string(), symbol.to_string());
            Some(params)
        } else {
            None
        };

        self.client.get_signed("/fapi/v1/openOrders", params).await
    }

    /// Place multiple orders
    pub async fn batch_orders(&self, orders: Vec<NewOrderRequest>) -> Result<Vec<Order>> {
        let batch_orders: Vec<HashMap<String, String>> = orders
            .into_iter()
            .map(|order| self.order_to_params(&order))
            .collect::<Result<Vec<_>>>()?;

        let batch_orders_json = serde_json::to_string(&batch_orders)?;
        
        let mut params = HashMap::new();
        params.insert("batchOrders".to_string(), batch_orders_json);

        self.client.post_signed("/fapi/v1/batchOrders", Some(params)).await
    }

    /// Get account trade list
    pub async fn user_trades(
        &self,
        symbol: &str,
        start_time: Option<u64>,
        end_time: Option<u64>,
        from_id: Option<u64>,
        limit: Option<u32>,
    ) -> Result<Vec<UserTrade>> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        
        if let Some(start_time) = start_time {
            params.insert("startTime".to_string(), start_time.to_string());
        }
        
        if let Some(end_time) = end_time {
            params.insert("endTime".to_string(), end_time.to_string());
        }
        
        if let Some(from_id) = from_id {
            params.insert("fromId".to_string(), from_id.to_string());
        }
        
        if let Some(limit) = limit {
            params.insert("limit".to_string(), limit.to_string());
        }

        self.client.get_signed("/fapi/v1/userTrades", Some(params)).await
    }

    /// Convert NewOrderRequest to HashMap for API call
    fn order_to_params(&self, order: &NewOrderRequest) -> Result<HashMap<String, String>> {
        let mut params = HashMap::new();
        
        params.insert("symbol".to_string(), order.symbol.clone());
        params.insert("side".to_string(), serde_json::to_string(&order.side)?.trim_matches('"').to_string());
        params.insert("type".to_string(), serde_json::to_string(&order.order_type)?.trim_matches('"').to_string());
        
        if let Some(position_side) = &order.position_side {
            params.insert("positionSide".to_string(), serde_json::to_string(position_side)?.trim_matches('"').to_string());
        }
        
        if let Some(time_in_force) = &order.time_in_force {
            params.insert("timeInForce".to_string(), serde_json::to_string(time_in_force)?.trim_matches('"').to_string());
        }
        
        if let Some(quantity) = &order.quantity {
            params.insert("quantity".to_string(), quantity.clone());
        }
        
        if let Some(reduce_only) = order.reduce_only {
            params.insert("reduceOnly".to_string(), reduce_only.to_string());
        }
        
        if let Some(price) = &order.price {
            params.insert("price".to_string(), price.clone());
        }
        
        if let Some(client_order_id) = &order.new_client_order_id {
            params.insert("newClientOrderId".to_string(), client_order_id.clone());
        }
        
        if let Some(stop_price) = &order.stop_price {
            params.insert("stopPrice".to_string(), stop_price.clone());
        }
        
        if let Some(close_position) = order.close_position {
            params.insert("closePosition".to_string(), close_position.to_string());
        }
        
        if let Some(activation_price) = &order.activation_price {
            params.insert("activationPrice".to_string(), activation_price.clone());
        }
        
        if let Some(callback_rate) = &order.callback_rate {
            params.insert("callbackRate".to_string(), callback_rate.clone());
        }
        
        if let Some(working_type) = &order.working_type {
            params.insert("workingType".to_string(), serde_json::to_string(working_type)?.trim_matches('"').to_string());
        }
        
        if let Some(price_protect) = order.price_protect {
            params.insert("priceProtect".to_string(), price_protect.to_string());
        }
        
        Ok(params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::{HttpClient, Credentials};
    use crate::types::common::{OrderSide, OrderType, TimeInForce};

    #[test]
    fn test_trading_api_creation() {
        let credentials = Credentials::new("test_key".to_string(), "test_secret".to_string());
        let client = HttpClient::new_with_credentials(credentials);
        let trading_api = TradingApi::new(client);
        // Just test that we can create the API instance
        assert!(true);
    }

    #[test]
    fn test_order_to_params() {
        let credentials = Credentials::new("test_key".to_string(), "test_secret".to_string());
        let client = HttpClient::new_with_credentials(credentials);
        let trading_api = TradingApi::new(client);
        
        let order = NewOrderRequest::new(
            "BTCUSDT".to_string(),
            OrderSide::Buy,
            OrderType::Limit,
        )
        .quantity("1.0".to_string())
        .price("50000.0".to_string())
        .time_in_force(TimeInForce::Gtc);

        let params = trading_api.order_to_params(&order).unwrap();
        
        assert_eq!(params.get("symbol").unwrap(), "BTCUSDT");
        assert_eq!(params.get("side").unwrap(), "BUY");
        assert_eq!(params.get("type").unwrap(), "LIMIT");
        assert_eq!(params.get("quantity").unwrap(), "1.0");
        assert_eq!(params.get("price").unwrap(), "50000.0");
        assert_eq!(params.get("timeInForce").unwrap(), "GTC");
    }
}
