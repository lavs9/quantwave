import marimo

__generated_with = "0.4.5"
app = marimo.App(width="medium")

@app.cell
def __():
    import marimo as mo
    import polars as pl
    from quantwave.backtest import BacktestEngine, BacktestConfig
    
    mo.md(
        """
        # QuantWave Backtest Engine — Full `.bt` Tour
        
        Link: [Capability Matrix](../../guides/backtest/capability_matrix.md)
        """
    )
    return BacktestConfig, BacktestEngine, mo, pl

@app.cell
def __(pl):
    df_synthetic = pl.DataFrame({
        "timestamp": list(range(80)),
        "close": [100.0 + i * 0.5 + (i % 3) for i in range(80)],
        "feature_col": [i % 5 for i in range(80)],
    })
    return (df_synthetic,)

@app.cell
def __(df_synthetic, mo):
    mo.md("## Section A — Basic backtest + metrics")
    
    # Simple signal: long when feature > 2
    df_a = df_synthetic.with_columns(
        (pl.col("feature_col") > 2).cast(pl.Float64).alias("signal")
    )
    report_a = df_a.lazy().bt.backtest_with_report(
        signal="signal",
        commission_bps=0.0,
        slippage_bps=0.0
    )
    assert report_a.result.trades.height >= 1
    
    metrics_keys = list(report_a.metrics().keys())
    return df_a, metrics_keys, report_a

@app.cell
def __(df_synthetic, mo, pl):
    mo.md("## Section B — Costs, filters, sizing")
    
    df_b = df_synthetic.with_columns(
        (pl.col("feature_col") > 2).cast(pl.Float64).alias("signal"),
        (pl.col("feature_col") > 3).alias("entry_filter"),
        (pl.col("feature_col") * 2).alias("size")
    )
    report_b = df_b.lazy().bt.backtest_with_report(
        signal="signal",
        entry_filter_col="entry_filter",
        size_multiplier_col="size",
        commission_bps=5.0
    )
    return df_b, report_b

@app.cell
def __(df_a, mo):
    mo.md("## Section C — Fast metrics path")
    
    metrics_c = df_a.lazy().bt.backtest_metrics(signal="signal")
    return metrics_c,

@app.cell
def __(df_synthetic, mo, pl):
    mo.md("## Section D — Param sweep (pre-built cols)")
    
    df_d = df_synthetic.with_columns(
        (pl.col("feature_col") > 2).cast(pl.Float64).alias("signal_1"),
        (pl.col("feature_col") > 3).cast(pl.Float64).alias("signal_2"),
        (pl.col("feature_col") > 4).cast(pl.Float64).alias("signal_3")
    )
    sweep_df = df_d.lazy().bt.sweep(
        param_values=[1, 2, 3],
        signal_cols=["signal_1", "signal_2", "signal_3"]
    )
    return df_d, sweep_df

@app.cell
def __(df_synthetic, mo, pl):
    mo.md("## Section E — Param sweep (callback rebuild)")
    
    def build_fn(p, df_lazy):
        return df_lazy.with_columns(
            (pl.col("feature_col") > p).cast(pl.Float64).alias("signal")
        )
        
    sweep_cb_df = df_synthetic.lazy().bt.sweep_callback(
        param_grid=[2, 3, 4],
        build_fn=build_fn,
        signal="signal"
    )
    return build_fn, sweep_cb_df

@app.cell
def __(df_a, mo):
    mo.md("## Section F — Walk-forward OOS")
    
    wfo_folds = df_a.lazy().bt.walk_forward(
        signal="signal",
        train_bars=40,
        test_bars=20
    )
    return wfo_folds,

@app.cell
def __(df_synthetic, mo, pl):
    mo.md("## Section G — Walk-forward optimize")
    
    def build_fn_wfo(p, df_lazy):
        return df_lazy.with_columns(
            (pl.col("feature_col") > p).cast(pl.Float64).alias("signal")
        )
        
    wfo_opt = df_synthetic.lazy().bt.walk_forward_optimize(
        param_grid=[2, 3],
        build_fn=build_fn_wfo,
        signal="signal",
        train_bars=40,
        test_bars=20,
        objective="sharpe_ratio"
    )
    return build_fn_wfo, wfo_opt

@app.cell
def __(mo, pl):
    mo.md("## Section H — Cross-sectional panel")
    
    panel_df = pl.DataFrame({
        "timestamp": list(range(10)) * 3,
        "symbol": ["A"] * 10 + ["B"] * 10 + ["C"] * 10,
        "close": [100 + i for i in range(30)],
        "factor": [i % 3 for i in range(30)]
    })
    
    report_cs = panel_df.lazy().bt.cross_sectional_backtest(
        factor_col="factor",
        transform="zscore"
    )
    return panel_df, report_cs

@app.cell
def __(mo):
    mo.md("## Section I — Monte Carlo\n\nSee `quantwave_backtest::monte_carlo_trade_bootstrap` in Rust for the trade bootstrap logic.")
    return

@app.cell
def __(mo):
    mo.md("## Section J — PA moat pointer\n\nSee [PA Flag Breakout](pa_flag_breakout_strategy.md) for canonical PA E2E.")
    return

@app.cell
def __(mo):
    mo.md("## Section K — Parity callout\n\nQuantWave guarantees exact parity between streaming and batch mode. See [Batch & Streaming](../batch-streaming.md) and [ML Feature Parity](ml_feature_backtest_parity.md).")
    return

if __name__ == "__main__":
    app.run()
