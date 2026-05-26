# QuantWave

**High-performance, Polars-native technical analysis for Rust**

150+ indicators · Full Ehlers DSP suite · Regime Detection · Options India Analytics · Streaming & Batch Parity

**Python**: `pip install quantwave`  
**Rust**: `cargo add quantwave`

[Get Started →](./getting-started/python.md){ .md-button .md-button--primary }
[Explore Indicators →](./guides/indicators/){ .md-button }
[Learn Our Purpose →](./purpose.md){ .md-button }

---

## Why QuantWave?

Most quantitative libraries force you to choose between **performance** and **usability**.

Python-only libraries are slow on large datasets and live streams. Pure Rust libraries are fast but painful to integrate into modern Polars-based research and production workflows.

QuantWave was built to remove this tradeoff.

We deliver **institutional-grade performance** through a Rust core with zero-copy Polars expressions, while offering a clean, productive experience in both Python and Rust. Every indicator is designed around a single source of truth — the `Next<T>` trait — which guarantees that batch results (via Polars) and real-time streaming results are **bit-identical**.

## What We’ve Built

QuantWave now ships with meaningful depth across several domains:

- **150+ Technical Indicators** with strong TA-Lib parity and one of the most complete open-source Ehlers Digital Signal Processing suites available.
- **Full Regime Detection Suite** — HMM, GMM, PELT change-point detection, volatility clustering, and conditioned risk metrics.
- **Complete Options India Stack** — Black-Scholes Greeks, implied volatility solvers, and advanced chain analytics (Max Pain, PCR, GEX, OI Zones), all exposed as native Polars expressions.
- **Streaming & Batch Parity** — The same mathematical logic powers both high-speed Polars pipelines and low-latency streaming.

## Explore the Indicators

- [All Indicators →](./guides/indicators/)
- [Ehlers DSP Suite →](./guides/indicators/ehlers/)
- [Regime Detection →](./guides/indicators/regimes/)
- [Options India →](./guides/options_india/)

---

**Made with ❤️ for the quant community**
