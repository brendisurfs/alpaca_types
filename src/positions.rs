use crate::{f64_from_opt_string, serialize_qty};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::orders::AssetClass;
// TODO: Finish this
/// OpenPosition
/// Describes the data layout of an open position on alpaca.
#[derive(Debug, Deserialize, Serialize)]
pub struct OpenPosition {
    pub asset_id: Uuid,
    pub symbol: String,
    pub exchange: String,
    pub asset_class: AssetClass,
    /// the average price this order filled at.
    #[serde(deserialize_with = "f64_from_opt_string")]
    pub filled_avg_price: Option<f64>,

    #[serde(serialize_with = "serialize_qty")]
    pub qty: f64,
    pub qty_available: String,
    pub side: String,
    pub market_value: String,
    pub cost_basis: String,
    pub unrealized_pl: String,
    pub unrealized_plpc: String,
    pub unrealized_intraday_pl: String,
    pub unrealized_intraday_plpc: String,
    pub current_price: String,
    pub lastday_price: String,
    pub change_today: String,
    pub asset_marginable: bool,
}
