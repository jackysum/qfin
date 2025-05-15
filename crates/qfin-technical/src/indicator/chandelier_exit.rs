use polars::prelude::col;
use qfin_data::{dataset::dsl::high, DataColumn};

const DEFAULT_PERIOD: usize = 22;
const DEFAULT_MULT: f64 = 3.0;

pub fn alias(period: usize, mult: f64) -> String {
    format!("chandelier_exit_{}_{}", period, mult)
}

pub fn calculate(period: Option<usize>, mult: Option<f64>) -> DataColumn {
    let period = period.unwrap_or(DEFAULT_PERIOD);
    let mult = mult.unwrap_or(DEFAULT_MULT);
    let alias = alias(period, mult);

    DataColumn::Expr(high().alias(alias))
}
