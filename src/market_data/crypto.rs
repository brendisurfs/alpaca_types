use std::collections::HashMap;

use serde::Deserialize;
use time::OffsetDateTime;

/// The latest minute-aggregated historical bar data for each of the crypto symbols provided.
#[derive(Debug, Deserialize)]
pub struct LatestCryptoBars {
    pub bars: HashMap<String, CryptoBar>,
}

/// CryptoBar represents OHLC candlestick data (minute or daily)
/// The differen between Bar types is the volume, as cryptocurrencies are usually traded in fractional amounts.
#[derive(Debug, Deserialize, Clone)]
pub struct CryptoBar {
    #[serde(rename = "t", with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,

    #[serde(rename = "o")]
    pub open: f64,

    #[serde(rename = "h")]
    pub high: f64,

    #[serde(rename = "l")]
    pub low: f64,

    #[serde(rename = "c")]
    pub close: f64,

    #[serde(rename = "v")]
    pub volume: f64,

    #[serde(rename = "n")]
    pub trade_count: i64,

    #[serde(rename = "vw")]
    pub vwap: f64,
}

#[cfg(test)]
mod tests {
    use super::LatestCryptoBars;

    #[test]
    fn crypto_bar_parses() {
        let str_bars = r#"
            {
              "bars": {
                "BTC/USD": {
                  "t": "2022-05-27T10:18:00Z",
                  "o": 28999,
                  "h": 29003,
                  "l": 28999,
                  "c": 29003,
                  "v": 0.01,
                  "n": 4,
                  "vw": 29001
                }
              }
            }
            "#;

        assert!(serde_json::from_str::<LatestCryptoBars>(str_bars).is_ok());
    }
}
