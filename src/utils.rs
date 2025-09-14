use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Get current timestamp in milliseconds
pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}

/// Convert timestamp to DateTime
pub fn timestamp_to_datetime(timestamp: u64) -> DateTime<Utc> {
    DateTime::from_timestamp(timestamp as i64 / 1000, ((timestamp % 1000) * 1_000_000) as u32)
        .unwrap_or_else(|| Utc::now())
}

/// Build query string from parameters
pub fn build_query_string(params: &[(&str, &str)]) -> String {
    params
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<_>>()
        .join("&")
}

/// Build query string from HashMap parameters
pub fn build_query_string_from_map(params: &HashMap<String, String>) -> String {
    params
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<_>>()
        .join("&")
}

/// Format decimal number to string with proper precision
pub fn format_decimal(value: f64, precision: usize) -> String {
    format!("{:.precision$}", value, precision = precision)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_timestamp() {
        let ts = get_timestamp();
        assert!(ts > 0);
    }

    #[test]
    fn test_build_query_string() {
        let params = [("symbol", "BTCUSDT"), ("limit", "100")];
        let query = build_query_string(&params);
        assert_eq!(query, "symbol=BTCUSDT&limit=100");
    }

    #[test]
    fn test_format_decimal() {
        assert_eq!(format_decimal(1.23456, 2), "1.23");
        assert_eq!(format_decimal(1.0, 4), "1.0000");
    }
}
