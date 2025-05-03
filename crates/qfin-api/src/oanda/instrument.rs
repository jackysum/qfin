use serde::Deserialize;

use crate::de_decimal;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    pub name: String,
    #[serde(rename = "type")]
    pub instrument_type: Type,
    pub display_name: String,
    pub pip_location: i32,
    pub display_precision: i32,
    pub trade_units_precision: i32,
    #[serde(deserialize_with = "de_decimal")]
    pub minimum_trade_size: f64,
    #[serde(deserialize_with = "de_decimal")]
    pub maximum_trailing_stop_distance: f64,
    #[serde(deserialize_with = "de_decimal")]
    pub minimum_trailing_stop_distance: f64,
    #[serde(deserialize_with = "de_decimal")]
    pub maximum_position_size: f64,
    #[serde(deserialize_with = "de_decimal")]
    pub maximum_order_units: f64,
    #[serde(deserialize_with = "de_decimal")]
    pub margin_rate: f64,
    pub guaranteed_stop_loss_order_mode: GuaranteedStopLossOrderMode,
    pub tags: Vec<Tag>,
    pub financing: Financing,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Financing {
    #[serde(deserialize_with = "de_decimal")]
    pub long_rate: f64,
    #[serde(deserialize_with = "de_decimal")]
    pub short_rate: f64,
    pub financing_days_of_week: Vec<FinancingDayOfWeek>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FinancingDayOfWeek {
    pub day_of_week: DayOfWeek,
    pub days_charged: i32,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum GuaranteedStopLossOrderMode {
    Allowed,
    Disabled,
    Required,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    #[serde(rename = "type")]
    pub tag_type: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Type {
    Cfd,
    Currency,
    Metal,
}
