use qfin_data::{
    dataset::dsl::{high, lit, low, period_max, period_min},
    DataColumn,
};

const DEFAULT_PERIOD: usize = 50;

pub fn alias(period: usize) -> String {
    format!("tether_line_{}", period)
}

pub fn calculate(period: Option<usize>) -> DataColumn {
    let period = period.unwrap_or(DEFAULT_PERIOD);
    let alias = alias(period);

    let period_high = period_max(high(), period);
    let period_low = period_min(low(), period);

    let res = ((period_high + period_low) / lit(2)).alias(alias);

    DataColumn::Expr(res)
}
