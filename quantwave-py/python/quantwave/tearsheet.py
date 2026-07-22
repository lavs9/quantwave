"""HTML tear sheets for backtest reports."""

from __future__ import annotations

from pathlib import Path
from typing import Optional, Union

from quantwave.backtest import BacktestReport


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