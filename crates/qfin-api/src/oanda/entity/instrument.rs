use serde::Deserialize;

use crate::oanda::entity::de_decimal_number;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    pub name: String,
    #[serde(rename = "type")]
    pub instrument_type: Type,
    pub display_name: String,
    pub pip_location: i64,
    pub display_precision: i64,
    pub trade_units_precision: i64,
    #[serde(deserialize_with = "de_decimal_number")]
    pub minimum_trade_size: f64,
    #[serde(deserialize_with = "de_decimal_number")]
    pub maximum_trailing_stop_distance: f64,
    #[serde(deserialize_with = "de_decimal_number")]
    pub minimum_trailing_stop_distance: f64,
    #[serde(deserialize_with = "de_decimal_number")]
    pub maximum_position_size: f64,
    #[serde(deserialize_with = "de_decimal_number")]
    pub maximum_order_units: f64,
    #[serde(deserialize_with = "de_decimal_number")]
    pub margin_rate: f64,
    pub guaranteed_stop_loss_order_mode: GuaranteedStopLossOrderMode,
    pub tags: Vec<Tag>,
    pub financing: Financing,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Financing {
    #[serde(deserialize_with = "de_decimal_number")]
    pub long_rate: f64,
    #[serde(deserialize_with = "de_decimal_number")]
    pub short_rate: f64,
    pub financing_days_of_week: Vec<FinancingDayOfWeek>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinancingDayOfWeek {
    pub day_of_week: DayOfWeek,
    pub days_charged: i64,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum GuaranteedStopLossOrderMode {
    Allowed,
    Disabled,
    Required,
}

#[derive(Debug, Deserialize)]
pub struct Tag {
    #[serde(rename = "type")]
    pub tag_type: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Type {
    Cfd,
    Currency,
    Metals,
}
