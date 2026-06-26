# Plugin vs `.ta` — When to Use Which

QuantWave exposes the same indicator math through two Polars integration paths. Both delegate to `quantwave-core` (`Next<T>`); results are bit-identical when inputs and parameters match.

---

## Quick decision tree

```text
Need LazyFrame research ergonomics on many columns?
  └─ Yes → quantwave-polars: df.lazy().ta().rsi(14)
         (import via `use quantwave_polars::QuantWaveNamespace` in Rust,
          or `pip install "quantwave[polars]"` in Python)

Need maximum vectorized throughput / expression-plugin zero-copy?
  └─ Yes → quantwave-plugins: pl.col("close").ta.rsi(14)
         (Polars Expression Plugins; separate wheel / workspace crate)

Live streaming / bar-by-bar?
  └─ quantwave-core Next<T> or Python streaming_class + wrap_streaming
     (not plugins — stateful per-row)
```

---

## `quantwave-polars` — `.ta()` on LazyFrame

**Best for:** research notebooks, feature engineering pipelines, backtest prep, anything that already uses `LazyFrame`.

```python
import polars as pl

df = (
    pl.read_parquet("ohlcv.parquet")
    .lazy()
    .ta()
    .rsi("close", 14)
    .ta()
    .supertrend("high", "low", "close", 10, 3.0)
)
```

| Pros | Cons |
|------|------|
| Rich namespace: PA, regimes, `.ta.features.*`, backtest hooks | Slightly more allocation than raw plugins on huge frames |
| Natural fit with `.bt` backtest namespace | Requires `quantwave-polars` / `[polars]` extra |
| ~205 indicator methods + ML features | |

**Rust:** `quantwave-polars` crate — `lf.ta().method(...)`.

---

## `quantwave-plugins` — expression plugins

**Best for:** hot paths, embedding indicators inside complex `with_columns` expressions, plugin-native Polars 1.x workflows.

```python
import polars as pl
import quantwave_plugins  # registers expression plugins

df = df.with_columns(
    pl.col("close").ta.rsi(14).alias("rsi"),
    pl.col("close").ta.ema(20).alias("ema"),
)
```

| Pros | Cons |
|------|------|
| Zero-copy Arrow buffers where possible | Separate install path (`quantwave-plugins` wheel) |
| ~219 `.ta` methods — full parity with lazy map path | No `.ta.features` / PA struct helpers on this surface yet |
| Ideal inside `select`/`with_columns` expr trees | |

**When plugins win:** repeated indicator columns on 10M+ row frames, tight integration with other Polars expr plugins.

**When `.ta()` wins:** multi-output structs (CyberCycle, S/R monitor), regime + feature joins, strategy notebooks.

---

## Parity guarantee

Both paths implement the same `Next<T>` core. Verify with:

```python
import quantwave as qw
qw.assert_parity("rsi", {"period": 14}, closes)
```

CI runs `scripts/quantwave_verify.sh` (nextest + pytest) on every push to `main`.

---

## Sources

- `quantwave-polars/src/lib.rs` — LazyFrame `.ta()` namespace
- `quantwave-plugins/` — expression plugin implementations
- Closed epic `quantwave-3f7g` — full plugin ↔ `.ta` parity