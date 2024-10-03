use crate::orders::AssetClass;
use crate::{f64_from_opt_string, serialize_qty, string_to_f64};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ts")]
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "ts", derive(TS), ts(export))]
pub enum PositionSide {
    Long,
    Short,
}

// TODO: Finish this
/// OpenPosition
/// Describes the data layout of an open position on alpaca.
#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "ts", derive(TS), ts(export))]
pub struct OpenPosition {
    pub asset_id: Uuid,
    pub symbol: String,
    pub exchange: String,
    pub asset_class: AssetClass,
    #[serde(deserialize_with = "f64_from_opt_string")]
    pub avg_entry_price: Option<f64>,
    #[serde(serialize_with = "serialize_qty", deserialize_with = "string_to_f64")]
    pub qty: f64,
    #[serde(serialize_with = "serialize_qty", deserialize_with = "string_to_f64")]
    pub qty_available: f64,
    pub side: PositionSide,
    #[serde(deserialize_with = "string_to_f64")]
    pub market_value: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub cost_basis: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub unrealized_pl: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub unrealized_plpc: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub unrealized_intraday_pl: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub unrealized_intraday_plpc: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub current_price: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub lastday_price: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub change_today: f64,
    pub asset_marginable: bool,
}

#[cfg(test)]
mod test {
    use crate::positions::OpenPosition;

    #[test]
    fn position_parses() {
        let input = r#"
            {
                "asset_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                "symbol": "AAPL",
                "exchange": "NYSE",
                "asset_class": "us_equity",
                "avg_entry_price": "12.5",
                "qty": "105.0",
                "qty_available": "0.0",
                "side": "long",
                "market_value": "1020.5",
                "cost_basis": "1005.0",
                "unrealized_pl": "15.0",
                "unrealized_plpc": "1.0",
                "unrealized_intraday_pl": "1.5",
                "unrealized_intraday_plpc": "0.4",
                "current_price": "12.0",
                "lastday_price": "11.23",
                "change_today": "0.5",
                "asset_marginable": true
            }
            "#;

        let parsed = serde_json::from_str::<OpenPosition>(input);
        assert!(parsed.is_ok());
    }
}
