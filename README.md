# Binance Futures Rust SDK

一个专为Binance合约交易设计的Rust客户端库，提供完整的REST API和WebSocket实时数据流支持。

## 主要特性

- **完整的API覆盖**: 支持所有Binance期货官方API端点
- **市场数据**: 实时价格、深度、K线图、24小时统计等
- **交易功能**: 下单、撤单、查询订单、持仓管理
- **WebSocket流**: 实时市场数据和用户数据推送
- **账户管理**: 访问账户信息、余额、持仓和交易历史
- **安全认证**: 使用HMAC-SHA256签名的安全认证
- **测试网支持**: 支持Binance测试网环境
- **错误处理**: 详细的错误类型和错误信息
- **类型安全**: 强类型API，支持serde序列化/反序列化

## 安装

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
binance-futures-rs = "0.1.1"
tokio = { version = "1.0", features = ["full"] }
```

## 快速开始

### 市场数据（无需认证）

```rust
use binance_futures_rs::{BinanceClient, KlineInterval};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端（公共端点无需认证）
    let client = BinanceClient::new();
    
    // 获取当前比特币价格
    let price = client.market().price_ticker(Some("BTCUSDT")).await?;
    println!("BTC价格: {}", price[0].price);
    
    // 获取订单簿深度
    let depth = client.market().depth("BTCUSDT", Some(10)).await?;
    println!("最佳买价: {}", depth.bids[0][0]);
    println!("最佳卖价: {}", depth.asks[0][0]);
    
    // 获取K线数据
    let klines = client.market().klines(
        "BTCUSDT", 
        KlineInterval::OneHour, 
        None, None, Some(10)
    ).await?;
    
    Ok(())
}
```

### 交易功能（需要认证）

```rust
use binance_futures_rs::{
    BinanceClient, Credentials, NewOrderRequest,
    OrderSide, OrderType, TimeInForce
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 设置API凭证
    let credentials = Credentials::new(
        "your_api_key".to_string(),
        "your_secret_key".to_string(),
    );
    
    // 创建认证客户端（建议使用测试网）
    let client = BinanceClient::testnet_with_credentials(credentials);
    
    // 下限价买单
    let order = NewOrderRequest::new(
        "BTCUSDT".to_string(),
        OrderSide::Buy,
        OrderType::Limit,
    )
    .quantity("0.001".to_string())
    .price("30000.0".to_string())
    .time_in_force(TimeInForce::Gtc);
    
    let result = client.trading().new_order(order).await?;
    println!("订单ID: {}", result.order_id);
    
    Ok(())
}
```

### 账户信息

```rust
use binance_futures_rs::{BinanceClient, Credentials};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = Credentials::new(
        "your_api_key".to_string(),
        "your_secret_key".to_string(),
    );
    
    let client = BinanceClient::testnet_with_credentials(credentials);
    
    // 获取账户信息
    let account = client.account().account_info().await?;
    println!("总钱包余额: {}", account.total_wallet_balance);
    println!("可用余额: {}", account.available_balance);
    
    // 获取持仓信息
    let positions = client.account().position_risk(None).await?;
    for position in positions {
        if position.position_amt != "0" {
            println!("持仓: {} {}", position.symbol, position.position_amt);
        }
    }
    
    Ok(())
}
```

## API 覆盖

### 市场数据 API
- ✅ 订单簿深度 (`/fapi/v1/depth`)
- ✅ 最新价格 (`/fapi/v1/ticker/price`)
- ✅ 24小时统计 (`/fapi/v1/ticker/24hr`)
- ✅ K线数据 (`/fapi/v1/klines`)
- ✅ 最新成交 (`/fapi/v1/trades`)
- ✅ 历史成交 (`/fapi/v1/historicalTrades`)
- ✅ 聚合成交 (`/fapi/v1/aggTrades`)
- ✅ 标记价格 (`/fapi/v1/premiumIndex`)
- ✅ 交易所信息 (`/fapi/v1/exchangeInfo`)

### 交易 API
- ✅ 下单 (`POST /fapi/v1/order`)
- ✅ 撤单 (`DELETE /fapi/v1/order`)
- ✅ 查询订单 (`GET /fapi/v1/order`)
- ✅ 查询所有订单 (`GET /fapi/v1/allOrders`)
- ✅ 查询当前挂单 (`GET /fapi/v1/openOrders`)
- ✅ 批量下单 (`POST /fapi/v1/batchOrders`)
- ✅ 用户交易记录 (`GET /fapi/v1/userTrades`)

### 账户 API
- ✅ 账户信息 (`GET /fapi/v2/account`)
- ✅ 余额信息 (`GET /fapi/v2/balance`)
- ✅ 持仓风险 (`GET /fapi/v2/positionRisk`)
- ✅ 收益历史 (`GET /fapi/v1/income`)
- ✅ 杠杆分层标准 (`GET /fapi/v1/leverageBracket`)
- ✅ 持仓ADL队列估算 (`GET /fapi/v1/adlQuantile`)
- ✅ 用户强平单 (`GET /fapi/v1/forceOrders`)
- ✅ 手续费率 (`GET /fapi/v1/commissionRate`)

## 示例程序

运行示例程序：

```bash
# 市场数据示例（无需API密钥）
cargo run --example market_data

# 交易示例（需要设置环境变量）
export BINANCE_API_KEY="your_testnet_api_key"
export BINANCE_SECRET_KEY="your_testnet_secret_key"
cargo run --example trading_example

# 账户信息示例
cargo run --example account_example
```

## 测试网使用

强烈建议在测试网上进行开发和测试：

1. 访问 [Binance测试网](https://testnet.binancefuture.com/)
2. 创建测试账户并获取API密钥
3. 使用 `BinanceClient::testnet_with_credentials()` 创建客户端

```rust
let client = BinanceClient::testnet_with_credentials(credentials);
```

## 错误处理

库提供了详细的错误类型：

```rust
use binance_futures_rs::{BinanceError, Result};

match client.market().price_ticker(Some("BTCUSDT")).await {
    Ok(price) => println!("价格: {}", price[0].price),
    Err(BinanceError::Api { code, msg }) => {
        println!("API错误 {}: {}", code, msg);
    }
    Err(BinanceError::Http(e)) => {
        println!("网络错误: {}", e);
    }
    Err(e) => {
        println!("其他错误: {}", e);
    }
}
```

## 安全注意事项

1. **永远不要**在代码中硬编码API密钥
2. 使用环境变量存储敏感信息
3. 在生产环境中使用IP白名单
4. 定期轮换API密钥
5. 为不同用途创建不同权限的API密钥

## 开发状态

- ✅ 核心HTTP客户端和认证
- ✅ 市场数据API
- ✅ 交易API
- ✅ 账户API
- ⏳ WebSocket数据流（计划中）
- ⏳ 高级订单类型
- ⏳ 更多错误处理和重试机制

## 贡献

欢迎提交Issue和Pull Request！

## 许可证

MIT License

## 免责声明

此库仅供教育和开发目的。使用此库进行实际交易时，请自行承担风险。作者不对任何交易损失负责。
