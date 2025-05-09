use chrono::NaiveDateTime;

pub trait Datapoint {
    fn time(&self) -> NaiveDateTime;
    fn open(&self) -> f64;
    fn high(&self) -> f64;
    fn low(&self) -> f64;
    fn close(&self) -> f64;
}
