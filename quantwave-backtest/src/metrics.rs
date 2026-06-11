//! Performance analytics computed from [`BacktestResult`] (quantwave-cr6v.1).
//!
//! Clean-room implementation — concepts aligned with industry practice, no
//! copied third-party code.
//!
//! ## Formula sources (v1)
//! - **Max drawdown %**: peak-to-trough decline on the equity curve
//!   ([StockCharts — Drawdown](https://chartschool.stockcharts.com/table-of-contents/overview)).
//! - **Sharpe ratio**: per-bar returns, annualized with √252 trading days
//!   ([QuantConnect — Sharpe Ratio](https://www.quantconnect.com/docs/v2/writing-algorithms/indicators/supported-indicators/sharpe-ratio)).
//! - **Sortino ratio**: same return series; downside deviation uses only
//!   negative returns (clean-room; analogous to QuantConnect Sortino semantics).
//! - **CAGR**: `(final/initial)^(252/n_bars) - 1` on daily-bar synthetic tests;
//!   bar count from equity curve length (clean-room annualization).
//! - **Win rate / profit factor / avg trade PnL**: aggregated from trade blotter
//!   `pnl_net` column (clean-room).

use crate::BacktestResult;

/// Bundle of raw backtest output plus computed analytics.
#[derive(Debug)]
pub struct BacktestReport {
    pub result: BacktestResult,
    pub metrics: PerformanceMetrics,
}

/// Summary performance statistics for a completed backtest run.
#[derive(Debug, Clone, PartialEq)]
pub struct PerformanceMetrics {
    pub num_trades: f64,
    pub win_rate: f64,
    pub profit_factor: f64,
    pub max_drawdown_pct: f64,
    pub cagr: f64,
    pub sharpe_ratio: f64,
    pub sortino_ratio: f64,
    pub total_return: f64,
    pub final_equity: f64,
    pub avg_trade_pnl: f64,
}

impl PerformanceMetrics {
    /// Compute metrics from a [`BacktestResult`].
    ///
    /// Uses `stats` for initial/final equity when present; falls back to the
    /// equity curve endpoints.
    pub fn from_result(result: &BacktestResult) -> Self {
        let initial_cash = result
            .stats
            .get("initial_cash")
            .copied()
            .or_else(|| equity_first(result))
            .unwrap_or(0.0);

        let final_equity = result
            .stats
            .get("final_equity")
            .copied()
            .or_else(|| equity_last(result))
            .unwrap_or(initial_cash);

        let total_return = if initial_cash.abs() > f64::EPSILON {
            (final_equity - initial_cash) / initial_cash
        } else {
            0.0
        };

        let trade_pnls = extract_trade_pnls(result);
        let num_trades = trade_pnls.len() as f64;
        let max_drawdown_pct = compute_max_drawdown_pct(result);

        if num_trades == 0.0 && total_return.abs() < 1e-12 {
            return Self::zero_trades_flat(final_equity, max_drawdown_pct);
        }

        let (win_rate, profit_factor, avg_trade_pnl) = aggregate_trade_stats(&trade_pnls);
        let n_bars = equity_len(result);
        let cagr = compute_cagr(initial_cash, final_equity, n_bars);
        let returns = per_bar_returns(result);
        let sharpe_ratio = compute_sharpe(&returns);
        let sortino_ratio = compute_sortino(&returns);

        Self {
            num_trades,
            win_rate,
            profit_factor,
            max_drawdown_pct,
            cagr,
            sharpe_ratio,
            sortino_ratio,
            total_return,
            final_equity,
            avg_trade_pnl,
        }
    }

    fn zero_trades_flat(final_equity: f64, max_drawdown_pct: f64) -> Self {
        Self {
            num_trades: 0.0,
            win_rate: 0.0,
            profit_factor: 0.0,
            max_drawdown_pct,
            cagr: 0.0,
            sharpe_ratio: 0.0,
            sortino_ratio: 0.0,
            total_return: 0.0,
            final_equity,
            avg_trade_pnl: 0.0,
        }
    }
}

fn extract_trade_pnls(result: &BacktestResult) -> Vec<f64> {
    let Ok(col) = result.trades.column("pnl_net") else {
        return Vec::new();
    };
    let Ok(ca) = col.f64() else {
        return Vec::new();
    };
    ca.into_iter().map(|v| v.unwrap_or(0.0)).collect()
}

/// Win rate, profit factor, and average trade PnL from closed-trade `pnl_net` values.
fn aggregate_trade_stats(pnls: &[f64]) -> (f64, f64, f64) {
    let n = pnls.len() as f64;
    if n == 0.0 {
        return (0.0, 0.0, 0.0);
    }

    let wins = pnls.iter().filter(|&&p| p > 0.0).count() as f64;
    let win_rate = wins / n;

    let gross_profit: f64 = pnls.iter().filter(|&&p| p > 0.0).copied().sum();
    let gross_loss: f64 = pnls
        .iter()
        .filter(|&&p| p < 0.0)
        .map(|p| p.abs())
        .sum();

    let profit_factor = if gross_loss > f64::EPSILON {
        gross_profit / gross_loss
    } else if gross_profit > f64::EPSILON {
        f64::INFINITY
    } else {
        0.0
    };

    let avg_trade_pnl = pnls.iter().sum::<f64>() / n;

    (win_rate, profit_factor, avg_trade_pnl)
}

/// Peak-to-trough drawdown on the equity curve as a fraction (0.10 = 10%).
fn compute_max_drawdown_pct(result: &BacktestResult) -> f64 {
    let equity = portfolio_equity_values(result);
    if equity.is_empty() {
        return 0.0;
    }

    let mut peak = 0.0;
    let mut max_dd = 0.0;
    let mut seen = false;

    for eq in equity {
        if !seen {
            peak = eq;
            seen = true;
        } else if eq > peak {
            peak = eq;
        }
        if peak > f64::EPSILON {
            let dd = (peak - eq) / peak;
            if dd > max_dd {
                max_dd = dd;
            }
        }
    }

    max_dd
}

fn equity_len(result: &BacktestResult) -> usize {
    portfolio_equity_values(result).len()
}

/// CAGR annualized with 252 trading days per year (clean-room).
fn compute_cagr(initial: f64, final_equity: f64, n_bars: usize) -> f64 {
    if initial <= f64::EPSILON || n_bars == 0 {
        return 0.0;
    }
    let ratio = final_equity / initial;
    if ratio <= 0.0 {
        return 0.0;
    }
    ratio.powf(252.0 / n_bars as f64) - 1.0
}

fn per_bar_returns(result: &BacktestResult) -> Vec<f64> {
    let equity = portfolio_equity_values(result);
    equity
        .windows(2)
        .filter_map(|w| {
            if w[0].abs() > f64::EPSILON {
                Some((w[1] - w[0]) / w[0])
            } else {
                None
            }
        })
        .collect()
}

const TRADING_DAYS_PER_YEAR: f64 = 252.0;

/// Sharpe ratio: √252 × mean(returns) / std(returns), risk-free = 0.
fn compute_sharpe(returns: &[f64]) -> f64 {
    if returns.len() < 2 {
        return 0.0;
    }
    let mean = returns.iter().sum::<f64>() / returns.len() as f64;
    let variance = returns
        .iter()
        .map(|r| {
            let d = r - mean;
            d * d
        })
        .sum::<f64>()
        / (returns.len() - 1) as f64;
    let std = variance.sqrt();
    if std <= f64::EPSILON {
        return 0.0;
    }
    (mean / std) * TRADING_DAYS_PER_YEAR.sqrt()
}

/// Sortino ratio: √252 × mean(returns) / downside deviation (negative returns only).
fn compute_sortino(returns: &[f64]) -> f64 {
    if returns.is_empty() {
        return 0.0;
    }
    let mean = returns.iter().sum::<f64>() / returns.len() as f64;
    let downside: Vec<f64> = returns.iter().copied().filter(|&r| r < 0.0).collect();
    if downside.is_empty() {
        return f64::INFINITY;
    }
    let downside_var = downside.iter().map(|r| r * r).sum::<f64>() / downside.len() as f64;
    let downside_std = downside_var.sqrt();
    if downside_std <= f64::EPSILON {
        return f64::INFINITY;
    }
    (mean / downside_std) * TRADING_DAYS_PER_YEAR.sqrt()
}

/// Equity series for analytics. When a `symbol` column exists, use portfolio rows
/// (`symbol` null) to avoid double-counting per-symbol curves.
fn portfolio_equity_values(result: &BacktestResult) -> Vec<f64> {
    let Ok(eq_col) = result.equity_curve.column("equity") else {
        return Vec::new();
    };
    let Ok(eq_ca) = eq_col.f64() else {
        return Vec::new();
    };

    if let Ok(sym_col) = result.equity_curve.column("symbol") {
        if let Ok(sym_ca) = sym_col.str() {
            return eq_ca
                .into_iter()
                .zip(sym_ca.into_iter())
                .filter_map(|(eq, sym)| {
                    if sym.is_none() {
                        eq
                    } else {
                        None
                    }
                })
                .collect();
        }
    }

    eq_ca.into_iter().flatten().collect()
}

fn equity_first(result: &BacktestResult) -> Option<f64> {
    portfolio_equity_values(result).first().copied()
}

fn equity_last(result: &BacktestResult) -> Option<f64> {
    portfolio_equity_values(result).last().copied()
}