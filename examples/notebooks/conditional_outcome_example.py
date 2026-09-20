"""Conditional-outcome query example.

Plain runnable script (same style as robustness_tearsheet_example.py in this
directory) demonstrating `quantwave.research.conditional_outcome`: "given
this condition on my data, how often did this outcome occur" -- with a
sample size, a 95% Wilson confidence interval, a minimum-sample-size guard,
and a chronological stability split, rather than a bare percentage.

Pipeline: deterministic synthetic OHLCV (same generator style as
strategy_backtest.py) -> a toy RSI-like oscillator + a forward-return label
column, both hand-built here (no `.ta` dependency needed to demonstrate the
query layer) -> `conditional_outcome()` on a couple of conditions, including
one that is deliberately too small to meet the min-sample guard.

Usage:
    python3 docs/examples/notebooks/conditional_outcome_example.py
"""

from __future__ import annotations

import numpy as np
import polars as pl

from quantwave.research import conditional_outcome


def generate_synthetic_frame(n: int = 2000) -> pl.DataFrame:
    """Deterministic (no RNG) oscillator + forward-return label, built so
    that "oscillator deeply oversold" is genuinely (if noisily) predictive
    of a positive forward return -- a synthetic "edge" to query."""
    t = np.arange(n, dtype=np.float64)
    # An oscillator in roughly [0, 100], mixing a couple of frequencies.
    osc = 50.0 + 40.0 * np.sin(t * 0.05) + 8.0 * np.sin(t * 0.31)
    osc = np.clip(osc, 0.0, 100.0)
    # Forward return: mean-reverting bias when osc is extreme, plus noise
    # (a fixed, deterministic "noise" term derived from t, not randomness,
    # to keep the script fully reproducible without seeding an RNG).
    mean_reversion_bias = (50.0 - osc) * 0.0006
    wobble = 0.01 * np.sin(t * 1.7 + 0.3)
    fwd_return_5 = mean_reversion_bias + wobble
    return pl.DataFrame({"osc": osc, "fwd_return_5": fwd_return_5})


def main() -> None:
    df = generate_synthetic_frame(2000)

    print("=== Query 1: oscillator < 15 -> forward return positive? ===")
    result = conditional_outcome(
        df,
        outcome=pl.col("fwd_return_5") > 0,
        condition=pl.col("osc") < 15,
        min_samples=30,
    )
    _print_result(result)

    print("\n=== Query 2: oscillator < 2 (very rare) -> forward return positive? ===")
    rare_result = conditional_outcome(
        df,
        outcome=pl.col("fwd_return_5") > 0,
        condition=pl.col("osc") < 2,
        min_samples=30,
    )
    _print_result(rare_result)  # expect computed=False -- too few matching rows


def _print_result(result: dict) -> None:
    print(f"n = {result['n']}")
    if not result["computed"]:
        print(
            f"computed = False (min_samples_met={result['min_samples_met']}) "
            "-- not enough eligible rows to report an honest estimate."
        )
        return
    print(
        f"point_estimate = {result['point_estimate']:.3f}  "
        f"95% Wilson CI = [{result['wilson_ci_low']:.3f}, {result['wilson_ci_high']:.3f}]"
    )
    for label, half in (("first_half", result["first_half"]), ("second_half", result["second_half"])):
        if half is None or not half["computed"]:
            print(f"  {label}: n={half['n'] if half else 0} (below min_samples, not computed)")
        else:
            print(
                f"  {label}: n={half['n']} point_estimate={half['point_estimate']:.3f} "
                f"CI=[{half['wilson_ci_low']:.3f}, {half['wilson_ci_high']:.3f}]"
            )


if __name__ == "__main__":
    main()
