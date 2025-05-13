use ::polars::prelude::cum_fold_exprs;
use polars::prelude as polars;

pub fn lit<L>(t: L) -> polars::Expr
where
    L: polars::Literal,
{
    polars::lit(t)
}

pub fn period_max(col_name: &str, period: usize) -> polars::Expr {
    polars::col(col_name).rolling_max(polars::RollingOptionsFixedWindow {
        window_size: period,
        min_periods: 1,
        weights: None,
        center: false,
        fn_params: None,
    })
}

pub fn period_min(col_name: &str, period: usize) -> polars::Expr {
    polars::col(col_name).rolling_min(polars::RollingOptionsFixedWindow {
        window_size: period,
        min_periods: 1,
        weights: None,
        center: false,
        fn_params: None,
    })
}
