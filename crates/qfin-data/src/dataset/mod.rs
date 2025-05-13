use chrono::NaiveDateTime;
use polars::{
    df,
    error::PolarsResult,
    frame::DataFrame,
    prelude::{Expr, IntoLazy},
};

use crate::Datapoint;

pub mod dsl;

pub const TIME: &str = "time";
pub const OPEN: &str = "open";
pub const HIGH: &str = "high";
pub const LOW: &str = "low";
pub const CLOSE: &str = "close";

#[derive(Clone)]
pub struct Dataset {
    pub dataframe: DataFrame,
}

impl Dataset {
    pub fn new<D: Datapoint>(datapoints: Vec<D>) -> Self {
        let mut times: Vec<NaiveDateTime> = vec![];
        let mut opens: Vec<f64> = vec![];
        let mut highs: Vec<f64> = vec![];
        let mut lows: Vec<f64> = vec![];
        let mut closes: Vec<f64> = vec![];

        datapoints.iter().for_each(|d| {
            times.push(d.time());
            opens.push(d.open());
            highs.push(d.high());
            lows.push(d.low());
            closes.push(d.close());
        });

        let dataframe = df!(
            TIME => times,
            OPEN => opens,
            HIGH => highs,
            LOW => lows,
            CLOSE => closes,
        )
        .unwrap();

        Dataset { dataframe }
    }

    pub fn select<E: AsRef<[Expr]>>(&self, exprs: E) -> PolarsResult<DataFrame> {
        self.dataframe.clone().lazy().select(exprs).collect()
    }
}
