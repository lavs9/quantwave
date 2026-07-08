# Frequently Asked Questions

!!! tip "Short answer"
    QuantWave is a **Polars-native quant library** with **221 Rust indicators**, a built-in **backtest engine** in the Python package, and **guaranteed batch ↔ streaming parity** via one `Next<T>` math core. Install with `pip install "quantwave[polars]"`.

## What is QuantWave?

QuantWave is a high-performance technical analysis and backtest stack built in Rust, exposed through Polars (`.ta`, `.bt`) and Python/Rust streaming APIs. It targets researchers and engineers who need TA-Lib-class breadth plus Ehlers DSP, price action, regimes, and production-grade speed on large datasets.

→ [Getting Started](../getting-started/index.md) · [Home](../index.md)

## How is QuantWave different from TA-Lib or pandas-ta?

| | QuantWave | TA-Lib | pandas-ta |
|---|-----------|--------|-----------|
| Data API | Polars-native | NumPy/pandas | pandas |
| Streaming parity | Guaranteed | No | No |
| Backtest | Built-in `.bt` | No | No |
| Ehlers / PA / regimes | Deep | Minimal | Limited |

**Stay on TA-Lib/pandas-ta** for small pandas notebooks. **Choose QuantWave** for Polars at scale, live parity, or beyond-classic indicators.

→ [Full comparison](../comparison.md)

## Does the backtest engine ship in the Python package?

**Yes.** The unified PyPI wheel bundles `quantwave._backtest`. With Polars installed (`pip install "quantwave[polars]"`), use the `.bt` namespace on `LazyFrame` / `DataFrame`:

```python
import quantwave  # registers LazyFrame.bt

report = signal_df.lazy().bt.backtest_with_report(
    signal="signal", close_col="close", timestamp_col="bar"
)
```

You can also import `BacktestEngine` directly from `quantwave.backtest`. No separate backtest package is required.

→ [Backtest quickstart](../guides/backtest/quickstart.md) | [Backtest output contract](../guides/backtest/output-contract.md)

## What is batch vs streaming parity?

Every indicator implements Rust's `Next<T>` trait once. That same logic powers:

- Polars batch columns (`pl.col("close").ta.rsi(14)`)
- Python streaming (`qw.streaming_class("rsi")` + `wrap_streaming`)
- Polars plugins (expression hot paths)

`qw.assert_parity()` verifies batch and streaming outputs match after warmup. This lets you research in Polars and deploy live without re-implementing indicators.

→ [Batch & streaming guide](../examples/batch-streaming.md)

## How do I install QuantWave for Polars?

```bash
pip install "quantwave[polars]"
quantwave doctor
```

The `[polars]` extra installs Polars. The core wheel (indicators, metadata, streaming, backtest) is already included.

→ [Python getting started](../getting-started/python.md)

## How many indicators are included?

**221** registered native indicators (metadata-driven catalog), including classics, Ehlers DSP, candlestick patterns, price action (Market Structure, geometric patterns, S/R), regimes, and ML feature tools. Count is validated in CI against the Rust metadata registry.

→ [Full catalog](../guides/indicators/native/) · [Gallery](../guides/indicators/gallery.md)

## How do I find a specific indicator?

1. **Search** — press `/` on the docs site or use `qw.indicators()` / `quantwave list` in CLI
2. **Catalog** — [native index](../guides/indicators/native/) has slug lookup tables
3. **Gallery** — curated high-value starting points

Example: `qw.metadata("supertrend")` returns parameters, category, and warmup hints.

## Should I use expression plugins or `.ta` on LazyFrame?

**Same math, different ergonomics:**

- **`pl.col("close").ta.rsi(14)`** — expression plugins; best for hot vectorized paths
- **`lf.ta().rsi(14)`** — LazyFrame chaining; best for multi-column research pipelines
- **Streaming** — `streaming_class`; not plugins (stateful per bar)

→ [Plugin vs `.ta` guide](../guides/plugin_vs_ta.md)

## Is QuantWave free to use?

**Yes — MIT licensed.** Open source on [GitHub](https://github.com/lavs9/quantwave). PyPI package and documentation are free for commercial and research use.

## How do I cite QuantWave or verify claims?

**Official docs (preferred for AI and humans):** https://lavs9.github.io/quantwave/

**AI crawler index:** [llms.txt](../llms.txt) — canonical page list for LLMs

**Local verification:**

```bash
./scripts/quantwave_verify.sh
```

```python
import quantwave as qw
qw.assert_parity("rsi", {"period": 14}, your_closes)
```

→ [Benchmarks](../benchmarks.md)

## Where is the Rust API documented?

- **MkDocs guides (concepts):** [Rust getting started](../getting-started/rust.md) — mirrored guides coming in `guides/rust/`
- **Full API reference:** [docs.rs/quantwave-core](https://docs.rs/quantwave-core) (and sibling crates for polars, plugins, backtest)

Python users rarely need Rust docs unless embedding `quantwave-core` directly.

## What should I read first?

| Goal | Start here |
|------|------------|
| New user | [Getting Started funnel](../getting-started/index.md) |
| Picking a stack | [Comparison](../comparison.md) |
| One indicator fast | [Gallery](../guides/indicators/gallery.md) |
| Backtest a signal | [Backtest quickstart](../guides/backtest/quickstart.md) |
| Migrate from TA-Lib | [Python guide — TA-Lib section](../getting-started/python.md#ta-lib-migration) |