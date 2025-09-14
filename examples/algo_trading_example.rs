//! Algorithmic trading strategies example
//! 
//! This example demonstrates algorithmic trading capabilities including:
//! - DCA (Dollar Cost Averaging) strategy
//! - Grid trading strategy
//! - TWAP (Time Weighted Average Price) execution
//! - VWAP (Volume Weighted Average Price) execution
//! - Position sizing calculations

use binance_futures_rs::{
    BinanceClient, Credentials, OrderSide, PositionSide,
    api::algo_trading::{DcaConfig, GridTradingConfig, TwapConfig, VwapConfig, PositionSizingConfig},
};
use std::env;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量获取API凭证
    let api_key = env::var("BINANCE_API_KEY")
        .expect("请设置 BINANCE_API_KEY 环境变量");
    let secret_key = env::var("BINANCE_SECRET_KEY")
        .expect("请设置 BINANCE_SECRET_KEY 环境变量");

    let credentials = Credentials::new(api_key, secret_key);
    let client = BinanceClient::new_with_credentials(credentials);

    println!("=== Binance Futures 算法交易策略示例 ===\n");

    // 1. DCA策略示例
    println!("1. 执行DCA策略...");
    let dca_config = DcaConfig {
        symbol: "BTCUSDT".to_string(),
        side: OrderSide::Buy,
        total_amount: "1000.0".to_string(),
        order_count: 5,
        interval: Duration::from_secs(300), // 5分钟间隔
        price_deviation_threshold: Some(0.02), // 2%价格偏差阈值
        position_side: Some(PositionSide::Long),
    };

    match client.algo_trading().execute_dca(dca_config).await {
        Ok(result) => {
            println!("✅ DCA策略执行完成:");
            println!("   总订单数: {}", result.total_orders);
            println!("   总执行金额: ${}", result.total_executed_amount);
            for order in &result.orders {
                println!("   订单#{}: {} @ ${}", 
                    order.order_number, order.quantity, order.price);
            }
        }
        Err(e) => println!("❌ DCA策略失败: {}", e),
    }

    // 2. 网格交易策略示例
    println!("\n2. 执行网格交易策略...");
    let grid_config = GridTradingConfig {
        symbol: "ETHUSDT".to_string(),
        lower_price: 2800.0,
        upper_price: 3200.0,
        grid_count: 10,
        quantity_per_grid: "0.01".to_string(),
        position_side: Some(PositionSide::Long),
    };

    match client.algo_trading().execute_grid_trading(grid_config).await {
        Ok(result) => {
            println!("✅ 网格交易策略部署完成:");
            println!("   网格层数: {}", result.grid_levels);
            println!("   使用资金: ${}", result.total_capital_used);
            for order_pair in &result.orders {
                println!("   网格#{}: 买入@${} 卖出@${}", 
                    order_pair.level, order_pair.buy_price, order_pair.sell_price);
            }
        }
        Err(e) => println!("❌ 网格交易策略失败: {}", e),
    }

    // 3. TWAP执行示例
    println!("\n3. 执行TWAP订单...");
    let twap_config = TwapConfig {
        symbol: "ADAUSDT".to_string(),
        side: OrderSide::Buy,
        total_quantity: "1000.0".to_string(),
        duration: Duration::from_secs(3600), // 1小时
        slices: 12, // 12个切片，每5分钟一次
        position_side: Some(PositionSide::Long),
    };

    match client.algo_trading().execute_twap(twap_config).await {
        Ok(result) => {
            println!("✅ TWAP执行完成:");
            println!("   总切片数: {}", result.total_slices);
            println!("   平均价格: ${}", result.average_price);
            println!("   总执行数量: {}", result.total_executed_quantity);
            for slice in &result.orders {
                println!("   切片#{}: {} @ ${}", 
                    slice.slice_number, slice.quantity, slice.price);
            }
        }
        Err(e) => println!("❌ TWAP执行失败: {}", e),
    }

    // 4. VWAP执行示例
    println!("\n4. 执行VWAP订单...");
    let vwap_config = VwapConfig {
        symbol: "BNBUSDT".to_string(),
        side: OrderSide::Buy,
        total_quantity: "50.0".to_string(),
        duration: Duration::from_secs(1800), // 30分钟
        max_slices: 10,
        participation_rate: 0.1, // 10%市场参与率
        position_side: Some(PositionSide::Long),
    };

    match client.algo_trading().execute_vwap(vwap_config).await {
        Ok(result) => {
            println!("✅ VWAP执行完成:");
            println!("   总切片数: {}", result.total_slices);
            println!("   VWAP价格: ${}", result.vwap_price);
            println!("   总执行数量: {}", result.total_executed_quantity);
            println!("   剩余数量: {}", result.remaining_quantity);
            for slice in &result.orders {
                println!("   切片#{}: {} @ ${} (市场量: {})", 
                    slice.slice_number, slice.quantity, slice.price, slice.market_volume);
            }
        }
        Err(e) => println!("❌ VWAP执行失败: {}", e),
    }

    // 5. 仓位大小计算示例
    println!("\n5. 计算最优仓位大小...");
    let position_config = PositionSizingConfig {
        symbol: "BTCUSDT".to_string(),
        risk_percentage: 0.02, // 2%风险
        stop_loss_price: 48000.0,
        take_profit_price: 55000.0,
        max_position_size: 0.1,
    };

    match client.algo_trading().calculate_position_size(position_config).await {
        Ok(result) => {
            println!("✅ 仓位大小计算完成:");
            println!("   推荐仓位: {}", result.recommended_size);
            println!("   风险金额: ${}", result.risk_amount);
            println!("   当前价格: ${}", result.current_price);
            println!("   止损距离: ${}", result.stop_distance);
            println!("   风险回报比: {:.2}", result.risk_reward_ratio);
        }
        Err(e) => println!("❌ 仓位计算失败: {}", e),
    }

    println!("\n=== 算法交易策略示例完成 ===");
    println!("\n💡 算法交易优势:");
    println!("- DCA: 平均成本，降低时机风险");
    println!("- 网格: 震荡市场中获利，自动化交易");
    println!("- TWAP: 时间分散，减少市场冲击");
    println!("- VWAP: 跟随市场节奏，最小化滑点");
    println!("- 仓位管理: 科学风控，最大化风险调整收益");

    Ok(())
}
