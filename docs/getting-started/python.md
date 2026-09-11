# Getting Started with Python

QuantWave is designed to feel like a natural extension of Polars.

!!! tip "New here?"
    Start with the [Getting Started hub](index.md) — it walks zero → a real
    RSI column printed to your terminal in 5 copy-paste commands, with every
    output block verified against a real run. This page is the deeper
    reference once you've seen that work.
    Migrating from TA-Lib or pandas-ta? See [QuantWave vs alternatives](../comparison.md).

## Installation

```bash
pip install quantwave
# Polars batch/backtest examples also need:
pip install "quantwave[polars]"
```

The PyPI wheel bundles the core extension, Polars expression plugins (`pl.col().ta`), and the backtest engine. The `[polars]` extra installs the Polars Python package.

Verify your install:

```bash
quantwave doctor
```

```text
quantwave 0.7.0
  ✓ core extension (_quantwave)
  ✓ metadata registry
  ✓ streaming (RSI)
  ✓ polars installed
  ✓ backtest native (_backtest)
  ✓ Polars .bt namespace
  ✓ Polars expression plugins (pl.col().ta)

All checks passed.
```

See [Interpreting `doctor` output](#interpreting-doctor-output) below for what each check means and how to fix a `✗`.

```bash
quantwave list --category "Classic"
```

```text
alligator
apo
atr_ts
avgprice
beta
cmo
correl
linreg
medprice
natr
ppo
sar
stddev
trima
trix
true_range
ttm_squeeze
typprice
ultosc
vortex
```

## Quick Start

### Polars batch (recommended)

No file of your own needed — `quantwave.datasets.load_sample()` ships a
bundled, deterministic, synthetic OHLCV dataset:

```python
import polars as pl
import quantwave  # registers pl.col().ta and LazyFrame.bt
from quantwave import datasets

df = datasets.load_sample().filter(pl.col("symbol") == "NIFTY")

df = df.lazy().with_columns(
    pl.col("close").ta.rsi(timeperiod=14).alias("rsi"),
    pl.col("close").ta.ema(period=20).alias("ema"),
).collect()

print(df.select("ts", "close", "rsi", "ema").head())
```

```text
shape: (5, 4)
┌────────────────────────────┬──────────────┬─────┬──────────────┐
│ ts                         ┆ close        ┆ rsi ┆ ema          │
│ ---                        ┆ ---          ┆ --- ┆ ---          │
│ datetime[μs, Asia/Kolkata] ┆ f64          ┆ f64 ┆ f64          │
╞════════════════════════════╪══════════════╪═════╪══════════════╡
│ 2015-01-01 09:15:00 IST    ┆ 20096.573176 ┆ NaN ┆ 20096.573176 │
│ 2015-01-02 09:15:00 IST    ┆ 19809.830047 ┆ NaN ┆ 20069.264306 │
│ 2015-01-03 09:15:00 IST    ┆ 19968.148478 ┆ NaN ┆ 20059.634227 │
│ 2015-01-04 09:15:00 IST    ┆ 19838.827503 ┆ NaN ┆ 20038.605015 │
│ 2015-01-05 09:15:00 IST    ┆ 19566.377805 ┆ NaN ┆ 19993.630995 │
└────────────────────────────┴──────────────┴─────┴──────────────┘
```

`rsi`'s `NaN`s clear after 14 bars. `ema` here is a "cumulative from bar 1"
style indicator (see the conventions table in
[Warmup and NaN Semantics](#warmup-and-nan-semantics) below) — it produces a
value immediately, but that value isn't period-stable until `warmup_bars`
have accumulated. Check `qw.boundary_info("ema")` / `qw.metadata("ema")`
rather than assuming every indicator NaNs the same way.

Swap in your own data by replacing the `datasets.load_sample()` line with
`pl.read_parquet("ohlcv.parquet")` (or any Polars-loadable OHLCV source) —
everything downstream is unchanged as long as it has an `open`/`high`/`low`/`close`/`volume` schema.

### List-based batch API

```python
import quantwave as qw

closes = [float(x) for x in range(1, 100)]
rsi = qw.ta.rsi(14, closes)
print(rsi[-5:])
```

```text
[100.0, 100.0, 100.0, 100.0, 100.0]
```

(A monotonically rising input pins RSI at 100 once warmup clears — that's
expected, not a bug in the example.)

## Batch vs Streaming

Polars batch and streaming share the same math. For live or tick-by-tick use:

```python
import polars as pl
import quantwave as qw
from quantwave import datasets

closes = datasets.load_sample().filter(pl.col("symbol") == "NIFTY")["close"].head(16).to_list()

cls = qw.streaming_class("rsi")
wrapped = qw.wrap_streaming(cls(14), name="rsi")

for price in closes:
    val = wrapped.next(price)
    if wrapped.is_ready:
        print(round(val, 4) if val == val else val)  # val == val is False for NaN
```

```text
nan
48.5968
48.2092
```

`is_ready` flips as soon as `warmup_bars` (14) prices have been consumed, but
the very next value can still legitimately be `NaN` for indicators whose
math needs one bar more than their nominal warmup — check the printed value,
not just `is_ready`, when warmup is on a knife's edge like this.

The streaming API is powered by the universal `Next<T>` trait. Every indicator implements this single trait, which is the same mathematical core used by the Polars expressions. This design guarantees that batch results (via the `ta` namespace or `.ta` on LazyFrame) and streaming results are **bit-identical**.

!!! warning "Multi-input streaming indicators take positional args, not a tuple"
    Single-input indicators like RSI take `next(price)`. Multi-input
    indicators (e.g. `supertrend`, which needs high/low/close) take
    `next(high, low, close)` on the **raw** streaming class — but
    `StreamingWrapper.next()` (what `wrap_streaming()` returns) only forwards
    a single positional value, so it cannot currently drive a multi-input
    indicator. Use the raw `qw.streaming_class(...)` instance directly for
    multi-input indicators and track readiness yourself against
    `qw.warmup_bars(...)`.

## Warmup and NaN Semantics

Most indicators need a **warmup period** before their output is meaningful. During warmup, batch columns contain `NaN` and streaming `next()` returns `NaN` until enough history is accumulated.

!!! danger "Warmup is `NaN`, not `null` — `drop_nulls()` does nothing"

    This is the highest-surprise convention in QuantWave. Read it once and you
    will save yourself a wrong backtest.

    ```python
    import polars as pl
    import quantwave
    from quantwave import datasets

    df = datasets.load_sample().filter(pl.col("symbol") == "NIFTY")
    df = df.with_columns(pl.col("close").ta.rsi(14).alias("rsi"))

    print(df["rsi"].null_count())    # 0   <- there are NO nulls
    print(df["rsi"].is_nan().sum())  # 14  <- the warmup is NaN

    print(df.drop_nulls().height)    # SILENT NO-OP: all rows survive, warmup included
    print(df.drop_nans().height)     # this one actually drops warmup
    ```

    ```text
    0
    14
    2520
    2506
    ```

    Two consequences:

    1. **`.drop_nulls()` / `.dropna()` is a complete no-op on indicator warmup.**
       Warmup rows flow straight into backtests, feature matrices and
       aggregations with no error raised anywhere.
    2. **NaN comparisons are always `False`.** `NaN < 30` is `False`, so
       `(pl.col("rsi") < 30).cast(pl.Float64)` yields `0.0` across the whole
       warmup — indistinguishable from a genuine no-signal period. Your
       strategy looks like it simply chose not to trade for 14 bars.

    Use [`qw.trim_warmup()`](#trimming-warmup) instead. It is alignment-preserving:
    `drop_nans()` drops rows per column set, so which rows disappear depends on
    which columns you happen to be holding at the time.

```python
import quantwave as qw

# How many leading bars to skip before trusting the signal?
n = qw.warmup_bars("rsi", {"period": 14})
print(n)

meta = qw.metadata("macd")
print(meta.warmup_bars)  # curated default when available
```

```text
14
26
```

**Conventions:**

| Style | Behavior | Examples |
|-------|----------|----------|
| NaN until ready | Output is `NaN` for the first `warmup_bars` bars | RSI, EMA, MACD, ATR |
| Cumulative from bar 1 | Value exists immediately but is not period-stable | OBV, NVI |
| Event / struct | Empty events or default structs early on | Market Structure, S/R monitor |

Use `qw.assert_parity()` for batch vs streaming checks — it compares warmup bars for agreement, then enforces equality on post-warmup values.

### Trimming warmup

`qw.trim_warmup()` slices off the **maximum** warmup across every indicator you
name, so columns with different warmups stay row-aligned:

```python
import polars as pl
import quantwave as qw
from quantwave import datasets

df = datasets.load_sample().filter(pl.col("symbol") == "NIFTY")
df = df.with_columns(
    pl.col("close").ta.rsi(14).alias("rsi"),
    pl.col("close").ta.ema(50).alias("ema"),
)

clean = df.pipe(qw.trim_warmup, "rsi", ("ema", {"period": 50}))
print(df.height, "->", clean.height)
print(clean.select("rsi", "ema").head(2))
```

```text
2520 -> 2470
shape: (2, 2)
┌───────────┬──────────────┐
│ rsi       ┆ ema          │
│ ---       ┆ ---          │
│ f64       ┆ f64          │
╞═══════════╪══════════════╡
│ 48.78492  ┆ 19686.970601 │
│ 48.147722 ┆ 19679.825328 │
└───────────┴──────────────┘
```

50 leading rows dropped (the larger of RSI's 14 and EMA's 50); `rsi` and `ema` are both finite from row 0, still aligned.

Accepted spec forms, freely mixed:

| Form | Example |
|------|---------|
| Indicator name | `qw.trim_warmup(df, "rsi")` |
| Name + the params you called it with | `qw.trim_warmup(df, ("rsi", {"period": 21}))` |
| Mapping | `qw.trim_warmup(df, {"rsi": {"period": 21}, "ema": {"period": 50}})` |
| Explicit bar count (custom/derived columns) | `qw.trim_warmup(df, "rsi", 30)` |

Options:

- `extra=` — extra bars to drop for transforms chained *after* the indicator
  (a `diff()`, a `shift()`), which add warmup QuantWave cannot see.
- `strict=` — defaults to `True`: a misspelled indicator name raises instead of
  silently contributing `0` bars and trimming nothing. Pass `strict=False` to opt out.

Works on `DataFrame`, `LazyFrame` and `Series`. `qw.warmup_rows(*specs)` returns
the row count on its own if you want to slice by hand.

## Discovery, categories & boundaries

```python
import quantwave as qw

print(len(qw.indicators()))
print(qw.categories())
print(qw.category("Ehlers DSP")[:5])

meta = qw.metadata("rsi")
info = qw.boundary_info("rsi")  # warmup, NaN, invalid-param semantics
print(meta.name, meta.category, meta.warmup_bars)
print(info)
```

```text
221
['Candlestick', 'Classic', 'Cycle / Ehlers', 'Ehlers DSP', 'ML Features', 'Modern', 'Momentum', 'Moving Averages', 'Overlap', 'Patterns', 'Price Action', 'Regime', 'Regime / Statistics', 'Rocket Science', 'Statistics', 'Support/Resistance', 'Trend', 'Uncategorized', 'Volatility', 'Volatility / Trend', 'Volume', 'Volume / Momentum', 'Wilder']
['am_detector', 'autotune_filter', 'bandpass', 'butterworth2', 'butterworth3']
rsi Momentum 14
BoundaryInfo(warmup_behavior='Leading bars return NaN until warmup_bars is satisfied.', period_gt_len='When period exceeds series length, output is all NaN.', nan_inputs='NaN in input propagates to output (NaN out).', invalid_params='Non-positive period or missing required params raise ValueError.', empty_data='Empty input returns an empty result series.', conventions=())
```

!!! warning "`quantwave info <name>` (the CLI subcommand) is currently broken"
    `quantwave info rsi` raises `AttributeError: 'IndicatorMeta' object has no
    attribute 'display_name'` on this build — a pre-existing bug in
    `cli.py`, unrelated to this page. Use `qw.metadata("rsi")` from Python
    (shown above) until it's fixed; `quantwave list` and `quantwave doctor`
    are unaffected.

## TA-Lib migration

```python
from quantwave import talib as ta

print(ta.list_functions()[:5])   # uppercase TA-Lib names in this build
closes = [float(x) for x in range(1, 30)]
rsi = ta.RSI(closes, timeperiod=14)
print(rsi[-3:])
```

```text
['ACOS', 'AD', 'ADOSC', 'ADX', 'ADXR']
[100. 100. 100.]
```

## Exception contract

```python
import quantwave as qw

closes = [float(x) for x in range(1, 30)]
try:
    qw.assert_parity("rsi", {"period": 14}, closes)
    print("parity ok")
except qw.ParityError:
    print("batch vs streaming mismatch")
except qw.IndicatorNotFoundError:
    print("unknown name")
except qw.QuantwaveError:
    print("any library-specific error")

print(qw.__version__)
```

```text
parity ok
0.7.0
```

`qw.__version__` is exposed via `importlib.metadata`.

## ML features & backtesting

- [ML Feature Engineering guide](../guides/ml_features.md)
- [Backtest Quickstart](../guides/backtest/quickstart.md)
- [ML Features → Backtest (E2E)](../examples/notebooks/ml_feature_backtest_parity.md)

## Options (India)

Options chain analytics and Black–Scholes helpers live under `quantwave.options` (not the top-level indicator namespace):

```python
from quantwave import options

print(options.bs_call_price(s=100, k=100, r=0.07, t=0.1, sigma=0.2))
print(options.nse_lot_size("NIFTY"))
```

```text
2.8780680705463553
50
```

`bs_call_price` takes `s` (spot), not `spot=` — the parameter is short-form,
unlike the keyword-argument style used elsewhere in the package.

Legacy `import quantwave; quantwave.bs_call_price(...)` still works but emits a `DeprecationWarning`.

## Backtesting

QuantWave includes a Polars-native, high-performance backtest engine. You can run backtests, param sweeps, and walk-forward optimizations directly on your dataframes using the `.bt` namespace. For a 5-minute introduction, see the [Backtest Quickstart](../guides/backtest/quickstart.md).

## Troubleshooting

### Import failures

| Symptom | Cause | Fix |
|---|---|---|
| `ModuleNotFoundError: No module named 'quantwave'` | Not installed in the active environment | `pip install "quantwave[polars]"`, confirm with `python -c "import quantwave"` in the same interpreter you're running |
| `AttributeError: 'LazyFrame'/'Series'/'Expr' object has no attribute 'ta'` or `'bt'` | You imported `polars` but never `import quantwave` | `.ta` and `.bt` are registered as a **side effect** of `import quantwave` — add a bare `import quantwave` even if you never reference the name directly |
| `ImportError: ... _quantwave` / `_backtest` / `_ta_namespace` | Native extension missing or ABI mismatch (e.g. mixing a wheel built for one Python minor version with an unsupported interpreter, or a broken editable install) | Reinstall: `pip install --force-reinstall "quantwave[polars]"`; contributors building from source should re-run `maturin develop` |
| `quantwave: command not found` after `pip install` | `pip`'s script directory isn't on `PATH` (common in some venv/pyenv setups) | Run `python -m quantwave.cli doctor` instead, or fix `PATH` |

### Platform / Python-version wheel matrix

QuantWave ships a single **abi3** wheel per platform (one wheel covers every
supported Python minor version — no per-Python-version rebuild needed). Per
the release pipeline (`.github/workflows/release.yml`):

| Platform | Built | Python versions covered |
|---|---|---|
| Linux x86_64 | ✓ | 3.9 – 3.13 (abi3) |
| Linux arm64 | ✓ | 3.9 – 3.13 (abi3) |
| macOS (Apple Silicon / Universal, `MACOSX_DEPLOYMENT_TARGET=11.0`) | ✓ | 3.9 – 3.13 (abi3) |
| Windows x86_64 | ✓ | 3.9 – 3.13 (abi3) |

`requires-python = ">=3.9"` (see `quantwave-py/pyproject.toml`); every wheel
is smoke-tested in CI against Python 3.9, 3.11, 3.12 and 3.13. If `pip
install quantwave` tries to build from source instead of pulling a wheel,
you're likely on a platform/Python combination outside this matrix (or
using `--no-binary`) — check the [PyPI files page](https://pypi.org/project/quantwave/#files)
for what's actually published for the current release.

### Interpreting `doctor` output

`quantwave doctor` runs a fixed sequence of checks and prints `✓`/`✗` per
line; a `✗` means that specific check's `import`/call raised, and the
exception text after `:` is the real Python error, not a canned message:

| Check | What it verifies | If it's `✗` |
|---|---|---|
| `core extension (_quantwave)` | The native Rust extension loaded at all | Reinstall the wheel; likely an ABI/platform mismatch (see the matrix above) |
| `metadata registry` | `qw.metadata("rsi")` resolves | Same as above — the registry ships inside the native extension |
| `streaming (RSI)` | A streaming class can be constructed | Same as above |
| `polars installed` | `import polars` succeeds | `pip install "quantwave[polars]"` |
| `backtest native (_backtest)` | The backtest native module loaded | Reinstall; only runs if Polars is present |
| `Polars .bt namespace` | `LazyFrame.bt` is registered | Usually resolves itself once `_backtest` loads — reinstall if it persists |
| `Polars expression plugins (pl.col().ta)` | `.ta` is registered | Reinstall the **unified** wheel — a partial/split install is the usual cause |

`doctor`'s exit code is nonzero if any check failed — safe to use in a CI
smoke test: `quantwave doctor || exit 1`.

## Where to go next

| Goal | Next step |
|------|-----------|
| Compare stacks | [QuantWave vs TA-Lib & pandas-ta](../comparison.md) |
| Browse indicators | [Indicators overview](../guides/indicators/index.md) · [Gallery](../guides/indicators/gallery.md) |
| Batch ↔ streaming deep dive | [Examples guide](../examples/batch-streaming.md) |
| Plugin vs `.ta` | [When to use which](../guides/plugin_vs_ta.md) |
| Backtest | [Quickstart](../guides/backtest/quickstart.md) · [Strategy notebook](../examples/notebooks/strategy_backtest.md) |
| Full funnel | [Getting Started hub](../index.md) |
