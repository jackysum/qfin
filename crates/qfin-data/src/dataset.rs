use chrono::NaiveDateTime;
use polars::{df, frame::DataFrame};

use crate::Datapoint;

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
            "time" => times,
            "open" => opens,
            "high" => highs,
            "low" => lows,
            "close" => closes,
        )
        .unwrap();

        Dataset { dataframe }
    }
}
