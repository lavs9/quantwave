# QuantWave (Python)

**Polars-native quantitative finance** — 221 Rust indicators, Ehlers DSP, price action, regimes, and a **built-in backtest engine** with batch ↔ streaming parity.

```bash
pip install "quantwave[polars]"
quantwave doctor
```

[Documentation](https://lavs9.github.io/quantwave/) · [vs TA-Lib & pandas-ta](https://lavs9.github.io/quantwave/comparison/) · [FAQ](https://lavs9.github.io/quantwave/faq/) · [llms.txt](https://lavs9.github.io/quantwave/llms.txt)

---

## Install

```bash
pip install "quantwave[polars]"
```

The wheel bundles `quantwave._quantwave` (indicators), `quantwave_plugins` (expression `.ta`), and `quantwave._backtest`. The `[polars]` extra adds Polars for `.ta` / `.bt` namespaces.

---

## Example 1 — Polars batch (`.ta`)

```python
import polars as pl
import quantwave  # registers pl.col().ta and LazyFrame.bt

df = pl.DataFrame({
    "high":  [101.0, 102.0, 103.0, 102.0, 104.0],
    "low":   [99.0, 100.0, 101.0, 100.0, 102.0],
    "close": [100.0, 101.0, 102.0, 101.0, 103.0],
})

out = (
    df.lazy()
    .with_columns(
        pl.col("close").ta.rsi(timeperiod=14).alias("rsi"),
        pl.col("close").ta.supertrend("high", "low", period=10, multiplier=3.0).alias("st"),
    )
    .collect()
)
print(out.tail())
```

---

## Example 2 — Live streaming (parity with batch)

```python
import quantwave as qw

cls = qw.streaming_class("rsi")
rsi = qw.wrap_streaming(cls(period=14), name="rsi")

for price in closes:
    val = rsi.next(price)
    if rsi.is_ready:
        print(price, val)

# Optional: qw.assert_parity("rsi", {"period": 14}, closes)
```

---

## Example 3 — Backtest (`.bt` in the same package)

```python
import polars as pl
import quantwave

bars = pl.DataFrame({
    "timestamp": range(20),
    "close": [100 + i * 0.5 for i in range(20)],
    "signal": [0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, -1.0, -1.0, 0.0] * 2,
})

report = (
    bars.lazy()
    .bt.backtest_with_report(signal="signal", close_col="close", timestamp_col="timestamp")
)
print(report.metrics())
```

See `examples/03_backtest_quick.py` in this directory.

---

## Architecture

```text
pip install quantwave[polars]
        │
        ├── quantwave._quantwave   # Rust indicators (Next<T> core)
        ├── quantwave_plugins    # pl.col("close").ta.* expression plugins
        ├── quantwave._backtest  # Rust simulation engine
        │
        ├── import quantwave
        │     ├── pl.col().ta.*        batch indicators (plugins)
        │     ├── df.lazy().bt.*       backtest namespace (bt_polars)
        │     ├── qw.streaming_class   live bar-by-bar
        │     ├── qw.metadata / assert_parity / CLI
        │     └── quantwave.backtest   BacktestEngine direct API
        │
        └── Docs: lavs9.github.io/quantwave/api/ (mkdocstrings)
```

One Rust math path powers batch, streaming, and backtest signal evaluation — validated by gold-standard tests.

---

## Discovery & CLI

```python
import quantwave as qw

print(len(qw.indicators()), "names")
meta = qw.metadata("rsi")
print(meta.warmup_bars, meta.category)
```

```bash
quantwave list --category "Classic"
quantwave info supertrend
quantwave doctor
```

---

## Examples in this repo

| Script | What it shows |
|--------|----------------|
| `examples/01_rsi_polars.py` | Polars `.ta` batch |
| `examples/02_supertrend_streaming.py` | RSI streaming + readiness |
| `examples/03_backtest_quick.py` | `.bt.backtest_with_report` |

---

## Learn more

- [Getting Started funnel](https://lavs9.github.io/quantwave/getting-started/)
- [Python guide](https://lavs9.github.io/quantwave/getting-started/python/)
- [Backtest quickstart](https://lavs9.github.io/quantwave/guides/backtest/quickstart/)
- [Indicator catalog](https://lavs9.github.io/quantwave/guides/indicators/native/)
- [Changelog](https://github.com/lavs9/quantwave/blob/main/docs/changelog.md)

MIT licensed.