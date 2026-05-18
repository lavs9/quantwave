# Indicator Suite

The QuantWave indicator suite is divided into two primary categories to give you maximum flexibility and coverage:

- **Native Indicators**: Highly optimized, modern indicators implemented natively in Rust. These include modern DSP suites, order flow tools, and advanced moving averages.
- **TA-Lib Wrappers**: A comprehensive suite of 158 classic indicators wrapping the battle-tested `ta-lib` C library.

Every single indicator, regardless of its category, supports both live streaming (`Next` trait) and batch Polars processing (`.ta()` namespace).
