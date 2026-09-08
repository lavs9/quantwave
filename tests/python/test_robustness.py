"""Tests for quantwave.robustness (quantwave-j2rl).

Known-input/known-output cases for the pure-Python metric functions, plus an
integration test against a real BacktestReport for compute_tearsheet() and
the HTML renderer end-to-end.
"""

import math

import pytest

polars = pytest.importorskip("polars")
pl = polars

from quantwave import robustness as rb
from quantwave.backtest import BacktestEngine


# ---------------------------------------------------------------------------
# Drawdown: known equity path with an exact, hand-computed drawdown.
# ---------------------------------------------------------------------------


def _equity_curve(values, start_ts=0):
    return pl.DataFrame(
        {
            "ts": list(range(start_ts, start_ts + len(values))),
            "equity": values,
        }
    )


def test_max_drawdown_pct_known_case():
    # Peak 100 -> valley 80 -> recovers to 110. Drawdown = 1 - 80/100 = 20%.
    eq = _equity_curve([100.0, 90.0, 80.0, 95.0, 110.0])
    assert rb.max_drawdown_pct(eq) == pytest.approx(0.20)


def test_drawdown_episodes_single_unrecovered():
    eq = _equity_curve([100.0, 80.0, 60.0])
    episodes = rb.drawdown_episodes(eq)
    assert len(episodes) == 1
    ep = episodes[0]
    assert ep.recovery_ts is None
    assert ep.depth_pct == pytest.approx(0.40)
    assert ep.duration_bars == 2


def test_top_drawdowns_ranked_deepest_first():
    # Two episodes: -20% then a deeper -50%.
    eq = _equity_curve([100.0, 80.0, 100.0, 50.0, 100.0])
    top = rb.top_drawdowns(eq, n=5)
    assert len(top) == 2
    assert top[0].rank == 1
    assert top[0].depth_pct == pytest.approx(0.50)
    assert top[1].rank == 2
    assert top[1].depth_pct == pytest.approx(0.20)


def test_no_drawdown_when_monotonic_up():
    eq = _equity_curve([100.0, 101.0, 102.0, 103.0])
    assert rb.max_drawdown_pct(eq) == 0.0
    assert rb.drawdown_episodes(eq) == []


# ---------------------------------------------------------------------------
# Returns from equity curve.
# ---------------------------------------------------------------------------


def test_bar_returns_known_case():
    eq = _equity_curve([100.0, 110.0, 99.0])
    returns = rb.bar_returns(eq)
    assert returns == pytest.approx([0.10, -0.10])


# ---------------------------------------------------------------------------
# Statistics helpers: normal distribution sanity checks.
# ---------------------------------------------------------------------------


def test_norm_cdf_standard_values():
    assert rb._norm_cdf(0.0) == pytest.approx(0.5, abs=1e-9)
    assert rb._norm_cdf(1.959963985) == pytest.approx(0.975, abs=1e-6)


def test_norm_ppf_is_inverse_of_norm_cdf():
    for p in (0.01, 0.05, 0.25, 0.5, 0.75, 0.95, 0.99):
        z = rb._norm_ppf(p)
        assert rb._norm_cdf(z) == pytest.approx(p, abs=1e-6)


def test_skew_and_kurtosis_zero_for_symmetric_data():
    xs = [-2.0, -1.0, 0.0, 1.0, 2.0]
    assert rb._skewness(xs) == pytest.approx(0.0, abs=1e-9)
    # Excess kurtosis of a small discrete uniform-ish symmetric sample is a
    # fixed, computable number, not necessarily 0 — just check it's finite.
    assert math.isfinite(rb._kurtosis(xs))


# ---------------------------------------------------------------------------
# PSR / DSR: sanity bounds and known degenerate cases.
# ---------------------------------------------------------------------------


def test_psr_of_zero_sharpe_vs_zero_benchmark_is_half():
    # SR_hat == benchmark == 0 -> z == 0 -> Phi(0) == 0.5, for normal
    # (skew=0, kurtosis=0) returns.
    psr = rb.probabilistic_sharpe_ratio(0.0, 0.0, num_observations=100, skewness=0.0, kurtosis=0.0)
    assert psr == pytest.approx(0.5, abs=1e-9)


def test_psr_increases_with_higher_observed_sharpe():
    low = rb.probabilistic_sharpe_ratio(0.01, 0.0, 250, 0.0, 0.0)
    high = rb.probabilistic_sharpe_ratio(0.05, 0.0, 250, 0.0, 0.0)
    assert high > low


def test_dsr_with_one_trial_equals_psr_at_zero():
    sharpe, t, skew, kurt = 0.03, 250, 0.1, 0.5
    dsr = rb.deflated_sharpe_ratio(sharpe, t, skew, kurt, num_trials=1)
    psr = rb.probabilistic_sharpe_ratio(sharpe, 0.0, t, skew, kurt)
    assert dsr == pytest.approx(psr)


def test_dsr_decreases_as_trials_increase():
    sharpe, t, skew, kurt = 0.03, 250, 0.1, 0.5
    dsr_1 = rb.deflated_sharpe_ratio(sharpe, t, skew, kurt, num_trials=1)
    dsr_50 = rb.deflated_sharpe_ratio(sharpe, t, skew, kurt, num_trials=50)
    assert dsr_50 < dsr_1


# ---------------------------------------------------------------------------
# Monte Carlo drawdown: deterministic seed, sanity bounds.
# ---------------------------------------------------------------------------


def test_monte_carlo_drawdown_reproducible_with_seed():
    trades = pl.DataFrame({"pnl_net": [100.0, -50.0, 200.0, -80.0, 30.0]})
    a = rb.monte_carlo_drawdown(trades, initial_cash=10_000.0, n_simulations=500, seed=7)
    b = rb.monte_carlo_drawdown(trades, initial_cash=10_000.0, n_simulations=500, seed=7)
    assert a == b


def test_monte_carlo_drawdown_percentiles_ordered():
    trades = pl.DataFrame({"pnl_net": [100.0, -500.0, 200.0, -800.0, 30.0, -20.0]})
    mc = rb.monte_carlo_drawdown(trades, initial_cash=10_000.0, n_simulations=1000, seed=1)
    assert 0.0 <= mc.p50_max_drawdown_pct <= mc.p95_max_drawdown_pct <= mc.p99_max_drawdown_pct <= 1.0


def test_monte_carlo_drawdown_empty_trades():
    trades = pl.DataFrame({"pnl_net": pl.Series([], dtype=pl.Float64)})
    mc = rb.monte_carlo_drawdown(trades, initial_cash=10_000.0)
    assert mc.n_trades_sampled == 0
    assert math.isnan(mc.p50_max_drawdown_pct)


# ---------------------------------------------------------------------------
# Fama-French: graceful "not computed" when no factor data, and a recovered
# known-alpha regression when factor data IS supplied.
# ---------------------------------------------------------------------------


def test_fama_french_not_computed_without_factor_data():
    result = rb.fama_french_alpha([0.01, 0.02, -0.01], factor_returns=None)
    assert result["computed"] is False
    assert "factor_returns" in result["reason"]


def test_fama_french_recovers_known_alpha():
    # Construct strategy returns as a known linear function of one factor
    # plus a fixed daily alpha, with zero noise: alpha should be recovered
    # near-exactly by OLS.
    true_alpha = 0.001
    true_beta = 1.5
    factor_vals = [0.001 * i for i in range(-10, 10)]
    strategy_returns = [true_alpha + true_beta * f for f in factor_vals]
    factor_df = pl.DataFrame({"mkt_rf": factor_vals, "rf": [0.0] * len(factor_vals)})

    result = rb.fama_french_alpha(strategy_returns, factor_returns=factor_df)
    assert result["computed"] is True
    assert result["alpha"] == pytest.approx(true_alpha, abs=1e-9)
    assert result["betas"]["mkt_rf"] == pytest.approx(true_beta, abs=1e-9)


def test_fama_french_not_enough_observations():
    result = rb.fama_french_alpha(
        [0.01, 0.02],
        factor_returns=pl.DataFrame({"mkt_rf": [0.01, 0.02], "smb": [0.0, 0.01], "rf": [0.0, 0.0]}),
    )
    assert result["computed"] is False


# ---------------------------------------------------------------------------
# Integration: compute_tearsheet() against a real BacktestReport.
# ---------------------------------------------------------------------------


def _sample_report():
    n = 120
    close = [100.0 + (i % 7 - 3) * 0.7 + i * 0.2 for i in range(n)]
    df = pl.DataFrame(
        {
            "timestamp": list(range(0, n * 86400, 86400)),  # 1 bar/day, real epoch seconds
            "close": close,
            "signal": [1.0 if (i // 5) % 2 == 0 else 0.0 for i in range(n)],
        }
    )
    return BacktestEngine.with_default_costs().backtest_with_report(df)


def test_compute_tearsheet_full_period_only():
    report = _sample_report()
    result = rb.compute_tearsheet(report)
    assert result.full.num_bars == 120
    assert result.in_sample is None
    assert result.out_of_sample is None
    assert result.robustness.fama_french["computed"] is False
    assert isinstance(result.robustness.dsr, float)
    assert isinstance(result.robustness.psr, float)


def test_compute_tearsheet_with_is_oos_split():
    report = _sample_report()
    split_ts = 80 * 86400
    result = rb.compute_tearsheet(report, split_date=split_ts, trials=3)
    assert result.in_sample is not None
    assert result.out_of_sample is not None
    assert result.in_sample.num_bars + result.out_of_sample.num_bars == result.full.num_bars
    assert result.robustness.sharpe_degradation_pct is not None


def test_compute_tearsheet_monthly_heatmap_has_entries():
    report = _sample_report()
    result = rb.compute_tearsheet(report)
    assert len(result.monthly_heatmap) >= 1
    for year, months in result.monthly_heatmap.items():
        assert all(1 <= m <= 12 for m in months)


def test_compute_tearsheet_top_drawdowns_sorted():
    report = _sample_report()
    result = rb.compute_tearsheet(report, top_n_drawdowns=3)
    depths = [d.depth_pct for d in result.top_drawdowns]
    assert depths == sorted(depths, reverse=True)


def test_render_robustness_html_contains_all_sections():
    from quantwave import tearsheet

    report = _sample_report()
    html = tearsheet.render_robustness_html(report, title="Test Tearsheet")
    assert "<!DOCTYPE html>" in html
    assert "Executive Summary KPIs" in html
    assert "Comprehensive Performance" in html
    assert "Overfitting" in html
    assert "Monthly Returns Heatmap" in html


def test_save_robustness_html_writes_file(tmp_path):
    from quantwave import tearsheet

    report = _sample_report()
    path = tearsheet.save_robustness_html(report, tmp_path / "tearsheet.html", title="Demo")
    assert path.exists()
    assert "Deflated Sharpe" in path.read_text(encoding="utf-8")
