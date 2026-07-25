import marimo

__generated_with = "0.4.5"
app = marimo.App(width="medium")


@app.cell
def __():
    import math

    import marimo as mo
    import polars as pl
    import quantwave  # registers LazyFrame.bt namespace

    return math, mo, pl, quantwave


@app.cell
def __(mo):
    mo.md(
        """
        # Execution-Aware Research — Orders, Risk Overlays, Benchmark-Relative Reporting

        This is the canonical end-to-end walkthrough of QuantWave's
        **execution-realism** layer: first-class order types, risk overlays,
        and benchmark-relative reporting — all on the same engine that
        guarantees **batch == streaming** parity.

        Sections:

        1. **Order-mode fills** — market / limit / stop / stop-limit resolved
           against each bar's OHLC (`.bt.order_backtest`).
        2. **Risk overlays** — vol-targeting / inverse-vol / position-limit /
           pre-trade filters (`risk_model=`).
        3. **Benchmark-relative reporting** — alpha / beta / excess return,
           plus Calmar / VaR / CVaR (`metrics_with_benchmark`,
           `extended_metrics`).

        Design notes: [Order-Mode Execution ADR](../../../planning/ORDER_MODE_EXECUTION_ADR.md)
        · [Capability Matrix](../../guides/backtest/capability_matrix.md)
        """
    )
    return


@app.cell
def __(math, pl):
    # A deterministic OHLC series: gentle uptrend + a cyclical wobble so
    # limits and stops actually get touched. No randomness → reproducible.
    N = 160
    close = [100.0]
    for i in range(1, N):
        drift = 0.10
        wobble = 1.8 * math.sin(i / 6.0)
        close.append(close[-1] + drift + wobble * 0.18)

    open_ = [close[0]] + close[:-1]
    high = [max(o, c) + 0.6 for o, c in zip(open_, close)]
    low = [min(o, c) - 0.6 for o, c in zip(open_, close)]

    bars = pl.DataFrame(
        {
            "timestamp": list(range(N)),
            "open": open_,
            "high": high,
            "low": low,
            "close": close,
        }
    )
    bars.tail(4)
    return N, bars, close, high, low, open_


@app.cell
def __(mo):
    mo.md(
        """
        ## 1 — Order-mode fills

        `.bt.order_backtest(orders, ...)` drives the flat/single-position
        order-execution core directly, with an explicit long-format order
        spec instead of a signal column. Every fill is a **pure function of
        one bar's OHLC** — so batch and streaming agree by construction (the
        parity moat), and same-bar ambiguity is resolved by the conservative
        stop-before-target convention (see the ADR).

        Here: a **limit buy** rests at a price below market (fills only if the
        bar trades down to it), and a protective **stop sell** exits on a
        breakdown.
        """
    )
    return


@app.cell
def __(bars, close, pl):
    entry_bar, exit_bar = 8, 90
    orders = pl.DataFrame(
        {
            "bar_index": [entry_bar, exit_bar],
            "side": ["buy", "sell"],
            "type": ["limit", "stop"],
            "qty": [400.0, 400.0],
            # limit rests 1.5 below the entry bar's close; stop 4.0 below the exit bar's close
            "price": [close[entry_bar] - 1.5, None],
            "trigger": [None, close[exit_bar] - 4.0],
        }
    )

    order_trades, order_equity = bars.lazy().bt.order_backtest(
        orders,
        commission_bps=1.0,
        slippage_bps=1.0,
    )
    assert order_equity.height == bars.height
    order_trades
    return entry_bar, exit_bar, order_equity, order_trades, orders


@app.cell
def __(mo):
    mo.md(
        """
        ### Bracket / OCO exits

        Attach a protective **take-profit + stop-loss** pair to an entry by
        adding `take_profit` / `stop_loss` columns. Once the position opens,
        the bracket is checked against every subsequent bar's OHLC; a same-bar
        double-touch resolves **stop-before-target** (pessimistic). Here a
        single market entry is protected by a bracket instead of a manual exit
        order.
        """
    )
    return


@app.cell
def __(bars, close, entry_bar, pl):
    bracket_orders = pl.DataFrame(
        {
            "bar_index": [entry_bar],
            "side": ["buy"],
            "type": ["market"],
            "qty": [400.0],
            "price": [None],
            "trigger": [None],
            "take_profit": [close[entry_bar] + 6.0],
            "stop_loss": [close[entry_bar] - 3.0],
        }
    )
    bracket_trades, _bracket_equity = bars.lazy().bt.order_backtest(
        bracket_orders,
        commission_bps=1.0,
        slippage_bps=1.0,
    )
    bracket_trades
    return bracket_orders, bracket_trades


@app.cell
def __(mo):
    mo.md(
        """
        ## 2 — Risk overlays (`risk_model=`)

        Overlays resize the desired exposure at **one shared point** in the
        engine, so they apply identically in batch and streaming. Semantics
        are **entry-time**: the engine has no intra-trade resizing, so a
        continuous scaler (vol-target, inverse-vol) takes effect when a
        position *opens*.

        Below, a always-long signal is run three ways: unscaled, vol-targeted
        to 15% annual, and position-limited. Note how the entry quantity
        changes.
        """
    )
    return


@app.cell
def __(N, close, pl):
    # Flat during the vol warm-up, then always-on — so the position opens
    # *after* the lookback window and the continuous scaler is in effect.
    warmup = 20
    signals = pl.DataFrame(
        {
            "timestamp": list(range(N)),
            "close": close,
            "signal": [0.0] * warmup + [1.0] * (N - warmup),
        }
    ).lazy()

    baseline = signals.bt.backtest_with_report(
        signal="signal", commission_bps=0.0, slippage_bps=0.0
    )
    vol_targeted = signals.bt.backtest_with_report(
        signal="signal",
        commission_bps=0.0,
        slippage_bps=0.0,
        risk_model={
            "vol_target": {
                "target_annual_vol": 0.15,
                "lookback": warmup,
                "annualization": 252.0,
            }
        },
    )
    position_limited = signals.bt.backtest_with_report(
        signal="signal",
        commission_bps=0.0,
        slippage_bps=0.0,
        risk_model={"position_limit": {"max_abs_exposure": 3.0}},
    )

    sizing = pl.DataFrame(
        {
            "overlay": ["none", "vol_target(15%)", "position_limit(3)"],
            "entry_qty": [
                baseline.result.trades["quantity"].abs().max(),
                vol_targeted.result.trades["quantity"].abs().max(),
                position_limited.result.trades["quantity"].abs().max(),
            ],
        }
    )
    sizing
    return baseline, position_limited, signals, sizing, vol_targeted, warmup


@app.cell
def __(mo):
    mo.md(
        """
        ## 3 — Benchmark-relative reporting

        `metrics_with_benchmark(benchmark_returns)` adds alpha / beta /
        cumulative-vs-benchmark to the stable metric contract;
        `extended_metrics()` adds Calmar / VaR-95 / CVaR-95. The benchmark
        here is buy-and-hold of the underlying (per-bar simple returns,
        aligned by index).
        """
    )
    return


@app.cell
def __(baseline, close, pl):
    buy_and_hold = [0.0] + [close[i] / close[i - 1] - 1.0 for i in range(1, len(close))]

    with_bench = baseline.metrics_with_benchmark(buy_and_hold)
    bench = with_bench["benchmark"]

    report_rows = pl.DataFrame(
        {
            "metric": [
                "alpha",
                "beta",
                "strategy_cum_return",
                "benchmark_cum_return",
                "excess_cum_return",
                "calmar_ratio",
                "var_95",
                "cvar_95",
            ],
            "value": [
                bench["alpha"],
                bench["beta"],
                bench["cumulative_return"],
                bench["benchmark_cumulative_return"],
                bench["excess_cumulative_return"],
                with_bench["calmar_ratio"],
                with_bench["var_95"],
                with_bench["cvar_95"],
            ],
        }
    )
    report_rows
    return bench, buy_and_hold, report_rows, with_bench


@app.cell
def __(mo):
    mo.md(
        """
        ### Takeaways

        - **One engine, one parity guarantee.** Order-mode fills, overlays,
          and reporting all ride the same batch/streaming-parity core — no
          second engine, no "parity tier".
        - **OHLC-touched convention.** With only OHLC the intrabar path is
          unknowable; QuantWave uses the industry-standard conservative
          convention (stop before target on a same-bar double-touch) — for
          both bracket exits and standalone stops. Tick / lower-timeframe
          fidelity is deliberately out of scope (see the ADR).
        - **Entry-time overlays.** Continuous scalers size at entry; there is
          no intra-trade resizing.

        **Sources.** Execution convention & risk-overlay design:
        `planning/ORDER_MODE_EXECUTION_ADR.md`. Gap analysis motivating this
        layer: QuantJourney-bt (execution realism + reporting) vs QuantWave's
        Rust/Polars parity core.
        """
    )
    return


if __name__ == "__main__":
    app.run()
