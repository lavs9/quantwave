# QuantWave

**High-performance, Polars-native technical analysis & backtesting — in Python and Rust**

[![PyPI version](https://img.shields.io/pypi/v/quantwave?color=blue)](https://pypi.org/project/quantwave/)
[![Python versions](https://img.shields.io/pypi/pyversions/quantwave)](https://pypi.org/project/quantwave/)
[![Downloads](https://static.pepy.tech/badge/quantwave)](https://pepy.tech/project/quantwave)
[![CI](https://github.com/lavs9/quantwave/actions/workflows/ci.yml/badge.svg)](https://github.com/lavs9/quantwave/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/lavs9/quantwave/blob/main/LICENSE)

221 Native Indicators · Full Ehlers DSP suite · Regime Detection · Backtest engine · Bit-identical streaming & batch

**Python** `pip install quantwave` (or `pip install "quantwave[polars]"` for the Polars integration layer) **Rust** `cargo add quantwave`

[📖 Documentation](https://lavs9.github.io/quantwave/) • [📦 PyPI](https://pypi.org/project/quantwave/) • [⭐ GitHub](https://github.com/lavs9/quantwave) • [![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/lavs9/quantwave)

**221 indicators • Polars-native • Streaming & batch parity • MIT licensed**

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

- **221 Native Indicators** with gold-standard validation and extensive Ehlers DSP coverage — all 221 are implemented in QuantWave's own Rust, including all 61 candlestick patterns. No C TA-Lib, and no third-party TA crate in the shipped dependency graph; `talib-rs` is a test-only parity oracle.
- **Full Regime Detection Suite** (HMM, GMM, PELT, clustering, conditioned risk metrics)
- **Execution-Aware Backtest Engine** — first-class order types (market/limit/stop/stop-limit + bracket/OCO), risk overlays (vol-target, inverse-vol, position-limit), portfolio rebalance policies, walk-forward optimization (grid + Bayesian TPE), Monte Carlo, and benchmark-relative reporting (alpha/beta/Calmar/VaR/CVaR) — all via the `.bt` Polars namespace
- **Complete Options India Stack** — Black-Scholes Greeks, IV solvers, chain analytics (Max Pain, PCR, GEX, OI Zones), and NSE utilities, all exposed as native Polars expressions
- **Streaming & Batch Parity** — The same mathematical logic powers both high-speed Polars pipelines and low-latency streaming via the universal `Next<T>` trait — *including the backtester*, so a strategy backtests and trades from one codebase
- **Gold-Standard Validation** — Every indicator is tested against reference implementations for correctness

## Core Strengths

- **Performance** — Rust core with zero-copy Polars expressions
- **Correctness** — Validated against gold-standard reference vectors
- **Parity** — Bit-identical results between batch and streaming
- **Breadth** — Classic indicators + advanced Ehlers DSP + regime detection + Options India
- **Developer Experience** — Clean Python API (`from quantwave import ta`) and idiomatic Rust

## Real-World Performance

- **Memory footprint** on realistic multi-ticker data: **2–5× lower** than Pandas (measured — see benchmarks)
- **Speed & latency**: published only from the reproducible harness in `benchmarks/`; earlier unmeasured throughput figures have been removed

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

### Backtest a strategy (`.bt`)

```python
import polars as pl
import quantwave  # registers pl.col().ta and LazyFrame.bt

df = pl.read_parquet("ohlcv.parquet").lazy().with_columns(
    (pl.col("close").ta.rsi(timeperiod=14) < 30).cast(pl.Float64).alias("signal")
)

report = df.bt.backtest_with_report(commission_bps=5.0, slippage_bps=2.0)
print(report.metrics())                      # Sharpe, Sortino, max DD, CAGR, win rate…
print(report.extended_metrics())             # Calmar, VaR-95, CVaR-95
report.save_html("tearsheet.html")           # self-contained tear sheet
```

The same engine runs order-driven fills (`.bt.order_backtest`), risk overlays (`risk_model=`), and multi-symbol portfolios (`.bt.portfolio_backtest`) — with batch results guaranteed bit-identical to streaming.

[More examples → Documentation](https://lavs9.github.io/quantwave/examples/batch-streaming/)

## Get Started

**Primary paths**
- [Getting Started funnel](https://lavs9.github.io/quantwave/getting-started/)
- [Get Started with Python](https://lavs9.github.io/quantwave/getting-started/python/)
- [vs TA-Lib & pandas-ta](https://lavs9.github.io/quantwave/comparison/)

**Explore further**
- [Browse All Indicators](https://lavs9.github.io/quantwave/guides/indicators/)
- [See Real Benchmarks](https://lavs9.github.io/quantwave/benchmarks/)
- [Agent Skill](https://lavs9.github.io/quantwave/guides/agent-skill/) — teach your coding agent QuantWave's conventions (and its silent footguns)
- [llms.txt](https://lavs9.github.io/quantwave/llms.txt) (AI crawler index)
- [Latest Release Notes](https://github.com/lavs9/quantwave/releases/latest)
- [Ask DeepWiki](https://deepwiki.com/lavs9/quantwave)

---

**Made with ❤️ for the quant community.**
