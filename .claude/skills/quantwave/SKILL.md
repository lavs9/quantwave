---
name: quantwave
description: Correct usage of QuantWave — the Polars-native technical analysis and backtesting library (221 Rust indicators, `.ta` / `.bt` namespaces, batch↔streaming parity). Covers the happy path plus the silent numeric footguns that produce wrong-but-plausible results. Use when writing, reviewing, or debugging code that imports `quantwave`, uses `pl.col().ta.*`, `lf.ta()`, `LazyFrame.bt.*`, `quantwave.backtest`, `quantwave.talib`, `quantwave.options`, or `streaming_class`/`wrap_streaming`; when computing indicators (RSI, EMA, ATR, MACD, SuperTrend, Ehlers DSP, regimes) on Polars frames; when backtesting a signal column or interpreting Sharpe/drawdown/trade output; or when migrating from TA-Lib or pandas-ta.
---

# QuantWave

Polars-native TA + backtesting. One Rust math core (`Next<T>`) powers batch columns,
expression plugins, and streaming — results are bit-identical across all three.

**Before writing any backtest or interpreting any metric, read [PITFALLS.md](PITFALLS.md).**
Most QuantWave mistakes are silent: they return a number, and the number is wrong.

## Install

```bash
pip install "quantwave[polars]"   # the [polars] extra is what registers .ta / .bt
quantwave doctor                  # verify the native extension loaded
```

## Quick start

```python
import polars as pl
import quantwave              # side-effect import: registers pl.col().ta and LazyFrame.bt

df = pl.read_parquet("ohlcv.parquet").lazy().with_columns(
    pl.col("close").ta.rsi(timeperiod=14).alias("rsi"),   # note: timeperiod
    pl.col("close").ta.ema(period=20).alias("ema"),       # note: period
).collect()
```

Parameter names are **not** uniform — TA-Lib-derived indicators take `timeperiod`,
native ones take `period`. Never guess; confirm with `qw.metadata("<slug>")`.

## Pick the right surface

| You are doing | Use | Why |
|---|---|---|
| Research on a LazyFrame, many columns | `lf.ta().rsi("close", 14)` | Full namespace: PA, regimes, `.ta.features.*` |
| Hot vectorized path, 10M+ rows | `pl.col("close").ta.rsi(14)` | Expression plugin, zero-copy Arrow |
| Live / bar-by-bar | `qw.streaming_class(...)` + `qw.wrap_streaming(...)` | Stateful; plugins cannot do this |
| Backtesting a signal | `lf.bt.backtest_with_report(...)` | See [BACKTEST.md](BACKTEST.md) |
| Explicit orders (limit/stop/bracket) | `lf.bt.order_backtest(orders, ...)` | See [BACKTEST.md](BACKTEST.md) |
| Drop-in TA-Lib replacement | `from quantwave import talib as ta` | Uppercase TA-Lib names, TA-Lib semantics |
| Options / Greeks / chain analytics | `from quantwave import options` | Not on the indicator namespace |

## Discovery — never guess a name or parameter

```python
import quantwave as qw

qw.indicators()                    # 221 slugs
qw.is_indicator("supertrend")      # membership check before calling
qw.metadata("macd")                # params, category, warmup_bars
qw.warmup_bars("rsi", {"period": 14})   # -> 14 leading bars to discard
qw.boundary_info("rsi")            # warmup / NaN / invalid-param semantics
```

CLI equivalents: `quantwave list`, `quantwave info rsi`.

## Warmup is not optional

Indicators emit `NaN` for the first `warmup_bars` bars. Slice them off before feeding
anything downstream — a backtest, a metric, or an ML model — or the first trades and
statistics are computed on garbage.

```python
n = qw.warmup_bars("rsi", {"period": 14})
clean = df.slice(n)
```

Streaming has the same rule, expressed as a flag: gate on `wrapped.is_ready`, and
pass `name=` to `wrap_streaming` or `is_ready` cannot be computed.

## Verify parity when it matters

```python
qw.assert_parity("rsi", {"period": 14}, closes)   # raises qw.ParityError on mismatch
```

Use this when porting a strategy from research to live, not on every call.

## Reference files

- **[PITFALLS.md](PITFALLS.md)** — how *not* to use QuantWave. Verified silent-wrongness
  cases: `roc` vs `rocp` (100×), `stddev` ddof=0 vs pandas ddof=1, `atr` vs `ta_atr`,
  default 1-unit position sizing, `same_bar` fills, `hmm_bull_bear` look-ahead.
- **[BACKTEST.md](BACKTEST.md)** — `.bt` input requirements, sizing model, output
  contract (units, signs, schemas), and the order/portfolio APIs.
- `scripts/check_usage.py` — static-lints a file for the known anti-patterns:
  `python scripts/check_usage.py strategy.py`

Official docs: <https://lavs9.github.io/quantwave/> · `llms.txt`: <https://lavs9.github.io/quantwave/llms.txt>
