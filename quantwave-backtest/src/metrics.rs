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
//!
//! ## Formula sources (v2 — additive, quantwave-b5gr)
//! - **Calmar ratio**: `CAGR / max_drawdown_pct` — standard return-over-pain
//!   ratio ([Investopedia — Calmar Ratio](https://www.investopedia.com/terms/c/calmarratio.asp)).
//! - **VaR / CVaR (95%)**: historical simulation on per-bar returns — the 5th
//!   percentile loss (VaR) and the mean loss beyond it (CVaR), both reported as
//!   positive fractions (clean-room; standard historical VaR/CVaR definitions).
//! - **Alpha / beta**: OLS single-factor regression of per-bar strategy returns
//!   on per-bar benchmark returns — `beta = Cov(r_s, r_b) / Var(r_b)`,
//!   `alpha = mean(r_s) - beta * mean(r_b)`, annualized ×252
//!   ([Investopedia — Alpha](https://www.investopedia.com/terms/a/alpha.asp),
//!   [Investopedia — Beta](https://www.investopedia.com/terms/b/beta.asp)).
//! - **Cumulative return vs. benchmark**: `Π(1 + r) - 1` over the aligned window
//!   for both strategy and benchmark return series (clean-room).

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

    // --- Additive (quantwave-b5gr) — new optional/extra fields only.
    // Not part of `column_names()`/`values()`/`row_iter()` so existing sweep,
    // walk-forward, and Python `.metrics()` contracts are unaffected.
    /// `CAGR / max_drawdown_pct` (0.0 when drawdown is ~0 with non-positive CAGR;
    /// `f64::INFINITY` when drawdown is ~0 with positive CAGR).
    pub calmar_ratio: f64,
    /// Historical 95% Value-at-Risk on per-bar returns, positive fraction (loss magnitude).
    pub var_95: f64,
    /// Historical 95% Conditional VaR (Expected Shortfall) on per-bar returns, positive fraction.
    pub cvar_95: f64,
    /// Benchmark-relative analytics (alpha/beta/cumulative return), `None` unless a
    /// benchmark return series was supplied via [`PerformanceMetrics::with_benchmark`]
    /// or [`PerformanceMetrics::from_result_with_benchmark`].
    pub benchmark: Option<BenchmarkMetrics>,
}

/// Benchmark-relative analytics (quantwave-b5gr): alpha, beta, and cumulative
/// return comparison against a supplied benchmark return series.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BenchmarkMetrics {
    /// Annualized alpha (×252): `mean(r_s) - beta * mean(r_b)`, annualized.
    pub alpha: f64,
    /// Beta: `Cov(r_s, r_b) / Var(r_b)`.
    pub beta: f64,
    /// Strategy cumulative return over the aligned window: `Π(1 + r_s) - 1`.
    pub cumulative_return: f64,
    /// Benchmark cumulative return over the aligned window: `Π(1 + r_b) - 1`.
    pub benchmark_cumulative_return: f64,
    /// `cumulative_return - benchmark_cumulative_return`.
    pub excess_cumulative_return: f64,
}

impl PerformanceMetrics {
    /// Column names for sweep / tabular export (stable order).
    pub const fn column_names() -> &'static [&'static str] {
        &[
            "num_trades",
            "win_rate",
            "profit_factor",
            "max_drawdown_pct",
            "cagr",
            "sharpe_ratio",
            "sortino_ratio",
            "total_return",
            "final_equity",
            "avg_trade_pnl",
        ]
    }

    /// Metric values in [`Self::column_names`] order.
    pub fn values(&self) -> [f64; 10] {
        [
            self.num_trades,
            self.win_rate,
            self.profit_factor,
            self.max_drawdown_pct,
            self.cagr,
            self.sharpe_ratio,
            self.sortino_ratio,
            self.total_return,
            self.final_equity,
            self.avg_trade_pnl,
        ]
    }

    /// Iterate (column name, value) pairs for sweep row assembly.
    pub fn row_iter(&self) -> impl Iterator<Item = (&'static str, f64)> {
        Self::column_names().iter().copied().zip(self.values())
    }

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
        let calmar_ratio = compute_calmar(cagr, max_drawdown_pct);
        let (var_95, cvar_95) = compute_var_cvar(&returns, 0.95);

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
            calmar_ratio,
            var_95,
            cvar_95,
            benchmark: None,
        }
    }

    /// Compute metrics from a [`BacktestResult`], attaching benchmark-relative
    /// analytics (alpha/beta/cumulative return) computed against `benchmark_returns`
    /// (per-bar simple returns, aligned to the strategy's per-bar returns by index).
    pub fn from_result_with_benchmark(result: &BacktestResult, benchmark_returns: &[f64]) -> Self {
        let mut metrics = Self::from_result(result);
        let strategy_returns = per_bar_returns(result);
        metrics.benchmark = compute_benchmark_metrics(&strategy_returns, benchmark_returns);
        metrics
    }

    /// Attach benchmark-relative analytics computed from explicit strategy/benchmark
    /// per-bar return series. Returns a copy with `benchmark` populated (or left
    /// `None` if either series is empty).
    #[must_use]
    pub fn with_benchmark(mut self, strategy_returns: &[f64], benchmark_returns: &[f64]) -> Self {
        self.benchmark = compute_benchmark_metrics(strategy_returns, benchmark_returns);
        self
    }

    pub fn from_raw(
        trades: &[crate::Trade],
        equity: &[crate::EquityPoint],
        initial_cash: f64,
    ) -> Self {
        let final_equity = equity.last().map(|e| e.equity).unwrap_or(initial_cash);
        let total_return = if initial_cash.abs() > f64::EPSILON {
            (final_equity - initial_cash) / initial_cash
        } else {
            0.0
        };

        let mut peak = 0.0;
        let mut max_drawdown_pct = 0.0;
        let mut seen = false;
        for e in equity {
            let eq = e.equity;
            if !seen {
                peak = eq;
                seen = true;
            } else if eq > peak {
                peak = eq;
            }
            if peak > f64::EPSILON {
                let dd = (peak - eq) / peak;
                if dd > max_drawdown_pct {
                    max_drawdown_pct = dd;
                }
            }
        }

        let num_trades = trades.len() as f64;
        if num_trades == 0.0 && total_return.abs() < 1e-12 {
            return Self::zero_trades_flat(final_equity, max_drawdown_pct);
        }

        let mut wins = 0.0;
        let mut gross_profit = 0.0;
        let mut gross_loss = 0.0;
        let mut sum_pnl = 0.0;
        for t in trades {
            let pnl = t.pnl_net;
            sum_pnl += pnl;
            if pnl > 0.0 {
                wins += 1.0;
                gross_profit += pnl;
            } else {
                gross_loss += pnl.abs();
            }
        }

        let win_rate = wins / num_trades;
        let profit_factor = if gross_loss > f64::EPSILON {
            gross_profit / gross_loss
        } else if gross_profit > f64::EPSILON {
            f64::INFINITY
        } else {
            0.0
        };
        let avg_trade_pnl = sum_pnl / num_trades;

        let n_bars = equity.len();
        let cagr = compute_cagr(initial_cash, final_equity, n_bars);

        let returns: Vec<f64> = equity
            .windows(2)
            .filter_map(|w| {
                if w[0].equity.abs() > f64::EPSILON {
                    Some((w[1].equity - w[0].equity) / w[0].equity)
                } else {
                    None
                }
            })
            .collect();

        let sharpe_ratio = compute_sharpe(&returns);
        let sortino_ratio = compute_sortino(&returns);
        let calmar_ratio = compute_calmar(cagr, max_drawdown_pct);
        let (var_95, cvar_95) = compute_var_cvar(&returns, 0.95);

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
            calmar_ratio,
            var_95,
            cvar_95,
            benchmark: None,
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
            calmar_ratio: 0.0,
            var_95: 0.0,
            cvar_95: 0.0,
            benchmark: None,
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
    let gross_loss: f64 = pnls.iter().filter(|&&p| p < 0.0).map(|p| p.abs()).sum();

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

/// Calmar ratio: `CAGR / max_drawdown_pct`.
///
/// `max_drawdown_pct` is a positive fraction (0.0 = no drawdown). When there is
/// effectively no drawdown, returns `0.0` for non-positive CAGR or `f64::INFINITY`
/// for positive CAGR (mirrors the Sortino "no downside" convention above).
fn compute_calmar(cagr: f64, max_drawdown_pct: f64) -> f64 {
    if max_drawdown_pct <= f64::EPSILON {
        return if cagr > 0.0 { f64::INFINITY } else { 0.0 };
    }
    cagr / max_drawdown_pct
}

/// Historical VaR/CVaR at `confidence` (e.g. 0.95) on per-bar returns.
///
/// Both are reported as positive fractions (loss magnitude): VaR is the loss at
/// the `(1 - confidence)` empirical quantile of the return distribution; CVaR is
/// the mean loss beyond that quantile (Expected Shortfall).
fn compute_var_cvar(returns: &[f64], confidence: f64) -> (f64, f64) {
    if returns.is_empty() {
        return (0.0, 0.0);
    }
    let mut sorted: Vec<f64> = returns.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let tail_frac = (1.0 - confidence).clamp(0.0, 1.0);
    let n = sorted.len();
    // Number of worst observations in the tail (at least 1). Subtract a small
    // epsilon before ceiling so exact fractions (e.g. 0.05 * 20 = 1.0) aren't
    // pushed to the next integer by floating-point representation error.
    let tail_n = ((tail_frac * n as f64 - 1e-9).ceil() as usize).clamp(1, n);
    let cutoff_idx = tail_n - 1;

    let var_95 = (-sorted[cutoff_idx]).max(0.0);
    let cvar_95 = {
        let tail_mean = sorted[..tail_n].iter().sum::<f64>() / tail_n as f64;
        (-tail_mean).max(0.0)
    };
    (var_95, cvar_95)
}

/// Benchmark-relative alpha/beta/cumulative-return analytics (quantwave-b5gr).
///
/// `strategy_returns` and `benchmark_returns` are per-bar simple returns, aligned
/// by index (truncated to the shorter of the two). Returns `None` if either series
/// has fewer than 2 aligned observations or the benchmark has ~zero variance.
fn compute_benchmark_metrics(
    strategy_returns: &[f64],
    benchmark_returns: &[f64],
) -> Option<BenchmarkMetrics> {
    let n = strategy_returns.len().min(benchmark_returns.len());
    if n < 2 {
        return None;
    }
    let rs = &strategy_returns[..n];
    let rb = &benchmark_returns[..n];

    let mean_s = rs.iter().sum::<f64>() / n as f64;
    let mean_b = rb.iter().sum::<f64>() / n as f64;

    let cov: f64 = rs
        .iter()
        .zip(rb)
        .map(|(s, b)| (s - mean_s) * (b - mean_b))
        .sum::<f64>()
        / (n - 1) as f64;
    let var_b: f64 = rb.iter().map(|b| (b - mean_b).powi(2)).sum::<f64>() / (n - 1) as f64;

    if var_b <= f64::EPSILON {
        return None;
    }

    let beta = cov / var_b;
    let alpha = (mean_s - beta * mean_b) * TRADING_DAYS_PER_YEAR;

    let cumulative_return = rs.iter().fold(1.0, |acc, r| acc * (1.0 + r)) - 1.0;
    let benchmark_cumulative_return = rb.iter().fold(1.0, |acc, r| acc * (1.0 + r)) - 1.0;

    Some(BenchmarkMetrics {
        alpha,
        beta,
        cumulative_return,
        benchmark_cumulative_return,
        excess_cumulative_return: cumulative_return - benchmark_cumulative_return,
    })
}

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

    if let Ok(sym_col) = result.equity_curve.column("symbol")
        && let Ok(sym_ca) = sym_col.str()
    {
        return eq_ca
            .into_iter()
            .zip(sym_ca)
            .filter_map(|(eq, sym)| if sym.is_none() { eq } else { None })
            .collect();
    }

    eq_ca.into_iter().flatten().collect()
}

fn equity_first(result: &BacktestResult) -> Option<f64> {
    portfolio_equity_values(result).first().copied()
}

fn equity_last(result: &BacktestResult) -> Option<f64> {
    portfolio_equity_values(result).last().copied()
}

#[cfg(test)]
mod additive_metrics_tests {
    use super::*;

    /// Beta of a series against itself is 1.0, and alpha is 0.0 (analytic case).
    #[test]
    fn beta_of_series_vs_itself_is_one_alpha_is_zero() {
        let returns = vec![0.01, -0.02, 0.015, 0.005, -0.01, 0.02, 0.0, -0.005, 0.012];
        let m = compute_benchmark_metrics(&returns, &returns).expect("benchmark metrics");
        assert!((m.beta - 1.0).abs() < 1e-9, "beta = {}", m.beta);
        assert!(m.alpha.abs() < 1e-9, "alpha = {}", m.alpha);
        assert!((m.cumulative_return - m.benchmark_cumulative_return).abs() < 1e-12);
        assert!(m.excess_cumulative_return.abs() < 1e-12);
    }

    /// Doubling the benchmark's moves (r_s = 2 * r_b) gives beta = 2, alpha = 0.
    #[test]
    fn beta_scales_with_leverage() {
        let benchmark: Vec<f64> = vec![0.01, -0.02, 0.015, 0.005, -0.01, 0.02, 0.0, -0.005];
        let strategy: Vec<f64> = benchmark.iter().map(|r| r * 2.0).collect();
        let m = compute_benchmark_metrics(&strategy, &benchmark).expect("benchmark metrics");
        assert!((m.beta - 2.0).abs() < 1e-9, "beta = {}", m.beta);
        assert!(m.alpha.abs() < 1e-9, "alpha = {}", m.alpha);
    }

    /// A constant offset above the benchmark on every bar produces positive alpha,
    /// beta unchanged.
    #[test]
    fn alpha_reflects_constant_outperformance() {
        let benchmark: Vec<f64> = vec![0.01, -0.02, 0.015, 0.005, -0.01, 0.02, 0.0, -0.005];
        let offset = 0.001;
        let strategy: Vec<f64> = benchmark.iter().map(|r| r + offset).collect();
        let m = compute_benchmark_metrics(&strategy, &benchmark).expect("benchmark metrics");
        assert!((m.beta - 1.0).abs() < 1e-9, "beta = {}", m.beta);
        assert!(
            (m.alpha - offset * TRADING_DAYS_PER_YEAR).abs() < 1e-9,
            "alpha = {}",
            m.alpha
        );
    }

    /// Zero-variance benchmark (flat) yields no benchmark metrics (undefined beta).
    #[test]
    fn zero_variance_benchmark_returns_none() {
        let strategy = vec![0.01, -0.02, 0.015, 0.005];
        let flat_benchmark = vec![0.0, 0.0, 0.0, 0.0];
        assert!(compute_benchmark_metrics(&strategy, &flat_benchmark).is_none());
    }

    /// Too-short series (< 2 aligned points) yields None.
    #[test]
    fn short_series_returns_none() {
        assert!(compute_benchmark_metrics(&[0.01], &[0.02]).is_none());
        assert!(compute_benchmark_metrics(&[], &[]).is_none());
    }

    #[test]
    fn calmar_ratio_basic() {
        // 20% CAGR, 10% max drawdown -> Calmar = 2.0.
        assert!((compute_calmar(0.20, 0.10) - 2.0).abs() < 1e-12);
    }

    #[test]
    fn calmar_ratio_zero_drawdown_positive_cagr_is_infinite() {
        assert_eq!(compute_calmar(0.05, 0.0), f64::INFINITY);
    }

    #[test]
    fn calmar_ratio_zero_drawdown_nonpositive_cagr_is_zero() {
        assert_eq!(compute_calmar(0.0, 0.0), 0.0);
        assert_eq!(compute_calmar(-0.05, 0.0), 0.0);
    }

    #[test]
    fn var_cvar_on_known_distribution() {
        // 20 uniformly spaced returns from -0.10 to +0.09 (step 0.01), sorted.
        // At 95% confidence, tail_n = ceil(0.05 * 20) = 1 -> worst single observation.
        let returns: Vec<f64> = (0..20).map(|i| -0.10 + i as f64 * 0.01).collect();
        let (var_95, cvar_95) = compute_var_cvar(&returns, 0.95);
        assert!((var_95 - 0.10).abs() < 1e-9, "var_95 = {var_95}");
        assert!((cvar_95 - 0.10).abs() < 1e-9, "cvar_95 = {cvar_95}");
    }

    #[test]
    fn var_cvar_wider_tail() {
        // 20 returns, 90% confidence -> tail_n = ceil(0.10 * 20) = 2 -> worst two: -0.10, -0.09.
        let returns: Vec<f64> = (0..20).map(|i| -0.10 + i as f64 * 0.01).collect();
        let (var_90, cvar_90) = compute_var_cvar(&returns, 0.90);
        assert!((var_90 - 0.09).abs() < 1e-9, "var_90 = {var_90}");
        assert!((cvar_90 - 0.095).abs() < 1e-9, "cvar_90 = {cvar_90}");
    }

    #[test]
    fn var_cvar_empty_returns_zero() {
        assert_eq!(compute_var_cvar(&[], 0.95), (0.0, 0.0));
    }

    #[test]
    fn with_benchmark_builder_attaches_metrics() {
        let base = PerformanceMetrics {
            num_trades: 1.0,
            win_rate: 1.0,
            profit_factor: 1.0,
            max_drawdown_pct: 0.0,
            cagr: 0.0,
            sharpe_ratio: 0.0,
            sortino_ratio: 0.0,
            total_return: 0.0,
            final_equity: 100.0,
            avg_trade_pnl: 0.0,
            calmar_ratio: 0.0,
            var_95: 0.0,
            cvar_95: 0.0,
            benchmark: None,
        };
        let returns = vec![0.01, -0.02, 0.015, 0.005];
        let with_bench = base.with_benchmark(&returns, &returns);
        let b = with_bench.benchmark.expect("benchmark attached");
        assert!((b.beta - 1.0).abs() < 1e-9);
    }
}
