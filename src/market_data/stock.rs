use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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
