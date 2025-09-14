use serde::Deserialize;
use crate::types::common::PositionSide;

/// Account information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub fee_tier: i32,
    pub can_trade: bool,
    pub can_deposit: bool,
    pub can_withdraw: bool,
    pub update_time: u64,
    pub total_initial_margin: String,
    pub total_maint_margin: String,
    pub total_wallet_balance: String,
    pub total_unrealized_pnl: String,
    pub total_margin_balance: String,
    pub total_position_initial_margin: String,
    pub total_open_order_initial_margin: String,
    pub total_cross_wallet_balance: String,
    pub total_cross_un_pnl: String,
    pub available_balance: String,
    pub max_withdraw_amount: String,
    pub assets: Vec<AssetBalance>,
    pub positions: Vec<Position>,
}

/// Asset balance
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetBalance {
    pub asset: String,
    pub wallet_balance: String,
    pub unrealized_pnl: String,
    pub margin_balance: String,
    pub maint_margin: String,
    pub initial_margin: String,
    pub position_initial_margin: String,
    pub open_order_initial_margin: String,
    pub cross_wallet_balance: String,
    pub cross_un_pnl: String,
    pub available_balance: String,
    pub max_withdraw_amount: String,
    pub margin_available: bool,
    pub update_time: u64,
}

/// Position information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub symbol: String,
    pub initial_margin: String,
    pub maint_margin: String,
    pub unrealized_pnl: String,
    pub position_initial_margin: String,
    pub open_order_initial_margin: String,
    pub leverage: String,
    pub isolated: bool,
    pub entry_price: String,
    pub max_notional: String,
    pub position_side: PositionSide,
    pub position_amt: String,
    pub notional: String,
    pub isolated_wallet: String,
    pub update_time: u64,
    pub bid_notional: String,
    pub ask_notional: String,
}

/// Balance information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub account_alias: String,
    pub asset: String,
    pub balance: String,
    pub cross_wallet_balance: String,
    pub cross_un_pnl: String,
    pub available_balance: String,
    pub max_withdraw_amount: String,
    pub margin_available: bool,
    pub update_time: u64,
}

/// Position risk
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionRisk {
    pub symbol: String,
    pub position_amt: String,
    pub entry_price: String,
    pub mark_price: String,
    pub un_realized_pnl: String,
    pub liquidation_price: String,
    pub leverage: String,
    pub max_notional_value: String,
    pub margin_type: String,
    pub isolated_margin: String,
    pub is_auto_add_margin: bool,
    pub position_side: PositionSide,
    pub notional: String,
    pub isolated_wallet: String,
    pub update_time: u64,
    pub bid_notional: String,
    pub ask_notional: String,
}

/// Income history
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Income {
    pub symbol: String,
    pub income_type: String,
    pub income: String,
    pub asset: String,
    pub info: String,
    pub time: u64,
    pub tran_id: u64,
    pub trade_id: String,
}

/// Leverage bracket
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageBracket {
    pub symbol: String,
    pub brackets: Vec<Bracket>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bracket {
    pub bracket: i32,
    pub initial_leverage: i32,
    pub notional_cap: i64,
    pub notional_floor: i64,
    pub maint_margin_ratio: f64,
    pub cum: f64,
}

/// ADL quantile
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdlQuantile {
    pub symbol: String,
    pub adl_quantile: AdlQuantileInfo,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct AdlQuantileInfo {
    pub long: i32,
    pub short: i32,
    pub hedge: i32,
}

/// Force orders
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForceOrder {
    pub symbol: String,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub avg_price: String,
    pub status: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub order_type: String,
    pub side: String,
    pub time: u64,
}

/// Commission rate
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommissionRate {
    pub symbol: String,
    pub maker_commission_rate: String,
    pub taker_commission_rate: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_info_deserialization() {
        let json = r#"
        {
            "feeTier": 0,
            "canTrade": true,
            "canDeposit": true,
            "canWithdraw": true,
            "updateTime": 1640995200000,
            "totalInitialMargin": "1000.0",
            "totalMaintMargin": "500.0",
            "totalWalletBalance": "10000.0",
            "totalUnrealizedPnl": "100.0",
            "totalMarginBalance": "10100.0",
            "totalPositionInitialMargin": "800.0",
            "totalOpenOrderInitialMargin": "200.0",
            "totalCrossWalletBalance": "9000.0",
            "totalCrossUnPnl": "50.0",
            "availableBalance": "8000.0",
            "maxWithdrawAmount": "7500.0",
            "assets": [],
            "positions": []
        }
        "#;

        let account_info: AccountInfo = serde_json::from_str(json).unwrap();
        assert_eq!(account_info.fee_tier, 0);
        assert!(account_info.can_trade);
        assert_eq!(account_info.total_wallet_balance, "10000.0");
    }
}
