use polars::prelude::Expr;
use qfin_data::dataset::{
    dsl::{lit, period_max, period_min},
    HIGH, LOW,
};

const DEFAULT_PERIOD: usize = 50;
const ALIAS: &str = "tether_line";

pub fn expr(period: Option<usize>) -> Expr {
    let period = period.unwrap_or(DEFAULT_PERIOD);
    let alias = format!("{}_period_{}", ALIAS, period);

    ((period_max(HIGH, period) + period_min(LOW, period)) / lit(2)).alias(alias)
}
