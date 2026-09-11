"""Robustness tearsheet: KPIs, risk metrics, drawdown analysis, and
overfitting/robustness diagnostics for a quantwave ``BacktestReport``.

This module is the Python-side complement to :mod:`quantwave.tearsheet`
(which wraps the Rust-native ``BacktestReport.to_html()``). The native HTML
already covers the equity curve, drawdown plot, monthly heatmap, rolling
Sharpe, and trade blotter. This module adds the layer described in
``docs/backtest/reference/tearsheet_template.md``:

* A structured, dict-of-dataclasses result (:func:`compute_tearsheet`)
  mirroring the reference doc's table groupings (KPI summary, comprehensive
  performance/risk matrix per IS/OOS/Full/Benchmark, top-N drawdowns,
  monthly-returns heatmap, and an overfitting/robustness audit).
* Best-effort implementations of the "needs new implementation" robustness
  statistics: Deflated Sharpe Ratio, Probabilistic Sharpe Ratio, Monte Carlo
  trade-resampling drawdown distribution, and a friction/slippage-breakeven
  estimate. Each cites its formula source in its docstring.
* Fama-French residual alpha, which genuinely needs externally supplied
  factor-return data — this reports ``computed=False`` (never fabricated)
  when no ``factor_returns`` frame is supplied.

Everything here is pure Python + Polars (no numpy/scipy dependency — this
package only ships ``polars`` as an optional core dependency; see
``quantwave-py/pyproject.toml``).
"""

from __future__ import annotations

import math
import random
import statistics
from dataclasses import dataclass, asdict
from datetime import datetime
from typing import Any, Callable, Optional, Sequence, Union

import polars as pl

from quantwave.quantstats_interop import backtest_returns as _backtest_returns

# Euler-Mascheroni constant, used by the DSR "expected max Sharpe of N
# trials" formula (Bailey & Lopez de Prado 2014, eq. 10).
_EULER_MASCHERONI = 0.5772156649015329


# ---------------------------------------------------------------------------
# Low-level statistics helpers (pure Python — no numpy/scipy dependency).
# ---------------------------------------------------------------------------


def _mean(xs: Sequence[float]) -> float:
    return statistics.fmean(xs) if xs else float("nan")


def _stdev(xs: Sequence[float], ddof: int = 1) -> float:
    n = len(xs)
    if n - ddof <= 0:
        return float("nan")
    m = _mean(xs)
    var = sum((x - m) ** 2 for x in xs) / (n - ddof)
    return math.sqrt(var)


def _skewness(xs: Sequence[float]) -> float:
    """Sample skewness (Fisher-Pearson, population moment ratio)."""
    n = len(xs)
    if n < 3:
        return float("nan")
    m = _mean(xs)
    s = _stdev(xs, ddof=0)
    if not s or math.isnan(s):
        return float("nan")
    return sum((x - m) ** 3 for x in xs) / n / s**3


def _kurtosis(xs: Sequence[float]) -> float:
    """Excess kurtosis (normal distribution -> 0.0)."""
    n = len(xs)
    if n < 4:
        return float("nan")
    m = _mean(xs)
    s = _stdev(xs, ddof=0)
    if not s or math.isnan(s):
        return float("nan")
    return sum((x - m) ** 4 for x in xs) / n / s**4 - 3.0


def _percentile(xs: Sequence[float], q: float) -> float:
    """Linear-interpolation percentile, q in [0, 1]."""
    if not xs:
        return float("nan")
    s = sorted(xs)
    if len(s) == 1:
        return s[0]
    pos = q * (len(s) - 1)
    lo = int(math.floor(pos))
    hi = int(math.ceil(pos))
    if lo == hi:
        return s[lo]
    frac = pos - lo
    return s[lo] * (1 - frac) + s[hi] * frac


def _norm_cdf(x: float) -> float:
    """Standard normal CDF via ``math.erf`` (exact to double precision)."""
    return 0.5 * (1.0 + math.erf(x / math.sqrt(2.0)))


def _norm_ppf(p: float) -> float:
    """Inverse standard normal CDF (Acklam's rational approximation).

    Peter Acklam's algorithm (public domain), accurate to ~1.15e-9 absolute
    error across (0, 1). Used by the DSR/PSR formulas below, which need
    ``Phi^-1`` and we do not depend on scipy.
    """
    if p <= 0.0:
        return float("-inf")
    if p >= 1.0:
        return float("inf")

    a = [-3.969683028665376e01, 2.209460984245205e02, -2.759285104469687e02,
         1.383577518672690e02, -3.066479806614716e01, 2.506628277459239e00]
    b = [-5.447609879822406e01, 1.615858368580409e02, -1.556989798598866e02,
         6.680131188771972e01, -1.328068155288572e01]
    c = [-7.784894002430293e-03, -3.223964580411365e-01, -2.400758277161838e00,
         -2.549732539343734e00, 4.374664141464968e00, 2.938163982698783e00]
    d = [7.784695709041462e-03, 3.224671290700398e-01, 2.445134137142996e00,
         3.754408661907416e00]

    p_low = 0.02425
    p_high = 1 - p_low

    if p < p_low:
        q = math.sqrt(-2 * math.log(p))
        return (((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5]) / \
               ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1)
    if p <= p_high:
        q = p - 0.5
        r = q * q
        return (((((a[0] * r + a[1]) * r + a[2]) * r + a[3]) * r + a[4]) * r + a[5]) * q / \
               (((((b[0] * r + b[1]) * r + b[2]) * r + b[3]) * r + b[4]) * r + 1)
    q = math.sqrt(-2 * math.log(1 - p))
    return -(((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5]) / \
            ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1)


# ---------------------------------------------------------------------------
# Equity curve / returns / drawdown extraction.
# ---------------------------------------------------------------------------


def _equity_rows(equity_curve: pl.DataFrame) -> list[tuple[int, float]]:
    frame = equity_curve.select(["ts", "equity"]).sort("ts")
    return list(zip(frame["ts"].to_list(), frame["equity"].to_list()))


def bar_returns(equity_curve: pl.DataFrame) -> list[float]:
    """Per-bar simple returns from an ``equity_curve`` polars frame."""
    equities = [e for _, e in _equity_rows(equity_curve)]
    return [
        equities[i] / equities[i - 1] - 1.0
        for i in range(1, len(equities))
        if equities[i - 1] != 0
    ]


@dataclass(frozen=True)
class DrawdownPeriod:
    """One peak-to-recovery drawdown episode."""

    rank: int
    peak_ts: int
    valley_ts: int
    recovery_ts: Optional[int]  # None if not yet recovered by end of series
    depth_pct: float  # positive fraction, e.g. 0.159 = -15.9%
    duration_bars: int  # peak -> recovery (or peak -> end if unrecovered)


def drawdown_episodes(equity_curve: pl.DataFrame) -> list[DrawdownPeriod]:
    """All peak-to-recovery drawdown episodes in the equity curve, unranked
    (see :func:`top_drawdowns` for the sorted top-N view)."""
    rows = _equity_rows(equity_curve)
    if len(rows) < 2:
        return []

    episodes: list[DrawdownPeriod] = []
    peak_ts, peak_eq = rows[0]
    peak_idx = 0
    in_drawdown = False
    valley_ts, valley_eq = peak_ts, peak_eq

    for idx, (ts, eq) in enumerate(rows):
        if eq >= peak_eq:
            if in_drawdown:
                # Recovered: peak_eq reached/exceeded again.
                depth = 1.0 - valley_eq / peak_eq
                episodes.append(
                    DrawdownPeriod(
                        rank=0,
                        peak_ts=peak_ts,
                        valley_ts=valley_ts,
                        recovery_ts=ts,
                        depth_pct=depth,
                        duration_bars=idx - peak_idx,
                    )
                )
                in_drawdown = False
            peak_ts, peak_eq, peak_idx = ts, eq, idx
            valley_ts, valley_eq = ts, eq
        else:
            in_drawdown = True
            if eq < valley_eq:
                valley_ts, valley_eq = ts, eq

    if in_drawdown:
        depth = 1.0 - valley_eq / peak_eq
        episodes.append(
            DrawdownPeriod(
                rank=0,
                peak_ts=peak_ts,
                valley_ts=valley_ts,
                recovery_ts=None,
                depth_pct=depth,
                duration_bars=len(rows) - 1 - peak_idx,
            )
        )
    return episodes


def top_drawdowns(equity_curve: pl.DataFrame, n: int = 5) -> list[DrawdownPeriod]:
    """Top ``n`` drawdown episodes by depth, ranked 1..n (deepest first)."""
    episodes = sorted(drawdown_episodes(equity_curve), key=lambda d: -d.depth_pct)[:n]
    return [
        DrawdownPeriod(
            rank=i + 1,
            peak_ts=d.peak_ts,
            valley_ts=d.valley_ts,
            recovery_ts=d.recovery_ts,
            depth_pct=d.depth_pct,
            duration_bars=d.duration_bars,
        )
        for i, d in enumerate(episodes)
    ]


def max_drawdown_pct(equity_curve: pl.DataFrame) -> float:
    episodes = drawdown_episodes(equity_curve)
    return max((d.depth_pct for d in episodes), default=0.0)


def longest_drawdown_duration(equity_curve: pl.DataFrame) -> int:
    episodes = drawdown_episodes(equity_curve)
    return max((d.duration_bars for d in episodes), default=0)


# ---------------------------------------------------------------------------
# Monthly returns heatmap (reuses quantstats_interop's returns derivation).
# ---------------------------------------------------------------------------


def monthly_returns_heatmap(report: Any) -> dict[int, dict[int, float]]:
    """Compound daily returns into a ``{year: {month: return_fraction}}`` map.

    Derives daily returns via :func:`quantwave.quantstats_interop.backtest_returns`
    (freq="1d"), then compounds within each ``(year, month)`` bucket:
    ``prod(1 + r) - 1``.
    """
    daily = _backtest_returns(report, freq="1d")
    if daily.is_empty():
        return {}
    daily = daily.with_columns(
        [pl.col("ts").dt.year().alias("year"), pl.col("ts").dt.month().alias("month")]
    )
    grouped = (
        daily.group_by(["year", "month"])
        .agg(((pl.col("return") + 1.0).product() - 1.0).alias("monthly_return"))
        .sort(["year", "month"])
    )
    out: dict[int, dict[int, float]] = {}
    for year, month, ret in zip(
        grouped["year"].to_list(), grouped["month"].to_list(), grouped["monthly_return"].to_list()
    ):
        out.setdefault(year, {})[month] = ret
    return out


def annual_returns(monthly: dict[int, dict[int, float]]) -> dict[int, float]:
    """Compound each year's monthly returns into an annual return."""
    return {
        year: math.prod(1.0 + r for r in months.values()) - 1.0
        for year, months in monthly.items()
    }


# ---------------------------------------------------------------------------
# Directly-derivable performance/risk metric group (per-period).
# ---------------------------------------------------------------------------


@dataclass(frozen=True)
class PeriodMetrics:
    """One column of the "Comprehensive Performance & Risk Matrix" (IS / OOS
    / Full / Benchmark) from ``tearsheet_template.md`` section 2."""

    num_bars: int
    cumulative_return: float
    cagr: float
    volatility: float
    sharpe_ratio: float
    sortino_ratio: float
    calmar_ratio: float
    omega_ratio: float
    max_drawdown_pct: float
    longest_dd_duration_bars: int
    var_95: float
    var_99: float
    cvar_95: float
    skewness: float
    kurtosis: float
    num_trades: int
    win_rate: float
    profit_factor: float
    payoff_ratio: float
    expectancy: float
    max_consecutive_losses: int
    annual_turnover: Optional[float]
    market_exposure_pct: Optional[float]
    avg_trade_duration_bars: Optional[float]

    def as_dict(self) -> dict[str, Any]:
        return asdict(self)


def _sharpe(returns: Sequence[float], rf_per_period: float, periods_per_year: int) -> float:
    if len(returns) < 2:
        return float("nan")
    excess = [r - rf_per_period for r in returns]
    sd = _stdev(excess, ddof=1)
    if not sd:
        return float("nan")
    return _mean(excess) / sd * math.sqrt(periods_per_year)


def _sortino(returns: Sequence[float], rf_per_period: float, periods_per_year: int) -> float:
    if len(returns) < 2:
        return float("nan")
    excess = [r - rf_per_period for r in returns]
    downside = [min(0.0, e) for e in excess]
    dd = math.sqrt(sum(x * x for x in downside) / len(downside)) if downside else float("nan")
    if not dd:
        return float("nan")
    return _mean(excess) / dd * math.sqrt(periods_per_year)


def _omega(returns: Sequence[float], threshold: float = 0.0) -> float:
    gains = sum(r - threshold for r in returns if r > threshold)
    losses = -sum(r - threshold for r in returns if r < threshold)
    if losses == 0:
        return float("nan")
    return gains / losses


def _cagr(equities: Sequence[float], first_ts: int, last_ts: int) -> float:
    if len(equities) < 2 or equities[0] <= 0:
        return float("nan")
    years = max((last_ts - first_ts) / (365.25 * 86400.0), 1e-9)
    total_return = equities[-1] / equities[0]
    if total_return <= 0:
        return float("nan")
    return total_return ** (1.0 / years) - 1.0


def _closed_trades(trades: pl.DataFrame) -> pl.DataFrame:
    """Trades that were actually executed out of a position (has a fill
    price on both legs). A trade with ``exit_price`` set but
    ``exit_fill_price`` null is a mark-to-market valuation of a still-open
    position at series end, not a realized exit, and is excluded."""
    if trades.is_empty():
        return trades
    cols = trades.columns
    mask = pl.col("pnl_net").is_not_null() if "pnl_net" in cols else pl.lit(True)
    if "exit_fill_price" in cols:
        mask = mask & pl.col("exit_fill_price").is_not_null()
    return trades.filter(mask)


def _trade_stats(
    trades: pl.DataFrame,
) -> tuple[float, float, float, float, int, Optional[float]]:
    """win_rate, profit_factor, payoff_ratio, expectancy, max_consecutive_losses,
    avg_trade_duration_bars from a trades frame with ``pnl_net``/``entry_ts``/``exit_ts``.

    Only closed trades (non-null ``pnl_net``) are considered — open trades at
    the end of the run have no realized PnL yet."""
    if trades.is_empty():
        return (float("nan"), float("nan"), float("nan"), float("nan"), 0, None)
    trades = _closed_trades(trades)
    if trades.is_empty():
        return (float("nan"), float("nan"), float("nan"), float("nan"), 0, None)
    pnls = trades["pnl_net"].to_list()
    wins = [p for p in pnls if p > 0]
    losses = [p for p in pnls if p < 0]
    win_rate = len(wins) / len(pnls) if pnls else float("nan")
    gross_profit = sum(wins)
    gross_loss = -sum(losses)
    profit_factor = gross_profit / gross_loss if gross_loss > 0 else float("nan")
    avg_win = _mean(wins) if wins else float("nan")
    avg_loss = _mean([-l for l in losses]) if losses else float("nan")
    payoff_ratio = (
        avg_win / avg_loss if losses and wins and avg_loss not in (0, float("nan")) else float("nan")
    )
    expectancy = _mean(pnls)

    max_consec = 0
    cur = 0
    for p in pnls:
        if p < 0:
            cur += 1
            max_consec = max(max_consec, cur)
        else:
            cur = 0

    if "entry_ts" in trades.columns and "exit_ts" in trades.columns:
        durations = [
            (ex - en)
            for en, ex in zip(trades["entry_ts"].to_list(), trades["exit_ts"].to_list())
            if en is not None and ex is not None
        ]
        avg_duration = _mean(durations) if durations else None
    else:
        avg_duration = None

    return (win_rate, profit_factor, payoff_ratio, expectancy, max_consec, avg_duration)


def _market_exposure_pct(equity_curve: pl.DataFrame) -> Optional[float]:
    if "position" not in equity_curve.columns or equity_curve.is_empty():
        return None
    positions = equity_curve["position"].to_list()
    active = sum(1 for p in positions if p not in (0, 0.0, None))
    return active / len(positions)


def _annual_turnover(trades: pl.DataFrame, first_ts: int, last_ts: int) -> Optional[float]:
    """Best-effort annualized turnover: sum of trade notional (entry leg)
    divided by elapsed years, expressed as a fraction (1.0 = 100%). Caller
    is responsible for dividing by average capital if a % of AUM figure is
    wanted; this returns raw annualized traded notional / initial_cash-scale
    is left to the caller since notional-to-equity normalization needs the
    report's initial cash, which this helper does not have."""
    if trades.is_empty() or "entry_price" not in trades.columns or "quantity" not in trades.columns:
        return None
    notional = sum(
        abs(p * q)
        for p, q in zip(trades["entry_price"].to_list(), trades["quantity"].to_list())
    )
    years = max((last_ts - first_ts) / (365.25 * 86400.0), 1e-9)
    return notional / years


def compute_period_metrics(
    equity_curve: pl.DataFrame,
    trades: pl.DataFrame,
    *,
    risk_free_rate: float = 0.0,
    periods_per_year: int = 252,
    initial_cash: Optional[float] = None,
) -> PeriodMetrics:
    """Compute the full :class:`PeriodMetrics` group for one equity-curve
    slice (a full run, or an IS/OOS sub-slice)."""
    rows = _equity_rows(equity_curve)
    equities = [e for _, e in rows]
    returns = bar_returns(equity_curve)
    rf_per_period = risk_free_rate / periods_per_year

    if len(rows) >= 2:
        cumulative_return = equities[-1] / equities[0] - 1.0
        cagr = _cagr(equities, rows[0][0], rows[-1][0])
    else:
        cumulative_return = float("nan")
        cagr = float("nan")

    volatility = _stdev(returns, ddof=1) * math.sqrt(periods_per_year) if returns else float("nan")
    sharpe = _sharpe(returns, rf_per_period, periods_per_year)
    sortino = _sortino(returns, rf_per_period, periods_per_year)
    mdd = max_drawdown_pct(equity_curve)
    calmar = cagr / mdd if mdd else float("nan")
    omega = _omega(returns)
    var_95 = -_percentile(returns, 0.05) if returns else float("nan")
    var_99 = -_percentile(returns, 0.01) if returns else float("nan")
    tail_95 = [r for r in returns if r <= -var_95] if returns else []
    cvar_95 = -_mean(tail_95) if tail_95 else float("nan")
    skewness = _skewness(returns)
    kurt = _kurtosis(returns)

    win_rate, profit_factor, payoff_ratio, expectancy, max_consec, avg_duration = _trade_stats(trades)
    exposure = _market_exposure_pct(equity_curve)
    turnover_notional = _annual_turnover(trades, rows[0][0], rows[-1][0]) if len(rows) >= 2 else None
    turnover = turnover_notional / initial_cash if (turnover_notional is not None and initial_cash) else None

    return PeriodMetrics(
        num_bars=len(rows),
        cumulative_return=cumulative_return,
        cagr=cagr,
        volatility=volatility,
        sharpe_ratio=sharpe,
        sortino_ratio=sortino,
        calmar_ratio=calmar,
        omega_ratio=omega,
        max_drawdown_pct=mdd,
        longest_dd_duration_bars=longest_drawdown_duration(equity_curve),
        var_95=var_95,
        var_99=var_99,
        cvar_95=cvar_95,
        skewness=skewness,
        kurtosis=kurt,
        num_trades=_closed_trades(trades).height,
        win_rate=win_rate,
        profit_factor=profit_factor,
        payoff_ratio=payoff_ratio,
        expectancy=expectancy,
        max_consecutive_losses=max_consec,
        annual_turnover=turnover,
        market_exposure_pct=exposure,
        avg_trade_duration_bars=avg_duration,
    )


def _split_equity_curve(
    equity_curve: pl.DataFrame, split_ts: int
) -> tuple[pl.DataFrame, pl.DataFrame]:
    is_slice = equity_curve.filter(pl.col("ts") < split_ts)
    oos_slice = equity_curve.filter(pl.col("ts") >= split_ts)
    return is_slice, oos_slice


def _split_trades(trades: pl.DataFrame, split_ts: int) -> tuple[pl.DataFrame, pl.DataFrame]:
    if trades.is_empty() or "exit_ts" not in trades.columns:
        return trades, trades
    is_trades = trades.filter(pl.col("exit_ts") < split_ts)
    oos_trades = trades.filter(pl.col("exit_ts") >= split_ts)
    return is_trades, oos_trades


# ---------------------------------------------------------------------------
# Overfitting / robustness audit (section 3 of the reference doc).
# ---------------------------------------------------------------------------


def probabilistic_sharpe_ratio(
    observed_sharpe: float,
    benchmark_sharpe: float,
    num_observations: int,
    skewness: float,
    kurtosis: float,
) -> float:
    """Probabilistic Sharpe Ratio (PSR).

    Source: Bailey, D.H. & Lopez de Prado, M. (2012), "The Sharpe Ratio
    Efficient Frontier", *Journal of Risk*, eq. 7-9. Returns the probability
    that the *true* (per-period) Sharpe ratio exceeds ``benchmark_sharpe``,
    given the observed (per-period, not annualized) Sharpe estimate,
    ``num_observations`` return samples, and the sample skewness/(regular,
    non-excess) kurtosis of the return series.

    ``skewness``/``kurtosis`` here follow this module's convention: ordinary
    (non-excess) kurtosis is ``_kurtosis(...) + 3``.
    """
    if num_observations < 2 or math.isnan(observed_sharpe):
        return float("nan")
    kurt = kurtosis + 3.0  # this module's _kurtosis() is excess kurtosis
    denom = 1.0 - skewness * observed_sharpe + (kurt - 1.0) / 4.0 * observed_sharpe**2
    if denom <= 0:
        return float("nan")
    z = (observed_sharpe - benchmark_sharpe) * math.sqrt(num_observations - 1) / math.sqrt(denom)
    return _norm_cdf(z)


def deflated_sharpe_ratio(
    observed_sharpe: float,
    num_observations: int,
    skewness: float,
    kurtosis: float,
    num_trials: int = 1,
) -> float:
    """Deflated Sharpe Ratio (DSR).

    Source: Bailey, D.H. & Lopez de Prado, M. (2014), "The Deflated Sharpe
    Ratio: Correcting for Selection Bias, Backtest Overfitting and
    Non-Normality", *Journal of Portfolio Management*, eq. 10-13.

    Deflates the observed (per-period) Sharpe ratio by the expected maximum
    Sharpe ratio one would observe across ``num_trials`` independent trials
    under the null of zero true skill, then evaluates PSR at that inflated
    benchmark. ``num_trials`` should be the number of strategy variants /
    parameter combinations actually searched before selecting this one
    (default 1 = no multiple-testing correction, i.e. DSR == PSR(0)).

    Simplification (matches common open-source implementations, e.g.
    mlfinlab): the variance of each trial's Sharpe estimate is assumed equal
    to this strategy's own Sharpe standard error (we do not have access to
    the per-trial Sharpe distribution, only this one realized backtest).
    """
    if num_observations < 2 or math.isnan(observed_sharpe):
        return float("nan")
    if num_trials <= 1:
        sr0 = 0.0
    else:
        kurt = kurtosis + 3.0
        # Standard error of the Sharpe estimate under the null (SR=0).
        var_sr = (1.0 - skewness * 0.0 + (kurt - 1.0) / 4.0 * 0.0) / max(num_observations - 1, 1)
        sigma_sr = math.sqrt(var_sr) if var_sr > 0 else float("nan")
        gamma = _EULER_MASCHERONI
        z1 = _norm_ppf(1.0 - 1.0 / num_trials)
        z2 = _norm_ppf(1.0 - 1.0 / (num_trials * math.e))
        sr0 = sigma_sr * ((1.0 - gamma) * z1 + gamma * z2)
    return probabilistic_sharpe_ratio(observed_sharpe, sr0, num_observations, skewness, kurtosis)


@dataclass(frozen=True)
class MonteCarloDrawdown:
    """Monte Carlo trade-resampling drawdown distribution."""

    n_simulations: int
    n_trades_sampled: int
    p50_max_drawdown_pct: float
    p95_max_drawdown_pct: float
    p99_max_drawdown_pct: float


def monte_carlo_drawdown(
    trades: pl.DataFrame,
    initial_cash: float,
    n_simulations: int = 2000,
    seed: int = 42,
) -> MonteCarloDrawdown:
    """Bootstrap closed-trade PnLs (with replacement, order reshuffled) into
    ``n_simulations`` synthetic equity paths and summarize the resulting
    max-drawdown distribution.

    Methodology: standard trade-return bootstrap for strategy robustness
    testing (e.g. Bacon, C. "Practical Portfolio Performance Measurement and
    Attribution", ch. on Monte Carlo simulation; also the approach used by
    this repo's Rust ``quantwave_backtest::monte_carlo`` module for terminal
    equity, extended here to a full drawdown path per simulation).
    """
    if trades.is_empty() or "pnl_net" not in trades.columns:
        return MonteCarloDrawdown(n_simulations, 0, float("nan"), float("nan"), float("nan"))
    closed = _closed_trades(trades)
    if closed.is_empty():
        return MonteCarloDrawdown(n_simulations, 0, float("nan"), float("nan"), float("nan"))
    pnls = closed["pnl_net"].to_list()
    n_trades = len(pnls)
    rng = random.Random(seed)
    max_dds: list[float] = []
    for _ in range(n_simulations):
        equity = initial_cash
        peak = initial_cash
        max_dd = 0.0
        for _ in range(n_trades):
            equity += pnls[rng.randrange(n_trades)]
            peak = max(peak, equity)
            if peak > 0:
                max_dd = max(max_dd, 1.0 - equity / peak)
        max_dds.append(max_dd)
    return MonteCarloDrawdown(
        n_simulations=n_simulations,
        n_trades_sampled=n_trades,
        p50_max_drawdown_pct=_percentile(max_dds, 0.50),
        p95_max_drawdown_pct=_percentile(max_dds, 0.95),
        p99_max_drawdown_pct=_percentile(max_dds, 0.99),
    )


@dataclass(frozen=True)
class SlippageBreakeven:
    """How much extra round-trip friction (in bps) the strategy can absorb
    before its net PnL turns to zero."""

    current_avg_friction_bps: Optional[float]
    breakeven_bps: Optional[float]
    method: str  # "linear_approximation" or "resimulated_binary_search"


def slippage_breakeven(
    trades: pl.DataFrame,
    resim_fn: Optional[Callable[[float], Any]] = None,
    bps_lo: float = 0.0,
    bps_hi: float = 200.0,
    tol_bps: float = 0.1,
) -> SlippageBreakeven:
    """Estimate the round-trip slippage (bps) at which total net PnL hits 0.

    Two modes:

    1. **Exact (``resim_fn`` supplied)**: ``resim_fn(slippage_bps)`` must
       rerun the backtest at the given slippage and return an object with a
       ``.metrics()["total_return"]`` (or a plain float). Binary search on
       ``[bps_lo, bps_hi]`` for the sign change, to within ``tol_bps``. This
       is the "closed-form / binary search on friction sweep" the design doc
       calls for, when the caller can afford to rerun the engine.
    2. **Best-effort closed form (default, no resim)**: without the ability
       to rerun the engine, infer the *currently realized* round-trip
       friction rate from each trade's fill price vs. its (frictionless)
       reference price — ``entry_fill_price``/``exit_fill_price`` vs.
       ``entry_price``/``exit_price`` — and linearly extrapolate: if the
       observed friction rate ``f0`` bps currently costs ``C0`` dollars and
       total net PnL is ``P``, the strategy can absorb an incremental
       ``P / (C0 / f0)`` bps before PnL hits zero (first-order Taylor
       approximation — ignores any behavioral/compounding second-order
       effects of larger frictions, and is therefore reported with
       ``method="linear_approximation"``).
    """
    if resim_fn is not None:
        def total_return_at(bps: float) -> float:
            result = resim_fn(bps)
            metrics = result.metrics() if hasattr(result, "metrics") else result
            return metrics["total_return"] if hasattr(metrics, "__getitem__") else float(metrics)

        lo, hi = bps_lo, bps_hi
        r_lo, r_hi = total_return_at(lo), total_return_at(hi)
        if r_lo <= 0:
            return SlippageBreakeven(None, lo, "resimulated_binary_search")
        if r_hi > 0:
            return SlippageBreakeven(None, hi, "resimulated_binary_search")
        while hi - lo > tol_bps:
            mid = (lo + hi) / 2.0
            if total_return_at(mid) > 0:
                lo = mid
            else:
                hi = mid
        return SlippageBreakeven(None, (lo + hi) / 2.0, "resimulated_binary_search")

    required = {"entry_price", "entry_fill_price", "exit_price", "exit_fill_price", "quantity", "pnl_net", "side"}
    if trades.is_empty() or not required.issubset(trades.columns):
        return SlippageBreakeven(None, None, "linear_approximation")

    closed_trades = _closed_trades(trades)
    if closed_trades.is_empty():
        return SlippageBreakeven(None, None, "linear_approximation")

    total_friction_cost = 0.0
    total_notional = 0.0
    for row in closed_trades.iter_rows(named=True):
        side = row["side"]  # +1 long, -1 short (per backtest_types convention)
        qty = abs(row["quantity"])
        entry_notional = abs(row["entry_price"]) * qty
        exit_notional = abs(row["exit_price"]) * qty
        total_notional += entry_notional + exit_notional
        if side >= 0:
            entry_cost = (row["entry_fill_price"] - row["entry_price"]) * qty
            exit_cost = (row["exit_price"] - row["exit_fill_price"]) * qty
        else:
            entry_cost = (row["entry_price"] - row["entry_fill_price"]) * qty
            exit_cost = (row["exit_fill_price"] - row["exit_price"]) * qty
        total_friction_cost += entry_cost + exit_cost

    if total_notional <= 0:
        return SlippageBreakeven(None, None, "linear_approximation")

    current_bps = (total_friction_cost / total_notional) * 10_000.0
    total_pnl = sum(closed_trades["pnl_net"].to_list())

    if total_friction_cost <= 0:
        # No detectable friction embedded (e.g. zero-cost synthetic run) —
        # cannot extrapolate a per-bps cost rate.
        return SlippageBreakeven(current_bps, None, "linear_approximation")

    cost_per_bp = total_friction_cost / current_bps if current_bps else float("nan")
    breakeven = current_bps + (total_pnl / cost_per_bp if cost_per_bp else float("nan"))
    return SlippageBreakeven(current_bps, max(breakeven, 0.0), "linear_approximation")


def fama_french_alpha(
    strategy_returns: Sequence[float],
    factor_returns: Optional[Any] = None,
) -> dict[str, Any]:
    """Residual alpha from a Fama-French factor regression.

    Source: Fama, E.F. & French, K.R. (2015), "A five-factor asset pricing
    model", *Journal of Financial Economics*. Regresses
    ``strategy_returns - rf`` on the factor columns (typically
    ``mkt_rf, smb, hml, rmw, cma``) via ordinary least squares (implemented
    here with plain Gaussian elimination on the normal equations — no
    numpy/scipy dependency); the regression intercept is the residual alpha,
    reported with its t-statistic.

    This is a genuinely optional input this module cannot fabricate: Fama-
    French factor returns are external data (e.g. Ken French's data library)
    that quantwave has no bundled source for. When ``factor_returns`` is
    ``None`` this returns ``{"computed": False, "reason": ...}`` rather than
    raising or inventing a number.

    ``factor_returns``, when supplied, must be array-like of rows aligned
    1:1 with ``strategy_returns`` (same length, same period), each row a
    sequence of factor values (e.g. ``[mkt_rf, smb, hml, rmw, cma, rf]`` with
    ``rf`` as the last column) OR a polars DataFrame with a ``rf`` column
    and one or more factor columns (any other columns).
    """
    if factor_returns is None:
        return {
            "computed": False,
            "reason": (
                "No factor_returns supplied. Fama-French factors are external "
                "data (e.g. Ken French's data library / a vendor feed) that "
                "quantwave does not bundle or fetch automatically. Pass a "
                "factor_returns DataFrame (columns: factor names + 'rf') "
                "aligned to the same period as the strategy returns to "
                "compute this."
            ),
        }

    if isinstance(factor_returns, pl.DataFrame):
        rf_col = "rf" if "rf" in factor_returns.columns else None
        factor_cols = [c for c in factor_returns.columns if c != rf_col]
        rows = factor_returns.select(factor_cols).rows()
        rf = factor_returns[rf_col].to_list() if rf_col else [0.0] * len(rows)
        factor_names = factor_cols
    else:
        rows = [list(r[:-1]) for r in factor_returns]
        rf = [r[-1] for r in factor_returns]
        factor_names = [f"factor_{i}" for i in range(len(rows[0]))] if rows else []

    n = min(len(strategy_returns), len(rows))
    if n < len(factor_names) + 2:
        return {
            "computed": False,
            "reason": f"Not enough aligned observations ({n}) for {len(factor_names)} factors + intercept.",
        }

    y = [strategy_returns[i] - rf[i] for i in range(n)]
    X = [[1.0] + list(rows[i]) for i in range(n)]  # design matrix with intercept
    k = len(X[0])

    # Normal equations: (X'X) beta = X'y, solved via Gaussian elimination.
    xtx = [[sum(X[r][a] * X[r][b] for r in range(n)) for b in range(k)] for a in range(k)]
    xty = [sum(X[r][a] * y[r] for r in range(n)) for a in range(k)]

    beta = _solve_linear_system(xtx, xty)
    if beta is None:
        return {"computed": False, "reason": "Design matrix is singular (collinear factors)."}

    residuals = [y[i] - sum(X[i][j] * beta[j] for j in range(k)) for i in range(n)]
    dof = n - k
    if dof <= 0:
        return {"computed": False, "reason": "No residual degrees of freedom."}
    sigma2 = sum(r * r for r in residuals) / dof

    xtx_inv = _invert_matrix(xtx)
    alpha = beta[0]
    if xtx_inv is None:
        se_alpha = float("nan")
    else:
        se_alpha = math.sqrt(max(sigma2 * xtx_inv[0][0], 0.0))
    t_stat = alpha / se_alpha if se_alpha else float("nan")

    return {
        "computed": True,
        "alpha": alpha,
        "alpha_annualized": (1 + alpha) ** 252 - 1 if not math.isnan(alpha) else float("nan"),
        "t_stat": t_stat,
        "betas": dict(zip(factor_names, beta[1:])),
        "num_observations": n,
        "residual_std": math.sqrt(sigma2),
    }


def _solve_linear_system(a: list[list[float]], b: list[float]) -> Optional[list[float]]:
    """Gaussian elimination with partial pivoting. Returns None if singular."""
    n = len(a)
    m = [row[:] + [b[i]] for i, row in enumerate(a)]
    for col in range(n):
        pivot = max(range(col, n), key=lambda r: abs(m[r][col]))
        if abs(m[pivot][col]) < 1e-12:
            return None
        m[col], m[pivot] = m[pivot], m[col]
        pv = m[col][col]
        m[col] = [x / pv for x in m[col]]
        for r in range(n):
            if r != col:
                factor = m[r][col]
                m[r] = [m[r][c] - factor * m[col][c] for c in range(n + 1)]
    return [m[i][n] for i in range(n)]


def _invert_matrix(a: list[list[float]]) -> Optional[list[list[float]]]:
    n = len(a)
    cols = []
    for i in range(n):
        e = [1.0 if j == i else 0.0 for j in range(n)]
        col = _solve_linear_system(a, e)
        if col is None:
            return None
        cols.append(col)
    return [[cols[j][i] for j in range(n)] for i in range(n)]


@dataclass(frozen=True)
class RobustnessAudit:
    """Section 3 of the reference doc: overfitting/robustness checks."""

    sharpe_is: Optional[float]
    sharpe_oos: Optional[float]
    sharpe_degradation_pct: Optional[float]  # (oos - is) / |is|
    dsr: float
    dsr_trials: int
    psr: float
    monte_carlo: MonteCarloDrawdown
    slippage: SlippageBreakeven
    fama_french: dict[str, Any]


# ---------------------------------------------------------------------------
# Top-level entry point.
# ---------------------------------------------------------------------------


@dataclass(frozen=True)
class TearsheetResult:
    """Everything :mod:`quantwave.tearsheet` needs to render the reference
    doc's Executive Summary / Perf Matrix / Robustness Audit / Top Drawdowns
    / Monthly Heatmap sections."""

    full: PeriodMetrics
    in_sample: Optional[PeriodMetrics]
    out_of_sample: Optional[PeriodMetrics]
    top_drawdowns: list[DrawdownPeriod]
    monthly_heatmap: dict[int, dict[int, float]]
    annual_returns: dict[int, float]
    robustness: RobustnessAudit
    meta: dict[str, Any]


def compute_tearsheet(
    report: Any,
    *,
    split_date: Optional[Union[datetime, int]] = None,
    factor_returns: Optional[Any] = None,
    risk_free_rate: float = 0.0,
    periods_per_year: int = 252,
    trials: int = 1,
    mc_simulations: int = 2000,
    mc_seed: int = 42,
    top_n_drawdowns: int = 5,
    slippage_resim_fn: Optional[Callable[[float], Any]] = None,
) -> TearsheetResult:
    """Compute the full robustness tearsheet result for a ``BacktestReport``
    (single-symbol or portfolio — both expose ``.equity_curve``, ``.trades``,
    and ``.stats()``).

    Parameters
    ----------
    report:
        A ``quantwave.backtest.BacktestReport`` (or the raw
        ``quantwave._backtest.BacktestReport``).
    split_date:
        Optional in-sample/out-of-sample split point. A ``datetime`` (naive
        treated as UTC) or a raw Unix-seconds ``int``, matching the
        equity_curve's ``ts`` column. Bars strictly before this are IS, from
        it onward are OOS. When ``None``, ``in_sample``/``out_of_sample`` are
        both ``None`` and only ``full`` + audit fields that don't need a
        split (DSR/PSR/MC/slippage/Fama-French) are populated.
    factor_returns:
        Optional Fama-French factor DataFrame — see :func:`fama_french_alpha`.
    trials:
        Number of independent strategy variants searched before selecting
        this one, for the Deflated Sharpe Ratio's multiple-testing
        correction. Default 1 (no correction; DSR reduces to PSR(0)).
    slippage_resim_fn:
        Optional ``f(slippage_bps) -> BacktestReport``-like callable enabling
        an exact binary-search slippage breakeven instead of the linear
        approximation — see :func:`slippage_breakeven`.
    """
    equity_curve = report.equity_curve
    trades = report.trades
    stats = report.stats()
    initial_cash = stats.initial_cash if hasattr(stats, "initial_cash") else stats["initial_cash"]

    full = compute_period_metrics(
        equity_curve, trades,
        risk_free_rate=risk_free_rate, periods_per_year=periods_per_year,
        initial_cash=initial_cash,
    )

    in_sample = out_of_sample = None
    sharpe_is = sharpe_oos = sharpe_degradation = None
    if split_date is not None:
        split_ts = int(split_date.timestamp()) if isinstance(split_date, datetime) else int(split_date)
        is_eq, oos_eq = _split_equity_curve(equity_curve, split_ts)
        is_tr, oos_tr = _split_trades(trades, split_ts)
        if is_eq.height >= 2:
            in_sample = compute_period_metrics(
                is_eq, is_tr, risk_free_rate=risk_free_rate,
                periods_per_year=periods_per_year, initial_cash=initial_cash,
            )
            sharpe_is = in_sample.sharpe_ratio
        if oos_eq.height >= 2:
            out_of_sample = compute_period_metrics(
                oos_eq, oos_tr, risk_free_rate=risk_free_rate,
                periods_per_year=periods_per_year, initial_cash=initial_cash,
            )
            sharpe_oos = out_of_sample.sharpe_ratio
        if sharpe_is and not math.isnan(sharpe_is) and sharpe_is != 0 and sharpe_oos is not None:
            sharpe_degradation = (sharpe_oos - sharpe_is) / abs(sharpe_is)

    returns = bar_returns(equity_curve)
    per_bar_sharpe = _sharpe(returns, risk_free_rate / periods_per_year, 1) if returns else float("nan")
    skewness = _skewness(returns)
    kurt = _kurtosis(returns)
    dsr = deflated_sharpe_ratio(per_bar_sharpe, len(returns), skewness, kurt, num_trials=trials)
    psr = probabilistic_sharpe_ratio(per_bar_sharpe, 0.0, len(returns), skewness, kurt)

    mc = monte_carlo_drawdown(trades, initial_cash, n_simulations=mc_simulations, seed=mc_seed)
    slip = slippage_breakeven(trades, resim_fn=slippage_resim_fn)
    ff = fama_french_alpha(returns, factor_returns)

    robustness = RobustnessAudit(
        sharpe_is=sharpe_is,
        sharpe_oos=sharpe_oos,
        sharpe_degradation_pct=sharpe_degradation,
        dsr=dsr,
        dsr_trials=trials,
        psr=psr,
        monte_carlo=mc,
        slippage=slip,
        fama_french=ff,
    )

    return TearsheetResult(
        full=full,
        in_sample=in_sample,
        out_of_sample=out_of_sample,
        top_drawdowns=top_drawdowns(equity_curve, n=top_n_drawdowns),
        monthly_heatmap=monthly_returns_heatmap(report),
        annual_returns=annual_returns(monthly_returns_heatmap(report)),
        robustness=robustness,
        meta={
            "periods_per_year": periods_per_year,
            "risk_free_rate": risk_free_rate,
            "num_bars": full.num_bars,
            "initial_cash": initial_cash,
        },
    )


__all__ = [
    "PeriodMetrics",
    "DrawdownPeriod",
    "MonteCarloDrawdown",
    "SlippageBreakeven",
    "RobustnessAudit",
    "TearsheetResult",
    "compute_tearsheet",
    "compute_period_metrics",
    "bar_returns",
    "drawdown_episodes",
    "top_drawdowns",
    "max_drawdown_pct",
    "longest_drawdown_duration",
    "monthly_returns_heatmap",
    "annual_returns",
    "probabilistic_sharpe_ratio",
    "deflated_sharpe_ratio",
    "monte_carlo_drawdown",
    "slippage_breakeven",
    "fama_french_alpha",
]
