use serde::{de, Deserialize, Deserializer};
use serde_json::Value;

pub(crate) fn de_decimal<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().map_err(de::Error::custom)?,
        Value::Number(num) => num.as_f64().ok_or(de::Error::custom("invalid number"))? as f64,
        _ => return Err(de::Error::custom("wrong type")),
    })
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::de_decimal;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestDecimal {
        #[serde(deserialize_with = "de_decimal")]
        value: f64,
    }

    #[test]
    fn test_de_decimal_number_string() {
        let json = r#"{"value":"0.1234"}"#;

        let want = 0.1234;
        let got: TestDecimal = serde_json::from_str(json).unwrap();

        assert_eq!(want, got.value)
    }

    #[test]
    fn test_de_decimal_string() {
        let json = r#"{"value":"not a number"}"#;
        let got: serde_json::Result<TestDecimal> = serde_json::from_str(json);

        assert!(got.is_err())
    }

    #[test]
    fn test_de_decimal_number() {
        let json = r#"{"value":0.1234}"#;

        let want = 0.1234;
        let got: TestDecimal = serde_json::from_str(json).unwrap();

        assert_eq!(want, got.value)
    }

    #[test]
    fn test_de_decimal_other_type_error() {
        let json = r#"{"value":false}"#;
        let got: serde_json::Result<TestDecimal> = serde_json::from_str(json);

        assert!(got.is_err())
    }
}
