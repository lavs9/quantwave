"""Prop-firm challenge Monte Carlo simulator: "if I re-ran this strategy's
closed trades in a different order, how likely am I to pass a prop-firm
evaluation" — answered honestly.

This module is a first, deliberately narrow slice of a prop-firm-challenge
simulator (inspired by LuxAlgo's prop-firm-sim tool, but not a port of it —
no exhaustive per-firm rule library, no overnight-holding simulation, no
expected-attempts-across-multiple-challenges modeling). It follows the same
house style as :mod:`quantwave.robustness` and :mod:`quantwave.research`:
pure Python + Polars (no numpy/scipy dependency), and a "can we honestly
compute this" discipline — when too few simulations are requested to say
anything meaningful, the result reports ``"computed": False`` and leaves
the numeric fields as ``None`` rather than a percentage that looks the same
whether it came from 50 simulations or 50,000. The pass-probability
confidence interval reuses :func:`quantwave.research.wilson_interval`
rather than reimplementing Wilson math a second time — a pass/fail outcome
across N simulations is exactly the same binomial-proportion-with-CI
problem that module already solves.

Why this can't reuse ``monte_carlo.rs``
-----------------------------------------
``quantwave-backtest/src/monte_carlo.rs``'s ``monte_carlo_trade_bootstrap``
resamples closed-trade ``pnl_net`` values with single-trade i.i.d.
replacement and only returns a *summary* of terminal equity (mean/p5/p50/
p95/probability_of_loss) — it discards the full per-simulation equity path.
Trailing-drawdown and daily-loss rules are inherently path-dependent (they
need the running peak equity and per-day P&L at every step of *each*
simulated path, not just its terminal value), so that function cannot be
reused as-is. Rather than reshape Rust core code for this first slice, this
module is implemented as a pure-Python analysis layer on top of the trades
DataFrame a report already exposes — the same pattern ``robustness.py`` and
``research.py`` use for analysis that doesn't need new Rust.

Why block bootstrap (not i.i.d.) resampling
---------------------------------------------
This module resamples trades using a **block bootstrap**: contiguous
*chunks* of consecutive trades (in their original, chronological order)
are drawn with replacement and concatenated until the simulated sequence
reaches the original trade count. This is a deliberate improvement over
``monte_carlo_trade_bootstrap``'s single-trade i.i.d. resampling, not a
copy of its weakness: i.i.d. resampling destroys any autocorrelation in
the trade sequence (e.g. a losing streak — trades that cluster because of
a shared regime, a broken parameter, or correlated market conditions —
gets shuffled into independence, understating how bad a real losing streak
can look). Block bootstrap preserves short-range sequence structure by
keeping those runs of trades together as a unit, which is exactly the
concern LuxAlgo's prop-firm-sim research flagged as important for
drawdown-sensitive rule evaluation. The default block size is
``max(1, min(block_size, n_trades))`` trades (default parameter: 8) — a
small-but-not-trivial chunk length chosen as a fixed default rather than a
fraction of ``n_trades`` because prop-firm rule breaches are driven by
*local* streak behavior (a handful of trades in a row), not by long-range
structure, and a fixed absolute size keeps that block length meaningful
regardless of whether the input history has 50 or 5,000 trades. Callers
with unusually long or short trade histories should tune ``block_size``
explicitly.

The two rule sets
-------------------
Both rule sets share a profit target and a daily-loss limit; they differ
only in how the max-drawdown floor is anchored:

* ``"static"``: the drawdown floor is fixed at
  ``initial_balance * (1 - max_drawdown_pct)`` for the whole challenge.
  The account fails the instant simulated equity drops to or below this
  fixed floor, regardless of how high equity climbed in between.
* ``"trailing"``: the drawdown floor *trails* the running peak equity —
  ``running_peak_equity * (1 - max_drawdown_pct)``, recomputed after every
  trade as equity reaches new highs. The account fails the instant equity
  drops to or below this floor. Trailing drawdown is strictly at least as
  strict as static drawdown once equity has made a new high above the
  initial balance (the floor only ever moves up from the static floor),
  which is why a path that makes a new high and then gives back gains can
  pass under ``"static"`` but fail under ``"trailing"``.

Both rule sets also enforce:

* A **daily-loss limit**: a fixed fraction of the initial balance,
  ``initial_balance * max_daily_loss_pct``. Trades are grouped by the UTC
  calendar date of their ``exit_ts`` (Unix timestamp, UTC, per
  ``quantwave-backtest/src/lib.rs``'s ``Trade.exit_ts``); the account fails
  the instant any single UTC calendar day's net P&L (summed over that
  day's simulated trades, in simulated order, evaluated trade-by-trade so
  the breach is detected on the exact trade that crosses the line) is more
  negative than ``-initial_balance * max_daily_loss_pct``.
* A **profit target**: the account passes the instant simulated equity
  reaches or exceeds ``initial_balance * (1 + profit_target_pct)``.

Each simulated path is walked trade-by-trade in the resampled order; the
*first* of {profit target reached, max-drawdown breached, daily-loss limit
breached} to occur ends that simulation (first event wins). A path that
exhausts all resampled trades without triggering any of the three is
recorded as a fourth outcome, "ran out" — a real, meaningful result (the
strategy neither clearly passed nor clearly failed within one challenge's
worth of trades) rather than being folded into "fail".

Statistical honesty
--------------------
* ``pass_probability`` is reported alongside a 95% Wilson score interval
  (via :func:`quantwave.research.wilson_interval`) treating "pass" as a
  binomial outcome across ``n_simulations`` independent simulated paths.
* A **minimum-simulations guard** (``n_simulations < 100``) refuses to
  report a probability/CI/breakdown at all (``"computed": False``, numeric
  fields ``None``) — 100 is the same order-of-magnitude floor
  ``research.py`` uses for ``min_samples`` (30) scaled up for a
  Monte-Carlo-simulation-count context where each individual simulation is
  itself a noisy draw over a whole resampled trade history rather than a
  single row, so a much smaller N is far less trustworthy here than it
  would be for a per-row hit rate.

Worked example
---------------
::

    import quantwave as qw  # noqa: F401 -- registers LazyFrame.bt
    from quantwave.propfirm import simulate_prop_firm_challenge

    report = signal_df.lazy().bt.backtest_with_report(
        signal="signal", timestamp_col="timestamp",
    )
    result = simulate_prop_firm_challenge(
        report,
        rule_set="trailing",
        initial_balance=100_000.0,
        profit_target_pct=0.10,
        max_drawdown_pct=0.10,
        max_daily_loss_pct=0.05,
        n_simulations=2000,
        block_size=8,
        seed=42,
    )
    # result["computed"] is True (n_simulations >= 100)
    # result["pass_probability"] ~= fraction of the 2000 resampled paths
    #     that hit the +10% profit target before breaching either
    #     drawdown rule
    # result["pass_ci_low"] / result["pass_ci_high"] bracket that estimate
    # result["breach_breakdown"] partitions the *failing* fraction into
    #     {"max_drawdown": ..., "daily_loss": ..., "ran_out_without_outcome": ...}
    #     (fractions of all simulations, not just of the failures)
    # result["trades_to_outcome"] gives {"p5", "median", "p95"} of how many
    #     resampled trades each simulation took before its outcome

Out of scope for this first slice (tracked as explicit follow-up, not
built here, matching how ``research.py`` documents its own deferred
scope): overnight-holding simulation (this module — like LuxAlgo's
prop-firm-sim — treats each trade as a same-day round trip for the
daily-loss grouping and does not model gap risk on positions held across
the UTC day boundary; that is a documented limitation, not a bug),
exhaustive real-world per-firm rule presets beyond the two generic rule
sets above, expected-attempts-across-multiple-challenges modeling,
cost/fee-adjusted EV calculation, consistency/minimum-winning-days gating
rules, and sensitivity analysis over the rule parameters. None of these
require reshaping this module's function signature or return shape to add
later.
"""

from __future__ import annotations

import random
import statistics
from datetime import datetime, timezone
from typing import Optional

import polars as pl

from quantwave.research import wilson_interval

_MIN_SIMULATIONS = 100  # below this, refuse to report a fabricated CI.

_VALID_RULE_SETS = ("static", "trailing")


def _utc_date(ts: Optional[int]) -> Optional[str]:
    """UTC calendar date (as an ISO string key) for a Unix timestamp, or
    None if the timestamp is missing (open trade with no exit_ts)."""
    if ts is None:
        return None
    return datetime.fromtimestamp(ts, tz=timezone.utc).date().isoformat()


def _extract_trades(report) -> tuple[list[float], list[Optional[str]]]:
    """Pull (pnl_net, exit_utc_date) lists from a report's trades frame,
    the same ``report.result.trades`` access pattern used by
    ``robustness.py`` / mirrored from ``monte_carlo.rs``'s
    ``extract_trade_pnls``."""
    trades: pl.DataFrame = report.result.trades
    if trades.is_empty() or "pnl_net" not in trades.columns:
        return [], []
    pnls = trades["pnl_net"].to_list()
    if "exit_ts" in trades.columns:
        exit_dates = [_utc_date(ts) for ts in trades["exit_ts"].to_list()]
    else:
        exit_dates = [None] * len(pnls)
    return pnls, exit_dates


def _block_bootstrap_indices(
    rng: random.Random, n_trades: int, block_size: int
) -> list[int]:
    """One resampled sequence of trade indices via block bootstrap:
    contiguous chunks of `block_size` consecutive original-order indices,
    drawn with replacement (chunk start position uniformly random),
    concatenated until length >= n_trades, then truncated to n_trades."""
    bs = max(1, min(block_size, n_trades))
    indices: list[int] = []
    while len(indices) < n_trades:
        start = rng.randrange(0, n_trades)
        for offset in range(bs):
            indices.append((start + offset) % n_trades)
            if len(indices) >= n_trades:
                break
    return indices


def _simulate_one_path(
    pnls: list[float],
    exit_dates: list[Optional[str]],
    order: list[int],
    rule_set: str,
    initial_balance: float,
    profit_target_pct: float,
    max_drawdown_pct: float,
    max_daily_loss_pct: float,
) -> tuple[str, int]:
    """Walk one resampled trade order trade-by-trade; return
    (outcome, trades_taken) where outcome is one of "pass",
    "max_drawdown", "daily_loss", "ran_out"."""
    equity = initial_balance
    peak_equity = initial_balance
    target = initial_balance * (1.0 + profit_target_pct)
    daily_loss_limit = initial_balance * max_daily_loss_pct
    day_pnl: dict[str, float] = {}

    for i, idx in enumerate(order, start=1):
        equity += pnls[idx]
        peak_equity = max(peak_equity, equity)

        if rule_set == "trailing":
            floor = peak_equity * (1.0 - max_drawdown_pct)
        else:  # "static"
            floor = initial_balance * (1.0 - max_drawdown_pct)

        day = exit_dates[idx]
        if day is not None:
            day_pnl[day] = day_pnl.get(day, 0.0) + pnls[idx]
            day_total = day_pnl[day]
        else:
            day_total = 0.0

        # First event wins: check drawdown/daily-loss before profit target
        # so a trade that simultaneously breaches and reaches the target
        # (edge case) is conservatively scored as a breach, not a pass.
        if equity <= floor:
            return "max_drawdown", i
        if day_total <= -daily_loss_limit:
            return "daily_loss", i
        if equity >= target:
            return "pass", i

    return "ran_out", len(order)


def simulate_prop_firm_challenge(
    report,
    rule_set: str,
    initial_balance: float,
    profit_target_pct: float,
    max_drawdown_pct: float,
    max_daily_loss_pct: float,
    n_simulations: int = 2000,
    block_size: int = 8,
    seed: int = 42,
) -> dict:
    """Monte Carlo-simulate prop-firm challenge outcomes by block-bootstrap
    resampling a backtest's closed-trade history.

    Parameters
    ----------
    report:
        Whatever object ``.bt.backtest_with_report()`` /
        ``.bt.portfolio_backtest()`` returns — a ``BacktestReport`` exposing
        ``report.result.trades`` (a Polars DataFrame with ``pnl_net``,
        ``entry_ts``, ``exit_ts`` columns; timestamps are Unix seconds,
        UTC).
    rule_set:
        ``"static"`` or ``"trailing"`` — see module docstring for the exact
        drawdown-anchor math.
    initial_balance:
        Starting challenge account balance.
    profit_target_pct:
        Fraction of ``initial_balance`` in profit required to pass, e.g.
        ``0.10`` for a 10% target.
    max_drawdown_pct:
        Fraction of the anchor (initial balance for ``"static"``, running
        peak equity for ``"trailing"``) that equity may give back before
        the account fails.
    max_daily_loss_pct:
        Fraction of ``initial_balance`` that a single UTC calendar day's
        net P&L may lose before the account fails.
    n_simulations:
        Number of block-bootstrap simulated trade orderings to run.
        Below :data:`_MIN_SIMULATIONS` (100), the result reports
        ``"computed": False`` rather than a CI built on too few draws —
        see module docstring.
    block_size:
        Contiguous-chunk length (in trades) for the block bootstrap.
        Clamped to ``[1, n_trades]`` internally. Default 8 — see module
        docstring for the reasoning.
    seed:
        RNG seed for reproducibility (uses the stdlib ``random`` module,
        seeded once per call).

    Returns
    -------
    dict with keys:
        ``n_simulations`` (int): the number of simulations requested.
        ``computed`` (bool): ``False`` if ``n_simulations < 100`` or there
            are zero trades to resample; all numeric fields below are
            ``None`` in that case.
        ``pass_probability`` (float | None): fraction of simulated paths
            that reached the profit target before any breach.
        ``pass_ci_low`` / ``pass_ci_high`` (float | None): 95% Wilson score
            interval around ``pass_probability`` (via
            :func:`quantwave.research.wilson_interval`).
        ``breach_breakdown`` (dict | None): fractions of *all* simulations
            (not just failures) keyed ``"max_drawdown"``, ``"daily_loss"``,
            ``"ran_out_without_outcome"`` — these three plus
            ``pass_probability`` sum to 1.0.
        ``trades_to_outcome`` (dict | None): ``{"p5", "median", "p95"}`` of
            the number of resampled trades each simulation took before its
            outcome (pass, either breach, or ran-out).

    See the module docstring for the block-bootstrap rationale, the exact
    rule-set math, and a worked example.
    """
    if rule_set not in _VALID_RULE_SETS:
        raise ValueError(
            f"rule_set must be one of {_VALID_RULE_SETS!r}, got {rule_set!r}"
        )

    pnls, exit_dates = _extract_trades(report)
    n_trades = len(pnls)

    if n_simulations < _MIN_SIMULATIONS or n_trades == 0:
        return {
            "n_simulations": n_simulations,
            "computed": False,
            "pass_probability": None,
            "pass_ci_low": None,
            "pass_ci_high": None,
            "breach_breakdown": None,
            "trades_to_outcome": None,
        }

    rng = random.Random(seed)
    outcomes: list[str] = []
    trade_counts: list[int] = []

    for _ in range(n_simulations):
        order = _block_bootstrap_indices(rng, n_trades, block_size)
        outcome, taken = _simulate_one_path(
            pnls,
            exit_dates,
            order,
            rule_set,
            initial_balance,
            profit_target_pct,
            max_drawdown_pct,
            max_daily_loss_pct,
        )
        outcomes.append(outcome)
        trade_counts.append(taken)

    n = len(outcomes)
    n_pass = sum(1 for o in outcomes if o == "pass")
    n_dd = sum(1 for o in outcomes if o == "max_drawdown")
    n_daily = sum(1 for o in outcomes if o == "daily_loss")
    n_ran_out = sum(1 for o in outcomes if o == "ran_out")

    pass_probability = n_pass / n
    ci_low, ci_high = wilson_interval(n_pass, n)

    trade_counts_sorted = sorted(trade_counts)

    def _pctile(p: float) -> float:
        if not trade_counts_sorted:
            return 0.0
        idx = round((len(trade_counts_sorted) - 1) * p)
        return float(trade_counts_sorted[idx])

    return {
        "n_simulations": n_simulations,
        "computed": True,
        "pass_probability": pass_probability,
        "pass_ci_low": ci_low,
        "pass_ci_high": ci_high,
        "breach_breakdown": {
            "max_drawdown": n_dd / n,
            "daily_loss": n_daily / n,
            "ran_out_without_outcome": n_ran_out / n,
        },
        "trades_to_outcome": {
            "p5": _pctile(0.05),
            "median": statistics.median(trade_counts),
            "p95": _pctile(0.95),
        },
    }
