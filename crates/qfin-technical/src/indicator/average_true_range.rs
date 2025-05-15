use polars::{
    frame::DataFrame,
    prelude::{Expr, IntoLazy},
    series::Series,
};
use qfin_data::{
    dataset::dsl::{close, high, last, low, row_max},
    DataColumn,
};

const DEFAULT_PERIOD: usize = 14;

pub fn alias(period: usize) -> String {
    format!("average_true_range_{}", period)
}

pub fn calculate(dataframe: DataFrame, period: Option<usize>) -> DataColumn {
    let period = period.unwrap_or(DEFAULT_PERIOD);
    let alias = alias(period);

    let series = dataframe
        .lazy()
        .select([true_range()])
        .collect()
        .unwrap()
        .column("true_range")
        .unwrap()
        .as_series()
        .unwrap()
        .clone();

    if period < 2 {
        return DataColumn::Series(series);
    }

    let mut idx = 0;
    let mut first_atr_value: f64 = 0.0;
    let mut prev_atr: Option<f64> = None;

    let result = series
        .f64()
        .unwrap()
        .into_iter()
        .map(|tr| {
            let true_range = tr.unwrap();

            if prev_atr.is_some() {
                let atr = ((prev_atr.unwrap() * (period - 1) as f64) + true_range) / period as f64;
                prev_atr = Some(atr);

                return Some(atr);
            }

            let res = if idx == 0 {
                None
            } else if idx < period {
                first_atr_value = first_atr_value + true_range;
                None
            } else {
                let atr = (first_atr_value + true_range) / period as f64;
                prev_atr = Some(atr);

                Some(atr)
            };

            idx = idx + 1;
            res
        })
        .collect::<Series>()
        .with_name(alias.into());

    DataColumn::Series(result)
}

fn true_range() -> Expr {
    row_max([
        (high() - low()),
        (high() - last(close())).abs(),
        (low() - last(close())).abs(),
    ])
    .alias("true_range")
}
