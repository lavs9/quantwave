# QuantWave

**High-performance quantitative finance library**  
Built in Rust · Native Polars support · 218 indicators · Full Ehlers DSP suite · Backtest `.bt`

**Python** `pip install quantwave` (or `pip install "quantwave[polars]"` for Polars layer)  
**Rust** `cargo add quantwave`

[📖 Full Documentation](https://lavs9.github.io/quantwave/)  
[⭐ GitHub](https://github.com/lavs9/quantwave)

---

## Purpose of Our Work

Most quant libraries force you to choose between **speed** and **ease of use**.  
We built QuantWave to give you both.

- **218 native indicators** with gold-standard validation
- **Complete Ehlers Digital Signal Processing suite** (the most advanced open-source cycle tools)  
- **Zero-copy Polars expressions** that run at Rust speed  
- **Seamless batch + streaming modes**  
- **Future-proof architecture** (Options Greeks, risk metrics, etc. coming soon)

**One library. Research to production. No compromises.**

---

## Quickstart (Python)

```bash
pip install "quantwave[polars]"
quantwave doctor
```

```python
import polars as pl
import quantwave  # registers pl.col().ta and LazyFrame.bt

df = pl.read_parquet("ohlcv.parquet")

df = df.lazy().with_columns(
    pl.col("close").ta.rsi(timeperiod=14).alias("rsi"),
    pl.col("close").ta.ema(period=20).alias("ema"),
).collect()
```

[Full examples → Documentation](https://lavs9.github.io/quantwave/examples/batch-streaming/)

## Features

- **Lightning fast** – Rust core with Polars native expressions.
- **Battle-tested** – Every indicator validated against reference implementations.
- **Modern** – Works perfectly in Jupyter, scripts, and live trading systems.
- **MIT licensed** – Free for commercial and personal use.

## Next Steps

- [Python Guide](https://lavs9.github.io/quantwave/getting-started/python/)
- [Rust Guide](https://lavs9.github.io/quantwave/getting-started/rust/)
- [Options Greeks & Pricing (roadmap)](https://lavs9.github.io/quantwave/purpose/)

## CLI

```bash
quantwave list                  # all indicators by category
quantwave info rsi              # metadata for one indicator
quantwave doctor                # verify core, polars, plugins, backtest
quantwave version
```

## Python 0.6.0 packaging

- **Unified PyPI wheel** — core + `quantwave_plugins` + `quantwave._backtest` in one install
- **`quantwave doctor`** — environment diagnostics for production pipelines
- **`pip install "quantwave[polars]"`** — adds Polars for batch `.ta` and `.bt` namespaces

## Python 0.5.2 DX Improvements (quantwave-p3z9)

We have made major improvements to the Python developer experience:

**New recommended APIs:**
- `quantwave.indicators()` – discover available indicators
- `quantwave.metadata("rsi")` – rich per-indicator metadata (params, data inputs, warmup, category, etc.)
- `quantwave.assert_parity(...)` – verify batch vs streaming bit-identical behavior
- `quantwave.streaming_class("rsi")` + `wrap_streaming(...)` – better streaming ergonomics with readiness tracking

**Reduced namespace pollution:**
- Result dataclasses and Options India helpers are now available under `quantwave.results` and `quantwave.options`.
- Old top-level access still works but emits `DeprecationWarning`.

**Other:**
- `quantwave.__version__` is now properly exposed.
- `quantwave.categories()` / `category(name)` for programmatic discovery.
- `quantwave.boundary_info(name)` for warmup and error semantics.
- `quantwave.talib` submodule with `list_functions()` for TA-Lib migration.
- Structured errors: `QuantwaveError`, `ParityError`, `IndicatorNotFoundError`, etc.
- Linux arm64 wheels are available.

**Documentation:** [https://lavs9.github.io/quantwave/](https://lavs9.github.io/quantwave/) — ML Features guide, Backtest quickstart, and API reference.

See the full list of changes in the main [changelog](https://github.com/lavs9/quantwave/blob/main/docs/changelog.md).

Made with ❤️ for the quant community.
