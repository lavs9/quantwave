# QuantWave

**High-performance, Polars-native technical analysis — in Python and Rust**

218 native indicators · Full Ehlers DSP suite · Regime Detection · Backtest engine · Bit-identical streaming & batch

**Python** `pip install quantwave` (or `pip install "quantwave[polars]"` for the Polars integration layer) **Rust** `cargo add quantwave`

[📖 Documentation](https://lavs9.github.io/quantwave/) • [⭐ GitHub](https://github.com/lavs9/quantwave) • [![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/lavs9/quantwave)

**218 indicators • Polars-native • Streaming & batch parity • MIT licensed**

---

## Why QuantWave?

Most quantitative libraries force an uncomfortable compromise.

**Python-first libraries** (pandas-ta, TA-Lib Python wrappers, etc.) are convenient but fall apart on large datasets, recursive indicators, or live streaming — often becoming 10-100x slower than native code.

**Pure Rust libraries** are fast, but they rarely integrate cleanly with modern Polars-based research pipelines and lack the breadth of advanced techniques (Ehlers DSP, regime detection, full Options India analytics).

**QuantWave removes the tradeoff.**

It delivers **institutional-grade Rust performance** through zero-copy Polars expressions, while offering a first-class, productive experience in both Python and Rust. Every indicator is built on a single mathematical source of truth — the `Next<T>` trait — guaranteeing that batch results (Polars) and real-time streaming results are **bit-identical**.

### How We Compare

| Approach                  | Speed on large data | Polars-native | Streaming parity | Breadth (Ehlers + Regimes + Options) |
|---------------------------|---------------------|---------------|------------------|--------------------------------------|
| pandas-ta / TA-Lib (Python) | Poor–Average       | Partial      | Rare             | Limited                              |
| Other Rust TA crates      | Excellent           | Poor         | Rare             | Limited                              |
| **QuantWave**             | **Excellent**       | **Native**   | **Guaranteed**   | **Strong**                           |

## What We’ve Built

QuantWave is no longer early-stage. It ships with production-ready depth across several domains:

- **218 Native Indicators** with gold-standard validation and extensive Ehlers DSP coverage
- **Full Regime Detection Suite** (HMM, GMM, PELT, clustering, conditioned risk metrics)
- **Complete Options India Stack** — Black-Scholes Greeks, IV solvers, chain analytics (Max Pain, PCR, GEX, OI Zones), and NSE utilities, all exposed as native Polars expressions
- **Streaming & Batch Parity** — The same mathematical logic powers both high-speed Polars pipelines and low-latency streaming via the universal `Next<T>` trait
- **Gold-Standard Validation** — Every indicator is tested against reference implementations for correctness

## Core Strengths

- **Performance** — Rust core with zero-copy Polars expressions
- **Correctness** — Validated against gold-standard reference vectors
- **Parity** — Bit-identical results between batch and streaming
- **Breadth** — Classic indicators + advanced Ehlers DSP + regime detection + Options India
- **Developer Experience** — Clean Python API (`from quantwave import ta`) and idiomatic Rust

## Real-World Performance

We don’t just claim to be fast — here’s what the numbers show on 1 million rows of realistic OHLCV data:

- **SuperTrend**: 7.4 ms (QuantWave) vs >200 ms (Pandas) → **~27× faster**
- **CyberCycle** (Ehlers): 5.0 ms vs >500 ms (Pandas) → **~100× faster**
- **Instantaneous Trendline**: 74 ms vs >2,000 ms (Pandas) → **~27× faster**
- **Memory footprint** on realistic multi-ticker data: **2–5× lower** than Pandas

Complex recursive indicators (the ones that matter most for real strategies) are where the gap becomes dramatic.

→ [Full benchmarks & methodology](https://lavs9.github.io/quantwave/benchmarks/)

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

[More examples → Documentation](https://lavs9.github.io/quantwave/examples/batch-streaming/)

## Get Started

**Primary paths**
- [Getting Started funnel](https://lavs9.github.io/quantwave/getting-started/)
- [Get Started with Python](https://lavs9.github.io/quantwave/getting-started/python/)
- [vs TA-Lib & pandas-ta](https://lavs9.github.io/quantwave/comparison/)

**Explore further**
- [Browse All Indicators](https://lavs9.github.io/quantwave/guides/indicators/)
- [See Real Benchmarks](https://lavs9.github.io/quantwave/benchmarks/)
- [llms.txt](https://lavs9.github.io/quantwave/llms.txt) (AI crawler index)
- [v0.4.0 Release Notes](https://github.com/lavs9/quantwave/releases/tag/v0.4.0)
- [Ask DeepWiki](https://deepwiki.com/lavs9/quantwave)

---

**Made with ❤️ for the quant community.**
