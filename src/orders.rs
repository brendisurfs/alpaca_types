use std::fmt::Display;

use serde::de;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use time::serde::rfc3339;

use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
}

impl From<String> for OrderSide {
    fn from(value: String) -> Self {
        match value.as_str() {
            "buy" => OrderSide::Buy,
            "sell" => OrderSide::Sell,
            other => panic!("Unknown string {other}"),
        }
    }
}

impl Display for OrderSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderSide::Buy => write!(f, "buy"),
            OrderSide::Sell => write!(f, "sell"),
        }
    }
}

// TODO: Finish these
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Market,
}

impl From<String> for OrderType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "market" => OrderType::Market,
            other => panic!("Unknown string {other}"),
        }
    }
}

impl Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderType::Market => write!(f, "market"),
        }
    }
}

// TODO: Finish these
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimeInForce {
    Day,
}

impl Display for TimeInForce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeInForce::Day => write!(f, "day"),
        }
    }
}

/// # OrderRequest
/// the structure of data to send an order to Alpacas API.
#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub struct OrderRequest {
    pub symbol: String,

    // Want to serialize this to String!
    #[serde(serialize_with = "serialize_qty")]
    pub qty: f64,

    //  buy or sell
    pub side: OrderSide,

    /// this is ALWAYS market.
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// always "day"
    pub time_in_force: TimeInForce,
}

/// custom serializer to convert quantity string to usize.
///
/// * `qty`:
/// * `serializer`:
fn serialize_qty<S>(qty: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let qty_str = qty.to_string();
    serializer.serialize_str(&qty_str)
}

/// # AssetClass
/// Serves to identify the nature of the financial instrument,
/// with options including "us_equity" for U.S. equities,
/// "us_option" for U.S. options, and "crypto" for cryptocurrencies.
#[derive(Serialize, Deserialize)]
pub enum AssetClass {
    USEquity,
    USOption,
    Crypto,
}

impl From<String> for AssetClass {
    fn from(value: String) -> Self {
        match value.as_str() {
            "crypto" => AssetClass::Crypto,
            "us_equity" => AssetClass::USEquity,
            "us_option" => AssetClass::USOption,
            other => panic!("Unknown string in asset class: {other}"),
        }
    }
}

/// # OrderResponse
/// the structure of data in response to an order being sent.
#[derive(Serialize, Deserialize)]
pub struct OrderResponse {
    pub id: Uuid,
    pub client_order_id: String,

    #[serde(deserialize_with = "rfc3339::deserialize")]
    pub created_at: OffsetDateTime,

    #[serde(deserialize_with = "rfc3339::deserialize")]
    pub updated_at: OffsetDateTime,

    #[serde(deserialize_with = "rfc3339::deserialize")]
    pub submitted_at: OffsetDateTime,

    #[serde(deserialize_with = "rfc3339::option::deserialize")]
    pub filled_at: Option<OffsetDateTime>,

    #[serde(deserialize_with = "rfc3339::option::deserialize")]
    pub expired_at: Option<OffsetDateTime>,

    #[serde(deserialize_with = "rfc3339::option::deserialize")]
    pub canceled_at: Option<OffsetDateTime>,

    #[serde(deserialize_with = "rfc3339::option::deserialize")]
    pub failed_at: Option<OffsetDateTime>,

    #[serde(deserialize_with = "rfc3339::option::deserialize")]
    pub replaced_at: Option<OffsetDateTime>,

    /// The order ID that this order was replaced by
    pub replaced_by: Option<String>,

    /// The order ID that this order replaces
    pub replaces: Option<String>,

    /// Asset ID (For options this represents the option contract ID)
    pub asset_id: Uuid,

    ///Asset symbol
    pub symbol: String,

    pub asset_class: AssetClass,

    /// Ordered notional amount. If entered, qty will be null. Can take up to 9 decimal points.
    pub notional: Option<String>,

    /// Ordered quantity. If entered, notional will be null. Can take up to 9 decimal points.
    #[serde(deserialize_with = "f64_from_opt_string")]
    pub qty: Option<f64>,

    /// quantity filled in this order.
    pub filled_qty: String,

    /// the average price this order filled at.
    #[serde(deserialize_with = "f64_from_opt_string")]
    pub filled_avg_price: Option<f64>,

    /// simple bracket oco oto
    pub order_class: String,

    #[serde(rename = "type")]
    pub order_type: OrderType,

    #[serde(rename = "side")]
    pub order_side: OrderSide,

    pub time_in_force: TimeInForce,

    #[serde(deserialize_with = "f64_from_opt_string")]
    pub limit_price: Option<f64>,

    #[serde(deserialize_with = "f64_from_opt_string")]
    pub stop_price: Option<f64>,

    #[serde(rename = "status")]
    pub status: OrderStatus,

    pub extended_hours: bool,

    #[serde(flatten)]
    pub legs: serde_json::Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_percent: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_price: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwm: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtag: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<serde_json::Value>,
}

/// custom deserialization for converting Option<String> to Option<f64> where needed.
fn f64_from_opt_string<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = match Option::deserialize(deserializer) {
        Err(why) => return Err(de::Error::custom(format!("{why}"))),
        Ok(opt) => opt,
    };

    match opt {
        None => Ok(None),
        Some(s) => match s.parse::<f64>() {
            Ok(num) => Ok(Some(num)),
            Err(why) => Err(de::Error::custom(format!(
                "Failed to parse f64 from string: {why}"
            ))),
        },
    }
}

#[derive(Deserialize, Debug)]
pub struct OrderError {
    pub code: i64,
    pub message: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    DoneForDay,
    Canceled,
    Expired,
    Replaced,
    PendingCancel,
    PendingReplace,
    Accepted,
    PendingNew,
    AcceptedForBidding,
    Stopped,
    Rejected,
    Suspended,
    Calculated,
}

impl From<&str> for OrderStatus {
    fn from(status: &str) -> Self {
        match status {
            "new" => OrderStatus::New,
            "partially_filled" => OrderStatus::PartiallyFilled,
            "filled" => OrderStatus::Filled,
            "done_for_day" => OrderStatus::DoneForDay,
            "canceled" => OrderStatus::Canceled,
            "expired" => OrderStatus::Expired,
            "replaced" => OrderStatus::Replaced,
            "pending_cancel" => OrderStatus::PendingCancel,
            "pending_replace" => OrderStatus::PendingReplace,
            "accepted" => OrderStatus::Accepted,
            "pending_new" => OrderStatus::PendingNew,
            "accepted_for_bidding" => OrderStatus::AcceptedForBidding,
            "stopped" => OrderStatus::Stopped,
            "rejected" => OrderStatus::Rejected,
            "suspended" => OrderStatus::Suspended,
            "calculated" => OrderStatus::Calculated,
            _ => panic!("Invalid order status: {}", status),
        }
    }
}

impl Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::New => write!(f, "new"),
            OrderStatus::PartiallyFilled => write!(f, "partially_filled"),
            OrderStatus::Filled => write!(f, "filled"),
            OrderStatus::DoneForDay => write!(f, "done_for_day"),
            OrderStatus::Canceled => write!(f, "canceled"),
            OrderStatus::Expired => write!(f, "expired"),
            OrderStatus::Replaced => write!(f, "replaced"),
            OrderStatus::PendingCancel => write!(f, "pending_cancel"),
            OrderStatus::PendingReplace => write!(f, "pending_replace"),
            OrderStatus::Accepted => write!(f, "accepted"),
            OrderStatus::PendingNew => write!(f, "pending_new"),
            OrderStatus::AcceptedForBidding => write!(f, "accepted_for_bidding"),
            OrderStatus::Stopped => write!(f, "stopped"),
            OrderStatus::Rejected => write!(f, "rejected"),
            OrderStatus::Suspended => write!(f, "suspended"),
            OrderStatus::Calculated => write!(f, "calculated"),
        }
    }
}

/// The response we get when we close a position.
#[derive(Serialize, Deserialize)]
pub struct PositionClosed {
    pub symbol: String,
    pub status: String,
    pub body: OrderResponse,
}

#[cfg(test)]
mod tests {

    use crate::orders::{OrderResponse, OrderType, TimeInForce};

    use super::{OrderRequest, OrderSide};

    #[test]
    fn test_parsing_order() {
        let test_order = r#"
        {
            "id": "7b08df51-c1ac-453c-99f9-323a5f075f0d",
            "client_order_id": "5680c4bc-9ac1-4a12-a44c-df427ba53032",
            "created_at": "2023-12-12T22:31:24.668464435Z",
            "updated_at": "2023-12-12T22:31:24.668464435Z",
            "submitted_at": "2023-12-12T22:31:24.577215743Z",
            "filled_at": null,
            "expired_at": null,
            "canceled_at": null,
            "failed_at": null,
            "replaced_at": null,
            "replaced_by": null,
            "replaces": null,
            "asset_id": "b0b6dd9d-8b9b-48a9-ba46-b9d54906e415",
            "symbol": "AAPL",
            "asset_class": "us_equity",
            "notional": null,
            "qty": "2",
            "filled_qty": "0",
            "filled_avg_price": null,
            "order_class": "",
            "order_type": "limit",
            "type": "limit",
            "side": "buy",
            "time_in_force": "gtc",
            "limit_price": "150",
            "stop_price": null,
            "status": "accepted",
            "extended_hours": false,
            "legs": null,
            "trail_percent": null,
            "trail_price": null,
            "hwm": null,
            "subtag": null,
            "source": null
        }
        "#;

        let data = serde_json::from_str::<OrderResponse>(test_order);
        assert!(data.is_ok());
    }

    #[test]
    fn test_order_request() {
        let wanted =
            r#"{"symbol":"PTON","qty":"10","side":"buy","type":"market","time_in_force":"day"}"#;

        let new_order = OrderRequest {
            qty: 10.0,
            side: OrderSide::Buy,
            symbol: "PTON".to_string(),
            order_type: OrderType::Market,
            time_in_force: TimeInForce::Day,
        };
        let got = serde_json::to_string(&new_order).expect("failed to parse");
        println!("{got}");
        assert!(wanted == &got);
    }
}
