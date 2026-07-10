"""HTML tear sheets for backtest reports."""

from __future__ import annotations

from pathlib import Path
from typing import Optional, Union

from quantwave.backtest import BacktestReport


def render_html(report: BacktestReport, title: Optional[str] = None) -> str:
    """Return a self-contained HTML tear sheet string."""
    return report.to_html(title=title)


def save_html(
    report: BacktestReport,
    path: Union[str, Path],
    title: Optional[str] = None,
) -> Path:
    """Write tear sheet HTML to disk; returns the path written."""
    out = Path(path)
    report.save_html(str(out), title=title)
    return out