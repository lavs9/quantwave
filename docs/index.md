<div class="qw-hero" markdown="1">

# QuantWave

<div class="qw-tagline">
High-performance, Polars-native quantitative finance — <strong>221 native indicators</strong>, full Ehlers DSP suite, regime detection, backtest engine, and bit-identical streaming parity.
</div>

[Get Started](getting-started/index.md){ .md-button .md-button--primary }
[Compare vs TA-Lib](comparison.md){ .md-button }
[Explore Indicators](guides/indicators/native/){ .md-button }

</div>

<div class="qw-stats" markdown="1">

<div class="qw-stat"><strong>221</strong><span>Native indicators</span></div>
<div class="qw-stat"><strong>30+</strong><span>Ehlers DSP tools</span></div>
<div class="qw-stat"><strong>1</strong><span>Mathematical truth (`Next&lt;T&gt;`)</span></div>
<div class="qw-stat"><strong>0</strong><span>Batch vs streaming drift</span></div>

</div>

## Why QuantWave?

Most quant stacks force a tradeoff: **Python convenience** or **Rust speed** — rarely both with **Polars-native** ergonomics and **research-to-production parity**.

QuantWave is built as a Rust workspace with a single source of mathematical truth. Every indicator implements `Next<T>`; Polars plugins and Python streaming wrappers consume the same logic, validated by gold-standard vectors and proptests.

| Approach | Large-data speed | Polars-native | Streaming parity | Ehlers + PA + regimes |
|----------|------------------|---------------|------------------|------------------------|
| pandas-ta / TA-Lib (Python) | Slow | Partial | Rare | Limited |
| Other Rust TA crates | Fast | Weak | Rare | Limited |
| **QuantWave** | **Fast** | **Native** | **Guaranteed** | **Deep** |

## What you get

<div class="qw-grid" markdown="1">

<div class="qw-card" markdown="1">

### Indicators
221 Rust-native indicators with metadata, gold-standard tests, and full docs. Classic TA, Ehlers DSP, candlestick patterns, price action, and fractional differencing.

[Browse catalog →](guides/indicators/native/)

</div>

<div class="qw-card" markdown="1">

### Polars `.ta()` + plugins
Zero-copy expression plugins for hot paths, or the ergonomic `.ta()` namespace for research. Same math either way.

[Plugin vs `.ta` →](guides/plugin_vs_ta.md)

</div>

<div class="qw-card" markdown="1">

### Backtest engine
Sweep, walk-forward, Monte Carlo, cross-sectional runs, and HTML tear sheets — Rust core with Python Polars integration.

[Backtest quickstart →](guides/backtest/quickstart.md)

</div>

<div class="qw-card" markdown="1">

### Python DX
`qw.indicators()`, `qw.metadata()`, `qw.assert_parity()`, `build_feature_matrix()`, and arm64 wheels.

[Python guide →](getting-started/python.md)

</div>

<div class="qw-card" markdown="1">

### Correctness
Gold-standard vectors, proptests, TA-Lib parity, and Python FFI parity — machine-counted on the validation page.

[Validation methodology →](validation.md)

</div>

</div>

## Quickstart

=== "Python"

```python
import polars as pl
import quantwave as qw

print(len(qw.indicators()), "indicators")
meta = qw.metadata("supertrend")

df = pl.DataFrame({"close": [100.0, 101.0, 102.0, 101.5, 103.0]})
out = (
    df.lazy()
    .with_columns(
        pl.col("close").ta.supertrend("high", "low", period=10, multiplier=3.0).alias("st")
    )
    .collect()
)
```

=== "Rust"

```rust
use quantwave_core::indicators::supertrend::SuperTrend;
use quantwave_core::Next;

let mut st = SuperTrend::new(10, 3.0);
let v = st.next((100.0, 105.0, 95.0, 102.0));
```

Install: `pip install "quantwave[polars]"` or `cargo add quantwave`.

## Performance snapshot

- **Memory**: 2–5× lower than pandas on multi-ticker workloads (measured — see [benchmarks](benchmarks.md))
- **Speed & latency**: reproducible harness in progress; we publish measured numbers only ([benchmarks](benchmarks.md))

[Full benchmarks →](benchmarks.md)

## Our mission

QuantWave exists because most quant stacks force a tradeoff: **Python convenience** or **Rust speed** — rarely both inside Polars with guaranteed batch ↔ streaming parity.

We built the **fastest, most complete Polars-native toolkit** in open source: 221 indicators, full Ehlers DSP, regime detection, options India helpers, and a research-grade backtest engine — one `Next<T>` implementation everywhere.

## Start here

**New users:** follow the [Getting Started funnel](getting-started/index.md) — install → first indicator → pick batch, streaming, or backtest.

**Evaluating stacks?** [QuantWave vs TA-Lib & pandas-ta](comparison.md)

<div class="qw-grid" markdown="1">

<div class="qw-card" markdown="1">

### 1 — Install & first indicator
[Getting Started hub](getting-started/index.md) → [Python](getting-started/python.md) or [Rust](getting-started/rust.md).

</div>

<div class="qw-card" markdown="1">

### 2 — Pick your path
[Indicators overview](guides/indicators/index.md) — learning paths for trend, PA, Ehlers, ML.

</div>

<div class="qw-card" markdown="1">

### 3 — Backtest a signal
[Backtest quickstart](guides/backtest/quickstart.md) → [Strategy notebook](examples/notebooks/strategy_backtest.md).

</div>

<div class="qw-card" markdown="1">

### 4 — Go deep
[Full catalog](guides/indicators/native/) · [Gallery](guides/indicators/gallery.md) · [API](api/)

</div>

</div>

## Explore

- [Getting Started funnel](getting-started/index.md) — install to first backtest in ~10 minutes
- [Comparison vs TA-Lib & pandas-ta](comparison.md) — migration and decision guide
- [FAQ](faq.md) — common questions (install, parity, backtest, vs TA-Lib)
- [llms.txt](llms.txt) — AI crawler index (canonical pages for LLMs)
- [Indicator gallery](guides/indicators/gallery.md) — curated high-value starting points
- [Ehlers DSP suite](guides/indicators/ehlers/index.md)
- [Backtest engine](guides/backtest/index.md)
- [ML features](guides/ml_features.md)
- [Notebooks](examples/notebooks/index.md)