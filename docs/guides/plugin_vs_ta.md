# Plugin vs `.ta` — When to Use Which !!! tip "Short answer" **Same math** (`Next<T>`). Use **`pl.col().ta.*`** for expression-plugin throughput; use **`lf.ta().*`** for LazyFrame research chains; use **streaming** for live bars. QuantWave exposes the same indicator math through two Polars integration paths. Both delegate to `` (`Next<T>`); results are bit-identical when inputs and parameters match. --- ## Quick decision tree ```text
Need LazyFrame research ergonomics on many columns? └─ Yes → s: df.lazy().ta().rsi(14) (import via `use quantwave_polars::QuantWaveNamespace` in Rust, or `pip install "quantwave[polars]"` in Python) Need maximum vectorized throughput / expression-plugin zero-copy? └─ Yes → ns: pl.col("close").ta.rsi(14) (Polars Expression Plugins; separate wheel / workspace crate) Live streaming / bar-by-bar? └─ Next<T> or Python streaming_class + wrap_streaming (not plugins — stateful per-row)
``` --- ## `s` — `.ta()` on LazyFrame **Best for:** research notebooks, feature engineering pipelines, backtest prep, anything that already uses `LazyFrame`. ```python
import polars as pl df = ( pl.read_parquet("ohlcv.parquet") .lazy() .ta() .rsi("close", 14) .ta() .supertrend("high", "low", "close", 10, 3.0)
)
``` | Pros | Cons |
|------|------|
| Rich namespace: PA, regimes, `.ta.features.*`, backtest hooks | Slightly more allocation than raw plugins on huge frames |
| Natural fit with `.bt` backtest namespace | Requires `s` / `[polars]` extra |
| ~205 indicator methods + ML features | | **Rust:** `s` crate — `lf.ta().method(...)`. --- ## `ns` — expression plugins **Best for:** hot paths, embedding indicators inside complex `with_columns` expressions, plugin-native Polars 1.x workflows. ```python
import polars as pl
import quantwave_plugins # registers expression plugins df = df.with_columns( pl.col("close").ta.rsi(14).alias("rsi"), pl.col("close").ta.ema(20).alias("ema"),
)
``` | Pros | Cons |
|------|------|
| Zero-copy Arrow buffers where possible | Separate install path (`ns` wheel) |
| ~219 `.ta` methods — full parity with lazy map path | No `.ta.features` / PA struct helpers on this surface yet |
| Ideal inside `select`/`with_columns` expr trees | | **When plugins win:** repeated indicator columns on 10M+ row frames, tight integration with other Polars expr plugins. **When `.ta()` wins:** multi-output structs (CyberCycle, S/R monitor), regime + feature joins, strategy notebooks. --- ## Parity guarantee Both paths implement the same `Next<T>` core. Verify with: ```python
import quantwave as qw
qw.assert_parity("rsi", {"period": 14}, closes)
``` CI (`.github/workflows/ci.yml`) runs `scripts/quantwave_verify.sh` on every push/PR to `main`; docs deploy and plugin wheels are jobs in the same workflow. See `.github/workflows/README.md`. --- ## Sources - `s/src/lib.rs` — LazyFrame `.ta()` namespace
- `ns/` — expression plugin implementations
- Full plugin ↔ `.ta` parity