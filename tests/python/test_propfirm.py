"""Tests for quantwave.propfirm (prop-firm challenge Monte Carlo simulator).

Covers: an obviously-passing trade set (high pass_probability, tight CI),
an obviously-failing trade set (low pass_probability, mostly max_drawdown
breach reason), differentiable "static" vs "trailing" rule-set behavior on
a synthetic dip-recover-dip trade sequence, the minimum-simulations guard,
and a zero-trades case that doesn't crash.

Follows tests/python/test_research.py's structure/conventions: a tiny fake
"report" object exposing report.result.trades (the same access pattern
`propfirm.py` documents reusing from `robustness.py`/`monte_carlo.rs`),
built directly as a Polars DataFrame rather than running a full backtest,
so these tests are fast and don't depend on the Rust extension being built.
"""

import pytest

polars = pytest.importorskip("polars")
pl = polars

from quantwave import propfirm


# ---------------------------------------------------------------------------
# Fake report helper: report.result.trades, matching the real
# BacktestReport.result.trades access pattern.
# ---------------------------------------------------------------------------


class _FakeResult:
    def __init__(self, trades: pl.DataFrame):
        self.trades = trades


class _FakeReport:
    def __init__(self, trades: pl.DataFrame):
        self.result = _FakeResult(trades)


def _make_report(pnls: list[float], day_spacing_seconds: int = 86400) -> _FakeReport:
    """Build a fake report with `n` trades, each on its own UTC day
    (spaced `day_spacing_seconds` apart) so daily-loss grouping is trivial
    unless a test wants same-day trades (pass day_spacing_seconds=0)."""
    n = len(pnls)
    base_ts = 1_700_000_000  # arbitrary fixed epoch anchor
    entry_ts = [base_ts + i * day_spacing_seconds for i in range(n)]
    exit_ts = [ts + 3600 for ts in entry_ts]  # exits 1h after entry, same day
    trades = pl.DataFrame(
        {
            "pnl_net": pnls,
            "entry_ts": entry_ts,
            "exit_ts": exit_ts,
        }
    )
    return _FakeReport(trades)


# ---------------------------------------------------------------------------
# Obviously-passing case.
# ---------------------------------------------------------------------------


def test_obviously_passing_trades_high_pass_probability_tight_ci():
    # 30 trades, all solidly positive, target reached well before exhausting
    # the trade list in almost any ordering.
    pnls = [2000.0] * 30
    report = _make_report(pnls)
    result = propfirm.simulate_prop_firm_challenge(
        report,
        rule_set="static",
        initial_balance=100_000.0,
        profit_target_pct=0.10,  # +10,000 = 5 winning trades
        max_drawdown_pct=0.10,
        max_daily_loss_pct=0.05,
        n_simulations=1000,
        block_size=5,
        seed=42,
    )
    assert result["computed"] is True
    assert result["pass_probability"] == pytest.approx(1.0)
    assert result["pass_ci_low"] > 0.99
    # tight CI: high - low small
    assert (result["pass_ci_high"] - result["pass_ci_low"]) < 0.02
    assert result["breach_breakdown"]["max_drawdown"] == pytest.approx(0.0)
    assert result["breach_breakdown"]["daily_loss"] == pytest.approx(0.0)


# ---------------------------------------------------------------------------
# Obviously-failing case.
# ---------------------------------------------------------------------------


def test_obviously_failing_trades_low_pass_probability_max_drawdown_reason():
    # Large negative PnLs immediately breach a 10% static drawdown floor.
    pnls = [-15000.0] * 10 + [100.0] * 20
    report = _make_report(pnls)
    result = propfirm.simulate_prop_firm_challenge(
        report,
        rule_set="static",
        initial_balance=100_000.0,
        profit_target_pct=0.10,
        max_drawdown_pct=0.10,
        max_daily_loss_pct=0.50,  # loosen daily-loss so drawdown dominates
        n_simulations=1000,
        block_size=5,
        seed=7,
    )
    assert result["computed"] is True
    assert result["pass_probability"] < 0.05
    assert result["breach_breakdown"]["max_drawdown"] > 0.9


# ---------------------------------------------------------------------------
# Static vs trailing differentiation.
# ---------------------------------------------------------------------------


def test_trailing_stricter_than_static_on_dip_recover_dip_path():
    # A trade sequence that: dips ~8% (survives a 10% floor), recovers to a
    # NEW high well above initial balance, then dips ~8% again *from that
    # new high*. Under "static" the floor never moves, so both dips (8% off
    # the fixed 100k baseline) survive a 10% max_drawdown. Under "trailing"
    # the floor re-anchors to the new peak after the recovery, so the
    # second ~8%-of-peak dip can be a materially larger dollar drawdown
    # relative to the (higher) peak and is more likely to breach.
    pnls = [
        -2000.0, -2000.0, -2000.0, -2000.0,  # -8,000 (8% dip from 100k)
        3000.0, 3000.0, 3000.0, 3000.0, 3000.0, 3000.0,  # +18,000 -> new peak 110k
        -3500.0, -3500.0, -3500.0, -3500.0,  # -14,000 from the 110k peak (~12.7% of peak)
    ]
    report = _make_report(pnls)

    common = dict(
        initial_balance=100_000.0,
        profit_target_pct=0.50,  # unreachably high -- force drawdown/ran-out outcomes only
        max_drawdown_pct=0.10,
        max_daily_loss_pct=0.90,  # effectively disable daily-loss so DD dominates
        n_simulations=2000,
        block_size=4,
        seed=11,
    )

    static_result = propfirm.simulate_prop_firm_challenge(
        report, rule_set="static", **common
    )
    trailing_result = propfirm.simulate_prop_firm_challenge(
        report, rule_set="trailing", **common
    )

    assert static_result["computed"] is True
    assert trailing_result["computed"] is True
    # Trailing's drawdown floor only ever moves up from static's fixed
    # floor, so trailing should breach at least as often (and, on this
    # constructed path, strictly more often).
    assert (
        trailing_result["breach_breakdown"]["max_drawdown"]
        >= static_result["breach_breakdown"]["max_drawdown"]
    )
    assert (
        trailing_result["breach_breakdown"]["max_drawdown"]
        > static_result["breach_breakdown"]["max_drawdown"]
    )


# ---------------------------------------------------------------------------
# Minimum-simulations guard.
# ---------------------------------------------------------------------------


def test_min_simulations_guard_refuses_tiny_n():
    pnls = [500.0, -300.0, 800.0, -200.0]
    report = _make_report(pnls)
    result = propfirm.simulate_prop_firm_challenge(
        report,
        rule_set="static",
        initial_balance=100_000.0,
        profit_target_pct=0.10,
        max_drawdown_pct=0.10,
        max_daily_loss_pct=0.05,
        n_simulations=50,  # below the 100 guard
        seed=1,
    )
    assert result["n_simulations"] == 50
    assert result["computed"] is False
    assert result["pass_probability"] is None
    assert result["pass_ci_low"] is None
    assert result["pass_ci_high"] is None
    assert result["breach_breakdown"] is None
    assert result["trades_to_outcome"] is None


def test_min_simulations_guard_boundary_100_is_accepted():
    pnls = [500.0, -300.0, 800.0, -200.0, 100.0]
    report = _make_report(pnls)
    result = propfirm.simulate_prop_firm_challenge(
        report,
        rule_set="trailing",
        initial_balance=100_000.0,
        profit_target_pct=0.10,
        max_drawdown_pct=0.10,
        max_daily_loss_pct=0.05,
        n_simulations=100,
        seed=1,
    )
    assert result["computed"] is True


# ---------------------------------------------------------------------------
# Zero-trades case.
# ---------------------------------------------------------------------------


def test_zero_trades_does_not_crash():
    report = _make_report([])
    result = propfirm.simulate_prop_firm_challenge(
        report,
        rule_set="static",
        initial_balance=100_000.0,
        profit_target_pct=0.10,
        max_drawdown_pct=0.10,
        max_daily_loss_pct=0.05,
        n_simulations=1000,
        seed=1,
    )
    assert result["computed"] is False
    assert result["pass_probability"] is None


def test_invalid_rule_set_raises():
    report = _make_report([100.0, -50.0])
    with pytest.raises(ValueError):
        propfirm.simulate_prop_firm_challenge(
            report,
            rule_set="bogus",
            initial_balance=100_000.0,
            profit_target_pct=0.10,
            max_drawdown_pct=0.10,
            max_daily_loss_pct=0.05,
            n_simulations=1000,
        )


# ---------------------------------------------------------------------------
# Determinism / seeding.
# ---------------------------------------------------------------------------


def test_deterministic_with_seed():
    pnls = [200.0, -150.0, 300.0, -100.0, 50.0, -400.0, 600.0, -200.0] * 4
    report = _make_report(pnls)
    kwargs = dict(
        rule_set="trailing",
        initial_balance=100_000.0,
        profit_target_pct=0.10,
        max_drawdown_pct=0.10,
        max_daily_loss_pct=0.05,
        n_simulations=500,
        block_size=6,
        seed=99,
    )
    a = propfirm.simulate_prop_firm_challenge(report, **kwargs)
    b = propfirm.simulate_prop_firm_challenge(report, **kwargs)
    assert a == b


# ---------------------------------------------------------------------------
# trades_to_outcome sanity.
# ---------------------------------------------------------------------------


def test_trades_to_outcome_percentiles_ordered():
    pnls = [200.0, -150.0, 300.0, -100.0, 50.0, -400.0, 600.0, -200.0] * 5
    report = _make_report(pnls)
    result = propfirm.simulate_prop_firm_challenge(
        report,
        rule_set="static",
        initial_balance=100_000.0,
        profit_target_pct=0.20,
        max_drawdown_pct=0.15,
        max_daily_loss_pct=0.10,
        n_simulations=500,
        block_size=6,
        seed=3,
    )
    assert result["computed"] is True
    tto = result["trades_to_outcome"]
    assert tto["p5"] <= tto["median"] <= tto["p95"]
