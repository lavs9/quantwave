# QuantWave

**High-performance, Polars-native technical analysis — in Python and Rust**

150+ indicators · Full Ehlers DSP suite · Regime Detection · Complete Options India stack · Bit-identical streaming & batch

**Python**: `pip install quantwave`  
**Rust**: `cargo add quantwave`

[Get Started (Python) →](./getting-started/python.md){ .md-button .md-button--primary }
[Get Started (Rust) →](./getting-started/rust.md){ .md-button }
[Explore Indicators →](./guides/indicators/){ .md-button }

---

## Why QuantWave?

Most quantitative libraries force an uncomfortable compromise.

**Python-first libraries** (pandas-ta, TA-Lib Python wrappers, etc.) are convenient but fall apart on large datasets, recursive indicators, or live streaming — often becoming 10-100× slower than native code.

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

QuantWave now ships with meaningful depth across several domains:

- **150+ Technical Indicators** with strong TA-Lib parity and one of the most complete open-source Ehlers Digital Signal Processing suites available.
- **Full Regime Detection Suite** — HMM, GMM, PELT change-point detection, volatility clustering, and conditioned risk metrics.
- **Complete Options India Stack** — Black-Scholes Greeks, implied volatility solvers, and advanced chain analytics (Max Pain, PCR, GEX, OI Zones), all exposed as native Polars expressions.
- **Streaming & Batch Parity** — The same mathematical logic powers both high-speed Polars pipelines and low-latency streaming.

## Real-World Performance

We don’t just claim to be fast. On 1 million rows of realistic OHLCV data:

- **SuperTrend**: 7.4 ms (QuantWave) vs >200 ms (Pandas) → **~27× faster**
- **CyberCycle** (Ehlers): 5.0 ms vs >500 ms (Pandas) → **~100× faster**
- **Memory footprint** on realistic multi-ticker data: **2–5× lower** than Pandas

→ [Full benchmarks](https://lavs9.github.io/quantwave/benchmarks/)

## Explore the Indicators

- [All Indicators →](./guides/indicators/)
- [Ehlers DSP Suite →](./guides/indicators/ehlers/)
- [Regime Detection →](./guides/indicators/regimes/)
- [Options India →](./guides/options_india/)

---

**Primary paths**
- [Get Started with Python](getting-started/python.md)
- [Get Started with Rust](getting-started/rust.md)

**Explore further**
- [Browse All Indicators](guides/indicators/)
- [See Real Benchmarks](benchmarks/)
- [Learn Our Purpose](purpose.md)

---

**Made with ❤️ for the quant community**
