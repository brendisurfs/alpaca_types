use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[cfg(feature = "ts")]
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[cfg_attr(feature = "ts", derive(TS), ts(export))]
pub struct Trade {
    #[serde(rename = "c")]
    pub condition_flags: Vec<String>,
    #[serde(rename = "i")]
    pub trade_id: i64,
    #[serde(rename = "p")]
    pub price: f64,
    #[serde(rename = "s")]
    pub trade_size: u32,
    #[serde(rename = "t")]
    pub timestamp: String,
    #[serde(rename = "x")]
    pub exchange_code: String,
    #[serde(rename = "z")]
    pub tape: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[cfg_attr(feature = "ts", derive(TS), ts(export))]
pub struct LatestTrades {
    pub trades: HashMap<String, Trade>,
}

#[cfg(test)]
mod test {
    use crate::market_data::stock::LatestTrades;

    #[test]
    fn latest_trade_parses() {
        let latest_trade = r#"
        {
            "trades": {
                "AAPL": {
                    "t": "2022-08-17T09:50:43.361102308Z",
                    "x": "Q",
                    "p": 172.78,
                    "s": 100,
                    "c": [
                        "@",
                        "F",
                        "T"
                    ],
                    "i": 826,
                    "z": "C"
                }
            }
        }
        "#;

        let got = serde_json::from_str::<LatestTrades>(latest_trade);
        assert!(got.is_ok());
    }
}
