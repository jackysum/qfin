use std::fmt;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::de_decimal;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Candle {
    pub time: DateTime<Utc>,
    pub bid: Option<Data>,
    pub mid: Option<Data>,
    pub ask: Option<Data>,
    pub volume: i32,
    pub complete: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Data {
    #[serde(rename = "o", deserialize_with = "de_decimal")]
    pub open: f64,
    #[serde(rename = "h", deserialize_with = "de_decimal")]
    pub high: f64,
    #[serde(rename = "l", deserialize_with = "de_decimal")]
    pub low: f64,
    #[serde(rename = "c", deserialize_with = "de_decimal")]
    pub close: f64,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum Granularity {
    S5,
    S10,
    S15,
    S30,
    M1,
    M2,
    M4,
    M5,
    M10,
    M15,
    M30,
    H1,
    H2,
    H3,
    H4,
    H6,
    H8,
    H12,
    D,
    W,
    M,
}

impl fmt::Display for Granularity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum WeeklyAlignment {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl fmt::Display for WeeklyAlignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use crate::oanda::candle::{Granularity, WeeklyAlignment};

    #[test]
    fn test_granularity_to_string() {
        assert_eq!("S5", Granularity::S5.to_string());
        assert_eq!("S10", Granularity::S10.to_string());
        assert_eq!("S15", Granularity::S15.to_string());
        assert_eq!("S30", Granularity::S30.to_string());
        assert_eq!("M1", Granularity::M1.to_string());
        assert_eq!("M2", Granularity::M2.to_string());
        assert_eq!("M4", Granularity::M4.to_string());
        assert_eq!("M5", Granularity::M5.to_string());
        assert_eq!("M10", Granularity::M10.to_string());
        assert_eq!("M15", Granularity::M15.to_string());
        assert_eq!("M30", Granularity::M30.to_string());
        assert_eq!("H1", Granularity::H1.to_string());
        assert_eq!("H2", Granularity::H2.to_string());
        assert_eq!("H3", Granularity::H3.to_string());
        assert_eq!("H4", Granularity::H4.to_string());
        assert_eq!("H6", Granularity::H6.to_string());
        assert_eq!("H8", Granularity::H8.to_string());
        assert_eq!("H12", Granularity::H12.to_string());
        assert_eq!("D", Granularity::D.to_string());
        assert_eq!("W", Granularity::W.to_string());
        assert_eq!("M", Granularity::M.to_string());
    }

    #[test]
    fn test_weekly_alignment_to_string() {
        assert_eq!("Monday", WeeklyAlignment::Monday.to_string());
        assert_eq!("Tuesday", WeeklyAlignment::Tuesday.to_string());
        assert_eq!("Wednesday", WeeklyAlignment::Wednesday.to_string());
        assert_eq!("Thursday", WeeklyAlignment::Thursday.to_string());
        assert_eq!("Friday", WeeklyAlignment::Friday.to_string());
        assert_eq!("Saturday", WeeklyAlignment::Saturday.to_string());
        assert_eq!("Sunday", WeeklyAlignment::Sunday.to_string());
    }
}
