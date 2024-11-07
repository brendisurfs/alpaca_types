pub mod errors;
pub mod market_data;
pub mod orders;
pub mod positions;
pub mod trading;

use serde::{de, Deserialize, Deserializer, Serializer};

/// custom deserialization for converting Option<String> to Option<f64> where needed.
fn f64_from_opt_string<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = match Option::deserialize(deserializer) {
        Err(why) => {
            return Err(de::Error::custom(format!(
                "Error formatting Option<String>: {why}"
            )))
        }
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
/// converts a string to f64
fn string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let de_string: String = match String::deserialize(deserializer) {
        Ok(de_string) => de_string,
        Err(why) => return Err(de::Error::custom(format!("{why}"))),
    };

    match de_string.parse::<f64>() {
        Err(why) => Err(de::Error::custom(format!("{why}"))),
        Ok(num) => Ok(num),
    }
}

/// converts a string to f64 with  a default to 0.0.
fn empty_field_is_zero<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let de_str = String::deserialize(deserializer)?;
    if de_str.is_empty() {
        Ok(0.0)
    } else {
        Ok(de_str.parse::<f64>().unwrap_or(0.0))
    }
}
/// custom serializer to convert quantity string to an f64.
/// * `qty`: float amount to parse
/// * `serializer`: S
fn serialize_qty<S>(qty: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let qty_str = qty.to_string();
    serializer.serialize_str(&qty_str)
}
