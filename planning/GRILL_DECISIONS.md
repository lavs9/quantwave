# QuantWave Project Planning - Grill Session

## Project Overview
**QuantWave** is a modern, Polars-native Rust technical analysis library designed to extend `talib-rs-core`. It aims to provide:
- High-performance indicator calculations using Polars expressions.
- A streaming API for real-time data processing.
- A comprehensive roadmap covering MVP, Ehlers DSP, Volume, and ML features.

## Decisions Log
1. **Core Architecture:** Hybrid + Plugin Integration.
    - Leverage `talib-rs-core` for classic indicators (pure Rust, SIMD, incremental).
    - Implement "Top 10" and custom indicators as **Polars Expression Plugins** (UDFs) for native performance and zero-copy.
2. **Integration Strategy:** Use `polars-ta` as a reference for "Golden Tests" but maintain QuantWave as a separate high-level crate focused on **Extensible Generics** and **Streaming-Batch Parity**.
3. **Parity Strategy:** "Universal Indicator" Trait. Every indicator must implement a trait that defines both the recursive logic (for Streaming) and the vectorized loop (for Plugins), enforced via `proptest`.
4. **Validation Strategy:** "Gold Standard" JSON Metadata + Cross-validation against `polars-ta` and C TA-Lib.
5. **Implementation Philosophy:** Depth over Breadth. Prioritize generic components (e.g., swappable MAs) and **Polars Namespace** ergonomics (`df.select([pl.col("close").ta.supertrend(10, 3)])`).

---

## Discussion Threads

### Round 1: Core Architecture & Dependency Management (RESOLVED)
- **Outcome:** Use Polars Plugins instead of simple `map_batches`. Build on `talib-rs-core` for classic coverage.

---

### Round 2: Streaming vs. Batch Parity (RESOLVED)
- **Outcome:** Use a unified trait-based state machine that powers both the streaming struct and the Polars plugin.

---

### Round 3: Data Integrity & Validation (RESOLVED)
- **Outcome:** JSON vectors + industry cross-benchmarking.

---
