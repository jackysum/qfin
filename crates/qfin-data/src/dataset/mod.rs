use chrono::NaiveDateTime;
use polars::{
    df,
    error::PolarsResult,
    frame::DataFrame,
    prelude::{Expr, IntoLazy},
    series,
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

pub enum DataColumn {
    Expr(polars::prelude::Expr),
    Series(series::Series),
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

    pub fn columns(&mut self, data_columns: Vec<DataColumn>) -> PolarsResult<DataFrame> {
        let mut exprs: Vec<Expr> = vec![];

        for data_column in data_columns {
            match data_column {
                DataColumn::Expr(e) => exprs.push(e),
                DataColumn::Series(s) => {
                    self.dataframe.with_column(s)?;
                }
            }
        }

        self.dataframe.clone().lazy().with_columns(exprs).collect()
    }
}
