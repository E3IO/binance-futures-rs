use binance_fu_rs::{
    BinanceClient, Credentials, Result,
    websocket::{StreamBuilder, UserDataStream, UserDataStreamConfig, WebSocketClient, WebSocketMessage},
};
use futures_util::StreamExt;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Binance Futures WebSocket 示例 ===\n");

    // 示例1: 市场数据流
    println!("1. 连接市场数据流...");
    if let Err(e) = market_data_streams().await {
        eprintln!("市场数据流错误: {}", e);
    }

    // 示例2: 用户数据流 (需要API密钥)
    println!("\n2. 连接用户数据流...");
    if let Err(e) = user_data_stream().await {
        eprintln!("用户数据流错误: {}", e);
    }

    Ok(())
}

/// 市场数据流示例
async fn market_data_streams() -> Result<()> {
    // 使用StreamBuilder创建多个数据流
    let mut ws = StreamBuilder::new()
        .depth("BTCUSDT", Some(5)) // BTC深度数据
        .trade("ETHUSDT") // ETH成交数据
        .kline("ADAUSDT", "1m") // ADA 1分钟K线
        .ticker("BNBUSDT") // BNB 24小时行情
        .connect()
        .await?;

    println!("已连接到市场数据流，开始接收数据...");

    let mut message_count = 0;
    const MAX_MESSAGES: usize = 20; // 限制接收消息数量用于演示

    while let Some(msg) = ws.next().await {
        match msg {
            Ok(tokio_tungstenite::tungstenite::Message::Text(text)) => {
                match WebSocketClient::parse_message(&text) {
                    Ok(ws_msg) => {
                        handle_market_message(ws_msg);
                        message_count += 1;
                        if message_count >= MAX_MESSAGES {
                            println!("已接收 {} 条消息，停止市场数据流演示", MAX_MESSAGES);
                            break;
                        }
                    }
                    Err(e) => eprintln!("解析消息错误: {}", e),
                }
            }
            Ok(tokio_tungstenite::tungstenite::Message::Ping(_)) => {
                println!("收到Ping消息");
            }
            Ok(tokio_tungstenite::tungstenite::Message::Close(_)) => {
                println!("WebSocket连接已关闭");
                break;
            }
            Err(e) => {
                eprintln!("WebSocket错误: {}", e);
                break;
            }
            _ => {}
        }
    }

    Ok(())
}

/// 处理市场数据消息
fn handle_market_message(message: WebSocketMessage) {
    match message {
        WebSocketMessage::DepthUpdate(depth) => {
            println!(
                "📊 深度更新 - {}: 买盘{}档, 卖盘{}档",
                depth.symbol,
                depth.bids.len(),
                depth.asks.len()
            );
            if !depth.bids.is_empty() {
                println!("   最高买价: {}", depth.bids[0][0]);
            }
            if !depth.asks.is_empty() {
                println!("   最低卖价: {}", depth.asks[0][0]);
            }
        }
        WebSocketMessage::Trade(trade) => {
            println!(
                "💰 成交 - {}: 价格={}, 数量={}, {}方主动",
                trade.symbol,
                trade.price,
                trade.quantity,
                if trade.is_buyer_maker { "卖" } else { "买" }
            );
        }
        WebSocketMessage::Kline(kline) => {
            let k = &kline.kline;
            println!(
                "📈 K线 - {}: O={}, H={}, L={}, C={}, V={}",
                k.symbol, k.open, k.high, k.low, k.close, k.volume
            );
        }
        WebSocketMessage::Ticker(ticker) => {
            println!(
                "📋 行情 - {}: 价格={}, 24h涨跌={}%, 成交量={}",
                ticker.symbol, ticker.last_price, ticker.price_change_percent, ticker.volume
            );
        }
        _ => {}
    }
}

/// 用户数据流示例 (需要API密钥)
async fn user_data_stream() -> Result<()> {
    // 检查环境变量
    let api_key = match env::var("BINANCE_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("⚠️  未设置BINANCE_API_KEY环境变量，跳过用户数据流演示");
            return Ok(());
        }
    };

    let secret_key = match env::var("BINANCE_SECRET_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("⚠️  未设置BINANCE_SECRET_KEY环境变量，跳过用户数据流演示");
            return Ok(());
        }
    };

    println!("使用测试网进行用户数据流演示...");

    // 创建认证凭据
    let credentials = Credentials::new(api_key, secret_key);

    // 创建测试网客户端
    let client = BinanceClient::testnet_with_credentials(credentials);

    // 创建用户数据流
    let config = UserDataStreamConfig::default();
    let mut user_stream = UserDataStream::new(client.http_client().clone(), config);

    // 启动用户数据流
    let listen_key = match user_stream.start().await {
        Ok(key) => {
            println!("✅ 用户数据流已启动，监听密钥: {}", &key[..8]);
            key
        }
        Err(e) => {
            eprintln!("❌ 启动用户数据流失败: {}", e);
            return Err(e);
        }
    };

    // 连接WebSocket
    let ws_client = WebSocketClient::testnet();
    let mut ws = ws_client.user_data_stream(&listen_key).await?;

    println!("已连接到用户数据流，等待账户和订单更新...");

    let mut message_count = 0;
    const MAX_USER_MESSAGES: usize = 10;

    while let Some(msg) = ws.next().await {
        match msg {
            Ok(tokio_tungstenite::tungstenite::Message::Text(text)) => {
                match WebSocketClient::parse_message(&text) {
                    Ok(ws_msg) => {
                        handle_user_message(ws_msg);
                        message_count += 1;
                        if message_count >= MAX_USER_MESSAGES {
                            println!("已接收 {} 条用户消息，停止演示", MAX_USER_MESSAGES);
                            break;
                        }
                    }
                    Err(e) => eprintln!("解析用户消息错误: {}", e),
                }
            }
            Ok(tokio_tungstenite::tungstenite::Message::Close(_)) => {
                println!("用户数据流连接已关闭");
                break;
            }
            Err(e) => {
                eprintln!("用户数据流错误: {}", e);
                break;
            }
            _ => {}
        }

        // 演示期间，如果没有消息，等待5秒后退出
        tokio::select! {
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(5)) => {
                println!("5秒内未收到用户数据，结束演示");
                break;
            }
        }
    }

    // 停止用户数据流
    if let Err(e) = user_stream.stop().await {
        eprintln!("停止用户数据流时出错: {}", e);
    } else {
        println!("✅ 用户数据流已停止");
    }

    Ok(())
}

/// 处理用户数据消息
fn handle_user_message(message: WebSocketMessage) {
    match message {
        WebSocketMessage::AccountUpdate(account) => {
            println!("👤 账户更新 - 原因: {}", account.account_update.event_reason);
            for balance in &account.account_update.balances {
                if balance.balance_change != "0" {
                    println!(
                        "   余额变化: {} = {}, 变化: {}",
                        balance.asset, balance.wallet_balance, balance.balance_change
                    );
                }
            }
            for position in &account.account_update.positions {
                if position.position_amount != "0" {
                    println!(
                        "   持仓: {} = {}, 未实现盈亏: {}",
                        position.symbol, position.position_amount, position.unrealized_pnl
                    );
                }
            }
        }
        WebSocketMessage::OrderUpdate(order) => {
            let o = &order.order;
            println!(
                "📋 订单更新 - {}: {} {:?} {}, 状态: {:?}",
                o.symbol, o.side, o.order_type, o.original_quantity, o.order_status
            );
            if o.last_filled_quantity != "0" {
                println!(
                    "   成交: 价格={}, 数量={}, 手续费={}",
                    o.last_filled_price, o.last_filled_quantity, o.commission_amount
                );
            }
        }
        _ => {}
    }
}
