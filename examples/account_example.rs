//! Account information example
//! 
//! This example demonstrates how to fetch account information from Binance Futures API.
//! Requires valid API credentials.

use binance_fu_rs::{BinanceClient, Credentials};
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
    let account = client.account();

    println!("=== Binance Futures Account Example ===\n");
    println!("⚠️  Using TESTNET - No real money involved\n");

    // Get account information
    println!("1. Getting account information...");
    match account.account_info().await {
        Ok(account_info) => {
            println!("✓ Account Info:");
            println!("  - Fee Tier: {}", account_info.fee_tier);
            println!("  - Can Trade: {}", account_info.can_trade);
            println!("  - Can Deposit: {}", account_info.can_deposit);
            println!("  - Can Withdraw: {}", account_info.can_withdraw);
            println!("  - Total Wallet Balance: {}", account_info.total_wallet_balance);
            println!("  - Total Unrealized PnL: {}", account_info.total_unrealized_pnl);
            println!("  - Total Margin Balance: {}", account_info.total_margin_balance);
            println!("  - Available Balance: {}\n", account_info.available_balance);

            // Show asset balances
            if !account_info.assets.is_empty() {
                println!("  Asset Balances:");
                for asset in account_info.assets.iter().take(5) {
                    if asset.wallet_balance != "0" {
                        println!("    - {}: {} (Available: {})", 
                            asset.asset, asset.wallet_balance, asset.available_balance);
                    }
                }
                println!();
            }

            // Show positions
            if !account_info.positions.is_empty() {
                println!("  Open Positions:");
                for position in account_info.positions.iter() {
                    if position.position_amt != "0" {
                        println!("    - {}: {} (Entry: {}, Unrealized PnL: {})", 
                            position.symbol, position.position_amt, 
                            position.entry_price, position.unrealized_pnl);
                    }
                }
                println!();
            }
        }
        Err(e) => {
            println!("❌ Failed to get account info: {}", e);
        }
    }

    // Get balance information
    println!("2. Getting balance information...");
    match account.balance().await {
        Ok(balances) => {
            println!("✓ Account Balances:");
            for balance in balances.iter() {
                if balance.balance != "0" {
                    println!("  - {}: {} (Available: {})", 
                        balance.asset, balance.balance, balance.available_balance);
                }
            }
            println!();
        }
        Err(e) => {
            println!("❌ Failed to get balances: {}", e);
        }
    }

    // Get position risk
    println!("3. Getting position risk for BTCUSDT...");
    match account.position_risk(Some("BTCUSDT")).await {
        Ok(positions) => {
            println!("✓ Position Risk:");
            for position in positions.iter() {
                println!("  - Symbol: {}", position.symbol);
                println!("  - Position Amount: {}", position.position_amt);
                println!("  - Entry Price: {}", position.entry_price);
                println!("  - Mark Price: {}", position.mark_price);
                println!("  - Unrealized PnL: {}", position.un_realized_pnl);
                println!("  - Liquidation Price: {}", position.liquidation_price);
                println!("  - Leverage: {}", position.leverage);
                println!("  - Margin Type: {}", position.margin_type);
                println!("  - Position Side: {:?}\n", position.position_side);
            }
        }
        Err(e) => {
            println!("❌ Failed to get position risk: {}", e);
        }
    }

    // Get income history
    println!("4. Getting recent income history...");
    match account.income_history(None, None, None, None, Some(5)).await {
        Ok(income_history) => {
            println!("✓ Recent Income History:");
            for (i, income) in income_history.iter().take(5).enumerate() {
                println!("  {}. {} - {} {} ({})", 
                    i + 1, income.symbol, income.income, income.asset, income.income_type);
            }
            println!();
        }
        Err(e) => {
            println!("❌ Failed to get income history: {}", e);
        }
    }

    // Get commission rate
    println!("5. Getting commission rate for BTCUSDT...");
    match account.commission_rate("BTCUSDT").await {
        Ok(commission) => {
            println!("✓ Commission Rate for {}:", commission.symbol);
            println!("  - Maker: {}", commission.maker_commission_rate);
            println!("  - Taker: {}\n", commission.taker_commission_rate);
        }
        Err(e) => {
            println!("❌ Failed to get commission rate: {}", e);
        }
    }

    println!("=== Account Example Completed ===");
    println!("💡 Tip: Set environment variables BINANCE_API_KEY and BINANCE_SECRET_KEY");
    println!("💡 Use testnet credentials to avoid real account data");
    
    Ok(())
}
