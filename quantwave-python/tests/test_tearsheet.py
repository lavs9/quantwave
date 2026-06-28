"""HTML tear sheets (quantwave-0gi1)."""

import polars as pl

from quantwave import tearsheet
from quantwave.backtest import BacktestEngine


def _mini_df():
    return pl.DataFrame({
        "timestamp": list(range(6)),
        "close": [100.0, 101.0, 102.5, 103.0, 102.0, 101.0],
        "signal": [0.0, 1.0, 1.0, 1.0, 0.0, 0.0],
    })


def test_report_to_html():
    report = BacktestEngine.with_default_costs().backtest_with_report(_mini_df())
    html = report.to_html(title="Test Strategy")
    assert "<!DOCTYPE html>" in html
    assert "Test Strategy" in html
    assert "Equity Curve" in html
    assert "Drawdown" in tearsheet.render_html(report)


def test_save_html(tmp_path):
    report = BacktestEngine.with_default_costs().backtest_with_report(_mini_df())
    path = tearsheet.save_html(report, tmp_path / "report.html", title="Demo")
    text = path.read_text(encoding="utf-8")
    assert "Performance Metrics" in text
    assert path.exists()