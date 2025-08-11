use serde::{de, Deserialize, Deserializer};
use serde_json::Value;

pub(crate) fn de_string_as_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().map_err(de::Error::custom)?,
        _ => return Err(de::Error::custom("error deserializing string as f64")),
    })
}
