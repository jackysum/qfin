use chrono::NaiveDateTime;
use polars::{
    df,
    error::PolarsResult,
    frame::DataFrame,
    prelude::{Expr, IntoLazy},
    series::Series,
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
    Expr(Expr),
    Exprs(Vec<Expr>),
    Series(Series),
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

    pub fn with_columns(self, data_columns: Vec<DataColumn>) -> PolarsResult<Dataset> {
        let mut dataframe = self.dataframe;
        let mut exprs: Vec<Expr> = vec![];

        for data_column in data_columns {
            match data_column {
                DataColumn::Expr(e) => exprs.push(e),
                DataColumn::Exprs(e) => exprs.extend(e),
                DataColumn::Series(s) => {
                    dataframe.with_column(s)?;
                }
            }
        }

        Ok(Dataset {
            dataframe: dataframe.lazy().with_columns(exprs).collect()?,
        })
    }
}
