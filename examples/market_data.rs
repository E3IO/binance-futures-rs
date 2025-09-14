//! Market data example
//! 
//! This example demonstrates how to fetch market data from Binance Futures API.
//! No authentication is required for these endpoints.

use binance_fu_rs::{BinanceClient, KlineInterval};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new client (no authentication required for market data)
    let client = BinanceClient::testnet();
    let market = client.market();

    println!("=== Binance Futures Market Data Example ===\n");

    // Test connectivity
    println!("1. Testing connectivity...");
    let ping = market.ping().await?;
    println!("✓ Ping successful: {:?}\n", ping);

    // Get server time
    println!("2. Getting server time...");
    let time = market.time().await?;
    println!("✓ Server time: {:?}\n", time);

    // Get exchange information
    println!("3. Getting exchange information...");
    let exchange_info = market.exchange_info().await?;
    println!("✓ Exchange timezone: {}", exchange_info.timezone);
    println!("✓ Number of symbols: {}\n", exchange_info.symbols.len());

    // Get current price for BTCUSDT
    println!("4. Getting BTCUSDT price...");
    let price = market.price_ticker(Some("BTCUSDT")).await?;
    if !price.is_empty() {
        println!("✓ BTC Price: {} USDT\n", price[0].price);
    }

    // Get 24hr ticker statistics
    println!("5. Getting 24hr ticker for BTCUSDT...");
    let ticker = market.ticker_24hr(Some("BTCUSDT")).await?;
    if !ticker.is_empty() {
        let t = &ticker[0];
        println!("✓ 24hr Stats:");
        println!("  - Open: {} USDT", t.open_price);
        println!("  - High: {} USDT", t.high_price);
        println!("  - Low: {} USDT", t.low_price);
        println!("  - Last: {} USDT", t.last_price);
        println!("  - Change: {}%\n", t.price_change_percent);
    }

    // Get order book depth
    println!("6. Getting order book depth for BTCUSDT...");
    let depth = market.depth("BTCUSDT", Some(5)).await?;
    println!("✓ Order Book (top 5):");
    println!("  Asks:");
    for ask in depth.asks.iter().take(3) {
        println!("    {} @ {}", ask[1], ask[0]);
    }
    println!("  Bids:");
    for bid in depth.bids.iter().take(3) {
        println!("    {} @ {}", bid[1], bid[0]);
    }
    println!();

    // Get recent trades
    println!("7. Getting recent trades for BTCUSDT...");
    let trades = market.trades("BTCUSDT", Some(5)).await?;
    println!("✓ Recent trades:");
    for trade in trades.iter().take(3) {
        println!("  {} BTCUSDT @ {} ({})", 
            trade.qty, trade.price, 
            if trade.is_buyer_maker { "SELL" } else { "BUY" }
        );
    }
    println!();

    // Get kline data
    println!("8. Getting 1-hour klines for BTCUSDT...");
    let klines = market.klines("BTCUSDT", KlineInterval::OneHour, None, None, Some(5)).await?;
    println!("✓ Recent 1h candles:");
    for kline in klines.iter().take(3) {
        println!("  O:{} H:{} L:{} C:{} V:{}", 
            kline.open, kline.high, kline.low, kline.close, kline.volume
        );
    }
    println!();

    // Get mark price
    println!("9. Getting mark price for BTCUSDT...");
    let mark_price = market.mark_price(Some("BTCUSDT")).await?;
    if !mark_price.is_empty() {
        let mp = &mark_price[0];
        println!("✓ Mark Price Info:");
        println!("  - Mark Price: {}", mp.mark_price);
        println!("  - Index Price: {}", mp.index_price);
        println!("  - Funding Rate: {}", mp.last_funding_rate);
    }

    println!("\n=== Market Data Example Completed Successfully! ===");
    Ok(())
}
