use serde::{de, Deserialize, Deserializer};
use serde_json::Value;

pub mod candle;
pub use candle::Candle;

pub mod instrument;
pub use instrument::Instrument;

pub(crate) fn de_decimal_number<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(match Value::deserialize(deserializer)? {
        Value::Number(num) => num.as_f64().ok_or(de::Error::custom("invalid number"))?,
        Value::String(s) => s.parse().map_err(de::Error::custom)?,
        _ => return Err(de::Error::custom("wrong type")),
    })
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::oanda::entity::de_decimal_number;

    #[derive(Deserialize)]
    struct TestStruct {
        #[serde(deserialize_with = "de_decimal_number")]
        decimal_number: f64,
    }

    #[test]
    fn de_decimal_number_success_float() {
        let data = r#"
            {
                "decimal_number": 1.23
            }
        "#;

        let got = serde_json::from_str::<TestStruct>(data).unwrap();
        assert_eq!(1.23, got.decimal_number)
    }

    #[test]
    fn de_decimal_number_success_int() {
        let data = r#"
            {
                "decimal_number": 23
            }
        "#;

        let got = serde_json::from_str::<TestStruct>(data).unwrap();
        assert_eq!(23.0, got.decimal_number)
    }

    #[test]
    fn de_decimal_number_success_float_string() {
        let data = r#"
            {
                "decimal_number": "1.23"
            }
        "#;

        let got = serde_json::from_str::<TestStruct>(data).unwrap();
        assert_eq!(1.23, got.decimal_number)
    }

    #[test]
    fn de_decimal_number_success_int_string() {
        let data = r#"
            {
                "decimal_number": "23"
            }
        "#;

        let got = serde_json::from_str::<TestStruct>(data).unwrap();
        assert_eq!(23.0, got.decimal_number)
    }

    #[test]
    fn de_decimal_number_err_non_number_string() {
        let data = r#"
            {
                "decimal_number": "twenty three"
            }
        "#;

        let got = serde_json::from_str::<TestStruct>(data);
        assert!(got.is_err())
    }

    #[test]
    fn de_decimal_number_err_invalid_type() {
        let data = r#"
            {
                "decimal_number": bool
            }
        "#;

        let got = serde_json::from_str::<TestStruct>(data);
        assert!(got.is_err())
    }
}
