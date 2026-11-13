"""Prop-firm challenge Monte Carlo simulator example.

Plain runnable script (same style as conditional_outcome_example.py /
robustness_tearsheet_example.py in this directory) demonstrating
`quantwave.propfirm.simulate_prop_firm_challenge`: given a backtest's
closed-trade history, block-bootstrap resample many possible orderings of
those trades and report the probability the strategy would pass a
prop-firm evaluation's rule set (profit target + drawdown/daily-loss
constraints), with an honest Wilson confidence interval.

Pipeline: deterministic synthetic OHLCV (same generator style as
strategy_backtest.py) -> a simple always-long signal ->
`.bt.backtest_with_report()` -> `simulate_prop_firm_challenge()` on the
resulting report, run for both the "static" and "trailing" preset rule
sets so their outputs can be compared side by side.

Usage:
    python3 docs/examples/notebooks/prop_firm_challenge_example.py
"""

from __future__ import annotations

from datetime import datetime, timedelta, timezone

import numpy as np
import polars as pl

import quantwave as qw  # noqa: F401 -- registers LazyFrame.bt
from quantwave.propfirm import simulate_prop_firm_challenge


def generate_deterministic_ohlcv(n: int = 800) -> pl.DataFrame:
    """Deterministic OHLCV (no RNG) -- same shape as strategy_backtest.py's
    generator: a slow secular uptrend plus oscillation, so a long-only
    signal produces a mix of winning and losing round-trip trades. Price
    level is scaled up (~$1,000s) rather than the usual ~$100 demo level so
    that per-trade PnL (quantity=1 unit by default) is large enough
    relative to a $100k prop-firm account to actually approach the
    profit-target / drawdown thresholds within a realistic trade count --
    the whole point of this example is to show a non-degenerate result."""
    t = np.arange(n, dtype=np.float64)
    base = 1000.0 + 0.5 * t
    wobble = 60.0 * np.sin(t * 0.12) + 25.0 * np.sin(t * 0.5)
    close = base + wobble
    ts0 = datetime(2023, 1, 1, tzinfo=timezone.utc)
    timestamps = [int((ts0 + timedelta(hours=int(i))).timestamp()) for i in range(n)]
    return pl.DataFrame(
        {
            "timestamp": timestamps,
            "close": close,
        }
    )


def build_signal(data: pl.DataFrame) -> pl.DataFrame:
    """Simple oscillator-crossing long/flat signal: long whenever price is
    above its own 20-bar rolling mean, flat otherwise -- produces a
    realistic mix of winning and losing round-trip trades on the wobble."""
    rolling_mean = data["close"].rolling_mean(window_size=20, min_samples=1)
    signal = (data["close"] > rolling_mean).cast(pl.Float64)
    return data.with_columns(signal.alias("signal"))


def _print_result(label: str, result: dict) -> None:
    print(f"\n--- {label} ---")
    print(f"n_simulations = {result['n_simulations']}")
    if not result["computed"]:
        print("computed = False -- below the minimum-simulations guard.")
        return
    print(
        f"pass_probability = {result['pass_probability']:.3f}  "
        f"95% Wilson CI = [{result['pass_ci_low']:.3f}, {result['pass_ci_high']:.3f}]"
    )
    bb = result["breach_breakdown"]
    print(
        f"breach_breakdown: max_drawdown={bb['max_drawdown']:.3f}  "
        f"daily_loss={bb['daily_loss']:.3f}  "
        f"ran_out_without_outcome={bb['ran_out_without_outcome']:.3f}"
    )
    tto = result["trades_to_outcome"]
    print(
        f"trades_to_outcome: p5={tto['p5']:.0f}  median={tto['median']:.0f}  "
        f"p95={tto['p95']:.0f}"
    )


def main() -> None:
    data = generate_deterministic_ohlcv(800)
    signal_df = build_signal(data)

    report = signal_df.lazy().bt.backtest_with_report(
        signal="signal",
        timestamp_col="timestamp",
        commission_bps=5.0,
        slippage_bps=2.0,
    )

    n_trades = report.metrics()["num_trades"]
    print(f"Backtest produced {n_trades} closed trades.")

    # Account size chosen to be in scale with this synthetic strategy's
    # per-trade PnL (quantity=1 unit, so dollar PnL per trade is small) --
    # a real prop-firm challenge on a full-size account would pair with a
    # correspondingly larger position size.
    common_kwargs = dict(
        initial_balance=1_000.0,
        profit_target_pct=0.50,
        max_drawdown_pct=0.08,
        max_daily_loss_pct=0.05,
        n_simulations=2000,
        block_size=8,
        seed=42,
    )

    static_result = simulate_prop_firm_challenge(report, rule_set="static", **common_kwargs)
    trailing_result = simulate_prop_firm_challenge(report, rule_set="trailing", **common_kwargs)

    _print_result("static rule set", static_result)
    _print_result("trailing rule set", trailing_result)


if __name__ == "__main__":
    main()
