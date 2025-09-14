# Binance 合约 API Rust 实现

## 项目概述
基于Binance官方合约API文档，开发一个专注于合约交易的Rust SDK。

## API 基础信息
- **基础端点**: `https://fapi.binance.com`
- **认证方式**: HMAC-SHA256 或 RSA 签名
- **数据格式**: JSON
- **时间戳**: 毫秒级Unix时间戳
- **限制**: IP限制和订单频率限制

## 核心功能模块

### 1. 核心客户端 (Core Client)
- HTTP客户端封装 (基于reqwest)
- 认证和签名机制 (HMAC-SHA256/RSA)
- 请求限制和重试逻辑
- 错误处理和类型定义

### 2. 市场数据 API (Market Data)
- **订单簿**: `/fapi/v1/depth`
- **最新价格**: `/fapi/v1/ticker/price`
- **24小时统计**: `/fapi/v1/ticker/24hr`
- **K线数据**: `/fapi/v1/klines`
- **最新成交**: `/fapi/v1/trades`
- **历史成交**: `/fapi/v1/historicalTrades`
- **聚合成交**: `/fapi/v1/aggTrades`

### 3. 交易 API (Trading)
- **下单**: `POST /fapi/v1/order`
- **撤单**: `DELETE /fapi/v1/order`
- **查询订单**: `GET /fapi/v1/order`
- **查询所有订单**: `GET /fapi/v1/allOrders`
- **批量下单**: `POST /fapi/v1/batchOrders`
- **批量撤单**: `DELETE /fapi/v1/batchOrders`

### 4. 账户信息 API (Account)
- **账户信息**: `GET /fapi/v2/account`
- **持仓信息**: `GET /fapi/v2/positionRisk`
- **余额信息**: `GET /fapi/v2/balance`
- **交易历史**: `GET /fapi/v1/userTrades`
- **资金费率历史**: `GET /fapi/v1/fundingRate`

### 5. WebSocket 数据流
- **订单簿推送**: `<symbol>@depth`
- **成交推送**: `<symbol>@trade`
- **K线推送**: `<symbol>@kline_<interval>`
- **24小时统计推送**: `<symbol>@ticker`
- **用户数据流**: 订单更新、账户更新、持仓更新

## 技术架构

### 依赖库选择
```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
hmac = "0.12"
sha2 = "0.10"
hex = "0.4"
url = "2.4"
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1.0"
tokio-tungstenite = "0.20"
futures-util = "0.3"
```

### 项目结构
```
src/
├── lib.rs              # 库入口
├── client/             # HTTP客户端
│   ├── mod.rs
│   ├── http.rs         # HTTP客户端实现
│   └── auth.rs         # 认证和签名
├── types/              # 数据类型定义
│   ├── mod.rs
│   ├── market.rs       # 市场数据类型
│   ├── trading.rs      # 交易相关类型
│   ├── account.rs      # 账户相关类型
│   └── common.rs       # 通用类型
├── api/                # API端点实现
│   ├── mod.rs
│   ├── market.rs       # 市场数据API
│   ├── trading.rs      # 交易API
│   └── account.rs      # 账户API
├── websocket/          # WebSocket实现
│   ├── mod.rs
│   ├── stream.rs       # 数据流处理
│   └── types.rs        # WebSocket消息类型
├── error.rs            # 错误定义
└── utils.rs            # 工具函数
```

## 开发优先级

### 第一阶段 (核心功能)
1. 实现HTTP客户端和认证机制
2. 实现基础市场数据API
3. 实现核心交易API (下单、撤单、查询)

### 第二阶段 (完善功能)
4. 实现账户信息API
5. 添加WebSocket数据流支持
6. 完善错误处理和重试机制

### 第三阶段 (优化和测试)
7. 性能优化和连接池
8. 完整的单元测试和集成测试
9. 文档和使用示例

## 参考资源
- [Binance合约API官方文档](https://developers.binance.com/docs/derivatives/usds-margined-futures)
- [官方Rust连接器](https://github.com/binance/binance-connector-rust)
- [社区Rust库参考](https://github.com/wisespace-io/binance-rs)
