use serde::{de, Deserialize, Deserializer, Serializer};

pub mod orders;
pub mod positions;

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

/// custom serializer to convert quantity string to usize.
/// * `qty`: float amount to parse
/// * `serializer`: S
fn serialize_qty<S>(qty: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let qty_str = qty.to_string();
    serializer.serialize_str(&qty_str)
}
