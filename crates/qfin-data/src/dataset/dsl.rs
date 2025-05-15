use polars::lazy::dsl::max_horizontal;
use polars::prelude as pl;

use crate::dataset::{CLOSE, HIGH, LOW, OPEN};

pub fn open() -> pl::Expr {
    col(OPEN)
}
pub fn high() -> pl::Expr {
    col(HIGH)
}

pub fn low() -> pl::Expr {
    col(LOW)
}

pub fn close() -> pl::Expr {
    col(CLOSE)
}

pub fn col<S>(name: S) -> pl::Expr
where
    S: Into<pl::PlSmallStr>,
{
    pl::col(name)
}

pub fn lit<L>(t: L) -> pl::Expr
where
    L: pl::Literal,
{
    pl::lit(t)
}

pub fn row_max<E: AsRef<[pl::Expr]>>(exprs: E) -> pl::Expr {
    max_horizontal(exprs).unwrap()
}

pub fn period_mean(expr: pl::Expr, period: usize) -> pl::Expr {
    expr.rolling_mean(pl::RollingOptionsFixedWindow {
        window_size: period,
        min_periods: period,
        weights: None,
        center: false,
        fn_params: None,
    })
}

pub fn period_max(expr: pl::Expr, period: usize) -> pl::Expr {
    expr.rolling_max(pl::RollingOptionsFixedWindow {
        window_size: period,
        min_periods: 1,
        weights: None,
        center: false,
        fn_params: None,
    })
}

pub fn period_min(expr: pl::Expr, period: usize) -> pl::Expr {
    expr.rolling_min(pl::RollingOptionsFixedWindow {
        window_size: period,
        min_periods: 1,
        weights: None,
        center: false,
        fn_params: None,
    })
}

pub fn shift(expr: pl::Expr, n: i32) -> pl::Expr {
    expr.shift(pl::lit(n))
}

pub fn last(expr: pl::Expr) -> pl::Expr {
    shift(expr, 1)
}
