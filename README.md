# QuantWave

**High-performance, Polars-native technical analysis for Rust**

150+ indicators · Full Ehlers DSP suite · Regime Detection · Options India Analytics · Streaming & Batch Parity

**Python** `pip install quantwave` **Rust** `cargo add quantwave`

[📖 Documentation](https://lavs9.github.io/quantwave/) • [⭐ GitHub](https://github.com/lavs9/quantwave) • [![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/lavs9/quantwave)

**150+ indicators • Polars-native • Streaming & batch parity • MIT licensed**

---

## Why QuantWave?

Most quantitative libraries force you to choose between **performance** and **usability**.

- Python-only libraries are slow on large datasets and live streams.
- Pure Rust libraries are fast but painful to integrate into modern Polars-based research and production workflows.

QuantWave was built to eliminate this tradeoff.

We deliver **institutional-grade performance** through a Rust core with zero-copy Polars expressions, while providing a clean, productive experience in both Python and Rust. Every indicator is designed around a single source of truth — the `Next<T>` trait — ensuring that batch results (via Polars) and real-time streaming results are **bit-identical**.

## What We’ve Built

QuantWave is no longer early-stage. It ships with production-ready depth across several domains:

- **150+ Technical Indicators** with TA-Lib parity and extensive Ehlers DSP coverage
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

## Quickstart (Python)

```bash
pip install quantwave
```

```python
import polars as pl
from quantwave import ta

df = pl.read_parquet("ohlcv.parquet")

df = df.with_columns(
    ta.rsi("close", 14).alias("rsi"),
    ta.mama("close").alias("mama"),
    ta.supertrend("high", "low", "close", period=10, multiplier=3.0).alias("supertrend"),
)
```

[More examples → Documentation](https://lavs9.github.io/quantwave/examples/batch-streaming/)

## Get Started

- [Full Python Guide](https://lavs9.github.io/quantwave/getting-started/python/)
- [Rust Guide](https://lavs9.github.io/quantwave/getting-started/rust/)
- [v0.4.0 Release Notes](https://github.com/lavs9/quantwave/releases/tag/v0.4.0)
- [Ask DeepWiki](https://deepwiki.com/lavs9/quantwave)

---

**Made with ❤️ for the quant community.**
