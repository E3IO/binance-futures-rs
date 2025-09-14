//! Advanced trading features example
//! 
//! This example demonstrates advanced trading features including:
//! - Conditional orders (stop-loss, take-profit, trailing stop)
//! - Bracket orders
//! - OCO (One-Cancels-Other) orders
//! - Order replacement strategies

use binance_futures_rs::{
    BinanceClient, Credentials, OrderSide, OrderType, PositionSide,
    api::advanced_trading::BracketOrderConfig,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量获取API凭证
    let api_key = env::var("BINANCE_API_KEY")
        .expect("请设置 BINANCE_API_KEY 环境变量");
    let secret_key = env::var("BINANCE_SECRET_KEY")
        .expect("请设置 BINANCE_SECRET_KEY 环境变量");

    let credentials = Credentials::new(api_key, secret_key);
    let client = BinanceClient::new_with_credentials(credentials);

    println!("=== Binance Futures 高级交易功能示例 ===\n");

    // 1. 止损单示例
    println!("1. 下止损单...");
    match client.advanced_trading().stop_loss_order(
        "BTCUSDT",
        OrderSide::Sell,
        "0.001",
        "45000.0", // 止损价格
        None,      // 市价止损
        Some(PositionSide::Long),
    ).await {
        Ok(order) => {
            println!("✅ 止损单已下达:");
            println!("   订单ID: {}", order.order_id);
            println!("   符号: {}", order.symbol);
            println!("   数量: {}", order.orig_qty);
        }
        Err(e) => println!("❌ 止损单失败: {}", e),
    }

    // 2. 止盈单示例
    println!("\n2. 下止盈单...");
    match client.advanced_trading().take_profit_order(
        "BTCUSDT",
        OrderSide::Sell,
        "0.001",
        "55000.0", // 止盈价格
        None,      // 市价止盈
        Some(PositionSide::Long),
    ).await {
        Ok(order) => {
            println!("✅ 止盈单已下达:");
            println!("   订单ID: {}", order.order_id);
            println!("   符号: {}", order.symbol);
            println!("   数量: {}", order.orig_qty);
        }
        Err(e) => println!("❌ 止盈单失败: {}", e),
    }

    // 3. 追踪止损单示例
    println!("\n3. 下追踪止损单...");
    match client.advanced_trading().trailing_stop_order(
        "BTCUSDT",
        OrderSide::Sell,
        "0.001",
        "1.0",     // 回调率 1%
        Some("52000.0"), // 激活价格
        Some(PositionSide::Long),
    ).await {
        Ok(order) => {
            println!("✅ 追踪止损单已下达:");
            println!("   订单ID: {}", order.order_id);
            println!("   符号: {}", order.symbol);
            println!("   数量: {}", order.orig_qty);
        }
        Err(e) => println!("❌ 追踪止损单失败: {}", e),
    }

    // 4. 括号单示例（入场 + 止损 + 止盈）
    println!("\n4. 下括号单...");
    let bracket_config = BracketOrderConfig {
        symbol: "ETHUSDT".to_string(),
        side: OrderSide::Buy,
        quantity: "0.01".to_string(),
        entry_order_type: OrderType::Market,
        entry_price: None,
        stop_loss_price: "2800.0".to_string(),
        stop_loss_limit_price: None,
        take_profit_price: "3200.0".to_string(),
        take_profit_limit_price: None,
        position_side: Some(PositionSide::Long),
    };

    match client.advanced_trading().bracket_order(bracket_config).await {
        Ok(result) => {
            println!("✅ 括号单已下达:");
            println!("   入场订单ID: {}", result.entry_order.order_id);
            println!("   止损订单ID: {}", result.stop_loss_order.order_id);
            println!("   止盈订单ID: {}", result.take_profit_order.order_id);
        }
        Err(e) => println!("❌ 括号单失败: {}", e),
    }

    // 5. OCO订单示例
    println!("\n5. 下OCO订单...");
    match client.advanced_trading().oco_order(
        "ADAUSDT",
        OrderSide::Sell,
        "100.0",
        "0.55",    // 限价单价格
        "0.45",    // 止损触发价格
        Some("0.44"), // 止损限价
        Some(PositionSide::Long),
    ).await {
        Ok(result) => {
            println!("✅ OCO订单已下达:");
            println!("   限价订单ID: {}", result.limit_order.order_id);
            println!("   止损订单ID: {}", result.stop_order.order_id);
        }
        Err(e) => println!("❌ OCO订单失败: {}", e),
    }

    println!("\n=== 高级交易功能示例完成 ===");
    println!("\n💡 提示:");
    println!("- 所有订单都包含完整的风险管理");
    println!("- 括号单可以同时设置止损和止盈");
    println!("- OCO订单实现了条件性订单管理");
    println!("- 追踪止损可以锁定利润并限制损失");

    Ok(())
}
