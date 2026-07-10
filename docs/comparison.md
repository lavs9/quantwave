# QuantWave vs TA-Lib & pandas-ta One-page guide for teams choosing a technical-analysis stack on Polars (or migrating from pandas). !!! tip "Short answer" **Stay on TA-Lib or pandas-ta** if you only need classic indicators in small pandas notebooks and never care about streaming parity. **Choose QuantWave** if you run Polars at scale, need Ehlers DSP / price action / regimes, or want one math core for batch research **and** live bars without drift. ## At a glance | Dimension | **QuantWave** | **TA-Lib** | **pandas-ta** |
|-----------|---------------|------------|---------------|
| **Core** | Rust (``) | C library | Pure Python (pandas) |
| **DataFrame API** | Polars-native (`.ta`, plugins, `.bt`) | NumPy arrays / pandas via wrappers | pandas `DataFrame` columns |
| **Registered indicators** | **221** native + metadata | ~158 functions | ~130 strategies |
| **Ehlers DSP** | 30+ dedicated tools | Minimal | A few community ports |
| **Price action** | Market Structure, flags/H&S, S/R monitor | Candlestick patterns only | Limited |
| **Regime detection** | HMM, GMM, PELT, vol clustering | No | No |
| **Batch ↔ streaming parity** | **[Guaranteed](../validation.md)** (`Next<T>` + proptests) | N/A (batch only) | No |
| **Backtest engine** | Built-in (sweep, WFO, Monte Carlo) | No | No |
| **Large-data speed** | See [benchmarks](../benchmarks.md) | Fast C for classics | Slow on 1M+ rows |
| **License** | MIT | BSD-like | MIT |
| **Migration path** | `quantwave.talib` shim + `.ta` names | — | Re-write columns to Polars `.ta` | --- ## Same indicator, three stacks ### RSI (14) on OHLCV === "QuantWave (Polars)" ```python
import polars as pl df = pl.read_parquet("ohlcv.parquet")
out = ( df.lazy() .with_columns(pl.col("close").ta.rsi(timeperiod=14).alias("rsi")) .collect()
)
``` === "TA-Lib (Python)" ```python
import talib
import pandas as pd df = pd.read_parquet("ohlcv.parquet")
df["rsi"] = talib.RSI(df["close"].values, timeperiod=14)
``` === "pandas-ta" ```python
import pandas as pd
import pandas_ta as ta df = pd.read_parquet("ohlcv.parquet")
df.ta.rsi(length=14, append=True) # adds RSI column
``` **Takeaway:** ergonomics are similar for classics. QuantWave's advantage shows on **Polars lazy pipelines**, **multi-indicator feature matrices**, and when you need the **same RSI** in a live stream without re-implementing warmup logic. ### SuperTrend (recursive — where speed diverges) SuperTrend is stateful bar-by-bar math. pandas-ta and naive pandas loops pay Python interpreter cost on every row; QuantWave compiles the same `Next<T>` path for batch plugins and streaming. Measured throughput comparisons are being rebuilt on a reproducible harness. Until those results publish, see [benchmarks](../benchmarks.md) for methodology and the memory measurements that are already verified. --- ## When QuantWave wins - **Polars is your execution engine** — you want `.ta` and `.bt` on `LazyFrame`, not `.apply()` bridges from pandas.
- **Research → production parity** — batch columns must match live `streaming_class()` output (`qw.assert_parity()`).
- **Beyond TA-Lib classics** — Ehlers cycle tools, MQL5-style price action, regime changepoints, fractional differentiation for ML.
- **One MIT stack** — indicators + backtest + metadata discovery without gluing TA-Lib + vectorbt + custom Rust.
- **Validated correctness** — gold-standard vectors in `/tests/gold_standard/`, not "looks close enough." ## When TA-Lib wins - **Minimal dependency** — C library + thin Python wrapper is enough; no Polars, no Rust toolchain.
- **Interop with legacy C extensions** — existing systems already pass NumPy buffers to `talib.*`.
- **Classic set only** — ~150 functions cover your roadmap; no Ehlers, PA, or backtest needs.
- **Tiny datasets** — performance gap vs Rust is irrelevant below ~50k rows. ## When pandas-ta wins - **Quick pandas prototypes** — one `pip install`, append columns in a notebook, ship a one-off analysis.
- **No Polars migration yet** — team is 100% pandas and not optimizing for scale.
- **No streaming or parity requirements** — research notebook never feeds a live system.
- **Familiar pandas-ta strategy names** — short-lived project where rewrite cost exceeds benefit. --- ## Migrating to QuantWave ### From TA-Lib QuantWave ships a compatibility layer for drop-in exploration: ```python
from quantwave import talib as ta rsi = ta.RSI(closes, timeperiod=14)
print(ta.list_functions()) # uppercase TA-Lib names in this build
``` For production Polars pipelines, prefer native names and metadata: ```python
import quantwave as qw meta = qw.metadata("rsi")
n = qw.warmup_bars("rsi", {"period": 14})
``` Full Python setup: [Getting Started (Python)](../getting-started/python.md#ta-lib-migration). ### From pandas-ta Typical migration steps: 1. **Move data to Polars** — `pl.from_pandas(df)` or read Parquet directly.
2. **Map strategy names** — use search or `qw.indicators()`; many pandas-ta names map 1:1 (`rsi`, `macd`, `bbands`).
3. **Replace `.ta.*` append calls** with lazy `.with_columns(pl.col(...).ta.*)`.
4. **Verify warmup** — `qw.warmup_bars()` and `qw.assert_parity()` before trusting live signals.
5. **Re-run benchmarks** on your schema — [benchmarks](../benchmarks.md) uses 1M-row OHLCV; your cardinality may differ. ```python
# pandas-ta habit
# df.ta.sma(length=20, append=True) # QuantWave equivalent
df = df.lazy().with_columns( pl.col("close").ta.sma(period=20).alias("sma_20")
).collect()
``` --- ## QuantWave surfaces (after you choose) | Surface | Best for |
|---------|----------|
| `pl.col("close").ta.rsi(14)` | Expression-plugin hot paths |
| `lf.ta().rsi(14)` | Multi-column LazyFrame research |
| `qw.streaming_class("rsi")` | Live bar-by-bar |
| `lf.bt.backtest_with_report()` | Strategy evaluation | See [Plugin vs `.ta`](../guides/plugin_vs_ta.md) and the [backtest capability matrix](../guides/backtest/capability_matrix.md). --- ## Also compared (brief) | Library | Role | vs QuantWave |
|---------|------|--------------|
| **vectorbt** | Vectorized portfolio backtest | Stronger if backtest-only is the product; AGPL license; bring your own indicators |
| **polars-backtest** | Long-format sim on Polars | Simpler sim, no indicator breadth |
| **Other Rust TA crates** | Fast scalar math | Rarely Polars-native; no guaranteed streaming parity or PA/regime depth | --- ## Verify claims yourself ```bash
./scripts/quantwave_verify.sh
``` ```python
import quantwave as qw qw.assert_parity("rsi", {"period": 14}, your_closes)
print(len(qw.indicators()), "registered names")
``` [Full benchmarks →](../benchmarks.md) · [Getting started →](../getting-started/index.md) · [Indicator gallery →](../guides/indicators/gallery.md)