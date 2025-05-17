use polars::prelude::col;
use qfin_data::{
    dataset::dsl::{high, lit, low, period_max, period_min},
    DataColumn,
};

use crate::average_true_range;

const DEFAULT_PERIOD: usize = 22;
const DEFAULT_MULT: f64 = 3.0;

pub fn alias_long(period: usize, mult: f64) -> String {
    format!("chandelier_exit_long_{}_{}", period, mult)
}

pub fn alias_short(period: usize, mult: f64) -> String {
    format!("chandelier_exit_short_{}_{}", period, mult)
}

pub fn calculate(period: Option<usize>, mult: Option<f64>) -> DataColumn {
    let period = period.unwrap_or(DEFAULT_PERIOD);
    let mult = mult.unwrap_or(DEFAULT_MULT);

    let atr = col(average_true_range::alias(period));

    let long = period_max(high(), period) - (atr.clone() * lit(mult));
    let short = period_min(low(), period) + (atr * lit(mult));

    DataColumn::Exprs(vec![
        long.alias(alias_long(period, mult)),
        short.alias(alias_short(period, mult)),
    ])
}
