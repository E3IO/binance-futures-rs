//! Trading example
//! 
//! This example demonstrates how to use the trading API.
//! IMPORTANT: This requires valid API credentials and will place actual orders on testnet.
//! Make sure to use testnet credentials for testing.

use binance_futures_rs::{
    BinanceClient, Credentials, NewOrderRequest, CancelOrderRequest, QueryOrderRequest,
    OrderSide, OrderType, TimeInForce, PositionSide
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get credentials from environment variables
    let api_key = env::var("BINANCE_API_KEY")
        .expect("Please set BINANCE_API_KEY environment variable");
    let secret_key = env::var("BINANCE_SECRET_KEY")
        .expect("Please set BINANCE_SECRET_KEY environment variable");

    let credentials = Credentials::new(api_key, secret_key);
    
    // Create testnet client with credentials
    let client = BinanceClient::testnet_with_credentials(credentials);
    let trading = client.trading();

    println!("=== Binance Futures Trading Example ===\n");
    println!("⚠️  Using TESTNET - No real money involved\n");

    // Get current open orders
    println!("1. Checking current open orders...");
    let open_orders = trading.open_orders(Some("BTCUSDT")).await?;
    println!("✓ Current open orders for BTCUSDT: {}\n", open_orders.len());

    // Place a limit buy order
    println!("2. Placing a limit buy order...");
    let new_order = NewOrderRequest::new(
        "BTCUSDT".to_string(),
        OrderSide::Buy,
        OrderType::Limit,
    )
    .quantity("0.001".to_string())
    .price("30000.0".to_string())  // Low price to avoid execution
    .time_in_force(TimeInForce::Gtc)
    .position_side(PositionSide::Both);

    match trading.new_order(new_order).await {
        Ok(order) => {
            println!("✓ Order placed successfully!");
            println!("  - Order ID: {}", order.order_id);
            println!("  - Client Order ID: {}", order.client_order_id);
            println!("  - Status: {:?}", order.status);
            println!("  - Symbol: {}", order.symbol);
            println!("  - Side: {:?}", order.side);
            println!("  - Quantity: {}", order.orig_qty);
            println!("  - Price: {}\n", order.price);

            // Query the order
            println!("3. Querying the order...");
            let query_req = QueryOrderRequest::new("BTCUSDT".to_string())
                .order_id(order.order_id);
            
            let queried_order = trading.query_order(query_req).await?;
            println!("✓ Order queried successfully!");
            println!("  - Status: {:?}", queried_order.status);
            println!("  - Executed Qty: {}\n", queried_order.executed_qty);

            // Cancel the order
            println!("4. Canceling the order...");
            let cancel_req = CancelOrderRequest::new("BTCUSDT".to_string())
                .order_id(order.order_id);
            
            let canceled_order = trading.cancel_order(cancel_req).await?;
            println!("✓ Order canceled successfully!");
            println!("  - Status: {:?}\n", canceled_order.status);
        }
        Err(e) => {
            println!("❌ Failed to place order: {}", e);
            println!("This might be due to:");
            println!("  - Invalid API credentials");
            println!("  - Insufficient balance");
            println!("  - Invalid order parameters");
            println!("  - API restrictions\n");
        }
    }

    // Get order history
    println!("5. Getting order history...");
    match trading.all_orders("BTCUSDT", None, None, None, Some(5)).await {
        Ok(orders) => {
            println!("✓ Recent orders for BTCUSDT: {}", orders.len());
            for (i, order) in orders.iter().take(3).enumerate() {
                println!("  {}. Order {} - {:?} {} {} @ {}", 
                    i + 1, order.order_id, order.status, order.side, order.orig_qty, order.price);
            }
        }
        Err(e) => {
            println!("❌ Failed to get order history: {}", e);
        }
    }

    println!("\n=== Trading Example Completed ===");
    println!("💡 Tip: Set environment variables BINANCE_API_KEY and BINANCE_SECRET_KEY");
    println!("💡 Use testnet credentials to avoid real trading");
    
    Ok(())
}
