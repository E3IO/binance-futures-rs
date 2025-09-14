use serde::{Deserialize, Serialize};
use crate::types::common::{OrderSide, OrderType, OrderStatus, TimeInForce, PositionSide, WorkingType};

/// New order request
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub position_side: Option<PositionSide>,
    pub time_in_force: Option<TimeInForce>,
    pub quantity: Option<String>,
    pub reduce_only: Option<bool>,
    pub price: Option<String>,
    pub new_client_order_id: Option<String>,
    pub stop_price: Option<String>,
    pub close_position: Option<bool>,
    pub activation_price: Option<String>,
    pub callback_rate: Option<String>,
    pub working_type: Option<WorkingType>,
    pub price_protect: Option<bool>,
}

impl NewOrderRequest {
    pub fn new(symbol: String, side: OrderSide, order_type: OrderType) -> Self {
        Self {
            symbol,
            side,
            order_type,
            position_side: None,
            time_in_force: None,
            quantity: None,
            reduce_only: None,
            price: None,
            new_client_order_id: None,
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        }
    }

    pub fn quantity(mut self, quantity: String) -> Self {
        self.quantity = Some(quantity);
        self
    }

    pub fn price(mut self, price: String) -> Self {
        self.price = Some(price);
        self
    }

    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.time_in_force = Some(time_in_force);
        self
    }

    pub fn position_side(mut self, position_side: PositionSide) -> Self {
        self.position_side = Some(position_side);
        self
    }

    pub fn reduce_only(mut self, reduce_only: bool) -> Self {
        self.reduce_only = Some(reduce_only);
        self
    }

    pub fn stop_price(mut self, stop_price: String) -> Self {
        self.stop_price = Some(stop_price);
        self
    }

    pub fn client_order_id(mut self, client_order_id: String) -> Self {
        self.new_client_order_id = Some(client_order_id);
        self
    }
}

/// Order response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub symbol: String,
    pub order_id: u64,
    pub order_list_id: i64,
    pub client_order_id: String,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub side: OrderSide,
    pub stop_price: String,
    pub ice_berg_qty: String,
    pub time: u64,
    pub update_time: u64,
    pub is_working: bool,
    pub working_time: u64,
    pub orig_quote_order_qty: String,
    pub position_side: PositionSide,
    pub price_protect: bool,
    pub close_position: bool,
    pub activation_price: Option<String>,
    pub callback_rate: Option<String>,
    pub working_type: WorkingType,
    pub price_match: Option<String>,
    pub self_trade_prevention_mode: Option<String>,
    pub good_till_date: Option<u64>,
}

/// Cancel order request
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    pub symbol: String,
    pub order_id: Option<u64>,
    pub orig_client_order_id: Option<String>,
}

impl CancelOrderRequest {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            order_id: None,
            orig_client_order_id: None,
        }
    }

    pub fn order_id(mut self, order_id: u64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    pub fn client_order_id(mut self, client_order_id: String) -> Self {
        self.orig_client_order_id = Some(client_order_id);
        self
    }
}

/// Query order request
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrderRequest {
    pub symbol: String,
    pub order_id: Option<u64>,
    pub orig_client_order_id: Option<String>,
}

impl QueryOrderRequest {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            order_id: None,
            orig_client_order_id: None,
        }
    }

    pub fn order_id(mut self, order_id: u64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    pub fn client_order_id(mut self, client_order_id: String) -> Self {
        self.orig_client_order_id = Some(client_order_id);
        self
    }
}

/// Batch order request
#[derive(Debug, Clone, Serialize)]
pub struct BatchOrderRequest {
    #[serde(rename = "batchOrders")]
    pub batch_orders: Vec<NewOrderRequest>,
}

impl BatchOrderRequest {
    pub fn new(orders: Vec<NewOrderRequest>) -> Self {
        Self {
            batch_orders: orders,
        }
    }
}

/// Trade information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTrade {
    pub symbol: String,
    pub id: u64,
    pub order_id: u64,
    pub side: OrderSide,
    pub price: String,
    pub qty: String,
    pub realized_pnl: String,
    pub margin_asset: String,
    pub quote_qty: String,
    pub commission: String,
    pub commission_asset: String,
    pub time: u64,
    pub position_side: PositionSide,
    pub buyer: bool,
    pub maker: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_order_request_builder() {
        let order = NewOrderRequest::new(
            "BTCUSDT".to_string(),
            OrderSide::Buy,
            OrderType::Limit,
        )
        .quantity("1.0".to_string())
        .price("50000.0".to_string())
        .time_in_force(TimeInForce::Gtc);

        assert_eq!(order.symbol, "BTCUSDT");
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.quantity, Some("1.0".to_string()));
        assert_eq!(order.price, Some("50000.0".to_string()));
        assert_eq!(order.time_in_force, Some(TimeInForce::Gtc));
    }

    #[test]
    fn test_cancel_order_request() {
        let cancel_req = CancelOrderRequest::new("BTCUSDT".to_string())
            .order_id(12345);

        assert_eq!(cancel_req.symbol, "BTCUSDT");
        assert_eq!(cancel_req.order_id, Some(12345));
    }
}
