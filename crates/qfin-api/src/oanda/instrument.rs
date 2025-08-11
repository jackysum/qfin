use serde::Deserialize;

use crate::deserializer::de_string_as_f64;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    pub name: String,
    #[serde(rename = "type")]
    pub instrument_type: String,
    pub display_name: String,
    pub pip_location: i8,
    pub display_precision: i8,
    pub trade_units_precision: i8,
    #[serde(deserialize_with = "de_string_as_f64")]
    pub minimum_trade_size: f64,
    #[serde(deserialize_with = "de_string_as_f64")]
    pub maximum_trailing_stop_distance: f64,
    #[serde(deserialize_with = "de_string_as_f64")]
    pub minimum_trailing_stop_distance: f64,
    #[serde(deserialize_with = "de_string_as_f64")]
    pub maximum_position_size: f64,
    #[serde(deserialize_with = "de_string_as_f64")]
    pub maximum_order_units: f64,
    #[serde(deserialize_with = "de_string_as_f64")]
    pub margin_rate: f64,
    pub guaranteed_stop_loss_order_mode: GuaranteedStopLossOrderModeForInstrument,
    pub tags: Vec<Tag>,
    pub financing: InstrumentFinancing,
}

#[derive(Debug, Deserialize)]
pub enum GuaranteedStopLossOrderModeForInstrument {
    DISABLED,
    ALLOWED,
    REQUIRED,
}

#[derive(Debug, Deserialize)]
pub struct Tag {
    #[serde(rename = "type")]
    pub tag_type: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentFinancing {
    #[serde(deserialize_with = "de_string_as_f64")]
    pub long_rate: f64,
    #[serde(deserialize_with = "de_string_as_f64")]
    pub short_rate: f64,
    pub financing_days_of_week: Vec<FinancingDayOfWeek>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinancingDayOfWeek {
    pub day_of_week: DayOfWeek,
    pub days_charged: i8,
}

#[derive(Debug, Deserialize)]
pub enum DayOfWeek {
    SUNDAY,
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
}

#[cfg(test)]
pub mod fixtures {
    use crate::oanda::Instrument;

    pub fn instruments() -> Vec<Instrument> {
        let json = instruments_json();
        serde_json::from_str(&json).unwrap()
    }

    pub fn instruments_json() -> String {
        r#"
        [
            {
                "name": "TRY_JPY",
                "type": "CURRENCY",
                "displayName": "TRY/JPY",
                "pipLocation": -2,
                "displayPrecision": 3,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "100.000",
                "minimumTrailingStopDistance": "0.050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.25",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.293",
                    "shortRate": "-0.424",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "HKD_JPY",
                "type": "CURRENCY",
                "displayName": "HKD/JPY",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.1",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0139",
                    "shortRate": "-0.0119",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "USD_PLN",
                "type": "CURRENCY",
                "displayName": "USD/PLN",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0166",
                    "shortRate": "-0.004",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "GBP_AUD",
                "type": "CURRENCY",
                "displayName": "GBP/AUD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0083",
                    "shortRate": "-0.0124",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "NZD_USD",
                "type": "CURRENCY",
                "displayName": "NZD/USD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.03",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0219",
                    "shortRate": "0.00090",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "EUR_ZAR",
                "type": "CURRENCY",
                "displayName": "EUR/ZAR",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.07",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0837",
                    "shortRate": "0.0132",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "AUD_JPY",
                "type": "CURRENCY",
                "displayName": "AUD/JPY",
                "pipLocation": -2,
                "displayPrecision": 3,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "100.000",
                "minimumTrailingStopDistance": "0.050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0233",
                    "shortRate": "-0.0441",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "USD_NOK",
                "type": "CURRENCY",
                "displayName": "USD/NOK",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.07",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0083",
                    "shortRate": "-0.0123",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "CAD_CHF",
                "type": "CURRENCY",
                "displayName": "CAD/CHF",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.03",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0167",
                    "shortRate": "-0.0386",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "GBP_SGD",
                "type": "CURRENCY",
                "displayName": "GBP/SGD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.009",
                    "shortRate": "-0.0352",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "USD_SEK",
                "type": "CURRENCY",
                "displayName": "USD/SEK",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.03",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0123",
                    "shortRate": "-0.0357",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "NZD_SGD",
                "type": "CURRENCY",
                "displayName": "NZD/SGD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0015",
                    "shortRate": "-0.0285",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "ZAR_JPY",
                "type": "CURRENCY",
                "displayName": "ZAR/JPY",
                "pipLocation": -2,
                "displayPrecision": 3,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "100.000",
                "minimumTrailingStopDistance": "0.005",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.07",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0294",
                    "shortRate": "-0.0996",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "SGD_JPY",
                "type": "CURRENCY",
                "displayName": "SGD/JPY",
                "pipLocation": -2,
                "displayPrecision": 3,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "100.000",
                "minimumTrailingStopDistance": "0.050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.001",
                    "shortRate": "-0.0266",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "GBP_ZAR",
                "type": "CURRENCY",
                "displayName": "GBP/ZAR",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.07",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0491",
                    "shortRate": "0.0082",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "USD_JPY",
                "type": "CURRENCY",
                "displayName": "USD/JPY",
                "pipLocation": -2,
                "displayPrecision": 3,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "100.000",
                "minimumTrailingStopDistance": "0.050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0301",
                    "shortRate": "-0.0503",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "EUR_TRY",
                "type": "CURRENCY",
                "displayName": "EUR/TRY",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "10000000",
                "marginRate": "0.25",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.4072",
                    "shortRate": "0.2762",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "EUR_JPY",
                "type": "CURRENCY",
                "displayName": "EUR/JPY",
                "pipLocation": -2,
                "displayPrecision": 3,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "100.000",
                "minimumTrailingStopDistance": "0.050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0059",
                    "shortRate": "-0.0262",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "AUD_SGD",
                "type": "CURRENCY",
                "displayName": "AUD/SGD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0056",
                    "shortRate": "-0.0344",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "EUR_NZD",
                "type": "CURRENCY",
                "displayName": "EUR/NZD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.03",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0234",
                    "shortRate": "0.0021",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "GBP_HKD",
                "type": "CURRENCY",
                "displayName": "GBP/HKD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.1",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0221",
                    "shortRate": "-0.0529",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "CHF_JPY",
                "type": "CURRENCY",
                "displayName": "CHF/JPY",
                "pipLocation": -2,
                "displayPrecision": 3,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "100.000",
                "minimumTrailingStopDistance": "0.050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0152",
                    "shortRate": "-0.0056",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "EUR_HKD",
                "type": "CURRENCY",
                "displayName": "EUR/HKD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.1",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0052",
                    "shortRate": "-0.0302",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "USD_THB",
                "type": "CURRENCY",
                "displayName": "USD/THB",
                "pipLocation": -2,
                "displayPrecision": 3,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "100.000",
                "minimumTrailingStopDistance": "0.050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0628",
                    "shortRate": "-0.0864",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "GBP_CHF",
                "type": "CURRENCY",
                "displayName": "GBP/CHF",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0302",
                    "shortRate": "-0.0512",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "AUD_CHF",
                "type": "CURRENCY",
                "displayName": "AUD/CHF",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.03",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0281",
                    "shortRate": "-0.0495",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "NZD_CHF",
                "type": "CURRENCY",
                "displayName": "NZD/CHF",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.03",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0227",
                    "shortRate": "-0.0449",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "AUD_HKD",
                "type": "CURRENCY",
                "displayName": "AUD/HKD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.1",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.02",
                    "shortRate": "-0.051",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "USD_CHF",
                "type": "CURRENCY",
                "displayName": "USD/CHF",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.03",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0347",
                    "shortRate": "-0.0553",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "CAD_HKD",
                "type": "CURRENCY",
                "displayName": "CAD/HKD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.1",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.012",
                    "shortRate": "-0.0374",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "USD_HKD",
                "type": "CURRENCY",
                "displayName": "USD/HKD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.1",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0266",
                    "shortRate": "-0.0571",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "AUD_NZD",
                "type": "CURRENCY",
                "displayName": "AUD/NZD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.03",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0056",
                    "shortRate": "-0.0156",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "CHF_ZAR",
                "type": "CURRENCY",
                "displayName": "CHF/ZAR",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.07",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.1048",
                    "shortRate": "0.0339",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "EUR_CHF",
                "type": "CURRENCY",
                "displayName": "EUR/CHF",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.03",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0106",
                    "shortRate": "-0.0313",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "USD_DKK",
                "type": "CURRENCY",
                "displayName": "USD/DKK",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.02",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0177",
                    "shortRate": "-0.0383",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "CAD_SGD",
                "type": "CURRENCY",
                "displayName": "CAD/SGD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0045",
                    "shortRate": "-0.0229",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "EUR_DKK",
                "type": "CURRENCY",
                "displayName": "EUR/DKK",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.1",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0068",
                    "shortRate": "-0.0145",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "USD_ZAR",
                "type": "CURRENCY",
                "displayName": "USD/ZAR",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.07",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0447",
                    "shortRate": "0.004",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "CAD_JPY",
                "type": "CURRENCY",
                "displayName": "CAD/JPY",
                "pipLocation": -2,
                "displayPrecision": 3,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "100.000",
                "minimumTrailingStopDistance": "0.050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0124",
                    "shortRate": "-0.0334",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "USD_HUF",
                "type": "CURRENCY",
                "displayName": "USD/HUF",
                "pipLocation": -2,
                "displayPrecision": 3,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "100.000",
                "minimumTrailingStopDistance": "0.050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.041",
                    "shortRate": "-0.00030",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "EUR_CAD",
                "type": "CURRENCY",
                "displayName": "EUR/CAD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.02",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0175",
                    "shortRate": "-0.0034",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "EUR_USD",
                "type": "CURRENCY",
                "displayName": "EUR/USD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.02",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0342",
                    "shortRate": "0.014",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "EUR_HUF",
                "type": "CURRENCY",
                "displayName": "EUR/HUF",
                "pipLocation": -2,
                "displayPrecision": 3,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "100.000",
                "minimumTrailingStopDistance": "0.050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.079",
                    "shortRate": "0.0099",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "CHF_HKD",
                "type": "CURRENCY",
                "displayName": "CHF/HKD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.1",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0159",
                    "shortRate": "-0.0095",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "GBP_NZD",
                "type": "CURRENCY",
                "displayName": "GBP/NZD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0034",
                    "shortRate": "-0.0176",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "USD_SGD",
                "type": "CURRENCY",
                "displayName": "USD/SGD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0137",
                    "shortRate": "-0.0391",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "EUR_SEK",
                "type": "CURRENCY",
                "displayName": "EUR/SEK",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.03",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0107",
                    "shortRate": "-0.0105",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "USD_TRY",
                "type": "CURRENCY",
                "displayName": "USD/TRY",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "10000000",
                "marginRate": "0.25",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.3701",
                    "shortRate": "0.258",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "GBP_JPY",
                "type": "CURRENCY",
                "displayName": "GBP/JPY",
                "pipLocation": -2,
                "displayPrecision": 3,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "100.000",
                "minimumTrailingStopDistance": "0.050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "50000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0257",
                    "shortRate": "-0.0461",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "GBP_PLN",
                "type": "CURRENCY",
                "displayName": "GBP/PLN",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0209",
                    "shortRate": "0.00010",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "EUR_PLN",
                "type": "CURRENCY",
                "displayName": "EUR/PLN",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0423",
                    "shortRate": "0.0187",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "AUD_CAD",
                "type": "CURRENCY",
                "displayName": "AUD/CAD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.03",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.00020",
                    "shortRate": "-0.0214",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "EUR_CZK",
                "type": "CURRENCY",
                "displayName": "EUR/CZK",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0385",
                    "shortRate": "-0.0094",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "GBP_USD",
                "type": "CURRENCY",
                "displayName": "GBP/USD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "50000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0144",
                    "shortRate": "-0.0058",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "USD_MXN",
                "type": "CURRENCY",
                "displayName": "USD/MXN",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.10",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0484",
                    "shortRate": "0.0239",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "GBP_CAD",
                "type": "CURRENCY",
                "displayName": "GBP/CAD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0024",
                    "shortRate": "-0.0234",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "SGD_CHF",
                "type": "CURRENCY",
                "displayName": "SGD/CHF",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0056",
                    "shortRate": "-0.0316",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "NZD_CAD",
                "type": "CURRENCY",
                "displayName": "NZD/CAD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.03",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0055",
                    "shortRate": "-0.0167",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "AUD_USD",
                "type": "CURRENCY",
                "displayName": "AUD/USD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.03",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0166",
                    "shortRate": "-0.0039",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "NZD_JPY",
                "type": "CURRENCY",
                "displayName": "NZD/JPY",
                "pipLocation": -2,
                "displayPrecision": 3,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "100.000",
                "minimumTrailingStopDistance": "0.050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0182",
                    "shortRate": "-0.0395",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "USD_CNH",
                "type": "CURRENCY",
                "displayName": "USD/CNH",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0164",
                    "shortRate": "-0.0455",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "EUR_GBP",
                "type": "CURRENCY",
                "displayName": "EUR/GBP",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0301",
                    "shortRate": "0.0095",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "USD_CZK",
                "type": "CURRENCY",
                "displayName": "USD/CZK",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0108",
                    "shortRate": "-0.0298",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "NZD_HKD",
                "type": "CURRENCY",
                "displayName": "NZD/HKD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.1",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0147",
                    "shortRate": "-0.0462",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "EUR_NOK",
                "type": "CURRENCY",
                "displayName": "EUR/NOK",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.07",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0326",
                    "shortRate": "0.0117",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "USD_CAD",
                "type": "CURRENCY",
                "displayName": "USD/CAD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.02",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "0.0067",
                    "shortRate": "-0.0275",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "EUR_AUD",
                "type": "CURRENCY",
                "displayName": "EUR/AUD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.03",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.028",
                    "shortRate": "0.0074",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "EUR_SGD",
                "type": "CURRENCY",
                "displayName": "EUR/SGD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    },
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    }
                ],
                "financing": {
                    "longRate": "-0.0107",
                    "shortRate": "-0.0151",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            }
        ]
        "#
        .to_owned()
    }
}
