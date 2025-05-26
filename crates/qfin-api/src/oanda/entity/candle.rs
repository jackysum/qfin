use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::oanda::entity::de_decimal_number;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Candle {
    pub time: DateTime<Utc>,
    pub bid: Option<Data>,
    pub ask: Option<Data>,
    pub mid: Option<Data>,
    pub volume: i64,
    pub complete: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Data {
    #[serde(rename = "o", deserialize_with = "de_decimal_number")]
    pub open: f64,
    #[serde(rename = "h", deserialize_with = "de_decimal_number")]
    pub high: f64,
    #[serde(rename = "l", deserialize_with = "de_decimal_number")]
    pub low: f64,
    #[serde(rename = "c", deserialize_with = "de_decimal_number")]
    pub close: f64,
}

#[derive(Debug, PartialEq)]
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

impl Display for Granularity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Granularity::S5 => write!(f, "S5"),
            Granularity::S10 => write!(f, "S10"),
            Granularity::S15 => write!(f, "S15"),
            Granularity::S30 => write!(f, "S30"),
            Granularity::M1 => write!(f, "M1"),
            Granularity::M2 => write!(f, "M2"),
            Granularity::M4 => write!(f, "M4"),
            Granularity::M5 => write!(f, "M5"),
            Granularity::M10 => write!(f, "M10"),
            Granularity::M15 => write!(f, "M15"),
            Granularity::M30 => write!(f, "M30"),
            Granularity::H1 => write!(f, "H1"),
            Granularity::H2 => write!(f, "H2"),
            Granularity::H3 => write!(f, "H3"),
            Granularity::H4 => write!(f, "H4"),
            Granularity::H6 => write!(f, "H6"),
            Granularity::H8 => write!(f, "H8"),
            Granularity::H12 => write!(f, "H12"),
            Granularity::D => write!(f, "D"),
            Granularity::W => write!(f, "W"),
            Granularity::M => write!(f, "M"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum WeeklyAlignment {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Display for WeeklyAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeeklyAlignment::Monday => write!(f, "Monday"),
            WeeklyAlignment::Tuesday => write!(f, "Tuesday"),
            WeeklyAlignment::Wednesday => write!(f, "Wednesday"),
            WeeklyAlignment::Thursday => write!(f, "Thursday"),
            WeeklyAlignment::Friday => write!(f, "Friday"),
            WeeklyAlignment::Saturday => write!(f, "Saturday"),
            WeeklyAlignment::Sunday => write!(f, "Sunday"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::oanda::entity::candle::{Granularity, WeeklyAlignment};

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
