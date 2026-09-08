"""HTML tear sheets for backtest reports."""

from __future__ import annotations

import math
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Optional, Union

from quantwave.backtest import BacktestReport
from quantwave import robustness as _robustness


def render_html(
    report: BacktestReport,
    title: Optional[str] = None,
    seed: Optional[int] = None,
    run_metadata: Optional[Union[list, dict]] = None,
    benchmark_returns: Optional[list] = None,
    rolling_window: Optional[int] = None,
) -> str:
    """Return a self-contained HTML tear sheet string.

    ``run_metadata`` (config key/values) and ``seed`` populate the report's
    reproducible run-metadata section; ``benchmark_returns`` (per-bar simple
    returns) adds a Benchmark-Relative (alpha/beta/cumulative return) section.
    """
    return report.to_html(
        title=title,
        seed=seed,
        run_metadata=run_metadata,
        benchmark_returns=benchmark_returns,
        rolling_window=rolling_window,
    )


def save_html(
    report: BacktestReport,
    path: Union[str, Path],
    title: Optional[str] = None,
    seed: Optional[int] = None,
    run_metadata: Optional[Union[list, dict]] = None,
    benchmark_returns: Optional[list] = None,
    rolling_window: Optional[int] = None,
) -> Path:
    """Write tear sheet HTML to disk; returns the path written."""
    out = Path(path)
    report.save_html(
        str(out),
        title=title,
        seed=seed,
        run_metadata=run_metadata,
        benchmark_returns=benchmark_returns,
        rolling_window=rolling_window,
    )
    return out


# ---------------------------------------------------------------------------
# Robustness tearsheet: native to_html() (equity/drawdown/heatmap/rolling
# Sharpe/trade blotter) + an appended section computed by
# :mod:`quantwave.robustness` (KPI summary, comprehensive perf/risk matrix,
# overfitting/robustness audit, top-N drawdowns, monthly heatmap) — the
# layout described in docs/backtest/reference/tearsheet_template.md.
# ---------------------------------------------------------------------------

_MONTH_ABBR = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]


def _fmt_pct(x: Optional[float], digits: int = 1) -> str:
    if x is None or (isinstance(x, float) and math.isnan(x)):
        return "—"
    return f"{x * 100:+.{digits}f}%"


def _fmt_ratio(x: Optional[float], digits: int = 2) -> str:
    if x is None or (isinstance(x, float) and math.isnan(x)):
        return "—"
    return f"{x:.{digits}f}"


def _fmt_bps(x: Optional[float], digits: int = 1) -> str:
    if x is None or (isinstance(x, float) and math.isnan(x)):
        return "—"
    return f"{x:.{digits}f} bps"


def _fmt_ts(ts: Optional[int]) -> str:
    if ts is None:
        return "—"
    try:
        return datetime.fromtimestamp(ts, tz=timezone.utc).strftime("%Y-%m-%d")
    except (OSError, OverflowError, ValueError):
        return str(ts)


def _status_badge(passed: Optional[bool]) -> str:
    if passed is None:
        return '<span style="color:#9ca3af">N/A</span>'
    color = "#10b981" if passed else "#ef4444"
    label = "PASS" if passed else "FAIL"
    return f'<span style="color:{color};font-weight:700">{label}</span>'


_ROBUSTNESS_CSS = """
<style>
  .qw-rb { font-family: system-ui, -apple-system, Segoe UI, Roboto, sans-serif;
           background:#0f1117; color:#e5e7eb; padding:24px; margin-top:24px;
           border-radius:8px; }
  .qw-rb h2 { font-size:1.1rem; margin:28px 0 10px; color:#f3f4f6;
              border-bottom:1px solid #2a2e3a; padding-bottom:6px; }
  .qw-rb h2:first-child { margin-top:0; }
  .qw-rb .kpi-grid { display:grid; grid-template-columns:repeat(auto-fit,minmax(160px,1fr));
                      gap:12px; }
  .qw-rb .kpi-card { background:#171a23; border:1px solid #2a2e3a; border-radius:8px;
                      padding:12px 14px; }
  .qw-rb .kpi-card .label { font-size:0.72rem; color:#9ca3af; text-transform:uppercase;
                             letter-spacing:0.03em; }
  .qw-rb .kpi-card .value { font-size:1.35rem; font-weight:700; margin-top:4px; color:#f9fafb; }
  .qw-rb table { width:100%; border-collapse:collapse; font-size:0.82rem; }
  .qw-rb th, .qw-rb td { text-align:left; padding:7px 10px; border-bottom:1px solid #232733; }
  .qw-rb th { color:#9ca3af; font-weight:600; }
  .qw-rb td.num, .qw-rb th.num { text-align:right; font-variant-numeric:tabular-nums; }
  .qw-rb .group-label { color:#6b7280; font-size:0.7rem; text-transform:uppercase; }
  .qw-rb .table-wrap { overflow-x:auto; }
  .qw-rb .heatmap td { text-align:right; font-variant-numeric:tabular-nums; }
  .qw-rb .heat-pos { background:rgba(16,185,129,0.18); }
  .qw-rb .heat-neg { background:rgba(239,68,68,0.18); }
  .qw-rb .note { color:#6b7280; font-size:0.75rem; margin-top:6px; }
</style>
"""


def _render_perf_matrix(result: "_robustness.TearsheetResult") -> str:
    periods = [("Full Period", result.full), ("In-Sample", result.in_sample), ("Out-of-Sample", result.out_of_sample)]
    periods = [(label, m) for label, m in periods if m is not None]
    header = "".join(f"<th class='num'>{label}</th>" for label, _ in periods)

    def row(label: str, fmt, attr: str) -> str:
        cells = "".join(f"<td class='num'>{fmt(getattr(m, attr))}</td>" for _, m in periods)
        return f"<tr><td>{label}</td>{cells}</tr>"

    rows = [
        row("CAGR", _fmt_pct, "cagr"),
        row("Cumulative Return", _fmt_pct, "cumulative_return"),
        row("Annualized Volatility", _fmt_pct, "volatility"),
        row("Sharpe Ratio", _fmt_ratio, "sharpe_ratio"),
        row("Sortino Ratio", _fmt_ratio, "sortino_ratio"),
        row("Calmar Ratio", _fmt_ratio, "calmar_ratio"),
        row("Omega Ratio", _fmt_ratio, "omega_ratio"),
        row("Max Drawdown", _fmt_pct, "max_drawdown_pct"),
        row("Longest DD Duration (bars)", lambda x: "—" if x is None else str(x), "longest_dd_duration_bars"),
        row("Daily VaR (95%)", _fmt_pct, "var_95"),
        row("Daily VaR (99%)", _fmt_pct, "var_99"),
        row("CVaR (95%)", _fmt_pct, "cvar_95"),
        row("Skewness", _fmt_ratio, "skewness"),
        row("Excess Kurtosis", _fmt_ratio, "kurtosis"),
        row("Win Rate", _fmt_pct, "win_rate"),
        row("Profit Factor", _fmt_ratio, "profit_factor"),
        row("Payoff Ratio", _fmt_ratio, "payoff_ratio"),
        row("Expectancy (per trade)", lambda x: "—" if x is None or math.isnan(x) else f"{x:,.2f}", "expectancy"),
        row("Max Consecutive Losses", lambda x: "—" if x is None else str(x), "max_consecutive_losses"),
        row("Market Exposure", _fmt_pct, "market_exposure_pct"),
        row("Num. Trades", lambda x: "—" if x is None else str(x), "num_trades"),
    ]
    return f"""
    <div class="table-wrap"><table>
      <thead><tr><th>Metric</th>{header}</tr></thead>
      <tbody>{''.join(rows)}</tbody>
    </table></div>
    """


def _render_robustness_audit(result: "_robustness.TearsheetResult") -> str:
    r = result.robustness
    deg = r.sharpe_degradation_pct
    deg_pass = None if deg is None else deg > -0.30
    dsr_pass = None if math.isnan(r.dsr) else r.dsr > 0.90
    psr_pass = None if math.isnan(r.psr) else r.psr > 0.95
    mc_p95 = r.monte_carlo.p95_max_drawdown_pct
    slip_bps = r.slippage.breakeven_bps
    slip_cur = r.slippage.current_avg_friction_bps
    slip_pass = None
    if slip_bps is not None and slip_cur is not None and slip_cur > 0:
        slip_pass = slip_bps >= 3 * slip_cur

    ff = r.fama_french
    if ff.get("computed"):
        ff_finding = f"Residual &alpha; = {_fmt_pct(ff['alpha_annualized'])} (t = {ff['t_stat']:.2f})"
        ff_pass = abs(ff["t_stat"]) > 2.0 if not math.isnan(ff["t_stat"]) else None
        ff_hurdle = "|t| &gt; 2.0"
    else:
        ff_finding = "Not computed — no factor_returns supplied"
        ff_pass = None
        ff_hurdle = "|t| &gt; 2.0"

    rows = [
        ("Sharpe Degradation (IS &rarr; OOS)",
         "—" if deg is None else f"{deg * 100:+.1f}% (IS {_fmt_ratio(r.sharpe_is)} &rarr; OOS {_fmt_ratio(r.sharpe_oos)})",
         "Degradation &gt; -30%", deg_pass,
         "Strategy holds predictive edge out of sample." if deg_pass else "No IS/OOS split supplied or edge decayed sharply."),
        ("Deflated Sharpe Ratio (DSR)", _fmt_ratio(r.dsr), "DSR &gt; 0.90", dsr_pass,
         f"Skill-adjusted for {r.dsr_trials} trial(s) tested."),
        ("Probabilistic Sharpe Ratio (PSR)", _fmt_pct(r.psr, 1), "PSR &gt; 95%", psr_pass,
         "Confidence the true Sharpe exceeds zero."),
        ("Monte Carlo 95th %ile MDD",
         "—" if math.isnan(mc_p95) else f"{mc_p95 * 100:.1f}%",
         f"{r.monte_carlo.n_simulations:,} trade-bootstrap resamples", None,
         f"p50={_fmt_pct(r.monte_carlo.p50_max_drawdown_pct)}, p99={_fmt_pct(r.monte_carlo.p99_max_drawdown_pct)}"),
        ("Slippage Breakeven",
         "—" if slip_bps is None else _fmt_bps(slip_bps),
         "&ge; 3&times; current friction" if slip_cur else "N/A", slip_pass,
         f"Current embedded friction &asymp; {_fmt_bps(slip_cur)} round-trip ({r.slippage.method})."),
        ("Fama-French 5-Factor Alpha", ff_finding, ff_hurdle, ff_pass, ff.get("reason", "")),
    ]
    body = "".join(
        f"<tr><td>{name}</td><td>{finding}</td><td>{hurdle}</td><td>{_status_badge(passed)}</td>"
        f"<td class='note' style='padding-top:7px'>{note}</td></tr>"
        for name, finding, hurdle, passed, note in rows
    )
    return f"""
    <div class="table-wrap"><table>
      <thead><tr><th>Robustness Check</th><th>Finding</th><th>Hurdle</th><th>Status</th><th>Interpretation</th></tr></thead>
      <tbody>{body}</tbody>
    </table></div>
    <p class="note">DSR/PSR use Bailey &amp; Lopez de Prado (2012, 2014); Monte Carlo is a trade-PnL bootstrap;
    slippage breakeven is a linear friction-sensitivity estimate unless a <code>slippage_resim_fn</code> was supplied.</p>
    """


def _render_top_drawdowns(result: "_robustness.TearsheetResult") -> str:
    if not result.top_drawdowns:
        return "<p class='note'>No drawdown episodes recorded.</p>"
    rows = "".join(
        f"<tr><td>{d.rank}</td><td>{_fmt_ts(d.peak_ts)}</td><td>{_fmt_ts(d.valley_ts)}</td>"
        f"<td>{_fmt_ts(d.recovery_ts) if d.recovery_ts else 'Ongoing'}</td>"
        f"<td class='num'>-{d.depth_pct * 100:.1f}%</td><td class='num'>{d.duration_bars} bars</td></tr>"
        for d in result.top_drawdowns
    )
    return f"""
    <div class="table-wrap"><table>
      <thead><tr><th>Rank</th><th>Peak</th><th>Valley</th><th>Recovery</th><th class='num'>Depth</th><th class='num'>Duration</th></tr></thead>
      <tbody>{rows}</tbody>
    </table></div>
    """


def _render_monthly_heatmap(result: "_robustness.TearsheetResult") -> str:
    years = sorted(result.monthly_heatmap.keys(), reverse=True)
    if not years:
        return "<p class='note'>Not enough data for a monthly breakdown.</p>"
    header = "".join(f"<th class='num'>{m}</th>" for m in _MONTH_ABBR) + "<th class='num'>Annual</th>"
    rows = []
    for year in years:
        months = result.monthly_heatmap[year]
        cells = []
        for m in range(1, 13):
            v = months.get(m)
            if v is None:
                cells.append("<td>—</td>")
            else:
                cls = "heat-pos" if v >= 0 else "heat-neg"
                cells.append(f"<td class='{cls}'>{v * 100:+.1f}%</td>")
        annual = result.annual_returns.get(year)
        cells.append(f"<td><strong>{_fmt_pct(annual)}</strong></td>")
        rows.append(f"<tr><td>{year}</td>{''.join(cells)}</tr>")
    return f"""
    <div class="table-wrap"><table class="heatmap">
      <thead><tr><th>Year</th>{header}</tr></thead>
      <tbody>{''.join(rows)}</tbody>
    </table></div>
    """


def render_robustness_html(
    report: BacktestReport,
    title: Optional[str] = None,
    result: Optional["_robustness.TearsheetResult"] = None,
    **compute_kwargs: Any,
) -> str:
    """Full tear sheet: native ``report.to_html()`` (equity curve, drawdown
    plot, rolling Sharpe, monthly heatmap, trade blotter) with an appended
    robustness section computed by :mod:`quantwave.robustness` — KPI
    summary, comprehensive performance/risk matrix (Full/IS/OOS), the
    overfitting/robustness audit (Sharpe degradation, DSR, PSR, Monte Carlo
    drawdown, slippage breakeven, Fama-French), top-N drawdown periods, and a
    monthly-returns heatmap. Matches the section layout of
    ``docs/backtest/reference/tearsheet_template.md``.

    Pass a precomputed ``result`` (:class:`quantwave.robustness.TearsheetResult`)
    to avoid recomputing, or ``**compute_kwargs`` (``split_date``,
    ``factor_returns``, ``trials``, etc. — see
    :func:`quantwave.robustness.compute_tearsheet`) to configure the
    computation.
    """
    base_html = report.to_html(title=title)
    if result is None:
        result = _robustness.compute_tearsheet(report, **compute_kwargs)

    m = result.full
    kpi_cards = "".join(
        f"<div class='kpi-card'><div class='label'>{label}</div><div class='value'>{value}</div></div>"
        for label, value in [
            ("CAGR", _fmt_pct(m.cagr)),
            ("Sharpe Ratio", _fmt_ratio(m.sharpe_ratio)),
            ("Max Drawdown", _fmt_pct(m.max_drawdown_pct)),
            ("Calmar Ratio", _fmt_ratio(m.calmar_ratio)),
            ("Deflated Sharpe (DSR)", _fmt_ratio(result.robustness.dsr)),
            ("Slippage Breakeven", _fmt_bps(result.robustness.slippage.breakeven_bps)),
        ]
    )

    section = f"""
    {_ROBUSTNESS_CSS}
    <div class="qw-rb">
      <h2>Executive Summary KPIs</h2>
      <div class="kpi-grid">{kpi_cards}</div>

      <h2>Comprehensive Performance &amp; Risk Matrix</h2>
      {_render_perf_matrix(result)}

      <h2>Overfitting &amp; Robustness Audit</h2>
      {_render_robustness_audit(result)}

      <h2>Top {len(result.top_drawdowns)} Drawdown Periods</h2>
      {_render_top_drawdowns(result)}

      <h2>Monthly Returns Heatmap</h2>
      {_render_monthly_heatmap(result)}
    </div>
    """

    if "</body>" in base_html:
        return base_html.replace("</body>", section + "</body>")
    return base_html + section


def save_robustness_html(
    report: BacktestReport,
    path: Union[str, Path],
    title: Optional[str] = None,
    result: Optional["_robustness.TearsheetResult"] = None,
    **compute_kwargs: Any,
) -> Path:
    """Compute :func:`render_robustness_html` and write it to ``path``."""
    out = Path(path)
    html = render_robustness_html(report, title=title, result=result, **compute_kwargs)
    out.write_text(html, encoding="utf-8")
    return out