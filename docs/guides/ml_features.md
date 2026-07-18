# ML Feature Engineering

QuantWave ships production-ready ML feature extractors on top of the Ehlers DSP
and regime-detection stack. Features follow the same **Universal Indicator**
pattern as classic TA: batch (Polars) and streaming (`Next<T>`) paths are
bit-identical.

---

## Quick start (Polars)

```python
import polars as pl

df = pl.read_parquet("ohlcv.parquet").lazy()

# Ehlers feature columns (struct-friendly outputs)
df = df.with_columns(
    pl.col("close").ta.cybercycle(30).alias("cybercycle"),
    pl.col("close").ta.trendflex(30).alias("trendflex"),
    pl.col("close").ta.instantaneous_trendline(30).alias("itl"),
    pl.col("close").ta.hurst_exponent(100).alias("hurst"),
)
```

See the [Indicator Gallery](indicators/gallery.md) for Ehlers DSP and regime
entries, and per-indicator guides under **ML Features** in the native index.

---

## Discovery & metadata

```python
import quantwave as qw

# Browse by category
for cat in qw.categories():
    if "ehlers" in cat.lower() or "regime" in cat.lower() or "ml" in cat.lower():
        print(cat, qw.category(cat)[:5], "...")

meta = qw.metadata("hurst_exponent")
print(meta.data_inputs, meta.warmup_bars, meta.category)

info = qw.boundary_info("cybercycle")
print(info.warmup_behavior)
```

---

## Streaming (live features)

```python
import quantwave as qw

cls = qw.streaming_class("rsi")  # classic example; same pattern for DSP features
wrapped = qw.wrap_streaming(cls(14), name="rsi")

for price in live_closes:
    val = wrapped.next(price)
    if wrapped.is_ready:
        ...  # feed to model / strategy
```

Use `qw.track_streaming(inst, name="hurst_exponent")` for warmup-aware readiness.

---

## Feature matrix for ML pipelines

### Python (recommended)

```python
import quantwave as qw

df = qw.build_feature_matrix(ohlcv_df, features="recommended")
# Or drop warmup rows before training:
df = qw.build_feature_matrix(ohlcv_df, drop_warmup=True, warmup_bars=100)
```

### Polars (Rust)

```python
# lf.ta().features().recommended_matrix()  # via quantwave-polars
```

Build a wide matrix without lookahead:

1. Use `qw.build_feature_matrix()` or `.ta.features.recommended_matrix()`.
2. Call `qw.warmup_bars(name, params)` per column and mask or drop leading rows.
3. Join regime labels (`market_state`, HMM outputs) on the same bar index.
4. Assert batch/streaming parity in CI with `qw.assert_parity(...)`.

---

## End-to-end: features → backtest

The canonical cross-epic notebook wires Hurst, CyberCycle, dominant cycle, and
regime features into a realistic strategy with **rich trade metadata** and
verified batch/streaming parity:

- [ML Features → Backtest (E2E)](../examples/notebooks/ml_feature_backtest_parity.md)
- Source: `docs/examples/notebooks/ml_feature_backtest_parity.py`

For backtest engine basics, see [Backtest Quickstart](backtest/quickstart.md).

Need the whole 200+ indicator surface as feature columns, not just the curated
Ehlers/regime preset? See [`df.ta.all()`](talib_migration.md#bulk-compute-dftaall)
for the bulk, registry-driven equivalent.

---

## Sources

- Ehlers DSP papers in `references/Ehlers Papers/implemented/`
- Regime modules: `quantwave-core/src/regimes/`
- Feature Polars surface: `quantwave-polars/src/features.rs`
- Stability validation: `docs/examples/notebooks/ml_feature_stability.md`