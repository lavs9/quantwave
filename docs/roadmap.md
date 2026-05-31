# Roadmap

QuantWave is evolving rapidly. Our goal is to provide the most comprehensive and high-performance technical analysis library for the Polars ecosystem.

## Current Status (v0.4.0)

- Regime Detection Suite (v0.2–v0.3): HMM, GMM, PELT change-point detection, volatility clustering, conditioned risk metrics — all with native Polars support
- Options India Suite (v0.4.0): Full Black-Scholes Greeks, implied volatility solvers, chain analytics (Max Pain, PCR, GEX, OI Zones), NSE utilities, and Polars expressions
- 150+ technical indicators, including the complete Ehlers Digital Signal Processing suite
- High-performance Polars expressions + Python bindings (`pip install quantwave`)
- Streaming and batch parity via the universal `Next<T>` trait
- Extensive documentation (200+ indicator guides)
- Gold-standard testing and batch/streaming parity validation

## In Progress

- 2D Kinematic Kalman Filter (position + velocity state for lower lag)
- Expanded native Polars expression plugin coverage
- Price Action Structure Foundation (MQL5 lynnchris Part 21 port): adaptive swings + confirmed BOS flips (market_structure.rs done).
- Geometric PA library (MQL5 Parts 66+69): Flags + H&S detectors (geometric_patterns.rs) now built on the foundation + rich structs from research; initial streaming + parity in place (ej8b child).
- Polars + Python surface + canonical notebook (5thj): .ta.market_structure() / .ta.geometric_patterns() (rich Structs with bias/flip/pole_atr), full Python streaming wrappers + batch helpers, runnable notebook demonstrating Flag breakout strategy on confirmed bullish MS + regime + ML filter + pole sizing + backtester sketch. (Delivered 2026-05-30 IST).

## Next Priorities

- GitHub Releases, topics, README hero, and overall visibility (this P0 epic)
- Example quality, streaming documentation, and visual gallery strategy
- Stronger positioning and competitive differentiation

## Future Horizons

- Machine Learning feature engineering toolkit
- Portfolio-level vectorized backtesting engine
- WASM / browser runtime support
- Additional data provider integrations and real-time bridges
